use crate::prelude::{Color, ColorOps};
use crate::renderer::pixel::Pixel;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageError, RgbImage};
use std::fs::File;
use std::io::{Error, Write};
pub type ColorVec = Vec<Color>;
pub type PixelVec = Vec<Pixel>;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
pub struct Canvas {
    width: usize,
    height: usize,
    pixel: PixelVec,
}

pub trait CanvasOps {
    fn new(width: usize, height: usize) -> Canvas;
    fn write_pixel(&mut self, x: usize, y: usize, c: Color);
    fn pixel_at(&self, x: usize, y: usize) -> &Pixel;

    fn calc_idx(&self, x: usize, y: usize) -> usize;

    fn get_pixels_mut(&mut self) -> &mut PixelVec;
    fn get_pixels(&self) -> &PixelVec;

    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

impl CanvasOps for Canvas {
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

    fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.pixel[y * self.width + x].color = c;
    }

    fn pixel_at(&self, x: usize, y: usize) -> &Pixel {
        assert!(x < self.width);
        assert!(y < self.height);

        &self.pixel[self.calc_idx(x, y)]
    }

    fn calc_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_pixels_mut(&mut self) -> &mut Vec<Pixel> {
        &mut self.pixel
    }

    fn get_pixels(&self) -> &PixelVec {
        &self.pixel
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
                let c = &self.get_pixels()[self.calc_idx(x, y)];
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
        let mut x = 0;
        let mut y = 0;
        let mut idx = 0;
        let mut image: RgbImage = ImageBuffer::new(self.get_width() as u32, self.get_height() as u32);

        for p in self.get_pixels().iter() {
            let pixel = image::Rgb([
                (p.color.r * 255.0) as u8,
                (p.color.g * 255.0) as u8,
                (p.color.b * 255.0) as u8,
            ]);
            // println!("pixels_vec = {:?}, pixel = {:?}", p, pixel);
            image.put_pixel(x as u32, y as u32, pixel);
            x = x + 1;
            idx = idx + 1;
            if x % self.get_width() == 0 {
                y = y + 1;
                x = 0;
            }
        }
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
