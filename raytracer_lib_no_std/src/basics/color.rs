use core::ops::{Add, Div, Mul, Sub};
use core::f32::MAX;

pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };
pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0 };

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub trait ColorOps {
    fn new(r: f32, g: f32, b: f32) -> Color;
    fn from_color(c: &Color) -> Color;
    fn fix_nan(&mut self);
    fn clamp_color(&mut self);
    fn replace_inf_with_max(&mut self);
}

impl ColorOps for Color {
    fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    fn from_color(c: &Color) -> Color {
        Color { r: c.r, g: c.g, b: c.b }
    }

    fn fix_nan(&mut self) {
        if self.r.is_nan() {
            self.r = 0.0;
        }
        if self.g.is_nan() {
            self.g = 0.0;
        }
        if self.b.is_nan() {
            self.b = 0.0;
        }
    }

    fn clamp_color(&mut self) {
        if self.r > 1.0 {
            self.r = 1.0;
        }
        if self.g > 1.0 {
            self.g = 1.0;
        }
        if self.b > 1.0 {
            self.b = 1.0;
        }
    }

    fn replace_inf_with_max(&mut self) {
        if self.r.is_infinite()  {
            self.r = MAX;
        }
        if self.g.is_infinite()  {
            self.g = MAX;
        }
        if self.b.is_infinite()  {
            self.b = MAX;
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl<'a, 'b> Add<&'b Color> for &'a Color {
    type Output = Color;

    fn add(self, other: &'b Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl<'a, 'b> Sub<&'b Color> for &'a Color {
    type Output = Color;

    fn sub(self, other: &'b Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl<'a> Mul<f32> for &'a Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl<'a, 'b> Mul<&'b Color> for &'a Color {
    type Output = Color;

    fn mul(self, rhs: &'b Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Color {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_color, assert_float};

    use super::*;

    #[test]
    fn test_add_color() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let c = c1 + c2;

        assert_float(c.r, 1.6);
        assert_float(c.g, 0.7);
        assert_float(c.b, 1.0);
    }

    #[test]
    fn test_sub_color() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let c = c1 - c2;

        let c_expected = Color::new(0.2, 0.5, 0.5);
        assert_color(&c, &c_expected);
    }

    #[test]
    fn test_mul_color_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        let c = c1 * 2.;

        let c_expected = Color::new(0.4, 0.6, 0.8);
        assert_color(&c, &c_expected);
    }

    #[test]
    fn test_mul_color_color() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let c = c1 * c2;

        let c_expected = Color::new(0.9, 0.2, 0.04);
        assert_color(&c, &c_expected);
    }
}
