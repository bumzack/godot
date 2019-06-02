use crate::basics::color::Color;
use crate::light::light::LightOps;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug)]
pub struct PointLight {
    pub position: Tuple4D,
    pub intensity: Color,
}

impl LightOps for PointLight {
    fn get_intensity(&self) -> &Color {
        &self.intensity
    }

    fn get_position(&self) -> &Tuple4D {
        &self.position
    }
}

impl PointLight {
   pub fn new(position: Tuple4D, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}
