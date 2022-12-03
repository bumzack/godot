use serde_derive::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Copy, Deserialize, Serialize, Debug)]
pub struct Tuple4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SceneConfig {
    width: usize,
    height: usize,
    from: Tuple4D,
    to: Tuple4D,
    up: Tuple4D,
    fov: f64,
}

impl SceneConfig {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_from(&self) -> Tuple4D {
        self.from
    }

    pub fn get_to(&self) -> Tuple4D {
        self.to
    }

    pub fn get_up(&self) -> Tuple4D {
        self.up
    }

    pub fn get_fov(&self) -> f64 {
        self.fov
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel {
            r,
            g,
            b,
            a,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            pixels: vec![],
        }
    }

    pub fn set_pixels(&mut self, pixels: Vec<Pixel>) {
        self.pixels = pixels;
    }
}
