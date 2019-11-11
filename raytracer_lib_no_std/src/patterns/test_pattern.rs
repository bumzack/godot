#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

use math::prelude::*;

use crate::{Color, ColorOps, Shape, ShapeOps, BLACK, WHITE};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
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
        if intri_floor(point.x) as i32 % 2 == 0 {
            Color::from_color(&pattern.get_color_a())
        } else {
            Color::from_color(&pattern.get_color_b())
        }
    }

    pub fn color_at_object(pattern: &TestPattern, shape: &Shape, world_point: &Tuple4D) -> Color {
        let object_point = shape.get_inverse_transformation() * world_point;
        let pattern_point = pattern.get_inverse_transformation() * &object_point;
        Color::new(pattern_point.x, pattern_point.y, pattern_point.z)
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
    use crate::assert_color;
    use crate::math::tuple4d::Tuple;
    use crate::patterns::patterns::Pattern;
    use crate::shape::shape::ShapeEnum;
    use crate::shape::sphere::Sphere;

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

    // page 134 / 1
    #[test]
    fn test_pattern_object_transformation() {
        let mut shape = Sphere::new();
        let matrix_scale = Matrix::scale(2.0, 2.0, 2.0);
        shape.set_transformation(matrix_scale);
        let shape = Shape::new(ShapeEnum::Sphere(shape));

        let mut p = TestPattern::new();

        let p = Pattern::TestPattern(p);
        let point = Tuple4D::new_point(2.0, 3.0, 4.0);
        let c = p.color_at_object(&shape, &point);

        let color_expected = Color::new(1.0, 1.5, 2.0);
        println!("c = {:?},       c_expectet = {:?}", c, color_expected);
        assert_color(&color_expected, &c);
    }

    // page 134 / 2
    #[test]
    fn test_pattern_pattern_transformation() {
        let mut shape = Sphere::new();
        let shape = Shape::new(ShapeEnum::Sphere(shape));

        let mut p = TestPattern::new();
        let matrix_scale = Matrix::scale(2.0, 2.0, 2.0);
        p.set_transformation(matrix_scale);

        let p = Pattern::TestPattern(p);
        let point = Tuple4D::new_point(2.0, 3.0, 4.0);
        let c = p.color_at_object(&shape, &point);

        let color_expected = Color::new(1.0, 1.5, 2.0);
        assert_color(&color_expected, &c);
    }

    // page 134 / 3
    #[test]
    fn test_pattern_pattern_and_object_transformation() {
        let mut shape = Sphere::new();
        let matrix_scale = Matrix::scale(2.0, 2.0, 2.0);
        shape.set_transformation(matrix_scale);
        let shape = Shape::new(ShapeEnum::Sphere(shape));

        let mut p = TestPattern::new();
        let matrix_translate = Matrix::translation(0.5, 1.0, 1.5);
        p.set_transformation(matrix_translate);

        let p = Pattern::TestPattern(p);
        let point = Tuple4D::new_point(2.5, 3.0, 3.5);
        let c = p.color_at_object(&shape, &point);

        let color_expected = Color::new(0.75, 0.5, 0.25);
        assert_color(&color_expected, &c);
    }
}
