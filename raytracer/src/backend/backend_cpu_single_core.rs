use std::error::Error;
use std::time::Instant;

use cpu_kernel_raytracer::CpuKernel;
use raytracer_lib_no_std::camera::Camera;
use raytracer_lib_std::{Canvas, World, CanvasOps, WorldOps};

use crate::{BackendOps,  get_antialiasing_params, calc_pixel_single};
use raytracer_lib_no_std::{Shape, Light, Ray, Color, CameraOps};

pub struct BackendCpuSingleCore {}

impl BackendOps for BackendCpuSingleCore {
    fn render_world(&self, world: &mut World, c: &Camera) -> Result<Canvas, Box<dyn Error>> {
        let start = Instant::now();
        let canvas = render_world_single_core(world, c, CpuKernel::color_at);
        let stopped = Instant::now();
        println!("cpu single core     duration: {:?} ", stopped.duration_since(start));
        Ok(canvas)
    }
}

impl BackendCpuSingleCore {
    pub fn new() -> BackendCpuSingleCore {
        BackendCpuSingleCore {}
    }
}

pub fn render_world_single_core<F>(world: &mut World, c: &Camera, f: F) -> Canvas
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
        .into_iter()
        .for_each(|p| calc_pixel_single(world, c, &f, n_samples, &jitter_matrix, &mut lights, p));
    canvas
}
