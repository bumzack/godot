use crate::prelude::{BLACK, Color};

#[derive(Clone, Debug)]
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
