use crate::{ Color, ColorOps, Shape, ShapeOps,  BLACK, WHITE};
use math::prelude::*;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct StripePattern {
    color_a: Color,
    color_b: Color,
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
}

impl StripePattern {
    pub fn new() -> StripePattern {
        StripePattern {
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

    pub fn stripe_at(pattern: &StripePattern, point: &Tuple4D) -> Color {
        // TODO: we copy here colors all the way -> may be there is a chance to returen references?
        if intri_floor(point.x) as i32 % 2 == 0 {
            Color::from_color(&pattern.get_color_a())
        } else {
            Color::from_color(&pattern.get_color_b())
        }
    }

    pub fn color_at_object(pattern: &StripePattern, shape: &Shape, world_point: &Tuple4D) -> Color {
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

//#[cfg(test)]
//mod tests {
//    use crate::math::raytracer_lib_no_std::assert_color;
//    use crate::math::tuple4d::Tuple;
//    use crate::shape::shape::ShapeEnum;
//    use crate::shape::sphere::{Sphere, SphereOps};
//
//    use super::*;
//
//    // page 128
//    #[test]
//    fn test_pattern_new() {
//        let p = StripePattern::new();
//
//        assert_color(p.get_color_a(), &WHITE);
//        assert_color(p.get_color_b(), &BLACK);
//    }
//
//    // page 129 top y
//    #[test]
//    fn test_pattern_stripe_constant_y() {
//        let p = StripePattern::new();
//
//        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
//        let c1 = StripePattern::stripe_at(&p, &point1);
//        assert_color(&c1, &WHITE);
//
//        let point2 = Tuple4D::new_point(0.0, 1.0, 0.0);
//        let c2 = StripePattern::stripe_at(&p, &point2);
//        assert_color(&c2, &WHITE);
//
//        let point3 = Tuple4D::new_point(0.0, 2.0, 0.0);
//        let c3 = StripePattern::stripe_at(&p, &point3);
//        assert_color(&c3, &WHITE);
//    }
//
//    // page 129 top z
//    #[test]
//    fn test_pattern_stripe_constant_z() {
//        let p = StripePattern::new();
//
//        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
//        let c1 = StripePattern::stripe_at(&p, &point1);
//        assert_color(&c1, &WHITE);
//
//        let point2 = Tuple4D::new_point(0.0, 0.0, 1.0);
//        let c2 = StripePattern::stripe_at(&p, &point2);
//        assert_color(&c2, &WHITE);
//
//        let point3 = Tuple4D::new_point(0.0, 0.0, 2.0);
//        let c3 = StripePattern::stripe_at(&p, &point3);
//        assert_color(&c3, &WHITE);
//    }
//
//    // page 129 top x
//    #[test]
//    fn test_pattern_stripe_constant_x() {
//        let p = StripePattern::new();
//
//        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
//        let c1 = StripePattern::stripe_at(&p, &point1);
//        assert_color(&c1, &WHITE);
//
//        let point2 = Tuple4D::new_point(0.9, 0.0, 0.0);
//        let c2 = StripePattern::stripe_at(&p, &point2);
//        assert_color(&c2, &WHITE);
//
//        let point3 = Tuple4D::new_point(1.0, 0.0, 0.0);
//        let c3 = StripePattern::stripe_at(&p, &point3);
//        assert_color(&c3, &BLACK);
//
//        let point4 = Tuple4D::new_point(-0.1, 0.0, 0.0);
//        let c4 = StripePattern::stripe_at(&p, &point4);
//        assert_color(&c4, &BLACK);
//
//        let point5 = Tuple4D::new_point(-1.0, 0.0, 0.0);
//        let c5 = StripePattern::stripe_at(&p, &point5);
//        assert_color(&c5, &BLACK);
//
//        let point6 = Tuple4D::new_point(-1.1, 0.0, 0.0);
//        let c6 = StripePattern::stripe_at(&p, &point6);
//        assert_color(&c6, &WHITE);
//    }
//
//    // page 131 part1
//    #[test]
//    fn test_material_with_pattern_transformation1() {
//        let transformation = Matrix::scale(2.0, 2.0, 2.0);
//        let mut s = Sphere::new();
//        s.set_transformation(transformation);
//        let shape = Shape::new(ShapeEnum::Sphere(s));
//
//        let pattern = StripePattern::new();
//
//        let p = Tuple4D::new_point(1.5, 0.0, 0.0);
//        let c = StripePattern::color_at_object(&pattern, &shape, &p);
//        assert_color(&c, &WHITE);
//    }
//
//    // page 131 part2
//    #[test]
//    fn test_material_with_pattern_transformation2() {
//        let s = Sphere::new();
//        let shape = Shape::new(ShapeEnum::Sphere(s));
//
//        let transformation = Matrix::scale(2.0, 2.0, 2.0);
//        let mut pattern = StripePattern::new();
//        pattern.set_transformation(transformation);
//
//        let p = Tuple4D::new_point(1.5, 0.0, 0.0);
//        let c = StripePattern::color_at_object(&pattern, &shape, &p);
//        assert_color(&c, &WHITE);
//    }
//
//    // page 131 part3
//    #[test]
//    fn test_material_with_pattern_transformation3() {
//        let transformation = Matrix::scale(2.0, 2.0, 2.0);
//        let mut s = Sphere::new();
//        s.set_transformation(transformation);
//        let shape = Shape::new(ShapeEnum::Sphere(s));
//
//        let transformation_pattern = Matrix::translation(0.5, 0.0, 0.0);
//        let mut pattern = StripePattern::new();
//        pattern.set_transformation(transformation_pattern);
//
//        let p = Tuple4D::new_point(2.5, 0.0, 0.0);
//        let c = StripePattern::color_at_object(&pattern, &shape, &p);
//        assert_color(&c, &WHITE);
//    }
//}
