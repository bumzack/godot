use crate::{render_world_single_core, BackendOps};
use cpu_kernel_raytracer::CpuKernel;
use raytracer_lib_no_std::{Camera};
use raytracer_lib_std::{Canvas, World};
use std::error::Error;

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
