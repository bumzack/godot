use crate::basics::color::{Color, BLACK, WHITE};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple4D;
use crate::shape::shape::Shape;

#[derive(Clone, Debug)]
pub struct GradientPattern {
    color_a: Color,
    color_b: Color,
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
}

impl GradientPattern {
    pub fn new() -> GradientPattern {
        GradientPattern {
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

    pub fn color_at(pattern: &GradientPattern, point: &Tuple4D) -> Color {
        let distance = pattern.get_color_b() - pattern.get_color_a();
        let fraction = point.x - point.x.floor();
        pattern.get_color_a() + &(distance * fraction)
    }

    pub fn color_at_object(pattern: &GradientPattern, shape: &Shape, world_point: &Tuple4D) -> Color {
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
    use crate::basics::color::ColorOps;
    use crate::math::common::assert_color;
    use crate::math::tuple4d::Tuple;

    use super::*;

    // page 128
    #[test]
    fn test_pattern_new() {
        let p = GradientPattern::new();
        assert_color(p.get_color_a(), &WHITE);
        assert_color(p.get_color_b(), &BLACK);
    }

    // page 135
    #[test]
    fn test_pattern_stripe_constant_y() {
        let p = GradientPattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = GradientPattern::color_at(&p, &point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.25, 0.0, 0.0);
        let c2 = GradientPattern::color_at(&p, &point2);
        let c2_expected = Color::new(0.75, 0.75, 0.75);
        assert_color(&c2, &c2_expected);

        let point3 = Tuple4D::new_point(0.5, 0.0, 0.0);
        let c3 = GradientPattern::color_at(&p, &point3);
        let c3_expected = Color::new(0.5, 0.5, 0.5);
        assert_color(&c3, &c3_expected);

        let point4 = Tuple4D::new_point(0.75, 0.0, 0.0);
        let c4 = GradientPattern::color_at(&p, &point4);
        let c4_expected = Color::new(0.25, 0.25, 0.25);
        assert_color(&c4, &c4_expected);
    }
}
