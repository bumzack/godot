use std::ops::{Add, BitXor, Div, Mul, Neg, Sub};

use crate::math::color::Color;
use crate::math::color::ColorOps;
use crate::math::common::float_equal;

struct Canvas {
    width: usize,
    height: usize,
    pixel: Vec<Color>,
}


trait CanvasOps {
    fn new(width: usize, height: usize) -> Canvas;
}

impl CanvasOps for Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixel: vec![Color::new(0.0, 0.0, 0.); width * height],
        }
    }
}


#[test]
fn test_new_canvas() {
    let c = Canvas::new(10, 20);

    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
}

