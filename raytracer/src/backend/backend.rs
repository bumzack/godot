use std::error::Error;

use raytracer_lib_no_std::Camera;
use raytracer_lib_std::{Canvas, World};

// TODO: use Vec<Light> if multiple light sources should be supported

pub trait Backend {
    fn render_world(&self, world: &mut World, c: &Camera) -> Result<(Canvas), Box<dyn Error>>;
}
