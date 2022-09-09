use std::fs::File;
use std::io::{Error, Write};

use image::{ImageBuffer, ImageError};
use image::RgbImage;

use crate::basics::{Canvas, CanvasOps};

pub trait CanvasOpsStd<'a> {
    fn write_ppm(&self, filename: &'a str) -> Result<(), Error>;
    fn write_png(&self, filename: &'a str) -> Result<(), ImageError>;
}

impl<'a> CanvasOpsStd<'a> for Canvas {
    fn write_ppm(&self, filename: &'a str) -> Result<(), Error> {
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

    fn write_png(&self, filename: &'a str) -> Result<(), ImageError> {
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
}
