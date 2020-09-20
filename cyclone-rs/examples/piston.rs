// based ion tutorial https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::time::Instant;

use glutin_window::GlutinWindow as Window;
use image::ImageBuffer;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use piston::event_loop::{Events, EventSettings};
use piston::input::{
    Button, Key, MouseButton, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key::{Left, Right};
use piston::input::mouse::MouseButton::Button6;
use piston::window::WindowSettings;
use piston_window::TextureSettings;

// use tetris::tetris::action::{TetrisAction, TetrisActionEnum};
// use tetris::tetris::block::Block;
// use tetris::tetris::block::Block::Empty;
// use tetris::tetris::stop_policy::StopPolicy;
// use tetris::tetris::tetris::Tetris;
// use tetris::tetris::tetris_state::TetrisState;

pub struct TetrisApp  {
    gl: GlGraphics,
    // OpenGL drawing backend.
    // tetris: Tetris<'a>,
    texture: Option<Texture>,
    // action: Option<TetrisActionEnum>,
}

impl TetrisApp  {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let start = Instant::now();

        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        if self.texture.is_some() {
            let texture = self.texture.as_ref().unwrap();

            self.gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                clear(BACKGROUND, gl);

                // draw the image in th window
                //   piston_window::image(&t, c.transform, g);
                image(texture, c.transform, gl);
            });
        }

        // println!("render took ms: {}", start.elapsed().as_millis());
    }

    fn update(&mut self, args: &UpdateArgs) {
        println!("update");
        let start = Instant::now();

        // update the environemt with the user action
        // let a = match &self.action {
        //     Some(a) => *a,
        //     None => TetrisActionEnum::NoMove,
        // };

        // let mut action = TetrisAction::new();
        // action.set_action(a);

        // let (new_state, reward) = self.tetris.step(action);
        // // reset action to no action
        // self.action = Some(TetrisActionEnum::NoMove);
        //
        // let texture = TetrisApp::pixels_to_image(&new_state);
        //
        // self.texture = Some(texture);

        println!("tetris.step took ms: {}", start.elapsed().as_millis());
        // println!("game status: {:?}", &new_state.game_status());
    }

    // fn pixels_to_image(new_state: &TetrisState) -> Texture {
    //     // convert out tetris pixel_board to a piston texture
    //     let mut buffer: Vec<u8> = vec![];
    //     ///   let state = self.state.as_ref().unwrap();
    //     new_state.pixel_board().get_pixels().iter().for_each(|p| {
    //         buffer.push(p.r());
    //         buffer.push(p.g());
    //         buffer.push(p.b());
    //         buffer.push(255);
    //     });
    //     let width = new_state.pixel_board().width();
    //     let height = new_state.pixel_board().height();
    //     let img = ImageBuffer::from_raw(width as u32, height as u32, buffer).unwrap();
    //     let texture = Texture::from_image(&img, &TextureSettings::new());
    //     texture
    // }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // let available_blocks = vec![Block::Smashboy, Block::Hero, Block::RhodeIslandZ];
    // let stop_policy = StopPolicy::NumberOfBlocks(5);
    // let mut tetris = Tetris::new(14, 24, 20, stop_policy, available_blocks);

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Something resembling Tetris ", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = TetrisApp {
        gl: GlGraphics::new(opengl),
        // tetris,
        texture: None,
        // action: None,
    };

    let mut frames = 0;
    let mut passed = 0.0;

    let mut events = Events::new(EventSettings::new());

    let mut last_updated = Instant::now();

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            passed += args.dt;
            // println!("args.dt: {}",args.dt);

            if passed > 0.25 {
                let fps = (frames as f64) / passed;

                // println!("FPS: {}", fps);

                frames = 0;
                passed = 0.0;

                app.update(&args);

                println!(
                    "duration since last update (ms) {}",
                    last_updated.elapsed().as_millis()
                );
                last_updated = Instant::now();
            }

            frames += 1;
        }

        if let Some(button) = e.press_args() {
            // match button {
            //     Button::Keyboard(Left) => app.action = Some(TetrisActionEnum::Left),
            //     Button::Keyboard(Right) => app.action = Some(TetrisActionEnum::Right),
            //     _ => app.action = Some(TetrisActionEnum::NoMove),
            // }
        }
    }
}
