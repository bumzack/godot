use crate::basics::color::{BLACK, Color, ColorOps, WHITE};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple4D;
use crate::shape::shape::Shape;

#[derive(Clone, Debug, PartialEq)]
pub struct TestPattern {
    color_a: Color,
    color_b: Color,
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
}

impl TestPattern {
    pub fn new() -> TestPattern {
        TestPattern {
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

    pub fn stripe_at(pattern: &TestPattern, point: &Tuple4D) -> Color {
        // TODO: we copy here colors all the way -> may be there is a chance to returen references?
        if point.x.floor() as i32 % 2 == 0 {
            Color::from_color(&pattern.get_color_a())
        } else {
            Color::from_color(&pattern.get_color_b())
        }
    }

    pub fn color_at_object(pattern: &TestPattern, shape: &Shape, world_point: &Tuple4D) -> Color {
        let object_point = shape.get_inverse_transformation() * world_point;
        let pattern_point = pattern.get_inverse_transformation() * &object_point;
        Self::stripe_at(pattern, &pattern_point)
    }

    pub fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix =
            Matrix::invert(&m).expect("StripePattern::set_transofrmation: cant unwrap inverse matrix");
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
    use crate::math::common::{assert_color, assert_matrix};
    use crate::math::tuple4d::Tuple;
    use crate::shape::shape::ShapeEnum;
    use crate::shape::sphere::{Sphere, SphereOps};

    use super::*;

    // page 133
    #[test]
    fn test_pattern_new() {
        let p = TestPattern::new();

        let matrix_expected = Matrix::new_identity_4x4();

        assert_matrix(&matrix_expected, p.get_transformation());
    }

    // page 133
    #[test]
    fn test_pattern_transformation() {
        let mut p = TestPattern::new();
        let matrix_transformation = Matrix::translation(1.0, 2.0, 3.0);
        let matrix_expected = matrix_transformation.clone();
        p.set_transformation(matrix_transformation);

        assert_matrix(&matrix_expected, p.get_transformation());
    }
}
