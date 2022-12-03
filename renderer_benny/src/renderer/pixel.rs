use crate::prelude::{Color, BLACK};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
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
