extern crate crossbeam_channel;
extern crate speedy2d;

use std::path::PathBuf;
use std::process::exit;
use std::thread;
use std::thread::{JoinHandle, ThreadId};
use std::time::{Duration, Instant};

use crossbeam_channel::{unbounded, Receiver, Sender};
use simple_logger::SimpleLogger;
use speedy2d::color::Color;
use speedy2d::dimen::{UVec2, Vec2, Vector2};
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::image::{ImageDataType, ImageHandle, ImageSmoothingMode};
use speedy2d::window::{
    KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowHandler, WindowHelper,
    WindowStartupInfo,
};
use speedy2d::{Graphics2D, Window};

use raytracer_challenge_reference_impl::basics::{Canvas, TileData};
use raytracer_challenge_reference_impl::example_scenes::chapter07::chapter07;
use raytracer_challenge_reference_impl::example_scenes::test_soft_shadow_multiple_lights::test_soft_shadow_multiple_lights;
use raytracer_challenge_reference_impl::prelude::{Camera, CameraOps, CanvasOps};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().unwrap();

    let scene_width = 3840;
    let scene_height = 2160;

    let window_with = scene_width + 40;
    let window_height = scene_height + 100;

    let font = Font::new(include_bytes!("../../res/NotoSans-Regular.ttf")).unwrap();

    let (sender, receiver) = unbounded::<TileData>();
    let window = Window::new_centered("Renderer", (window_with, window_height)).unwrap();

    let fps = 60;
    let dur = (1.0 / fps as f32) * 1000.0 * 1000.0;
    let expected_duration_micro_sec = dur as u128;

    println!("duration {}", expected_duration_micro_sec);

    let handler = MyWindowHandler {
        image: None,
        start: Instant::now(),
        duration: Default::default(),
        duration_total: Default::default(),
        cnt_frames: 0,
        font,
        mouse_pos: Vector2::ZERO,
        mouse_button_down: false,
        receiver,
        sender,
        buffer: vec![0; (scene_width * scene_height * 4) as usize],
        render_thread: None,
        scene_width,
        scene_height,
        expected_duration_micro_sec,
    };
    window.run_loop(handler);
}

struct MyWindowHandler {
    image: Option<ImageHandle>,
    start: Instant,
    duration: Duration,
    duration_total: Duration,
    cnt_frames: u32,
    font: Font,
    mouse_pos: Vec2,
    mouse_button_down: bool,
    receiver: Receiver<TileData>,
    sender: Sender<TileData>,
    buffer: Vec<u8>,
    render_thread: Option<JoinHandle<ThreadId>>,
    scene_width: u32,
    scene_height: u32,
    expected_duration_micro_sec: u128,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        // self.start = Instant::now();
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
            println!("updated  == true  ");
            let image = graphics
                .create_image_from_raw_pixels(
                    ImageDataType::RGBA,
                    ImageSmoothingMode::NearestNeighbor,
                    UVec2::new(self.scene_width, self.scene_height),
                    &self.buffer,
                )
                .unwrap();
            self.image = Some(image);
        }

        // log::info!("duration {:?}", self.duration);
        graphics.clear_screen(Color::WHITE);

        if self.image.is_some() {
            log::info!("drawing image");
            graphics.draw_image((0.0, 0.0), self.image.as_ref().unwrap());
        }

        let ended = Instant::now();
        self.duration = ended - self.start;
        self.start = ended;
        self.cnt_frames += 1;
        self.duration_total += self.duration;

        let avg = self.duration_total.as_micros() / self.cnt_frames as u128;
        let pause = if self.expected_duration_micro_sec > self.duration.as_micros() {
            self.expected_duration_micro_sec - self.duration.as_micros()
        } else {
            0
        };
        let dur = format!(
            "frame {}, duration {:4.2}, avg {:4.2}, pause {:4.2} = {:4.2} - {:4.2}   (expected_dur - actual_duration)",
            self.cnt_frames,
            self.duration.as_micros(),
            avg,
            pause,
            self.expected_duration_micro_sec,
            self.duration.as_micros()
        );

        log::info!("{}", &dur);
        let text = self.font.layout_text(&dur, 32.0, TextOptions::new());

        let test_pos = (20.0, self.scene_height as f32 + 30.0);
        let test_pos = (10.0, 10.0);
        graphics.draw_text(test_pos, Color::BLACK, &text);

        log::info!(
            "pause {} = {} - {}   (expected_dur - actual_duration)",
            pause,
            self.expected_duration_micro_sec,
            self.duration.as_micros()
        );
        if pause > 0 {
            log::info!("{}  sleeping {}", self.cnt_frames, pause);
            thread::sleep(Duration::from_micros(pause as u64));
        } else {
            log::info!("{}, TOO long {}", self.cnt_frames, self.duration.as_micros());
        }

        helper.request_redraw();
    }

    fn on_start(&mut self, helper: &mut WindowHelper, info: WindowStartupInfo) {
        log::info!("Got on_start callback: {:?}", info);
        helper.set_cursor_visible(false);
        helper.set_resizable(true);

        let w = self.scene_width;
        let h = self.scene_height;
        let sender = self.sender.clone();
        let render_thread = thread::spawn(move || {
            log::info!("starting renderer thread");
            let (world, camera) = chapter07(w as usize, h as usize);
            Camera::render_multi_core_tile_producer(&camera, &world, 20, 20, sender);
            thread::current().id()
        });

        self.start = Instant::now();
        self.render_thread = Some(render_thread);
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper, position: Vec2) {
        log::info!("Got on_mouse_move callback: ({:.1}, {:.1})", position.x, position.y);
        self.mouse_pos = position;
        // helper.request_redraw();
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        log::info!("Got on_mouse_button_down callback: {:?}", button);

        if button == MouseButton::Left {
            self.mouse_button_down = true;
        }

        // helper.request_redraw();
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        log::info!("Got on_mouse_button_up callback: {:?}", button);

        if button == MouseButton::Left {
            self.mouse_button_down = false;
        }

        // helper.request_redraw();
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
        if virtual_key_code == Some(VirtualKeyCode::Escape) {
            exit(0);
        }
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
}
