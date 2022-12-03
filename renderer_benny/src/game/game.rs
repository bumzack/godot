use std::cmp::Ordering;
use std::f32::consts::PI;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crossbeam::thread;
use crossbeam_channel::{Receiver, Sender};
use image::{ImageBuffer, Pixel};
use piston_window::{G2dTexture, G2dTextureContext, TextureContext};

use crate::prelude::{
    Camera, Canvas, CanvasOps, CanvasOpsStd, Matrix, MatrixOps, Mesh, Quaternion, RenderContext, Transform, Tuple,
    Tuple4D,
};

pub trait Game {
    fn start(&mut self);
    fn stop(&mut self);
    fn update(&mut self, delta: f32, ctx: &mut G2dTextureContext)->  G2dTexture;
}

pub struct MonkeyDisplay {
    pub mesh: Mesh,
    pub camera: Camera,
    pub running: bool,
    pub transform: Transform,
    pub width: usize,
    pub height: usize,
    pub texture: Canvas,
    // pub sender_buffer: Sender<Vec<u8>>,
    // pub recv_delta: Receiver<f32>,
    //pub ctx: TextureContext<Factory, Resources, CommandBuffer>,
}

impl Game for MonkeyDisplay {
    fn start(&mut self) {
        self.running = true;
        println!("start");
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn update(&mut self, frame: f32, ctx: &mut G2dTextureContext) ->  G2dTexture{
        self.camera.update(frame);
        let vp = self.camera.get_view_projection();

        let start = Instant::now();
        self.transform = self.transform.rotate(Quaternion::new_from_tuple_and_angle(
            Tuple4D::new_vector(0.0, 1.0, 0.0),
            // delta.as_secs_f32(),
            frame / 20.0,
        ));
        let dur = Instant::now() - start;
        println!("      frame {}  duration rotate {:?}",frame, dur);
        let start = Instant::now();

        let mut target = RenderContext::new(self.width, self.height);

        self.mesh
            .draw(&mut target, &vp, &self.transform.get_transformation(), &self.texture);

        let dur = Instant::now() - start;
        println!("      frame {}    draw mesh rotation {:?}",frame, dur);
        let start = Instant::now();
        let mut buffer: Vec<u8> = vec![0; self.width * self.height * 4];

        let x = target.canvas().get_pixels();
        for (i, p) in x.iter().enumerate() {
            buffer[i] = (p * 255.0) as u8;
        }

        let dur = Instant::now() - start;
        println!("      frame {}   copy canvas to u8 Vec buffer {:?}",frame, dur);
        let start = Instant::now();

        let img = ImageBuffer::from_raw(self.width as u32, self.height as u32, buffer).unwrap();

        // TODO get rid of this, but then the ImageBuffer cant be created
        // let opengl = piston_window::OpenGL::V3_2;
        // let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new("piston: image", [800, 600])
        //     .exit_on_esc(true)
        //     .graphics_api(opengl)
        //     .build()
        //     .unwrap();

        let t: piston_window::G2dTexture = piston_window::Texture::from_image(
            ctx,
            &img,
            &piston_window::TextureSettings::new(),
        )
            .unwrap();

        let dur = Instant::now() - start;
        println!("      frame {}   creating piston image buffer {:?}",frame, dur);
        t
    }
}

impl MonkeyDisplay {
    // pub fn init(ctx: TextureContext<Factory, Resources, CommandBuffer>) -> MonkeyDisplay {
    pub fn init( ) -> MonkeyDisplay {
        let assets = PathBuf::from("/Users/bumzack/stoff/rust/godot/renderer_benny/res/");
        println!("      {:?}", assets);

        let width = 1280;
        let height = 720;

        let width = 320;
        let height = 200;


        let texture = Canvas::read_bitmap("/Users/bumzack/stoff/rust/godot/renderer_benny/res/bricks2.jpg")
            .expect("could not find asset file");
        let monkey_mesh = Mesh::read_obj_file("/Users/bumzack/stoff/rust/godot/renderer_benny/res/smoothMonkey0.obj")
            .expect("could not find asset file");

        let transform = Transform::new_from_vector(Tuple4D::new_point(0.0, 0.0, 2.0));
        //    let terrain_transform = Transform::new_from_vector(Tuple4D::new_vector(0.0, -1.0, 0.0));

        let fov = 70.0 * PI / 180.0;
        let aspect_ratio = width as f32 / height as f32;
        let z_near = 0.1;
        let z_far = 10.0;
        let m = Matrix::init_perspective(fov, aspect_ratio, z_near, z_far);
        let camera = Camera::new(m);

        MonkeyDisplay {
            camera,
            running: false,
            mesh: monkey_mesh,
            transform,
            width: width,
            height: height,
            texture,
            // sender_buffer,
            // recv_delta,
        }
    }
}
