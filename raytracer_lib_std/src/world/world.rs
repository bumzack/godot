// TODO: use Vec<Light> if multiple light sources should be supported

use raytracer_lib_no_std::{Color, ColorOps, Light, PointLight, Shape, Tuple, Tuple4D};

#[derive(Clone, Debug)]
pub struct World<'a> {
    shapes: Vec<Shape<'a>>,
    light: Light,
}

pub trait WorldOps<'a> {
    fn new() -> World<'a>;
    fn set_light(&mut self, light: Light);
    fn get_light(&self) -> &Light;

    fn add_shape(&mut self, shape: Shape<'a>);
    fn get_shapes(&self) -> &Vec<Shape<'a>>;
    fn get_shapes_mut(&mut self) -> &mut Vec<Shape<'a>>;
}

impl<'a> WorldOps <'a> for World <'a>{
    fn new() -> World<'a> {
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

    fn add_shape(&mut self, shape: Shape<'a>) {
        self.shapes.push(shape);
    }

    fn get_shapes(&self) -> &Vec<Shape<'a>> {
        &self.shapes
    }

    fn get_shapes_mut(&mut self) -> &mut Vec<Shape<'a>> {
        &mut self.shapes
    }
}
