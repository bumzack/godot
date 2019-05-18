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
    fn write_pixel(&mut self, x: usize, y: usize, c: Color);
    fn pixel_at(&self, x: usize, y: usize) -> &Color;
}

impl CanvasOps for Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixel: vec![Color::new(0.5, 0.4, 0.3); width * height],
        }
    }

    fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixel[y * self.width + x] = c;
    }

    fn pixel_at(&self, x: usize, y: usize) -> &Color {
        assert!(x < self.width);
        assert!(y < self.height);

        &self.pixel[y * self.width + x]
    }
}


/// Immutable pixel iterator
//pub struct Pixels<'a, I: 'a> {
//    image: &'a I,
//    x: u32,
//    y: u32,
//    width: u32,
//    height: u32,
//}
//
//impl<'a, I: GenericImage> Iterator for Pixels<'a, I> {
//    type Item = (u32, u32, I::Pixel);
//
//    fn next(&mut self) -> Option<(u32, u32, I::Pixel)> {
//        if self.x >= self.width {
//            self.x = 0;
//            self.y += 1;
//        }
//
//        if self.y >= self.height {
//            None
//        } else {
//            let pixel = self.image.get_pixel(self.x, self.y);
//            let p = (self.x, self.y, pixel);
//
//            self.x += 1;
//
//            Some(p)
//        }
//    }
//}
#[test]
fn test_new_canvas() {
    let c = Canvas::new(10, 20);

    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
}

#[test]
fn test_default_pixel_color() {
    let c = Canvas::new(10, 20);

    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
}

#[test]
fn test_write_pixel() {
    let mut c = Canvas::new(10, 20);
    let red = Color::new(1.0, 0., 0.);
    c.write_pixel(2, 3, red);

    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
}



