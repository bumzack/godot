use std::f32::consts::PI;
use std::path::PathBuf;
use std::time::Instant;

use crate::prelude::{Camera, Canvas, CanvasOpsStd, Matrix, MatrixOps, Mesh, Quaternion, RenderContext, Transform, Tuple, Tuple4D};

pub trait GameSpeedy {
    fn start(&mut self);
    fn stop(&mut self);
    fn update(&mut self, delta: f32) -> RenderContext;
}

pub struct MonkeyDisplaySpeedy {
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

impl GameSpeedy for MonkeyDisplaySpeedy {
    fn start(&mut self) {
        self.running = true;
        println!("start");
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn update(&mut self, frame: f32) -> RenderContext {
        self.camera.update(frame);
        let vp = self.camera.get_view_projection();

        let start = Instant::now();
        self.transform = self.transform.rotate(Quaternion::new_from_tuple_and_angle(
            Tuple4D::new_vector(0.0, 1.0, 0.0),
            // delta.as_secs_f32(),
            frame / 20.0,
        ));
        let _dur = Instant::now() - start;
        //  println!("      frame {}  duration rotate {:?}",frame, dur);
        let start = Instant::now();

        println!("target  width {} height {}", self.width, self.height);
        let mut target = RenderContext::new(self.width, self.height);

        self.mesh
            .draw(&mut target, &vp, &self.transform.get_transformation(), &self.texture);

        let _dur = Instant::now() - start;

        target
    }
}

impl MonkeyDisplaySpeedy {
    pub fn init(width: usize, height: usize) -> MonkeyDisplaySpeedy {
        let assets = PathBuf::from("/Users/bumzack/stoff/rust/godot/renderer_benny/res/");
        println!("      {:?}", assets);

        // let width = 1280;
        // let height = 720;

        // let width = 320;
        // let height = 200;

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

        MonkeyDisplaySpeedy {
            camera,
            running: false,
            mesh: monkey_mesh,
            transform,
            width,
            height,
            texture,
        }
    }
}