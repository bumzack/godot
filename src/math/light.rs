use crate::math::color::Color;
use crate::math::common::assert_tuple;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug)]
pub struct PointLight {
    pub  position: Tuple4D,
    pub  intensitiy: Color,
}

pub trait LightOps {
    fn new(position: Tuple4D, intensitiy: Color) -> PointLight;
}

impl LightOps for PointLight {
    fn new(position: Tuple4D, intensitiy: Color) -> PointLight {
        PointLight {
            position,
            intensitiy,
        }
    }
}




