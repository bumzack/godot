use std::fs::File;
use std::io::{Error, Write};

use image::{ImageBuffer, ImageError, RgbImage};

use crate::prelude::{Color, ColorOps};
use crate::renderer::pixel::Pixel;

pub type ColorVec = Vec<Color>;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
pub struct Canvas {
    width: usize,
    height: usize,
    pub pixels: Vec<f32>,
}

pub trait CanvasOps {
    fn new(width: usize, height: usize) -> Self;
    fn write_pixel(&mut self, x: usize, y: usize, c: Color);
    fn pixel_at(&self, x: usize, y: usize) -> Pixel;

    fn calc_idx(&self, x: usize, y: usize) -> usize;

    fn get_pixels_mut(&mut self) -> &mut Vec<f32>;
    fn get_pixels(&self) -> &Vec<f32>;

    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

impl CanvasOps for Canvas {
    fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![0.0; width * height * 4],
        }
    }

    fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        assert!(x < self.width);
        assert!(y < self.height);

        let idx = self.calc_idx(x, y);
        self.pixels[idx] = c.r;
        self.pixels[idx + 1] = c.g;
        self.pixels[idx + 2] = c.b;
        self.pixels[idx + 3] = 1.0;
    }

    fn pixel_at(&self, x: usize, y: usize) -> Pixel {
        assert!(x < self.width);
        assert!(y < self.height);

        let idx = y * self.width + x;
        Pixel {
            color: Color {
                r: self.pixels[idx],
                g: self.pixels[idx + 1],
                b: self.pixels[idx + 2],
            },
            x,
            y,
        }
    }

    fn calc_idx(&self, x: usize, y: usize) -> usize {
        (y * self.width + x) * 4
    }

    fn get_pixels_mut(&mut self) -> &mut Vec<f32> {
        &mut self.pixels
    }

    fn get_pixels(&self) -> &Vec<f32> {
        &self.pixels
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}

pub trait CanvasOpsStd {
    fn write_ppm(&self, filename: &str) -> Result<(), Error>;
    fn write_png(&self, filename: &str) -> Result<(), ImageError>;
    fn read_bitmap(filename: &str) -> Result<Canvas, Box<dyn std::error::Error>>;
}

impl CanvasOpsStd for Canvas {
    fn write_ppm(&self, filename: &str) -> Result<(), Error> {
        let mut file = File::create(filename)?;

        let new_line = "\n";

        let w = format!("{}", self.get_width());
        let h = format!("{}", self.get_height());
        let s = w + " " + &h + new_line;

        let header = String::from("P3") + new_line;
        let max_pxiel_value = String::from("255") + new_line;

        file.write_all(header.as_bytes())?;
        file.write_all(s.as_bytes())?;
        file.write_all(max_pxiel_value.as_bytes())?;

        // for (x, y, p) in self

        for y in 0..self.get_height() {
            // let mut i = 0;
            let mut row = "".to_owned();
            for x in 0..self.get_width() {
                let c = &self.pixel_at(x, y);
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

    fn write_png(&self, filename: &str) -> Result<(), ImageError> {
        let _x = 0;
        let _y = 0;
        let _idx = 0;
        let image: RgbImage = ImageBuffer::new(self.get_width() as u32, self.get_height() as u32);

        // for p in self.get_pixels().iter() {
        //     let pixel = image::Rgb([
        //         (p..r * 255.0) as f32,
        //         (p.color.g * 255.0) as u8,
        //         (p.color.b * 255.0) as u8,
        //     ]);
        //     // println!("pixels_vec = {:?}, pixel = {:?}", p, pixel);
        //     image.put_pixel(x as u32, y as u32, pixel);
        //     x = x + 1;
        //     idx = idx + 1;
        //     if x % self.get_width() == 0 {
        //         y = y + 1;
        //         x = 0;
        //     }
        // }
        image.save(filename)
    }

    fn read_bitmap(filename: &str) -> Result<Canvas, Box<dyn std::error::Error>> {
        let img = image::open(filename)?;
        let img = img.as_rgb8().unwrap();

        let (width, height) = img.dimensions();
        let mut c = Canvas::new(width as usize, height as usize);
        for (x, y, pixel) in img.enumerate_pixels() {
            let color = Color::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);
            c.write_pixel(x as usize, y as usize, color);
        }
        Ok(c)
    }
}
