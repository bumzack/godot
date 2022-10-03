use crate::basics::color::{Color, ColorOps};
use crate::math::tuple4d::Tuple4D;
use crate::shape::shape::Shape;

#[derive(Clone, Debug, PartialEq)]
pub struct TestPattern {}

impl TestPattern {
    pub fn new() -> Self {
        TestPattern {}
    }

    pub fn pattern_at(&self, point: &Tuple4D) -> Color {
        Color::new(point.x, point.y, point.z)
    }
}

impl Default for TestPattern {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::common::{assert_color, assert_matrix};
    use crate::math::tuple4d::Tuple;
    use crate::patterns::patterns::PatternEnum;
    use crate::patterns::Pattern;
    use crate::prelude::{Matrix, MatrixOps, ShapeOps};
    use crate::shape::shape::ShapeEnum;
    use crate::shape::sphere::Sphere;

    use super::*;

    // page 133
    #[test]
    fn test_pattern_new() {
        let p = Pattern::new(PatternEnum::TestPatternEnum(TestPattern::new()));

        let matrix_expected = Matrix::new_identity_4x4();

        assert_matrix(&matrix_expected, p.get_transformation());
    }

    // page 133
    #[test]
    fn test_pattern_transformation() {
        let mut p = Pattern::new(PatternEnum::TestPatternEnum(TestPattern::new()));
        let matrix_transformation = Matrix::translation(1.0, 2.0, 3.0);
        let matrix_expected = matrix_transformation.clone();
        p.set_transformation(matrix_transformation);

        assert_matrix(&matrix_expected, p.get_transformation());
    }

    // page 134 / 1
    #[test]
    fn test_pattern_object_transformation() {
        let mut shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        let matrix_scale = Matrix::scale(2.0, 2.0, 2.0);
        shape.set_transformation(matrix_scale);

        let p = Pattern::new(PatternEnum::TestPatternEnum(TestPattern::new()));

        let point = Tuple4D::new_point(2.0, 3.0, 4.0);
        let c = p.pattern_at_shape(&shape, &point);

        let color_expected = Color::new(1.0, 1.5, 2.0);
        println!("c = {:?},       color_expected = {:?}", c, color_expected);
        assert_color(&color_expected, &c);
    }

    // page 134 / 2
    #[test]
    fn test_pattern_pattern_transformation() {
        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));

        let mut p = Pattern::new(PatternEnum::TestPatternEnum(TestPattern::new()));
        let matrix_scale = Matrix::scale(2.0, 2.0, 2.0);
        p.set_transformation(matrix_scale);

        let point = Tuple4D::new_point(2.0, 3.0, 4.0);
        let c = p.pattern_at_shape(&shape, &point);

        let color_expected = Color::new(1.0, 1.5, 2.0);
        assert_color(&color_expected, &c);
    }

    // page 134 / 3
    #[test]
    fn test_pattern_pattern_and_object_transformation() {
        let mut shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        let matrix_scale = Matrix::scale(2.0, 2.0, 2.0);
        shape.set_transformation(matrix_scale);

        let mut p = Pattern::new(PatternEnum::TestPatternEnum(TestPattern::new()));
        let matrix_translate = Matrix::translation(0.5, 1.0, 1.5);
        p.set_transformation(matrix_translate);

        let point = Tuple4D::new_point(2.5, 3.0, 3.5);
        let c = p.pattern_at_shape(&shape, &point);

        let color_expected = Color::new(0.75, 0.5, 0.25);
        assert_color(&color_expected, &c);
    }
}
