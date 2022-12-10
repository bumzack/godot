extern crate crossbeam_channel;
extern crate speedy2d;

mod raytracer;

use std::path::PathBuf;
use std::process::exit;
use std::thread;
use std::thread::{JoinHandle, ThreadId};
use std::time::{Duration, Instant};
use crossbeam_channel::unbounded;

use simple_logger::SimpleLogger;
use speedy2d::dimen::Vector2;
use speedy2d::font::Font;
use speedy2d::Window;
use raytracer_challenge_reference_impl::prelude::TileData;

use crate::raytracer::MyRaytracer;

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

    let handler = MyRaytracer {
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
