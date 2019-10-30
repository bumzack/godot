use serde::Deserialize;
use serde::Serialize;

use raytracer_lib_no_std::{BLACK, Color};

pub type PixelVec = Vec<Pixel>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pixel {
    pub color: Color,
    pub x: usize,
    pub y: usize,
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixel {
            color: BLACK,
            x: 0,
            y: 0,
        }
    }
}
