use serde::Deserialize;
use serde::Serialize;

use crate::{Color, ColorOps, Pattern};

pub const REFRACTION_VACUUM: f32 = 1.0;
pub const REFRACTION_AIR: f32 = 1.00029;
pub const REFRACTION_WATER: f32 = 1.333;
pub const REFRACTION_GLASS: f32 = 1.52;
pub const REFRACTION_DIAMOND: f32 = 2.417;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Material {
    color: Color,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
    pattern: Option<Pattern>,
    reflective: f32,
    transparency: f32,
    refractive_index: f32,
}

pub trait MaterialOps {
    fn new() -> Material;

    fn set_color(&mut self, c: Color);
    fn set_diffuse(&mut self, d: f32);
    fn set_specular(&mut self, s: f32);
    fn set_shininess(&mut self, s: f32);
    fn set_ambient(&mut self, a: f32);

    fn get_diffuse(&self) -> f32;
    fn get_specular(&self) -> f32;
    fn get_shininess(&self) -> f32;
    fn get_ambient(&self) -> f32;

    fn get_color(&self) -> &Color;

    fn set_pattern(&mut self, p: Pattern);
    fn get_pattern(&self) -> &Option<Pattern>;

    fn get_reflective(&self) -> f32;
    fn set_reflective(&mut self, a: f32);

    fn get_transparency(&self) -> f32;

    fn set_transparency(&mut self, transparency: f32);
    fn get_refractive_index(&self) -> f32;

    fn set_refractive_index(&mut self, refractive_index: f32);
}

impl MaterialOps for Material {
    fn new() -> Material {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }

    fn set_color(&mut self, c: Color) {
        self.color = c;
    }

    fn set_diffuse(&mut self, d: f32) {
        self.diffuse = d;
    }

    fn set_specular(&mut self, s: f32) {
        self.specular = s;
    }

    fn set_shininess(&mut self, s: f32) {
        self.shininess = s;
    }

    fn set_ambient(&mut self, a: f32) {
        self.ambient = a;
    }

    fn get_diffuse(&self) -> f32 {
        self.diffuse
    }

    fn get_specular(&self) -> f32 {
        self.specular
    }

    fn get_shininess(&self) -> f32 {
        self.shininess
    }

    fn get_ambient(&self) -> f32 {
        self.ambient
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn set_pattern(&mut self, p: Pattern) {
        self.pattern = Some(p);
    }

    fn get_pattern(&self) -> &Option<Pattern> {
        &self.pattern
    }

    fn get_reflective(&self) -> f32 {
        self.reflective
    }

    fn set_reflective(&mut self, a: f32) {
        self.reflective = a;
    }

    fn get_transparency(&self) -> f32 {
        self.transparency
    }

    fn set_transparency(&mut self, transparency: f32) {
        self.transparency = transparency;
    }

    fn get_refractive_index(&self) -> f32 {
        self.refractive_index
    }

    fn set_refractive_index(&mut self, refractive_index: f32) {
        self.refractive_index = refractive_index;
    }
}
