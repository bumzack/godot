use std::fs::File;
use std::io::{Error, Write};

use image::{ImageBuffer, RgbImage};

use raytracer_lib_no_std::Color;

use crate::Pixel;

pub type ColorVec = Vec<Color>;
pub type PixelVec = Vec<Pixel>;

#[derive(Clone, Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixel: PixelVec,
}

pub trait CanvasOps<'a> {
    fn new(width: usize, height: usize) -> Canvas;
    fn write_pixel(&mut self, x: usize, y: usize, c: Color);
    fn pixel_at(&self, x: usize, y: usize) -> &Pixel;

    fn write_ppm(&self, filename: &'a str) -> Result<(), Error>;
    fn write_png(&self, filename: &'a str) -> Result<(), Error>;

    fn calc_idx(&self, x: usize, y: usize) -> usize;

    fn get_pixels_mut(&mut self) -> &mut PixelVec;
}

impl<'a> CanvasOps<'a> for Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        let mut c = Canvas {
            width,
            height,
            pixel: vec![Pixel::new(); width * height],
        };
        for x in 0..width {
            for y in 0..height {
                c.pixel[y * width + x].x = x;
                c.pixel[y * width + x].y = y;
            }
        }

        c
    }

    fn write_pixel(&mut self, x: usize, y: usize, mut c: Color) {
        // println!("write_pixel at {}/{},   width = {}, height = {}", x, y, self.width, self.height);
        assert!(x < self.width);
        assert!(y < self.height);

        self.pixel[y * self.width + x].color = c;
    }

    fn pixel_at(&self, x: usize, y: usize) -> &Pixel {
        assert!(x < self.width);
        assert!(y < self.height);

        &self.pixel[self.calc_idx(x, y)]
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

        // for (x, y, p) in self

        for y in 0..self.height {
            // let mut i = 0;
            let mut row = "".to_owned();
            for x in 0..self.width {
                let c = &self.pixel[self.calc_idx(x, y)];
                row = row.to_owned() + &format!("{} ", (c.color.r * 255.0) as u8);
                // i += 1;
                row = row.to_owned() + &format!("{} ", (c.color.g * 255.0) as u8);
                // i += 1;
                row = row.to_owned() + &format!("{} ", (c.color.b * 255.0) as u8);
                // i += 1;
            }
            row = row.to_owned() + new_line;
            file.write_all(row.as_bytes())?;
        }
        Ok(())
    }

    fn write_png(&self, filename: &'a str) -> Result<(), Error> {
        let mut x = 0;
        let mut y = 0;
        let mut idx = 0;
        let mut image: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);

        for p in self.pixel.iter() {
            let pixel = image::Rgb([
                (p.color.r * 255.0) as u8,
                (p.color.g * 255.0) as u8,
                (p.color.b * 255.0) as u8,
            ]);
            // println!("pixels_vec = {:?}, pixel = {:?}", p, pixel);
            image.put_pixel(x as u32, y as u32, pixel);
            x = x + 1;
            idx = idx + 1;
            if x % self.width == 0 {
                y = y + 1;
                x = 0;
            }
        }
        image.save(filename)
    }

    fn calc_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_pixels_mut(&mut self) -> &mut Vec<Pixel> {
        &mut self.pixel
    }
}

//pub struct EnumeratePixels {
//    pixels: ColorVec,
//    x: u32,
//    y: u32,
//    width: u32,
//}

#[cfg(test)]
mod tests {
    use super::*;

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
}
