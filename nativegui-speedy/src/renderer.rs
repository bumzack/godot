extern crate crossbeam_channel;
extern crate render_benny;
extern crate speedy2d;
use crate::rayon::iter::IndexedParallelIterator;
use crate::rayon::iter::IntoParallelRefIterator;
use crate::rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefMutIterator;
use std::path::PathBuf;
use std::process::exit;
use std::thread;
use std::time::{Duration, Instant};

use speedy2d::color::Color;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::image::{ImageDataType, ImageHandle, ImageSmoothingMode};
use speedy2d::window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::Graphics2D;

use render_benny::prelude::GameSpeedy;

pub struct MyRenderer {
    pub(crate) image: Option<ImageHandle>,
    pub(crate) start: Instant,
    pub(crate) duration: Duration,
    pub(crate) duration_total: Duration,
    pub(crate) cnt_frames: u32,
    pub(crate) font: Font,
    pub(crate) scene_width: u32,
    pub(crate) scene_height: u32,
    pub(crate) expected_duration_micro_sec: u128,
    pub(crate) game: Box<dyn GameSpeedy>,
    pub(crate) buffer: Vec<u8>,
}

impl WindowHandler for MyRenderer {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let s = Instant::now();
        let render_context = self.game.update(1.0_f32);
        let dur_update = Instant::now() - s;
        log::info!(
            "update took {} μs  == {:6.4} ms",
            dur_update.as_micros(),
            dur_update.as_micros() / 1000
        );
        let pixels = &render_context.canvas().pixels;
        self.buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, pp)| *pp = (pixels[idx] * 255.0) as u8);

        let image = graphics
            .create_image_from_raw_pixels(
                ImageDataType::RGBA,
                ImageSmoothingMode::NearestNeighbor,
                UVec2::new(self.scene_width, self.scene_height),
                &self.buffer,
            )
            .unwrap();
        self.image = Some(image);

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
            "frame {:6.4}, duration {:8.4}, avg {:8.4}, pause {:8.4} = {:8.4} - {:8.4}   (expected_dur - actual_duration)",
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
            "pause {:6.4} = {:6.4} - {:6.4}   (expected_dur - actual_duration)",
            pause,
            self.expected_duration_micro_sec,
            self.duration.as_micros()
        );
        if pause > 0 {
            log::info!("{:6.4}  sleeping {:6.4}", self.cnt_frames, pause);
            thread::sleep(Duration::from_micros(pause as u64));
        } else {
            log::info!("{:6.4}, TOO long {:6.4}", self.cnt_frames, self.duration.as_micros());
        }

        helper.request_redraw();
    }

    fn on_start(&mut self, helper: &mut WindowHelper, info: WindowStartupInfo) {
        log::info!("Got on_start callback: {:?}", info);

        helper.set_cursor_visible(false);
        helper.set_resizable(true);

        let w = self.scene_width;
        let h = self.scene_height;

        let assets = PathBuf::from("/Users/bumzack/stoff/rust/godot/renderer_benny/res/");
        println!("{:?}", assets);

        self.start = Instant::now();
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
}
