use crate::math::color::Color;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug)]
pub struct PointLight {
    pub position: Tuple4D,
    pub intensity: Color,
}

pub trait LightOps {
    fn new(position: Tuple4D, intensitiy: Color) -> PointLight;
}

impl LightOps for PointLight {
    fn new(position: Tuple4D, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}
