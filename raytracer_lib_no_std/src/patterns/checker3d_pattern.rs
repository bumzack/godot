#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

use math::prelude::*;

use crate::{BLACK, Color, ColorOps, Shape, ShapeOps, WHITE};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Checker3DPattern {
    color_a: Color,
    color_b: Color,
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
}

impl Checker3DPattern {
    pub fn new() -> Checker3DPattern {
        Checker3DPattern {
            color_a: WHITE,
            color_b: BLACK,
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
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

    pub fn color_at(pattern: &Checker3DPattern, point: &Tuple4D) -> Color {
        if intri_floor(intri_abs(point.x) + intri_abs(point.y) + intri_abs(point.z)) as i32 % 2 == 0 {
            Color::from_color(&pattern.get_color_a())
        } else {
            Color::from_color(&pattern.get_color_b())
        }
    }

    pub fn color_at_object(pattern: &Checker3DPattern, shape: &Shape, world_point: &Tuple4D) -> Color {
        let object_point = shape.get_inverse_transformation() * world_point;
        let pattern_point = pattern.get_inverse_transformation() * &object_point;
        Self::color_at(pattern, &pattern_point)
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

#[cfg(test)]
mod tests {
    use crate::assert_color;

    use super::*;

    // page 128
    #[test]
    fn test_pattern_new() {
        let p = Checker3DPattern::new();
        assert_color(p.get_color_a(), &WHITE);
        assert_color(p.get_color_b(), &BLACK);
    }

    // page 137  x
    #[test]
    fn test_pattern_checker3D_repeat_x() {
        let p = Checker3DPattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = Checker3DPattern::color_at(&p, &point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.99, 0.0, 0.0);
        let c2 = Checker3DPattern::color_at(&p, &point2);
        assert_color(&c2, &WHITE);

        let point3 = Tuple4D::new_point(1.01, 0.0, 0.0);
        let c3 = Checker3DPattern::color_at(&p, &point3);
        assert_color(&c3, &BLACK);
    }

    // page 137  y
    #[test]
    fn test_pattern_checker3D_repeat_y() {
        let p = Checker3DPattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = Checker3DPattern::color_at(&p, &point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.0, 0.99, 0.0);
        let c2 = Checker3DPattern::color_at(&p, &point2);
        assert_color(&c2, &WHITE);

        let point3 = Tuple4D::new_point(0.0, 1.01, 0.0);
        let c3 = Checker3DPattern::color_at(&p, &point3);
        assert_color(&c3, &BLACK);
    }

    // page 137  z
    #[test]
    fn test_pattern_checker3D_repeat_z() {
        let p = Checker3DPattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = Checker3DPattern::color_at(&p, &point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.0, 0.0, 0.99);
        let c2 = Checker3DPattern::color_at(&p, &point2);
        assert_color(&c2, &WHITE);

        let point3 = Tuple4D::new_point(0.0, 0.0, 1.01);
        let c3 = Checker3DPattern::color_at(&p, &point3);
        assert_color(&c3, &BLACK);
    }
}
