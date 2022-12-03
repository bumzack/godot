extern crate piston_window;

use std::f32::consts::PI;
use std::path::PathBuf;
use std::time::Instant;

use image::ImageBuffer;
use piston_window::{
    clear, text, Button, Context, G2d, G2dTexture, Glyphs, Key, PistonWindow, PressEvent, Window, WindowSettings,
};
use pretty_env_logger::env_logger::fmt::Color;
use rand::thread_rng;

use render_benny::prelude::{
    Camera, Canvas, CanvasOps, CanvasOpsStd, Matrix, MatrixOps, Mesh, Pixel, Quaternion, RenderContext, Transform,
    Tuple, Tuple4D,
};

use crate::piston_window::EventLoop;
use crate::piston_window::Transformed;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window: PistonWindow = WindowSettings::new("Godot: Renderer", [1280, 720])
        .exit_on_esc(true)
        //.opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap();

    // let assets = find_folder::Search::ParentsThenKids(3, 3)
    //     .for_folder("assets").unwrap();

    let (assets, width, height, texture2, mut monkey_mesh, mut monkey_transform, mut camera) = init();

    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();
    let mut window = window.max_fps(60);

    let mut previous = Instant::now();
    window.set_lazy(true);

    let mut frame = 0;

    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
            println!("S pressed");
        }

        // camera.update(delta.as_secs_f32());
        camera.update(frame as f32);
        let vp = camera.get_view_projection();

        let start = Instant::now();
        monkey_transform = monkey_transform.rotate(Quaternion::new_from_tuple_and_angle(
            Tuple4D::new_vector(0.0, 1.0, 0.0),
            // delta.as_secs_f32(),
            frame as f32,
        ));
        let dur = Instant::now() - start;
        println!("duration rotate {:?}", dur);
        let start = Instant::now();

        let mut target = RenderContext::new(width, height);

        monkey_mesh.draw(&mut target, &vp, &monkey_transform.get_transformation(), &texture2);

        let dur = Instant::now() - start;
        println!("draw mesh rotation {:?}", dur);
        let start = Instant::now();

        let mut buffer: Vec<u8> = vec![0; width * height * 4];

        let pixels = target.canvas().get_pixels();
        for (i, p) in target.canvas().get_pixels().iter().enumerate() {
            let idx = i * 4;
            let c = p.color;
            buffer[idx] = (c.r * 255.0) as u8;
            buffer[idx + 1] = (c.g * 255.0) as u8;
            buffer[idx + 2] = (c.b * 255.0) as u8;
            buffer[idx + 3] = 255;
        }

        target.canvas().get_pixels().iter().for_each(|p| {});

        let dur = Instant::now() - start;
        println!("copy canvas to u8 Vec buffer {:?}", dur);
        let start = Instant::now();

        let img = ImageBuffer::from_raw(
            target.canvas().get_width() as u32,
            target.canvas().get_height() as u32,
            buffer,
        )
        .unwrap();

        let dur = Instant::now() - start;
        println!("creating piston image buffer {:?}", dur);
        let start = Instant::now();

        let texture: piston_window::G2dTexture = piston_window::Texture::from_image(
            &mut window.create_texture_context(),
            &img,
            &piston_window::TextureSettings::new(),
        )
        .unwrap();

        let dur = Instant::now() - start;
        println!("creating piston image G2dTexture {:?}", dur);
        let start = Instant::now();

        frame += 1;

        window.draw_2d(&e, |c, g, device| {
            let transform = c.transform.trans(10.0, 700.0);

            clear([0.0, 0.0, 0.0, 1.0], g);
            // let dur = Instant::now() - start;
            // println!("clear window {:?}", dur);
            // let start = Instant::now();
            //
            // text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
            //     .draw("Hello Godot!", &mut glyphs, &c.draw_state, transform, g)
            //     .unwrap();
            //
            // // Update glyphs before rendering.
            // glyphs.factory.encoder.flush(device);

            let img_transform = c.transform.trans(0.0, 0.0);

            piston_window::image(&texture, img_transform, g);
            let dur = Instant::now() - start;
            println!("drawing image in piston window {:?}", dur);
        });

        let new = Instant::now();
        let duration = new - previous;
        println!(
            "frame {},  got an event.  duration {} ms      ({} us)",
            frame,
            duration.as_millis(),
            duration.as_micros()
        );
        previous = new;
    }
    Ok(())
}

fn init() -> (PathBuf, usize, usize, Canvas, Mesh, Transform, Camera) {
    let assets = PathBuf::from("/Users/bumzack/stoff/rust/godot/renderer_benny/res/");
    println!("{:?}", assets);

    let width = 800;
    let height = 600;

    let texture2 = Canvas::read_bitmap("/Users/bumzack/stoff/rust/godot/renderer_benny/res/bricks2.jpg")
        .expect("could not find asset file");
    let monkey_mesh = Mesh::read_obj_file("/Users/bumzack/stoff/rust/godot/renderer_benny/res/smoothMonkey0.obj")
        .expect("could not find asset file");

    let mut monkey_transform = Transform::new_from_vector(Tuple4D::new_point(0.0, 0.0, 2.0));
    //    let terrain_transform = Transform::new_from_vector(Tuple4D::new_vector(0.0, -1.0, 0.0));
    println!("monkey_transform original = {:?}", monkey_transform);

    let fov = 70.0 * PI / 180.0;
    let aspect_ratio = width as f32 / height as f32;
    let z_near = 0.1;
    let z_far = 10.0;
    let m = Matrix::init_perspective(fov, aspect_ratio, z_near, z_far);
    let mut camera = Camera::new(m);
    (assets, width, height, texture2, monkey_mesh, monkey_transform, camera)
}
