extern crate crossbeam_channel;
extern crate image;
extern crate piston_window;

use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use crossbeam_channel::unbounded;
use piston_window::{clear, text, Button, Key, PressEvent, TextureContext};

use raytracer_challenge_reference_impl::basics::TileData;
use raytracer_challenge_reference_impl::example_scenes::chapter07::chapter07;
use raytracer_challenge_reference_impl::prelude::{Camera, CameraOps, CanvasOps};

use crate::piston_window::EventLoop;
use crate::piston_window::Transformed;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, recv) = unbounded::<TileData>();

    let scene_width = 640;
    let scene_height = 480;

    let render_thread = thread::spawn(move || {
        println!("starting renderer thread");
        let (world, camera) = chapter07(scene_width, scene_height);
        Camera::render_multi_core_tile_producer(&camera, &world, 5, 5, sender);
        thread::current().id()
    });

    let win_width = 1080;
    let win_height = 600;
    let opengl = piston_window::OpenGL::V4_5;
    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("piston: image", [win_width, win_height])
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();

    let fps = 25;

    let assets = PathBuf::from("/Users/bumzack/stoff/rust/godot/renderer_benny/res/");

    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();
    let mut window = window.max_fps(fps * 4);

    let mut previous = Instant::now();
    // window.set_lazy(true);

    let mut frame = 0;
    let mut delta = 0.0;
    let mut frame_duration = Duration::from_micros(0);
    let texture_context = &mut window.create_texture_context();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let fps_dur = Duration::from_micros(1000 * 1000 / fps);
    println!("fps_duration {:?},   fps {}", fps_dur, fps);

    let mut buffer_all: Vec<u8> = vec![0; scene_width * scene_height * 4];

    while let Some(e) = window.next() {
        let elapsed = Instant::now() - previous;
        // println!(
        //     "elapsed {},  frame duraion {} ",
        //     elapsed.as_millis(),
        //     fps_dur.as_millis()
        // );
        if elapsed > fps_dur {
            let start = Instant::now();

            frame += 1;
            delta += 0.01;

            window.draw_2d(&e, |c, g, device| {
                let mut buffer: Vec<u8> = vec![0; scene_width * scene_height * 4];

                let updated = match recv.try_recv() {
                    Ok(tile) => {
                        println!("got a tile  tile.idx {}", tile.get_idx());

                        tile.get_points().iter().for_each(|p| {
                            let idx = (p.get_y() * scene_width + p.get_x()) * 4;
                            buffer_all[idx] = (p.get_color().r * 255.0) as u8;
                            buffer_all[idx + 1] = (p.get_color().g * 255.0) as u8;
                            buffer_all[idx + 2] = (p.get_color().b * 255.0) as u8;
                            buffer_all[idx + 3] = 255;
                        });

                        true
                    }
                    Err(_e) => {
                        // println!("no tile available")
                        false
                    }
                };

                for i in 0..scene_width * scene_height * 4 {
                    buffer[i] = buffer_all[i];
                }

                let transform_text = c.transform.trans(10.0, (win_height - 30) as f64);

                clear([0.0, 0.0, 0.0, 1.0], g);
                let aa = (Instant::now() - previous).as_millis() as f64;
                let act_fps = 1000.0 / aa;

                let start = Instant::now();

                let s = format!(
                    "frame {:00000}, dur {:4.2} , exp fps {:4.2} act fps  {:4.2}    Hello Godot! ",
                    frame,
                    frame_duration.as_millis(),
                    fps,
                    act_fps
                );
                text::Text::new_color([0.0, 1.0, 1.0, 1.0], 32)
                    .draw(&s, &mut glyphs, &c.draw_state, transform_text, g)
                    .unwrap();

                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);

                let img_transform = c.transform.trans(20.0, 20.0);
                let img = image::ImageBuffer::from_raw(scene_width as u32, scene_height as u32, buffer).unwrap();
                let t: piston_window::G2dTexture = piston_window::Texture::from_image(
                    &mut texture_context,
                    &img,
                    &piston_window::TextureSettings::new(),
                )
                .unwrap();

                piston_window::image(&t, img_transform, g);

                let dur = Instant::now() - start;
                println!("drawing image in piston window {:?}", dur);
            });

            let new = Instant::now();
            frame_duration = new - previous;
            let end = Instant::now();
            let work_dur = end - start;
            // println!(
            //     "frame {},  got an event.  duration {:3.1} ms      ({:10.2} us)    work duration {:4.1}   ",
            //     frame,
            //     frame_duration.as_millis(),
            //     frame_duration.as_micros(),
            //     work_dur.as_millis()
            // );
            previous = new;
        }

        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
            println!("S pressed");
        }
    }
    let res = render_thread.join();

    println!("render thread finished   {:?} ", res.unwrap());

    Ok(())
}
