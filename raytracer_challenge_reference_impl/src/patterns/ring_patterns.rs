use crate::basics::color::{Color, ColorOps, BLACK, WHITE};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple4D;
use crate::prelude::ShapeOps;
use crate::shape::shape::Shape;

#[derive(Clone, Debug, PartialEq)]
pub struct RingPattern {
    color_a: Color,
    color_b: Color,
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
}

impl RingPattern {
    pub fn new() -> RingPattern {
        RingPattern {
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

    pub fn color_at(pattern: &RingPattern, point: &Tuple4D) -> Color {
        if (point.x.powf(2.0) + point.z.powf(2.0)).sqrt().floor() as i32 % 2 == 0 {
            Color::from_color(pattern.get_color_a())
        } else {
            Color::from_color(pattern.get_color_b())
        }
    }

    pub fn color_at_object(pattern: &RingPattern, shape: &Shape, world_point: &Tuple4D) -> Color {
        let object_point = shape.get_inverse_transformation() * world_point;
        let pattern_point = pattern.get_inverse_transformation() * &object_point;
        Self::color_at(pattern, &pattern_point)
    }

    pub fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix =
            Matrix::invert(&m).expect("RingPattern::set_transformation: cant unwrap inverse matrix");
        self.transformation_matrix = m;
    }

    pub fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }

    pub fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }
}

impl Default for RingPattern {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::common::assert_color;
    use crate::math::tuple4d::Tuple;

    use super::*;

    // page 128
    #[test]
    fn test_pattern_new() {
        let p = RingPattern::new();
        assert_color(p.get_color_a(), &WHITE);
        assert_color(p.get_color_b(), &BLACK);
    }

    // page 136
    #[test]
    fn test_pattern_stripe_constant_y() {
        let p = RingPattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = RingPattern::color_at(&p, &point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(1.0, 0.0, 0.0);
        let c2 = RingPattern::color_at(&p, &point2);
        assert_color(&c2, &BLACK);

        let point3 = Tuple4D::new_point(0.0, 0.0, 1.0);
        let c3 = RingPattern::color_at(&p, &point3);
        assert_color(&c3, &BLACK);

        let point4 = Tuple4D::new_point(0.709, 0.0, 0.709);
        let c4 = RingPattern::color_at(&p, &point4);
        assert_color(&c4, &BLACK);
    }
}
