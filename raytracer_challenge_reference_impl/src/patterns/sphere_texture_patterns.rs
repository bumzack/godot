use std::f64::consts::PI;

use crate::basics::color::Color;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple4D;
use crate::math::Tuple;
use crate::prelude::ShapeOps;
use crate::shape::shape::Shape;

#[derive(PartialEq, Debug, Clone)]
pub struct SphereTexturePattern {
    checker: Checker,
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
}

impl SphereTexturePattern {
    pub fn new(checker: Checker) -> SphereTexturePattern {
        SphereTexturePattern {
            checker,
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
        }
    }

    pub fn pattern_at(pattern: &SphereTexturePattern, p: &Tuple4D) -> Color {
        let (u, v) = spherical_map(p);
        uv_pattern_at(&pattern.checker, u, v)
    }

    pub fn color_at_object(pattern: &SphereTexturePattern, shape: &Shape, world_point: &Tuple4D) -> Color {
        let object_point = shape.get_inverse_transformation() * world_point;
        let pattern_point = pattern.get_inverse_transformation() * &object_point;
        SphereTexturePattern::pattern_at(pattern, &pattern_point)
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

#[derive(Clone, Debug, PartialEq)]
pub struct Checker {
    width: usize,
    height: usize,
    color_a: Color,
    color_b: Color,
}

pub fn uv_checkers(width: usize, height: usize, color_a: Color, color_b: Color) -> Checker {
    Checker {
        width,
        height,
        color_a,
        color_b,
    }
}

pub fn uv_pattern_at(checker: &Checker, u: f64, v: f64) -> Color {
    let u2 = (u * checker.width as f64).floor() as i32;
    let v2 = (v * checker.height as f64).floor() as i32;
    if (u2 + v2) % 2 == 0 {
        checker.color_a
    } else {
        checker.color_b
    }
}

pub fn spherical_map(p: &Tuple4D) -> (f64, f64) {
    let theta = p.x.atan2(p.z);
    let vector = Tuple4D::new_vector(p.x, p.y, p.z);
    let radius = Tuple4D::magnitude(&vector);
    let phi = (p.y / radius).acos();
    let raw_u = theta / (2.0 * PI);
    let u = 1.0 - (raw_u + 0.5);
    let v = 1.0 - phi / PI;
    (u, v)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use crate::basics::{BLACK, WHITE};
    use crate::math::assert_two_float;
    use crate::math::common::assert_color;
    use crate::math::tuple4d::Tuple;

    use super::*;

    // bonus Scenario outline: Checker pattern in 2D
    #[test]
    fn test_checker_pattern() {
        let p = uv_checkers(2, 2, BLACK, WHITE);

        assert_color(&uv_pattern_at(&p, 0.0, 0.0), &BLACK);
        assert_color(&uv_pattern_at(&p, 0.5, 0.0), &WHITE);
        assert_color(&uv_pattern_at(&p, 0.0, 0.5), &WHITE);
        assert_color(&uv_pattern_at(&p, 0.5, 0.5), &BLACK);
        assert_color(&uv_pattern_at(&p, 1.0, 1.0), &BLACK);
    }

    // bonus Scenario outline: Using a spherical mapping on a 3D point
    #[test]
    fn test_spherical_mapping_3d_point() {
        let actual = spherical_map(&Tuple4D::new_point(0.0, 0.0, -1.0));
        let expected = (0.0, 0.5);
        assert_tuple(actual, expected);

        let actual = spherical_map(&Tuple4D::new_point(1.0, 0.0, 0.0));
        let expected = (0.25, 0.5);
        assert_tuple(actual, expected);

        let actual = spherical_map(&Tuple4D::new_point(0.0, 0.0, 1.0));
        let expected = (0.5, 0.5);
        assert_tuple(actual, expected);

        let actual = spherical_map(&Tuple4D::new_point(-1.0, 0.0, 0.0));
        let expected = (0.75, 0.5);
        assert_tuple(actual, expected);

        let actual = spherical_map(&Tuple4D::new_point(0.0, -1.0, 0.0));
        let expected = (0.5, 0.0);
        assert_tuple(actual, expected);

        let actual = spherical_map(&Tuple4D::new_point(SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0));
        let expected = (0.25, 0.5);
        assert_tuple(actual, expected);
    }

    // bonus helper:  Scenario outline: Using a texture map pattern with a spherical map
    fn test_texture_mapping_using_spherical_map_helper(actual: &Color, expected: &Color) {
        println!("actual   {:?},   expected {:?}", actual, expected);
        assert_color(actual, expected);
    }

    // bonus: Scenario outline: Using a texture map pattern with a spherical map
    #[test]
    fn test_texture_mapping_using_spherical_map() {
        let checker = uv_checkers(16, 8, BLACK, WHITE);

        let pattern = SphereTexturePattern::new(checker);

        let p = Tuple4D::new_point(0.4315, 0.467, 0.7719);
        let actual = SphereTexturePattern::pattern_at(&pattern, &p);
        test_texture_mapping_using_spherical_map_helper(&actual, &WHITE);

        let p = Tuple4D::new_point(-0.9654, 0.2552, -0.0534);
        let actual = SphereTexturePattern::pattern_at(&pattern, &p);
        test_texture_mapping_using_spherical_map_helper(&actual, &BLACK);

        let p = Tuple4D::new_point(0.1039, 0.7090, 0.6975);
        let actual = SphereTexturePattern::pattern_at(&pattern, &p);
        test_texture_mapping_using_spherical_map_helper(&actual, &WHITE);

        let p = Tuple4D::new_point(-0.4986, -0.7856, -0.3663);
        let actual = SphereTexturePattern::pattern_at(&pattern, &p);
        test_texture_mapping_using_spherical_map_helper(&actual, &BLACK);

        let p = Tuple4D::new_point(-0.0317, -0.9395, 0.3411);
        let actual = SphereTexturePattern::pattern_at(&pattern, &p);
        test_texture_mapping_using_spherical_map_helper(&actual, &BLACK);

        let p = Tuple4D::new_point(0.4809, -0.7721, 0.4154);
        let actual = SphereTexturePattern::pattern_at(&pattern, &p);
        test_texture_mapping_using_spherical_map_helper(&actual, &BLACK);

        let p = Tuple4D::new_point(0.0285, -0.9612, -0.2745);
        let actual = SphereTexturePattern::pattern_at(&pattern, &p);
        test_texture_mapping_using_spherical_map_helper(&actual, &BLACK);

        let p = Tuple4D::new_point(-0.5734, -0.2162, -0.7903);
        let actual = SphereTexturePattern::pattern_at(&pattern, &p);
        test_texture_mapping_using_spherical_map_helper(&actual, &WHITE);

        let p = Tuple4D::new_point(0.07688, -0.147, 0.6223);
        let actual = SphereTexturePattern::pattern_at(&pattern, &p);
        test_texture_mapping_using_spherical_map_helper(&actual, &BLACK);

        let p = Tuple4D::new_point(-0.7652, 0.2175, 0.6060);
        let actual = SphereTexturePattern::pattern_at(&pattern, &p);
        test_texture_mapping_using_spherical_map_helper(&actual, &BLACK);
    }

    fn assert_tuple(actual: (f64, f64), expected: (f64, f64)) {
        assert_two_float(actual.0, expected.0);
        assert_two_float(actual.0, expected.0);
    }
}
