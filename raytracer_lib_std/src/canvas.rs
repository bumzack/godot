#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

use raytracer_lib_no_std::{Color, Pixel};

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
