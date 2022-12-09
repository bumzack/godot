use crate::basics::color::{Color, BLACK, WHITE};
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug, PartialEq)]
pub struct GradientPattern {
    color_a: Color,
    color_b: Color,
}

impl GradientPattern {
    pub fn new() -> GradientPattern {
        GradientPattern {
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
        let distance = self.get_color_b() - self.get_color_a();
        let fraction = point.x - point.x.floor();
        self.get_color_a() + &(distance * fraction)
    }
}

impl Default for GradientPattern {
    fn default() -> Self {
        Self::new()
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
        let c1 = p.pattern_at(&point1);
        assert_color(&c1, &WHITE);

        let point2 = Tuple4D::new_point(0.25, 0.0, 0.0);
        let c2 = p.pattern_at(&point2);
        let c2_expected = Color::new(0.75, 0.75, 0.75);
        assert_color(&c2, &c2_expected);

        let point3 = Tuple4D::new_point(0.5, 0.0, 0.0);
        let c3 = p.pattern_at(&point3);
        let c3_expected = Color::new(0.5, 0.5, 0.5);
        assert_color(&c3, &c3_expected);

        let point4 = Tuple4D::new_point(0.75, 0.0, 0.0);
        let c4 = p.pattern_at(&point4);
        let c4_expected = Color::new(0.25, 0.25, 0.25);
        assert_color(&c4, &c4_expected);
    }
}
