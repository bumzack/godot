use crate::basics::color::{Color, ColorOps, BLACK, WHITE};
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug, PartialEq)]
pub struct Checker3DPattern {
    color_a: Color,
    color_b: Color,
}

impl Checker3DPattern {
    pub fn new() -> Self {
        Checker3DPattern {
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
        if (point.x.abs() + point.y.abs() + point.z.abs()).floor() as i32 % 2 == 0 {
            Color::from_color(self.get_color_a())
        } else {
            Color::from_color(self.get_color_b())
        }
    }
}

impl Default for Checker3DPattern {
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
        let p = Checker3DPattern::new();
        assert_color(p.get_color_a(), &WHITE);
        assert_color(p.get_color_b(), &BLACK);
    }

    // page 137  x
    #[test]
    fn test_pattern_checker3d_repeat_x() {
        let p = Checker3DPattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = p.pattern_at(&point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.99, 0.0, 0.0);
        let c2 = p.pattern_at(&point2);
        assert_color(&c2, &WHITE);

        let point3 = Tuple4D::new_point(1.01, 0.0, 0.0);
        let c3 = p.pattern_at(&point3);
        assert_color(&c3, &BLACK);
    }

    // page 137  y
    #[test]
    fn test_pattern_checker3d_repeat_y() {
        let p = Checker3DPattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = p.pattern_at(&point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.0, 0.99, 0.0);
        let c2 = p.pattern_at(&point2);
        assert_color(&c2, &WHITE);

        let point3 = Tuple4D::new_point(0.0, 1.01, 0.0);
        let c3 = p.pattern_at(&point3);
        assert_color(&c3, &BLACK);
    }

    // page 137  z
    #[test]
    fn test_pattern_checker3d_repeat_z() {
        let p = Checker3DPattern::new();

        let point1 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let c1 = p.pattern_at(&point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.0, 0.0, 0.99);
        let c2 = p.pattern_at(&point2);
        assert_color(&c2, &WHITE);

        let point3 = Tuple4D::new_point(0.0, 0.0, 1.01);
        let c3 = p.pattern_at(&point3);
        assert_color(&c3, &BLACK);
    }
}
