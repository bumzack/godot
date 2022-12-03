extern crate piston_window;

use std::f32::consts::PI;
use std::time::Instant;

use image::ImageBuffer;
use render_benny::prelude::{
    Camera, Canvas, CanvasOps, CanvasOpsStd, Matrix, MatrixOps, Mesh, Quaternion, RenderContext, Transform, Tuple,
    Tuple4D,
};

use crate::piston_window::EventLoop;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //    let pos1 = Tuple4D::new_point(1.0, 2.0, 3.0);
    //    let pos2 = Tuple4D::new_point(2.0, 3.0, 4.0);
    //    let pos3 = Tuple4D::new_point(3.0, 4.0, 5.0);
    //    let tex_coords = Tuple4D::new_point(2.0, 2.0, 3.0);
    //    let normal = Tuple4D::new_vector(3.0, 2.0, 3.0);
    //    let v1 = Vertex::new(pos1, tex_coords.clone(), normal.clone());
    //    let v2 = Vertex::new(pos2, tex_coords.clone(), normal.clone());
    //    let v3 = Vertex::new(pos3, tex_coords.clone(), normal.clone());
    //
    //    let gradient = Gradient::new(&v1, &v2, &v3);
    //    let edge = Edge::new(&gradient, &v1, &v3, 0);
    //    println!("edge = {:?}", edge);

    let width = 800;
    let height = 600;

    let texture = Canvas::read_bitmap("/Users/bumzack/stoff/rust/godot/renderer_benny/res/bricks.jpg")?;
    let texture2 = Canvas::read_bitmap("/Users/bumzack/stoff/rust/godot/renderer_benny/res/bricks2.jpg")?;

    let monkey_mesh = Mesh::read_obj_file("/Users/bumzack/stoff/rust/godot/renderer_benny/res/triangle.obj")?;
    let monkey_mesh = Mesh::read_obj_file("/Users/bumzack/stoff/rust/godot/renderer_benny/res/smoothMonkey0.obj")?;
    //    let terrain_mesh = Mesh::read_obj_file("./res/terrain2.obj")?;

    let mut monkey_transform = Transform::new_from_vector(Tuple4D::new_point(0.0, 0.0, 2.0));
    //    let terrain_transform = Transform::new_from_vector(Tuple4D::new_vector(0.0, -1.0, 0.0));
    println!("monkey_transform original = {:?}", monkey_transform);

    //   show_bitmap(&bitmap);

    let fov = 70.0 * PI / 180.0;
    let aspect_ratio = width as f32 / height as f32;
    let z_near = 0.1;
    let z_far = 10.0;
    let m = Matrix::init_perspective(fov, aspect_ratio, z_near, z_far);
    let mut camera = Camera::new(m);

    let mut frame = 0;
    // let rot_counter = 0.0;

    let mut previous_time = Instant::now();

    let mut target = RenderContext::new(width, height);

    while frame < 1 {
        let current_time = Instant::now();
        let delta = (current_time - previous_time) / 1_000_000_000;
        previous_time = current_time;

        let delta = 1;

        // camera.update(delta.as_secs_f32());
        camera.update(delta as f32);
        let vp = camera.get_view_projection();

        monkey_transform = monkey_transform.rotate(Quaternion::new_from_tuple_and_angle(
            Tuple4D::new_vector(0.0, 1.0, 0.0),
            // delta.as_secs_f32(),
            delta as f32,
        ));

        println!(
            "monkey_transform.get_transformation() = {:?}",
            monkey_transform.get_transformation()
        );

        monkey_mesh.draw(&mut target, &vp, &monkey_transform.get_transformation(), &texture2);
        // terrain_mesh.draw(&mut target, &vp, &terrain_transform.get_transformation(), &texture);

        frame += 1;
    }

    let mut cnt = 0;
    target.canvas().get_pixels().iter().for_each(|p| {
        if p.color.r != 0.0 || p.color.b != 0.0 || p.color.g != 0.0 {
            println!("x={}, y ={}, color = {:?}", p.x, p.y, p.color);
            cnt += 1;
        }
    });
    println!("cnt = {}", cnt);
    show_bitmap(&target.canvas());

    Ok(())
}

fn show_bitmap(c: &Canvas) {
    let opengl = piston_window::OpenGL::V3_2;
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new("piston: image", [800, 600])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut buffer: Vec<u8> = vec![];

    c.get_pixels().iter().for_each(|p| {
        buffer.push((p.color.r * 255.0) as u8);
        buffer.push((p.color.g * 255.0) as u8);
        buffer.push((p.color.b * 255.0) as u8);
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
