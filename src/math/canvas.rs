use std::fs::File;
use std::io::{Error, Write};

use crate::math::color::Color;
use crate::math::color::ColorOps;

#[derive(Clone, Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixel: Vec<Color>,
}

pub trait CanvasOps<'a> {
    fn new(width: usize, height: usize) -> Canvas;
    fn write_pixel(&mut self, x: usize, y: usize, c: Color);
    fn pixel_at(&self, x: usize, y: usize) -> &Color;
    fn write_ppm(&self, filename: &'a str) -> Result<(), Error>;

    fn calc_idx(&self, x: usize, y: usize) -> usize;
}

impl<'a> CanvasOps<'a> for Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixel: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixel[y * self.width + x] = c;
    }

    fn pixel_at(&self, x: usize, y: usize) -> &Color {
        assert!(x < self.width);
        assert!(y < self.height);

        &self.pixel[self.calc_idx(x, y)]
    }
    fn calc_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn write_ppm(&self, filename: &'a str) -> Result<(), Error> {
        let mut file = File::create(filename)?;

        let new_line = "\n";

        let w = format!("{}", self.width);
        let h = format!("{}", self.height);
        let s = w + " " + &h + new_line;

        let header = String::from("P3") + new_line;
        let max_pxiel_value = String::from("255") + new_line;

        file.write_all(header.as_bytes())?;
        file.write_all(s.as_bytes())?;
        file.write_all(max_pxiel_value.as_bytes())?;

        for y in 0..self.height {
            let mut i = 0;
            let mut row = "".to_owned();
            for x in 0..self.width {
                let c = &self.pixel[self.calc_idx(x, y)];
                row = row.to_owned() + &format!("{} ", (c.r * 255.0) as u8);
                i += 1;
                row = row.to_owned() + &format!("{} ", (c.g * 255.0) as u8);
                i += 1;
                row = row.to_owned() + &format!("{} ", (c.b * 255.0) as u8);
                i += 1;
            }
            row = row.to_owned() + new_line;
            file.write_all(row.as_bytes())?;
        }
        Ok(())
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

#[test]
fn test_write_ppm() {
    let mut c = Canvas::new(10, 10);
    let red = Color::new(1.0, 0.0, 0.0);
    let green = Color::new(0.0, 1.0, 0.0);
    c.write_pixel(0, 0, red.clone());
    c.write_pixel(1, 1, red.clone());
    c.write_pixel(2, 2, red.clone());
    c.write_pixel(3, 3, red.clone());
    c.write_pixel(4, 4, red);
    c.write_pixel(5, 5, green.clone());
    c.write_pixel(6, 6, green.clone());
    c.write_pixel(7, 7, green.clone());
    c.write_pixel(8, 8, green.clone());
    c.write_pixel(9, 9, green);

    c.write_ppm("test_output.ppm");
}



