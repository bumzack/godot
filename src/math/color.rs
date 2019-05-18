use std::ops::{Add, BitXor, Div, Mul, Neg, Sub};

use crate::math::common::float_equal;

struct Color {
    r: f32,
    g: f32,
    b: f32,
}


trait ColorOps {
    fn new(r: f32, g: f32, b: f32) -> Color;
}

impl ColorOps for Color {
    fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            r,
            g,
            b,
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


#[test]
fn test_add_color() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    let c = c1 + c2;

    assert_eq!(float_equal(c.r, 1.6), true);
    assert_eq!(float_equal(c.g, 0.7), true);
    assert_eq!(float_equal(c.b, 1.0), true);
}

#[test]
fn test_sub_color() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    let c = c1 - c2;

    assert_eq!(float_equal(c.r, 0.2), true);
    assert_eq!(float_equal(c.g, 0.5), true);
    assert_eq!(float_equal(c.b, 0.5), true);
}

#[test]
fn test_mul_color_scalar() {
    let c1 = Color::new(0.2, 0.3, 0.4);
    let c = c1 * 2.;

    assert_eq!(float_equal(c.r, 0.4), true);
    assert_eq!(float_equal(c.g, 0.6), true);
    assert_eq!(float_equal(c.b, 0.8), true);
}

#[test]
fn test_mul_color_color() {
    let c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);
    let c = c1 * c2;

    assert_eq!(float_equal(c.r, 0.9), true);
    assert_eq!(float_equal(c.g, 0.2), true);
    assert_eq!(float_equal(c.b, 0.04), true);
}

