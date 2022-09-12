use std::f32::consts::PI;

use crate::basics::color::Color;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple4D;
use crate::patterns::{uv_pattern_at, Checker};
use crate::prelude::ShapeOps;
use crate::shape::shape::Shape;

#[derive(PartialEq, Debug, Clone)]
pub struct CylinderTexturePattern {
    checker: Checker,
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
}

impl CylinderTexturePattern {
    pub fn new(checker: Checker) -> CylinderTexturePattern {
        CylinderTexturePattern {
            checker,
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
        }
    }

    pub fn pattern_at(pattern: &CylinderTexturePattern, p: &Tuple4D) -> Color {
        let (u, v) = cylindrical_map(p);
        uv_pattern_at(&pattern.checker, u, v)
    }

    pub fn color_at_object(pattern: &CylinderTexturePattern, shape: &Shape, world_point: &Tuple4D) -> Color {
        let object_point = shape.get_inverse_transformation() * world_point;
        let pattern_point = pattern.get_inverse_transformation() * &object_point;
        CylinderTexturePattern::pattern_at(pattern, &pattern_point)
    }

    pub fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix = Matrix::invert(&m).unwrap();
        self.transformation_matrix = m;
    }

    pub fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }

    pub fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }
}

fn cylindrical_map(p: &Tuple4D) -> (f32, f32) {
    let theta = p.x.atan2(p.z);
    let raw_u = theta / (2.0 * PI);
    let u = 1.0 - (raw_u + 0.5);
    let v = p.y.rem_euclid(1.0);
    (u, v)
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_1_SQRT_2;

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

    fn assert_tuple(actual: (f32, f32), expected: (f32, f32)) {
        assert_two_float(actual.0, expected.0);
        assert_two_float(actual.0, expected.0);
    }
}
