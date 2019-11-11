use std::error::Error;
use std::time::Instant;

use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use cpu_kernel_raytracer::CpuKernel;
use raytracer_lib_no_std::{ Color, ColorOps, Light, Ray, Shape};
use raytracer_lib_no_std::camera::{Camera, CameraOps};
use raytracer_lib_no_std::MAX_REFLECTION_RECURSION_DEPTH;
use raytracer_lib_std::{Canvas, CanvasOps, World, WorldOps};

use crate::backend::backend_helper::{calc_pixel, get_antialiasing_params};
use crate::BackendOps;

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

pub fn render_world_multi_core<F: Sync + Send>(world: &mut World, c: &Camera, f: F) -> Canvas
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
        .for_each(|p| calc_pixel(world, c, &f, n_samples, &jitter_matrix, &lights, p));
    canvas
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
