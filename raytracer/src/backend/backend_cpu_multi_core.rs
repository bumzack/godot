use std::error::Error;
use std::time::Instant;


use cpu_kernel_raytracer::CpuKernel;
use raytracer_lib_no_std::camera::{Camera, CameraOps};
use raytracer_lib_no_std::{Color, ColorOps, Light, Ray, Shape, Pixel, BLACK};
use raytracer_lib_std::{Canvas, CanvasOps, World, WorldOps};

use raytracer_lib_no_std::MAX_REFLECTION_RECURSION_DEPTH;

use crate::{BackendOps, get_antialiasing_params};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

pub struct BackendCpuMultiCore {}

impl BackendOps for BackendCpuMultiCore {
    fn render_world(&self, world: &mut World, c: &Camera) -> Result<Canvas, Box<dyn Error>> {
        let start = Instant::now();
        let canvas = render_world_multi_core(world, c, CpuKernel::color_at);
        let stopped = Instant::now();
        println!("cpu multicore       duration  {:?}  \n ", stopped.duration_since(start));
        Ok(canvas)
    }
}

pub fn render_world_multi_core<F:Sync+ Send>(world: &mut World, c: &Camera,  f: F) -> Canvas
where
    F: Fn(&Vec<Shape>, &Vec<Light>, &Ray, i32, bool, bool, bool, bool) -> Color,
{
    let (n_samples, jitter_matrix) = get_antialiasing_params(c);

    let mut canvas = Canvas::new(c.get_hsize(), c.get_vsize());
    // TODO: remove, when WOrld has lights vector
  let mut lights = Vec::new();
    lights.push(world.get_light().clone());
    canvas
        .get_pixels_mut()
        .into_par_iter()
        .for_each(|p| calc_pixel_multi(world, c, &f, n_samples, &jitter_matrix, & lights, p));
    canvas
}


pub fn calc_pixel_multi<F>(
    world: &World,
    c: &Camera,
    f: & F,
    n_samples: usize,
    jitter_matrix: &Vec<f32>,
    lights: & Vec<Light>,
    p: &mut Pixel,
) -> ()
    where
        F: Fn(&Vec<Shape>, &Vec<Light>, &Ray, i32, bool, bool, bool, bool)  -> Color,
{
    let x = p.x;
    let y = p.y;
    if c.get_antialiasing() {
        let mut color = BLACK;

        // Accumulate light for N samples.
        for sample in 0..(n_samples * n_samples) {
            let delta_x = jitter_matrix[2 * sample] * c.get_pixel_size();
            let delta_y = jitter_matrix[2 * sample + 1] * c.get_pixel_size();
            let r = Camera::ray_for_pixel_anti_aliasing(c, x, y, delta_x, delta_y);
            let c = f(
                world.get_shapes(),
                &lights,
                &r,
                MAX_REFLECTION_RECURSION_DEPTH,
                c.get_calc_reflection(),
                c.get_calc_refraction(),
                c.get_calc_shadows(),
                false,
            );
            color = c + color;
        }
        color = color / (n_samples * n_samples) as f32;
        color.clamp_color();
        p.color.r = color.r;
        p.color.g = color.g;
        p.color.b = color.b;
    } else {
        let r = Camera::ray_for_pixel(c, x, y);
        let mut color = f(
            world.get_shapes(),
            &lights,
            &r,
            MAX_REFLECTION_RECURSION_DEPTH,
            c.get_calc_reflection(),
            c.get_calc_refraction(),
            c.get_calc_shadows(),
            false,
        );
        color.clamp_color();

        p.color.r = color.r;
        p.color.g = color.g;
        p.color.b = color.b;
    }
}

impl BackendCpuMultiCore {
    pub fn new() -> BackendCpuMultiCore {
        BackendCpuMultiCore {}
    }

    pub fn render_world_debug(
        &self,
        world: &mut World,
        c: &Camera,
        x: usize,
        y: usize,
    ) -> Result<Canvas, Box<dyn Error>> {
        let mut canvas = Canvas::new(c.get_hsize(), c.get_vsize());

        // TODO: remove, when WOrld has lights vector
        let mut lights = Vec::new();
        lights.push(world.get_light().clone());

        let r = Camera::ray_for_pixel(c, x, y);
        let mut color = CpuKernel::color_at(
            world.get_shapes(),
            &lights,
            &r,
            MAX_REFLECTION_RECURSION_DEPTH,
            c.get_calc_reflection(),
            c.get_calc_refraction(),
            c.get_calc_shadows(),
            true,
        );
        println!("'render_world_debug'   color   = {:?}", color);
        color.clamp_color();
        println!(
            "'render_world_debug'    after color clamp   color   = {:?}   ({}/{})",
            color, x, y
        );

        canvas.write_pixel(x, y, color);

        Ok(canvas)
    }
}
