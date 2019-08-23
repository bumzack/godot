use crate::basics::color::Color;
use crate::light::pointlight::PointLight;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug, DeviceCopy)]
pub enum Light {
    PointLight(PointLight),
}

pub trait LightOps {
    fn get_intensity(&self) -> &Color;
    fn get_position(&self) -> &Tuple4D;
}

impl LightOps for Light {
    fn get_intensity(&self) -> &Color {
        let res = match self {
            Light::PointLight(ref pl) => pl.get_intensity(),
        };
        res
    }

    fn get_position(&self) -> &Tuple4D {
        let res = match self {
            Light::PointLight(ref pl) => pl.get_position(),
        };
        res
    }
}
