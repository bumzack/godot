extern crate piston_window;

use std::f32::consts::PI;
use std::path::PathBuf;
use std::process::exit;
use std::thread;
use std::time::{Duration, Instant};

use piston_window::{Button, clear, Key, PistonWindow, PressEvent, text, TextureContext, WindowSettings};

use render_benny::prelude::{Game, MonkeyDisplay};

use crate::piston_window::EventLoop;
use crate::piston_window::Transformed;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let win_width = 1600;
    let win_height = 900;
    let opengl = piston_window::OpenGL::V3_2;
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new("piston: image", [win_width, win_height])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

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
    let mut window = window.max_fps(120);

    let mut previous = Instant::now();
    // window.set_lazy(true);

    let mut frame = 0;
    let mut delta = 0.0;
    let mut frame_duration = Duration::from_micros(0);
    let texture_context = &mut window.create_texture_context();

    let width = game.width;
    let height = game.height;

    let t = game.update(delta, texture_context);

    while let Some(e) = window.next() {
        let start = Instant::now();
        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
            println!("S pressed");
        }

        // crossbeam::scope(|s| {}).expect("crossbeam crash");
        //
        //
        // let texture = crossbeam::scope(|s| {
        //     let mut children = vec![];
        //
        //     for _i in 0..1 {
        //         children.push(s.spawn(move |_| {
        //             println!("     thread_id {:?}", thread::current().id());
        //
        //             let a = game.update(delta, texture_context);
        //
        //             (thread::current().id(), a)
        //         }));
        //     }
        //
        //     let mut res = vec![];
        //     for child in children {
        //         let dur = Instant::now() - start;
        //         let (thread_id, texture) = child.join().unwrap();
        //         res.push(texture);
        //         println!(
        //             "child thread {:?} finished. run for {:?}  ",
        //             thread_id, dur
        //         );
        //     }
        //     let dur = Instant::now() - start;
        //     Ok(res[0])
        // }).expect("thread ended with error");


        let t = game.update(delta, texture_context);

        // match recv_buffer.recv() {
        //     Ok(buffer) => {
        let dur = Instant::now() - start;
        println!("game.update frame {} {:?}", frame, dur);
        let start = Instant::now();

        //  let img = ImageBuffer::from_raw(width as u32, height as u32, buffer).unwrap();

        // let t: piston_window::G2dTexture = piston_window::Texture::from_image(
        //     texture_context,
        //     &img,
        //     &piston_window::TextureSettings::new(),
        // )
        //.unwrap();

        frame += 1;
        delta += 0.2;

        window.draw_2d(&e, |c, g, device| {
            let transform_text = c.transform.trans(10.0, (win_height-30)as f64);

            clear([0.0, 0.0, 0.0, 1.0], g);
            let dur = Instant::now() - start;
            println!("frame {} clear window {:?}", frame, dur);
            let start = Instant::now();

            let s = format!("frame {}, dur {:?}, Hello Godot! ", frame, frame_duration);
            text::Text::new_color([0.0, 1.0, 1.0, 1.0], 32)
                .draw(&s, &mut glyphs, &c.draw_state, transform_text, g)
                .unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);

            let img_transform = c.transform.trans(0.0, 0.0);

            piston_window::image(&t, img_transform, g);
            let dur = Instant::now() - start;
            println!("drawing image in piston window {:?}", dur);
        });
        //     }
        //     Err(e) => panic!("receiving buffer error {}", e),
        // }

        let new = Instant::now();
        frame_duration = new - previous;
        println!(
            "frame {},  got an event.  duration {} ms      ({} us)",
            frame,
            frame_duration.as_millis(),
            frame_duration.as_micros()
        );
        previous = new;
        // if duration.as_millis() > 20 && frame>1 {
        //     exit(1);
        // }
    }
    Ok(())
}
