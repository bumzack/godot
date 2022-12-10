extern crate crossbeam_channel;
extern crate speedy2d;

use std::path::PathBuf;
use std::process::exit;
use std::thread;
use std::thread::{JoinHandle, ThreadId};
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender, unbounded};
use simple_logger::SimpleLogger;
use speedy2d::{Graphics2D, Window};
use speedy2d::color::Color;
use speedy2d::dimen::{UVec2, Vec2, Vector2};
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::image::{ImageDataType, ImageHandle, ImageSmoothingMode};
use speedy2d::window::{
    KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowHandler, WindowHelper,
    WindowStartupInfo,
};

use raytracer_challenge_reference_impl::basics::{Canvas, TileData};
use raytracer_challenge_reference_impl::example_scenes::chapter07::chapter07;
use raytracer_challenge_reference_impl::example_scenes::test_soft_shadow_multiple_lights::test_soft_shadow_multiple_lights;
use raytracer_challenge_reference_impl::prelude::{Camera, CameraOps, CanvasOps};

struct MyWindowHandler {
    image: Option<ImageHandle>,
    start: Instant,
    duration: Duration,
    font: Font,
    mouse_pos: Vec2,
    mouse_button_down: bool,
    receiver: Receiver<TileData>,
    sender: Sender<TileData>,
    buffer: Vec<u8>,
    render_thread: Option<JoinHandle<ThreadId>>,
    scene_width: u32,
    scene_height: u32,
}

impl WindowHandler for MyWindowHandler {
    fn on_start(&mut self, helper: &mut WindowHelper, info: WindowStartupInfo) {
        log::info!("Got on_start callback: {:?}", info);
        helper.set_cursor_visible(false);
        helper.set_resizable(false);

        let w = self.scene_width;
        let h = self.scene_height;
        let sender = self.sender.clone();
        let render_thread = thread::spawn(move || {
            log::info!("starting renderer thread");
            let (world, camera) = chapter07(w as usize, h as usize);
            Camera::render_multi_core_tile_producer(&camera, &world, 20, 20, sender);
            thread::current().id()
        });

        self.render_thread = Some(render_thread);
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper, position: Vec2) {
        log::info!("Got on_mouse_move callback: ({:.1}, {:.1})", position.x, position.y);

        self.mouse_pos = position;

        helper.request_redraw();
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        log::info!("Got on_mouse_button_down callback: {:?}", button);

        if button == MouseButton::Left {
            self.mouse_button_down = true;
        }

        helper.request_redraw();
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        log::info!("Got on_mouse_button_up callback: {:?}", button);

        if button == MouseButton::Left {
            self.mouse_button_down = false;
        }

        helper.request_redraw();
    }

    fn on_mouse_wheel_scroll(&mut self, _helper: &mut WindowHelper<()>, delta: MouseScrollDistance) {
        log::info!("Got on_mouse_wheel_scroll callback: {:?}", delta);
    }

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        log::info!(
            "Got on_key_down callback: {:?}, scancode {}",
            virtual_key_code,
            scancode
        );
    }

    fn on_key_up(
        &mut self,
        _helper: &mut WindowHelper,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        log::info!("Got on_key_up callback: {:?}, scancode {}", virtual_key_code, scancode);
    }

    fn on_keyboard_char(&mut self, _helper: &mut WindowHelper, unicode_codepoint: char) {
        log::info!("Got on_keyboard_char callback: '{}'", unicode_codepoint);
    }

    fn on_keyboard_modifiers_changed(&mut self, _helper: &mut WindowHelper, state: ModifiersState) {
        log::info!("Got on_keyboard_modifiers_changed callback: {:?}", state);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        // if self.image.is_none() {
        //     let image = graphics
        //         .create_image_from_file_path(
        //             None,
        //             ImageSmoothingMode::NearestNeighbor,
        //             "gallery/chapter15_suzanne_smoothed_3840x2160_with_AA_3_arealight_8x8.png",
        //         )
        //         .unwrap();
        //     helper.set_size_pixels(*image.size() / 2);
        //     self.image = Some(image);
        // }


        let updated = match self.receiver.try_recv() {
            Ok(tile) => {
                println!("got a tile  tile.idx {}", tile.get_idx());
                tile.get_points().iter().for_each(|p| {
                    let idx = (p.get_y() * self.scene_width as usize + p.get_x()) * 4;
                    self.buffer[idx] = (p.get_color().r * 255.0) as u8;
                    self.buffer[idx + 1] = (p.get_color().g * 255.0) as u8;
                    self.buffer[idx + 2] = (p.get_color().b * 255.0) as u8;
                    self.buffer[idx + 3] = 255;
                });

                true
            }
            Err(e) => {
                println!("no tile available");
                false
            }
        };

        if updated {
            let image = graphics.create_image_from_raw_pixels(
                ImageDataType::RGBA,
                ImageSmoothingMode::NearestNeighbor,
                UVec2::new(self.scene_width, self.scene_height),
                &self.buffer,
            ).unwrap();
            self.image = Some(image);
        }

        let ended = Instant::now();
        self.duration = ended - self.start;
        self.start = ended;

        // log::info!("duration {:?}", self.duration);
        graphics.clear_screen(Color::WHITE);

        if self.image.is_some() {
            graphics.draw_image((0.0, 0.0), self.image.as_ref().unwrap());
        }

        let text = self
            .font
            .layout_text(&format!("duration {:?}", self.duration), 64.0, TextOptions::new());

        graphics.draw_text((20.0, 800.0 - 10.0), Color::BLACK, &text);

        helper.request_redraw();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::SimpleLogger::new().init().unwrap();

    let scene_width = 800;
    let scene_height = 600;

    let window_with = scene_width + 40;
    let window_height = scene_height + 100;

    let font = Font::new(include_bytes!("../../res/NotoSans-Regular.ttf")).unwrap();

    let (sender, recv) = unbounded::<TileData>();
    let window = Window::new_centered("Renderer", (window_with, window_height)).unwrap();

    window.run_loop(MyWindowHandler {
        image: None,
        start: Instant::now(),
        duration: Default::default(),
        font,
        mouse_pos: Vector2::ZERO,
        mouse_button_down: false,
        receiver: recv,
        sender,
        buffer: vec![0; (scene_width * scene_height * 4) as usize],
        render_thread: None,
        scene_width,
        scene_height,
    });

    // let scene_width = 800;
    // let scene_height = 600;
    //
    // let render_thread = thread::spawn(move || {
    //     println!("starting renderer thread");
    //
    //     let (world, camera) = test_soft_shadow_multiple_lights(scene_width, scene_height, false, 3);
    //
    //     Camera::render_multi_core_tile_producer(&camera, &world, 20, 20, sender);
    //
    //     thread::current().id()
    // });
    //
    // let win_width = (scene_width + 100) as u32;
    // let win_height = (scene_height + 200) as u32;
    // let opengl = piston_window::OpenGL::V4_5;
    // let mut window: piston_window::PistonWindow =
    //     piston_window::WindowSettings::new("piston: image", [win_width, win_height])
    //         .exit_on_esc(true)
    //         .graphics_api(opengl)
    //         .build()
    //         .unwrap();
    //
    // let fps = 60;
    //
    // let assets = PathBuf::from("/Users/bumzack/stoff/rust/godot/renderer_benny/res/");
    //
    // let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();
    // let mut window = window.max_fps(fps * 2);
    //
    // let mut previous = Instant::now();
    // // window.set_lazy(true);
    //
    // let mut frame = 0;
    // let mut delta = 0.0;
    // let mut frame_duration = Duration::from_micros(0);
    // let texture_context = &mut window.create_texture_context();
    //
    // let mut texture_context = TextureContext {
    //     factory: window.factory.clone(),
    //     encoder: window.factory.create_command_buffer().into(),
    // };
    //
    // let fps_dur = Duration::from_micros(1000 * 1000 / fps);
    // println!("fps_duration {:?},   fps {}", fps_dur, fps);
    //
    // let mut buffer_all: Vec<u8> = vec![0; scene_width * scene_height * 4];
    //
    // while let Some(e) = window.next() {
    //     let elapsed = Instant::now() - previous;
    //     // println!(
    //     //     "elapsed {},  frame duraion {} ",
    //     //     elapsed.as_millis(),
    //     //     fps_dur.as_millis()
    //     // );
    //     if elapsed > fps_dur {
    //         let start = Instant::now();
    //
    //         frame += 1;
    //         delta += 0.01;
    //
    //         window.draw_2d(&e, |c, g, device| {
    //             let mut buffer: Vec<u8> = vec![0; scene_width * scene_height * 4];
    //
    //             let updated = match recv.try_recv() {
    //                 Ok(tile) => {
    //                     println!("got a tile  tile.idx {}", tile.get_idx());
    //
    //                     tile.get_points().iter().for_each(|p| {
    //                         let idx = (p.get_y() * scene_width + p.get_x()) * 4;
    //                         buffer_all[idx] = (p.get_color().r * 255.0) as u8;
    //                         buffer_all[idx + 1] = (p.get_color().g * 255.0) as u8;
    //                         buffer_all[idx + 2] = (p.get_color().b * 255.0) as u8;
    //                         buffer_all[idx + 3] = 255;
    //                     });
    //
    //                     true
    //                 }
    //                 Err(e) => {
    //                     // println!("no tile available")
    //                     false
    //                 }
    //             };
    //
    //             for i in 0..scene_width * scene_height * 4 {
    //                 buffer[i] = buffer_all[i];
    //             }
    //
    //             let transform_text = c.transform.trans(10.0, (win_height - 30) as f64);
    //
    //             clear([0.0, 0.0, 0.0, 1.0], g);
    //             let aa = (Instant::now() - previous).as_millis() as f64;
    //             let act_fps = 1000.0 / aa;
    //
    //             let start = Instant::now();
    //
    //             let s = format!(
    //                 "frame {:00000}, dur {:4.2} , exp fps {:4.2} act fps  {:4.2}    Hello Godot! ",
    //                 frame,
    //                 frame_duration.as_millis(),
    //                 fps,
    //                 act_fps
    //             );
    //             text::Text::new_color([0.0, 1.0, 1.0, 1.0], 32)
    //                 .draw(&s, &mut glyphs, &c.draw_state, transform_text, g)
    //                 .unwrap();
    //
    //             // Update glyphs before rendering.
    //             glyphs.factory.encoder.flush(device);
    //
    //             let img_transform = c.transform.trans(20.0, 20.0);
    //             let img = image::ImageBuffer::from_raw(scene_width as u32, scene_height as u32, buffer).unwrap();
    //             let t: piston_window::G2dTexture = piston_window::Texture::from_image(
    //                 &mut texture_context,
    //                 &img,
    //                 &piston_window::TextureSettings::new(),
    //             )
    //             .unwrap();
    //
    //             piston_window::image(&t, img_transform, g);
    //
    //             let dur = Instant::now() - start;
    //             println!("drawing image in piston window {:?}", dur);
    //         });
    //
    //         let new = Instant::now();
    //         frame_duration = new - previous;
    //         let end = Instant::now();
    //         let work_dur = end - start;
    //         // println!(
    //         //     "frame {},  got an event.  duration {:3.1} ms      ({:10.2} us)    work duration {:4.1}   ",
    //         //     frame,
    //         //     frame_duration.as_millis(),
    //         //     frame_duration.as_micros(),
    //         //     work_dur.as_millis()
    //         // );
    //         previous = new;
    //     }
    //
    //     if let Some(Button::Keyboard(Key::S)) = e.press_args() {
    //         println!("S pressed");
    //     }
    // }
    // let res = render_thread.join();
    //
    // println!("render thread finished   {:?} ", res.unwrap());

    Ok(())
}
