use std::error::Error;

use cpu_kernel_raytracer::CpuKernel;
use raytracer_lib_no_std::Camera;
use raytracer_lib_std::{Canvas, World};

use crate::BackendOps;
use crate::backend::backend_cpu_single_core::render_world_single_core;

pub struct BackendWasm {}

impl BackendOps for BackendWasm {
    fn render_world(&self, world: &mut World, c: &Camera) -> Result<Canvas, Box<dyn Error>> {
        let canvas = render_world_single_core(world, c, CpuKernel::color_at);
        Ok(canvas)
    }
}

impl BackendWasm {
    pub fn new() -> BackendWasm {
        BackendWasm {}
    }
}
