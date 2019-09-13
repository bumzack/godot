// TODO: use Vec<Light> if multiple light sources should be supported

use raytracer_lib_no_std::{Color, ColorOps, Light, PointLight, Shape};
use math::prelude::*;

#[derive(Clone, Debug)]
pub struct World {
    shapes: Vec<Shape>,
    light: Light,
}

pub trait WorldOps {
    fn new() -> World;
    fn set_light(&mut self, light: Light);
    fn get_light(&self) -> &Light;

    fn add_shape(&mut self, shape: Shape);
    fn get_shapes(&self) -> &Vec<Shape>;
    fn get_shapes_mut(&mut self) -> &mut Vec<Shape>;
}

impl WorldOps for World {
    fn new() -> World {
        // TODO: default light ?!?!?! hmm - where, color why not different solution
        let pl = PointLight::new(Tuple4D::new_point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        World {
            shapes: Vec::new(),
            light: Light::PointLight(pl),
        }
    }

    fn set_light(&mut self, light: Light) {
        self.light = light;
    }

    fn get_light(&self) -> &Light {
        &self.light
    }

    fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    fn get_shapes(&self) -> &Vec<Shape> {
        &self.shapes
    }

    fn get_shapes_mut(&mut self) -> &mut Vec<Shape> {
        &mut self.shapes
    }
}
