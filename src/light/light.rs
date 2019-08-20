use crate::basics::color::Color;
use crate::light::pointlight::PointLight;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug)]
pub enum Light {
    PointLight(PointLight),
}

pub trait LightOps {
    fn get_intensity(&self) -> &Color;
    fn set_intensity(&mut self, intensity: Color);

    fn get_position(&self) -> &Tuple4D;
    fn set_position(&mut self, pos: Tuple4D);
}

impl LightOps for Light {
    fn get_intensity(&self) -> &Color {
        let res = match self {
            Light::PointLight(ref pl) => pl.get_intensity(),
        };
        res
    }

    fn set_intensity(&mut self, intensity: Color) {
        let res = match self {
            Light::PointLight(ref mut pl) => pl.set_intensity(intensity),
        };
    }

    fn get_position(&self) -> &Tuple4D {
        let res = match self {
            Light::PointLight(ref pl) => pl.get_position(),
        };
        res
    }

    fn set_position(&mut self, pos: Tuple4D) {
        let res = match self {
            Light::PointLight(ref mut pl) => pl.set_position(pos),
        };
    }
}
