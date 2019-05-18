use crate::math::canvas::{Canvas, CanvasOps};
use crate::math::color::Color;
use crate::math::color::ColorOps;

mod math;

fn main() {
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
