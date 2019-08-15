use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

use crate::basics::color::{BLACK, Color, ColorOps, WHITE};
use crate::basics::intersection::{Intersection, IntersectionListOps};
use crate::basics::intersection::IntersectionOps;
use crate::basics::ray::Ray;
use crate::basics::ray::RayOps;
use crate::material::material::Material;
use crate::material::material::MaterialOps;
use crate::math::common::{assert_float, assert_matrix, assert_tuple};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;
use crate::shape::shape::Shape;

#[derive(Clone, Debug)]
pub struct Pattern {
    color_a: Color,
    color_b: Color,
}

pub trait PatternOps {}

impl Pattern {
    pub(crate) fn new() -> Pattern {
        Pattern {
            color_a: WHITE,
            color_b: BLACK,
        }
    }

    pub fn set_color_a(&mut self, a: Color) {
        self.color_a = a;
    }

    pub fn set_color_b(&mut self, b: Color) {
        self.color_b = b;
    }

    pub fn get_color_a(&self) -> &Color {
        &self.color_a
    }

    pub fn get_color_b(&self) -> &Color {
        &self.color_b
    }

    pub fn stripe_at(&self, point: &Tuple4D) -> Color {
        // TODO: we copy here colors all the way -> may be there is a chance to returen references?
        if point.x.floor() % 2.0 == 0.0 {
            Color::from_color(&self.color_a)
        } else {
            Color::from_color(&self.color_b)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::common::{assert_color, assert_float, assert_matrix, assert_tuple, assert_two_float};

    use super::*;

    // page 128
    #[test]
    fn test_pattern_new() {
        let p = Pattern::new();

        assert_color(p.get_color_a(), &WHITE);
        assert_color(p.get_color_b(), &BLACK);
    }

    // page 129 top y
    #[test]
    fn test_pattern_stripe_constant_y() {
        let p = Pattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = p.stripe_at( &point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.0, 1.0, 0.0);
        let c2 = p.stripe_at( &point2);
        assert_color(&c2, &WHITE);

        let point3 = Tuple4D::new_point(0.0, 2.0, 0.0);
        let c3 = p.stripe_at( &point3);
        assert_color(&c3, &WHITE);
    }


    // page 129 top z
    #[test]
    fn test_pattern_stripe_constant_z() {
        let p = Pattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = p.stripe_at( &point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.0, 0.0, 1.0);
        let c2 = p.stripe_at( &point2);
        assert_color(&c2, &WHITE);

        let point3 = Tuple4D::new_point(0.0, 0.0, 2.0);
        let c3 = p.stripe_at( &point3);
        assert_color(&c3, &WHITE);
    }

    // page 129 top x
    #[test]
    fn test_pattern_stripe_constant_x() {
        let p = Pattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = p.stripe_at( &point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.9, 0.0, 0.0);
        let c2 = p.stripe_at( &point2);
        assert_color(&c2, &WHITE);

        let point3 = Tuple4D::new_point(1.0, 0.0, 0.0);
        let c3 = p.stripe_at( &point3);
        assert_color(&c3, &BLACK);

        let point4 = Tuple4D::new_point(-0.1, 0.0, 0.0);
        let c4 = p.stripe_at( &point4);
        assert_color(&c4, &BLACK);

        let point5 = Tuple4D::new_point(-1.0, 0.0, 0.0);
        let c5 = p.stripe_at( &point5);
        assert_color(&c5, &BLACK);

        let point6 = Tuple4D::new_point(-1.1, 0.0, 0.0);
        let c6 = p.stripe_at( &point6);
        assert_color(&c6, &WHITE);
    }
}
