use std::error::Error;

use raytracer_lib_std::world::world::World;
use raytracer_lib_std::Canvas;
use raytracer_lib_no_std::Camera;

// TODO: use Vec<Light> if multiple light sources should be supported

pub trait Backend {
    fn render_world(&self, world: &mut World, c: &Camera) -> Result<(Canvas), Box<dyn Error>>;
    fn render_world_multi_core(&self, world: &mut World, c: &Camera) -> Result<(Canvas), Box<dyn Error>>;
}
