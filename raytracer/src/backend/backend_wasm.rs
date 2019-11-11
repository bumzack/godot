use std::error::Error;

use raytracer_lib_no_std::Camera;
use raytracer_lib_std::{Canvas, World};

use crate::backend::backend_cpu_single_core::render_world_single_core;
use crate::BackendOps;
use raytracer_lib_no_std::kernel::raytracer_kernel::RaytracerKernel;

pub struct BackendWasm {}

impl BackendOps for BackendWasm {
    fn render_world(&self, world: &mut World, c: &Camera) -> Result<Canvas, Box<dyn Error>> {
        let canvas = render_world_single_core(world, c, RaytracerKernel::color_at);
        Ok(canvas)
    }
}

impl BackendWasm {
    pub fn new() -> BackendWasm {
        BackendWasm {}
    }
}
