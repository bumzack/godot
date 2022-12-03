extern crate piston_window;

use std::f32::consts::PI;
use std::path::PathBuf;
use std::process::exit;
use std::thread;
use std::time::{Duration, Instant};

use piston_window::math::mul;
use piston_window::{clear, text, Button, Key, PistonWindow, PressEvent, TextureContext, WindowSettings};

use render_benny::prelude::{Game, MonkeyDisplay};

use crate::piston_window::EventLoop;
use crate::piston_window::Transformed;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let win_width = 1600;
    let win_height = 900;
    let opengl = piston_window::OpenGL::V4_5;
    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("piston: image", [win_width, win_height])
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();

    let fps = 90;

    //
    // let texture_context = TextureContext {
    //     factory: window.factory.clone(),
    //     encoder: window.factory.create_command_buffer().into(),
    // };

    // let (sender, recv_buffer) = unbounded::<Vec<u8>>();
    // let (sender_delta, recv_delta) = unbounded::<f32>();

    let mut game = MonkeyDisplay::init();
    game.start();

    let assets = PathBuf::from("/Users/bumzack/stoff/rust/godot/renderer_benny/res/");
    println!("{:?}", assets);

    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();
    let mut window = window.max_fps(fps * 4);

    let mut previous = Instant::now();
    // window.set_lazy(true);

    let mut frame = 0;
    let mut delta = 0.0;
    let mut frame_duration = Duration::from_micros(0);
    let texture_context = &mut window.create_texture_context();

    let fps_dur = Duration::from_micros(1000 * 1000 / fps);
    println!("fps_duration {:?},   fps {}", fps_dur, fps);
    while let Some(e) = window.next() {
        let elapsed = Instant::now() - previous;
        println!(
            "elapsed {},  frame duraion {} ",
            elapsed.as_millis(),
            fps_dur.as_millis()
        );
        if elapsed > fps_dur {
            let start = Instant::now();
            let t = game.update(delta, texture_context);

            frame += 1;
            delta += 0.01;

            window.draw_2d(&e, |c, g, device| {
                let transform_text = c.transform.trans(10.0, (win_height - 30) as f64);

                clear([0.0, 0.0, 0.0, 1.0], g);
                let a = (Instant::now() - previous).as_millis() as f64;
                let act_fps = 1000.0 / a;

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

                let img_transform = c.transform.trans(0.0, 0.0);

                piston_window::image(&t, img_transform, g);
                let dur = Instant::now() - start;
                // println!("drawing image in piston window {:?}", dur);
            });

            let new = Instant::now();
            frame_duration = new - previous;
            let end = Instant::now();
            let work_dur = end - start;
            println!(
                "frame {},  got an event.  duration {:3.1} ms      ({:10.2} us)    work duration {:4.1}   ",
                frame,
                frame_duration.as_millis(),
                frame_duration.as_micros(),
                work_dur.as_millis()
            );
            previous = new;
        }

        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
            println!("S pressed");
        }
    }
    Ok(())
}
