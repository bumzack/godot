use std::f32::consts::PI;
use std::ops::Mul;

use crate::math::color::Color;
use crate::math::common::assert_tuple;
use crate::math::common::float_equal;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone,Debug)]
pub struct Light {
    position: Tuple4D,
    direction: Tuple4D,
    color: Color,
}

pub trait LightOps {
    fn new(position: Tuple4D, direction: Tuple4D, color: Color) -> Light;
}

impl LightOps for Light {
    fn new(position: Tuple4D, direction: Tuple4D, color: Color) -> Light {
        Light {
            position,
            direction,
            color
        }
    }
}



