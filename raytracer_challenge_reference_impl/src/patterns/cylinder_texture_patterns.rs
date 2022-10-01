use std::f64::consts::PI;

use crate::basics::color::Color;
use crate::math::tuple4d::Tuple4D;
use crate::patterns::{uv_pattern_at, Checker};

#[derive(PartialEq, Debug, Clone)]
pub struct CylinderTexturePattern {
    checker: Checker,
}

impl CylinderTexturePattern {
    pub fn new(checker: Checker) -> CylinderTexturePattern {
        CylinderTexturePattern { checker }
    }

    pub fn pattern_at(&self, p: &Tuple4D) -> Color {
        let (u, v) = cylindrical_map(p);
        uv_pattern_at(&self.checker, u, v)
    }
}

fn cylindrical_map(p: &Tuple4D) -> (f64, f64) {
    let theta = p.x.atan2(p.z);
    let raw_u = theta / (2.0 * PI);
    let u = 1.0 - (raw_u + 0.5);
    let v = p.y.rem_euclid(1.0);
    (u, v)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_1_SQRT_2;

    use crate::math::{assert_two_float, Tuple};

    use super::*;

    // bonus planar mapping  Scenario Outline: Using a cylindrical mapping on a 3D point
    #[test]
    fn test_cylndrical_mapping() {
        let p = Tuple4D::new_point(0.0, 0.0, -1.);
        let actual = cylindrical_map(&p);
        assert_tuple(actual, (0., 0.));

        let p = Tuple4D::new_point(0., 0.5, -1.);
        let actual = cylindrical_map(&p);
        assert_tuple(actual, (0., 0.5));

        let p = Tuple4D::new_point(0., 1., -1.);
        let actual = cylindrical_map(&p);
        assert_tuple(actual, (0., 0.));

        let p = Tuple4D::new_point(FRAC_1_SQRT_2, 0.5, -FRAC_1_SQRT_2);
        let actual = cylindrical_map(&p);
        assert_tuple(actual, (0.125, 0.5));

        let p = Tuple4D::new_point(1.0, 0.5, 0.0);
        let actual = cylindrical_map(&p);
        assert_tuple(actual, (0.25, 0.5));

        let p = Tuple4D::new_point(FRAC_1_SQRT_2, 0.5, FRAC_1_SQRT_2);
        let actual = cylindrical_map(&p);
        assert_tuple(actual, (0.375, 0.5));

        let p = Tuple4D::new_point(0., -0.25, 1.0);
        let actual = cylindrical_map(&p);
        assert_tuple(actual, (0.5, 0.75));

        let p = Tuple4D::new_point(-FRAC_1_SQRT_2, 0.5, FRAC_1_SQRT_2);
        let actual = cylindrical_map(&p);
        assert_tuple(actual, (0.625, 0.5));

        let p = Tuple4D::new_point(-1., 1.25, 0.0);
        let actual = cylindrical_map(&p);
        assert_tuple(actual, (0.75, 0.25));

        let p = Tuple4D::new_point(-FRAC_1_SQRT_2, 0.5, -FRAC_1_SQRT_2);
        let actual = cylindrical_map(&p);
        assert_tuple(actual, (0.875, 0.5));
    }

    fn assert_tuple(actual: (f64, f64), expected: (f64, f64)) {
        assert_two_float(actual.0, expected.0);
        assert_two_float(actual.0, expected.0);
    }
}
