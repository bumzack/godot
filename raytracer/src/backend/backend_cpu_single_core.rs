use std::error::Error;
use std::time::Instant;

use cpu_kernel_raytracer::CpuKernel;
use raytracer_lib_no_std::camera::{Camera};
use raytracer_lib_std::{Canvas, World};

use crate::{render_world_single_core, BackendOps};

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
