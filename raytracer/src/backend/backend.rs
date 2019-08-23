use raytracer_lib_no_std::basics::camera::{Camera, CameraOps};
use raytracer_lib_std::canvas::canvas::Canvas;
use raytracer_lib_std::world::world::World;
use std::error::Error;

// TODO: use Vec<Light> if multiple light sources should be supported

pub trait Backend {
    fn render_world(&self, world: &mut World, c: &Camera) -> Result<(Canvas), Box<dyn Error>>;
}
