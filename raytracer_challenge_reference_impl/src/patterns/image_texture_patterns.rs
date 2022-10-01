use crate::basics::color::Color;
use crate::basics::{Canvas, CanvasOps};
  use crate::math::tuple4d::Tuple4D;
use crate::patterns::spherical_map;

#[derive(PartialEq, Debug, Clone)]
pub struct ImageTexturePattern {
    image: Canvas,
}

impl ImageTexturePattern {
    pub fn new(image: Canvas) -> ImageTexturePattern {
        ImageTexturePattern { image }
    }

    pub fn pattern_at(&self, p: &Tuple4D) -> Color {
        let (u, v) = spherical_map(p);
        ImageTexturePattern::uv_pattern_at(&self.image, u, v)
    }

    pub fn uv_pattern_at(image: &Canvas, u: f64, v: f64) -> Color {
        let v = 1.0 - v;
        let x = u * (image.get_width() as f64 - 1.0);
        let y = v * (image.get_height() as f64 - 1.0);
        let color = image.pixel_at(x as usize, y as usize).color;
        println!(
            "x  = {}, y =  {}    color = {:?}      w {}, h {}",
            x,
            y,
            color,
            image.get_width(),
            image.get_height()
        );
        image.pixel_at(x as usize, y as usize).color
    }
}
