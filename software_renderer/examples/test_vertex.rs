extern crate math;
extern crate piston_window;

use crate::piston_window::EventLoop;
use image::ImageBuffer;
use math::{Tuple, Tuple4D};
use raytracer_lib_std::{Canvas, CanvasOps, CanvasOpsStd};
use software_renderer::prelude::{Edge, Gradient, Vertex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pos1 = Tuple4D::new_point(1.0, 2.0, 3.0);
    let pos2 = Tuple4D::new_point(2.0, 3.0, 4.0);
    let pos3 = Tuple4D::new_point(3.0, 4.0, 5.0);
    let tex_coords = Tuple4D::new_point(2.0, 2.0, 3.0);
    let normal = Tuple4D::new_vector(3.0, 2.0, 3.0);
    let v1 = Vertex::new(pos1, tex_coords.clone(), normal.clone());
    let v2 = Vertex::new(pos2, tex_coords.clone(), normal.clone());
    let v3 = Vertex::new(pos3, tex_coords.clone(), normal.clone());

    let gradient = Gradient::new(&v1, &v2, &v3);
    let edge = Edge::new(gradient, v1, v3, 0);
    println!("edge = {:?}", edge);

    let bitmap = Canvas::read_bitmap("./res/bricks.jpg")?;

    show_bitmap(&bitmap);

    Ok(())
}

fn show_bitmap(c: &Canvas) {
    let opengl = piston_window::OpenGL::V3_2;
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut buffer: Vec<u8> = vec![];

    c.get_pixels().iter().for_each(|p| {
        buffer.push(p.color.r as u8);
        buffer.push(p.color.g as u8);
        buffer.push(p.color.b as u8);
        buffer.push(255);
    });

    let img = ImageBuffer::from_raw(c.get_width() as u32, c.get_height() as u32, buffer).unwrap();

    let t: piston_window::G2dTexture = piston_window::Texture::from_image(
        &mut window.create_texture_context(),
        &img,
        &piston_window::TextureSettings::new(),
    )
    .unwrap();

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            piston_window::clear([1.0; 4], g);
            piston_window::image(&t, c.transform, g);
        });
    }
}
