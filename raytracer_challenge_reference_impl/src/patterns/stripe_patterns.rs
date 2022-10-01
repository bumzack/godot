use crate::basics::color::{Color, ColorOps, BLACK, WHITE};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple4D;
use crate::prelude::ShapeOps;
use crate::shape::shape::Shape;

#[derive(Clone, Debug, PartialEq)]
pub struct StripePattern {
    color_a: Color,
    color_b: Color,
}

impl StripePattern {
    pub fn new() -> Self {
        StripePattern {
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

    pub fn pattern_at(&self, point: &Tuple4D) -> Color {
        // TODO: we copy here colors all the way -> may be there is a chance to returen references?
        if point.x.floor() as i32 % 2 == 0 {
            Color::from_color(self.get_color_a())
        } else {
            Color::from_color(self.get_color_b())
        }
    }
}

impl Default for StripePattern {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::common::assert_color;
    use crate::math::tuple4d::Tuple;
    use crate::patterns::Pattern;
    use crate::prelude::PatternEnum::StripePatternEnum;
    use crate::shape::shape::ShapeEnum;
    use crate::shape::sphere::Sphere;

    use super::*;

    // page 128
    #[test]
    fn test_pattern_new() {
        let p = StripePattern::new();

        assert_color(p.get_color_a(), &WHITE);
        assert_color(p.get_color_b(), &BLACK);
    }

    // page 129 top y
    #[test]
    fn test_pattern_stripe_constant_y() {
        let p = StripePattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = StripePattern::pattern_at(&p, &point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.0, 1.0, 0.0);
        let c2 = StripePattern::pattern_at(&p, &point2);
        assert_color(&c2, &WHITE);

        let point3 = Tuple4D::new_point(0.0, 2.0, 0.0);
        let c3 = StripePattern::pattern_at(&p, &point3);
        assert_color(&c3, &WHITE);
    }

    // page 129 top z
    #[test]
    fn test_pattern_stripe_constant_z() {
        let p = StripePattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = StripePattern::pattern_at(&p, &point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.0, 0.0, 1.0);
        let c2 = StripePattern::pattern_at(&p, &point2);
        assert_color(&c2, &WHITE);

        let point3 = Tuple4D::new_point(0.0, 0.0, 2.0);
        let c3 = StripePattern::pattern_at(&p, &point3);
        assert_color(&c3, &WHITE);
    }

    // page 129 top x
    #[test]
    fn test_pattern_stripe_constant_x() {
        let p = StripePattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = StripePattern::pattern_at(&p, &point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.9, 0.0, 0.0);
        let c2 = StripePattern::pattern_at(&p, &point2);
        assert_color(&c2, &WHITE);

        let point3 = Tuple4D::new_point(1.0, 0.0, 0.0);
        let c3 = StripePattern::pattern_at(&p, &point3);
        assert_color(&c3, &BLACK);

        let point4 = Tuple4D::new_point(-0.1, 0.0, 0.0);
        let c4 = StripePattern::pattern_at(&p, &point4);
        assert_color(&c4, &BLACK);

        let point5 = Tuple4D::new_point(-1.0, 0.0, 0.0);
        let c5 = StripePattern::pattern_at(&p, &point5);
        assert_color(&c5, &BLACK);

        let point6 = Tuple4D::new_point(-1.1, 0.0, 0.0);
        let c6 = StripePattern::pattern_at(&p, &point6);
        assert_color(&c6, &WHITE);
    }

    // page 131 part1
    #[test]
    fn test_material_with_pattern_transformation1() {
        let transformation = Matrix::scale(2.0, 2.0, 2.0);
        let mut shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        shape.set_transformation(transformation);

        let pattern = Pattern::new(StripePatternEnum(StripePattern::new()));

        let p = Tuple4D::new_point(1.5, 0.0, 0.0);
        let c = pattern.pattern_at_shape(&shape, &p);
        assert_color(&c, &WHITE);
    }

    // page 131 part2
    #[test]
    fn test_material_with_pattern_transformation2() {
        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));

        let transformation = Matrix::scale(2.0, 2.0, 2.0);
        let mut pattern = Pattern::new(StripePatternEnum(StripePattern::new()));
        pattern.set_transformation(transformation);

        let p = Tuple4D::new_point(1.5, 0.0, 0.0);
        let c = pattern.pattern_at_shape(&shape, &p);
        assert_color(&c, &WHITE);
    }

    // page 131 part3
    #[test]
    fn test_material_with_pattern_transformation3() {
        let transformation = Matrix::scale(2.0, 2.0, 2.0);
        let mut shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        shape.set_transformation(transformation);

        let transformation_pattern = Matrix::translation(0.5, 0.0, 0.0);
        let mut pattern = Pattern::new(StripePatternEnum(StripePattern::new()));
        pattern.set_transformation(transformation_pattern);

        let p = Tuple4D::new_point(2.5, 0.0, 0.0);
        let c = pattern.pattern_at_shape(&shape, &p);

        println!("expected color {:?},   actual color {:?}", &WHITE, &c);

        assert_color(&c, &WHITE);
    }
}
