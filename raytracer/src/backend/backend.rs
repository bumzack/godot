use std::error::Error;

use cpu_kernel_raytracer::camera::Camera;
use raytracer_lib_std::Canvas;
use raytracer_lib_std::world::world::World;

// TODO: use Vec<Light> if multiple light sources should be supported

pub trait Backend {
    fn render_world(&self, world: &mut World, c: &Camera) -> Result<(Canvas), Box<dyn Error>>;
}
