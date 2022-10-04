use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crossbeam_channel::{Sender, unbounded};

use crate::basics::{CanvasOps, CanvasOpsStd, Color, Tile};
use crate::basics::canvas::Canvas;
use crate::basics::color::BLACK;
use crate::basics::ray::Ray;
use crate::basics::ray::RayOps;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;
use crate::world::world::{MAX_REFLECTION_RECURSION_DEPTH, World};
use crate::world::world::WorldOps;

#[derive(Clone, Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Matrix,
    half_view: f64,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    antialiasing: bool,
    antialiasing_size: usize, // 2 or 3
}

pub trait CameraOps {
    fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self;

    fn get_hsize(&self) -> usize;
    fn get_vsize(&self) -> usize;
    fn get_field_of_view(&self) -> f64;
    fn get_transform(&self) -> &Matrix;
    fn get_pixel_size(&self) -> f64;
    fn get_half_width(&self) -> f64;
    fn get_half_height(&self) -> f64;

    fn set_antialiasing(&mut self, aa: bool);
    fn get_antialiasing(&self) -> bool;

    fn set_antialiasing_size(&mut self, size: usize);
    fn get_antialiasing_size(&self) -> usize;

    fn calc_pixel_size(&mut self);

    fn set_transformation(&mut self, m: Matrix);

    fn ray_for_pixel(c: &Camera, x: usize, y: usize) -> Ray;
    fn ray_for_pixel_anti_aliasing(c: &Camera, x: usize, y: usize, x_offset: f64, y_offset: f64) -> Ray;

    fn render(c: &Camera, w: &World) -> Canvas;
    fn render_multi_core(c: &Camera, w: &World) -> Canvas;
    fn render_multi_core_tiled(c: &Camera, w: &World, x_tiles: usize, y_tiles: usize) -> Canvas;
    fn render_multi_core_tiled_sender(ca: &Camera, wo: &World, x_tiles: usize, y_tiles: usize, s: Sender<Canvas>)-> Canvas;
    fn render_debug(c: &Camera, w: &World, x: usize, y: usize) -> Canvas;
}

impl CameraOps for Camera {
    fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::new_identity_4x4(),
            half_view: 0.0,
            half_width: 0.0,
            half_height: 0.0,
            pixel_size: 0.0,
            antialiasing: false,
            antialiasing_size: 2,
        }
    }

    fn get_hsize(&self) -> usize {
        self.hsize
    }

    fn get_vsize(&self) -> usize {
        self.vsize
    }

    fn get_field_of_view(&self) -> f64 {
        self.field_of_view
    }

    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn calc_pixel_size(&mut self) {
        self.half_view = (self.field_of_view / 2.0).tan();
        let aspect = self.hsize as f64 / (self.vsize as f64);

        if aspect >= 1.0 {
            self.half_width = self.half_view;
            self.half_height = self.half_view / aspect;
        } else {
            self.half_width = self.half_view * aspect;
            self.half_height = self.half_view;
        }

        self.pixel_size = self.half_width * 2.0 / (self.hsize as f64);
    }

    fn get_pixel_size(&self) -> f64 {
        self.pixel_size
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.transform = m;
    }

    fn ray_for_pixel(c: &Camera, x: usize, y: usize) -> Ray {
        let camera_transform_inv =
            Matrix::invert(c.get_transform()).expect("ray_for_pixel:  cant calculate the inverse");

        let x_offset = (x as f64 + 0.5) * c.get_pixel_size();
        let y_offset = (y as f64 + 0.5) * c.get_pixel_size();

        let world_x = c.get_half_width() - x_offset;
        let world_y = c.get_half_height() - y_offset;
        // println!("no AA    (x/y) = ({}/{})  world_point = ({}/{})", x, y, world_x, world_y);

        let p = Tuple4D::new_point(world_x, world_y, -1.0);

        let o = Tuple4D::new_point(0.0, 0.0, 0.0);

        let pixel = &camera_transform_inv * &p;
        let mut origin = &camera_transform_inv * &o;
        let mut direction = Tuple4D::normalize(&(pixel - origin));

        // so the assert in Ray::new don't panic
        origin.w = 1.0;
        direction.w = 0.0;
        Ray::new(origin, direction)
    }

    fn ray_for_pixel_anti_aliasing(c: &Camera, x: usize, y: usize, delta_x: f64, delta_y: f64) -> Ray {
        let camera_transform_inv =
            Matrix::invert(c.get_transform()).expect("ray_for_pixel:  cant calculate the inverse");

        let x_offset = (x as f64 + 0.5) * c.get_pixel_size();
        let y_offset = (y as f64 + 0.5) * c.get_pixel_size();

        let _world_x_old = c.get_half_width() - x_offset;
        let _world_y_old = c.get_half_height() - y_offset;

        let world_x = c.get_half_width() - x_offset + delta_x;
        let world_y = c.get_half_height() - y_offset + delta_y;

        // println!("with AA    (x/y) = ({}/{})   world_point_old ({}/{})  world_point = ({}/{})     delta: ({}/{}) ", x, y, world_x_old, world_y_old, world_x, world_y, delta_x, delta_y);

        let p = Tuple4D::new_point(world_x, world_y, -1.0);

        let o = Tuple4D::new_point(0.0, 0.0, 0.0);

        let pixel = &camera_transform_inv * &p;
        let mut origin = &camera_transform_inv * &o;
        let mut direction = Tuple4D::normalize(&(pixel - origin));

        // so the assert in Ray::new don't panic
        origin.w = 1.0;
        direction.w = 0.0;
        Ray::new(origin, direction)
    }

    fn get_half_width(&self) -> f64 {
        self.half_width
    }

    fn get_half_height(&self) -> f64 {
        self.half_height
    }

    fn render(c: &Camera, w: &World) -> Canvas {
        //  https://computergraphics.stackexchange.com/questions/4248/how-is-anti-aliasing-implemented-in-ray-tracing
        let n_samples = c.get_antialiasing_size();
        let jitter_matrix = Self::get_jitter_for_sampling(n_samples);

        let mut canvas = Canvas::new(c.get_hsize(), c.get_vsize());

        for y in 0..c.get_vsize() {
            for x in 0..c.get_hsize() {
                if c.get_antialiasing() {
                    let mut color = BLACK;

                    // Accumulate light for N samples.
                    for sample in 0..n_samples {
                        let delta_x = jitter_matrix[2 * sample] * c.get_pixel_size();
                        let delta_y = jitter_matrix[2 * sample + 1] * c.get_pixel_size();

                        let r = Camera::ray_for_pixel_anti_aliasing(c, x, y, delta_x, delta_y);

                        // println!("ray {:?}  @ ({}/{})", &r, x, y);
                        color = color + World::color_at(w, &r, MAX_REFLECTION_RECURSION_DEPTH);
                    }
                    color = color / n_samples as f64;
                    // println!("with AA    color at ({}/{}): {:?}", x, y, color);
                    canvas.write_pixel(x, y, color);
                } else {
                    let r = Camera::ray_for_pixel(c, x, y);
                    // println!("ray {:?}  @ ({}/{})", &r, x, y);
                    let color = World::color_at(w, &r, MAX_REFLECTION_RECURSION_DEPTH);
                    // println!("no AA    color at ({}/{}): {:?}", x, y, color);
                    canvas.write_pixel(x, y, color);
                }
            }
            // println!("render line  {}", y);
        }
        canvas
    }

    fn render_multi_core(ca: &Camera, wo: &World) -> Canvas {
        let camera = ca.clone();
        let world = wo.clone();

        let start = Instant::now();
        let num_cores = num_cpus::get() / 2;

        println!("using {} cores", num_cores);

        let canvas = Canvas::new(camera.get_hsize(), camera.get_vsize());
        let data = Arc::new(Mutex::new(canvas));

        let act_y: usize = 0;
        let act_y_mutex = Arc::new(Mutex::new(act_y));

        let c = crossbeam::scope(|s| {
            let mut children = vec![];

            for _i in 0..num_cores {
                let n_samples = camera.get_antialiasing_size();
                let jitter_matrix = Self::get_jitter_for_sampling(n_samples);

                let cloned_data = Arc::clone(&data);
                let cloned_act_y = Arc::clone(&act_y_mutex);
                let height = camera.get_vsize();
                let width = camera.get_hsize();

                let c_clone = camera.clone();
                let w_clone = world.clone();

                children.push(s.spawn(move |_| {
                    let mut y: usize = 0;
                    let mut cnt_lines = 0;

                    println!(
                        "camera height / width  {}/{}     thread_id {:?}",
                        height,
                        width,
                        thread::current().id()
                    );

                    while *cloned_act_y.lock().unwrap() < height {
                        cnt_lines += 1;
                        if y < height {
                            let mut acty = cloned_act_y.lock().unwrap();
                            y = *acty;
                            *acty += 1;
                            println!("   thread_id {:?},   y = {}", thread::current().id(), acty);
                        }

                        for x in 0..width {
                            let color = Self::raytrace_pixel(n_samples, &jitter_matrix, &c_clone, &w_clone, y, x);
                            let mut canvas = cloned_data.lock().unwrap();
                            canvas.write_pixel(x, y, color);
                        }
                    }
                    (thread::current().id(), cnt_lines)
                }));
            }

            for child in children {
                let dur = Instant::now() - start;
                let (thread_id, cnt_lines) = child.join().unwrap();
                println!(
                    "child thread {:?} finished. run for {:?} , processed {:?} lines",
                    thread_id, dur, cnt_lines
                );
            }
            let dur = Instant::now() - start;
            Self::print_duration(camera, dur);

            data.lock().unwrap()
        })
            .unwrap();

        c.clone()
    }

    fn render_multi_core_tiled(ca: &Camera, wo: &World, x_tiles: usize, y_tiles: usize) -> Canvas {
        let camera = ca.clone();
        let world = wo.clone();

        let start = Instant::now();
        let num_cores = num_cpus::get();

        println!("using {} cores", num_cores);

        let canvas = Canvas::new(camera.get_hsize(), camera.get_vsize());
        let mut tiles = canvas.tiles(x_tiles, y_tiles);
        let data = Arc::new(Mutex::new(canvas));
        let tiles = Arc::new(Mutex::new(tiles));

        let c = crossbeam::scope(|s| {
            let mut children = vec![];

            for _i in 0..num_cores {
                let n_samples = camera.get_antialiasing_size();
                let jitter_matrix = Self::get_jitter_for_sampling(n_samples);

                let cloned_data = Arc::clone(&data);
                let height = camera.get_vsize();
                let width = camera.get_hsize();

                let c_clone = camera.clone();
                let w_clone = world.clone();

                let cloned_tiles = Arc::clone(&tiles);
                children.push(s.spawn(move |_| {
                    let mut cnt_tiles = 0;

                    println!(
                        "camera height / width  {}/{}     thread_id {:?}",
                        height,
                        width,
                        thread::current().id()
                    );

                    while cloned_tiles.lock().unwrap().peekable().peek().is_some() {
                        let tile_candidate;
                        {
                            tile_candidate = cloned_tiles.lock().unwrap().next();
                        }
                        match tile_candidate {
                            Some(ref tile) => {
                                println!("thread   {:?}    processing tile  {}", thread::current().id(), tile);

                                let mut pixels = vec![];

                                cnt_tiles += 1;
                                for y in tile.y_from()..tile.y_to() {
                                    for x in tile.x_from()..tile.x_to() {
                                        // println!("thread_id {:?}   raytracing pixel:  {}/{} ", thread::current().id(), x, y);
                                        let color =
                                            Self::raytrace_pixel(n_samples, &jitter_matrix, &c_clone, &w_clone, x, y);
                                        pixels.push((x, y, color));
                                    }
                                }

                                let mut canvas = cloned_data.lock().unwrap();
                                for p in pixels {
                                    canvas.write_pixel(p.0, p.1, p.2);
                                }
                            }
                            None => {
                                println!(" no more tiles for thread {:?}", thread::current().id());
                            }
                        };
                    }

                    (thread::current().id(), cnt_tiles)
                }));
            }

            for child in children {
                let dur = Instant::now() - start;
                let (thread_id, cnt_tiles) = child.join().unwrap();
                println!(
                    "child thread {:?} finished. run for {:?} , processed {:?} tiles",
                    thread_id, dur, cnt_tiles
                );
            }
            let dur = Instant::now() - start;
            Self::print_duration(camera, dur);
            data.lock().unwrap()
        })
            .unwrap();

        c.clone()
    }

    fn render_multi_core_tiled_sender(ca: &Camera, wo: &World, x_tiles: usize, y_tiles: usize, sender: Sender<Canvas>) -> Canvas {
        let camera = ca.clone();
        let world = wo.clone();

        let start = Instant::now();
        let num_cores = num_cpus::get();

        println!("using {} cores", num_cores);

        let canvas = Canvas::new(camera.get_hsize(), camera.get_vsize());
        let mut tiles = canvas.tiles(x_tiles, y_tiles);
        let data = Arc::new(Mutex::new(canvas));
        let tiles = Arc::new(Mutex::new(tiles));

        let c = crossbeam::scope(|s| {
            let mut children = vec![];

            for _i in 0..num_cores {
                let n_samples = camera.get_antialiasing_size();
                let jitter_matrix = Self::get_jitter_for_sampling(n_samples);

                let cloned_data = Arc::clone(&data);
                let height = camera.get_vsize();
                let width = camera.get_hsize();

                let c_clone = camera.clone();
                let w_clone = world.clone();

                let sender_thread = sender.clone();
                let cloned_tiles = Arc::clone(&tiles);
                children.push(s.spawn(move |_| {
                    let mut cnt_tiles = 0;


                    println!(
                        "camera height / width  {}/{}     thread_id {:?}",
                        height,
                        width,
                        thread::current().id()
                    );

                    while cloned_tiles.lock().unwrap().peekable().peek().is_some() {
                        let tile_candidate;
                        {
                            tile_candidate = cloned_tiles.lock().unwrap().next();
                        }
                        match tile_candidate {
                            Some(ref tile) => {
                                println!("thread   {:?}    processing tile  {}", thread::current().id(), tile);

                                let mut pixels = vec![];

                                cnt_tiles += 1;
                                for y in tile.y_from()..tile.y_to() {
                                    for x in tile.x_from()..tile.x_to() {
                                        // println!("thread_id {:?}   raytracing pixel:  {}/{} ", thread::current().id(), x, y);
                                        let color =
                                            Self::raytrace_pixel(n_samples, &jitter_matrix, &c_clone, &w_clone, x, y);
                                        pixels.push((x, y, color));
                                    }
                                }

                                let mut canvas = cloned_data.lock().unwrap();
                                for p in pixels {
                                    canvas.write_pixel(p.0, p.1, p.2);
                                }
                                sender_thread.send(canvas.clone());

                                match sender_thread.send(canvas.clone()) {
                                    Ok(_) => {
                                         println!("sent stuff");
                                        ()
                                    },
                                    Err(_)=> {
                                        println!("error sending stuff ");
                                        ()
                                    }
                                };

                            }
                            None => {
                                println!(" no more tiles for thread {:?}", thread::current().id());
                            }
                        };
                    }

                    (thread::current().id(), cnt_tiles)
                }));
            }

            for child in children {
                let dur = Instant::now() - start;
                let (thread_id, cnt_tiles) = child.join().unwrap();
                println!(
                    "child thread {:?} finished. run for {:?} , processed {:?} tiles",
                    thread_id, dur, cnt_tiles
                );
            }
            let dur = Instant::now() - start;
            Self::print_duration(camera, dur);
            data.lock().unwrap()
        })
            .unwrap();

        c.clone()
    }


    fn render_debug(c: &Camera, w: &World, x: usize, y: usize) -> Canvas {
        println!("DEBUG render point  {}/{}", x, y);

        let mut canvas = Canvas::new(c.get_hsize(), c.get_vsize());
        let r = Camera::ray_for_pixel(c, x, y);
        let c = World::color_at(w, &r, MAX_REFLECTION_RECURSION_DEPTH);
        if c.r != 0.0 || c.g != 0.0 || c.b != 0.0 {}
        canvas.write_pixel(x, y, c);
        canvas
    }

    fn set_antialiasing(&mut self, aa: bool) {
        self.antialiasing = aa;
    }

    fn get_antialiasing(&self) -> bool {
        self.antialiasing
    }

    fn set_antialiasing_size(&mut self, size: usize) {
        self.antialiasing_size = size;
    }

    fn get_antialiasing_size(&self) -> usize {
        self.antialiasing_size
    }
}

impl Camera {
    fn raytrace_pixel(
        n_samples: usize,
        mut jitter_matrix: &Vec<f64>,
        c_clone: &Camera,
        w_clone: &World,
        x: usize,
        y: usize,
    ) -> Color {
        let mut color = BLACK;
        if c_clone.get_antialiasing() {
            // Accumulate light for N samples.
            for sample in 0..n_samples {
                let delta_x = jitter_matrix[2 * sample] * c_clone.get_pixel_size();
                let delta_y = jitter_matrix[2 * sample + 1] * c_clone.get_pixel_size();

                let r = Camera::ray_for_pixel_anti_aliasing(&c_clone, x, y, delta_x, delta_y);
                // println!("ray {:?}  @ ({}/{})", &r, x, y);
                color = color + World::color_at(&w_clone, &r, MAX_REFLECTION_RECURSION_DEPTH);
            }
            color = color / n_samples as f64;
            // println!("with AA    color at ({}/{}): {:?}", x, y, color);
        } else {
            let r = Camera::ray_for_pixel(&c_clone, x, y);
            // println!("ray {:?}  @ ({}/{})", &r, x, y);
            color = World::color_at(&w_clone, &r, MAX_REFLECTION_RECURSION_DEPTH);
            // println!("no AA    color at ({}/{}): {:?}", x, y, color);
        }
        color
    }

    fn get_jitter_for_sampling(n_samples: usize) -> Vec<f64> {
        let mut jitter_matrix = Vec::new();
        if n_samples == 2 {
            jitter_matrix = vec![
                -1.0 / 4.0,
                1.0 / 4.0,
                1.0 / 4.0,
                1.0 / 4.0,
                -1.0 / 4.0,
                -1.0 / 4.0,
                1.0 / 4.0,
                -3.0 / 4.0,
            ];
        }

        if n_samples == 3 {
            let two_over_six = 2.0 / 6.0;
            jitter_matrix = vec![
                -two_over_six,
                two_over_six,
                0.0,
                two_over_six,
                two_over_six,
                two_over_six,
                -two_over_six,
                0.0,
                0.0,
                0.0,
                two_over_six,
                0.0,
                -two_over_six,
                -two_over_six,
                0.0,
                -two_over_six,
                two_over_six,
                -two_over_six,
            ];
        }
        jitter_matrix
    }

    fn print_duration(camera: Camera, dur: Duration) {
        if camera.get_antialiasing() {
            println!(
                "multi core duration: {:?} with AA size = {}",
                dur,
                camera.get_antialiasing_size()
            );
        } else {
            println!("multi core duration: {:?}, no AA", dur);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use crate::basics::color::{Color, ColorOps};
    use crate::math::common::{assert_color, assert_float, assert_matrix, assert_tuple};
    use crate::prelude::default_world;

    use super::*;

    // page 101
    #[test]
    fn test_camera_new() {
        let mut c = Camera::new(160, 120, PI / SQRT_2);
        c.calc_pixel_size();

        assert_eq!(c.get_hsize(), 160);
        assert_eq!(c.get_vsize(), 120);

        assert_float(c.get_field_of_view(), PI / SQRT_2);
        assert_matrix(c.get_transform(), &Matrix::new_identity_4x4());
    }

    // page 101 bottom
    #[test]
    fn test_camera_pixel_size_horizontal() {
        let mut c = Camera::new(200, 125, PI / 2.0);
        c.calc_pixel_size();
        assert_float(c.get_pixel_size(), 0.01);
        assert_matrix(c.get_transform(), &Matrix::new_identity_4x4());
    }

    // page 101 bottom part 2
    #[test]
    fn test_camera_pixel_size_vertical() {
        let mut c = Camera::new(125, 200, PI / 2.0);
        c.calc_pixel_size();
        assert_float(c.get_pixel_size(), 0.01);
        assert_matrix(c.get_transform(), &Matrix::new_identity_4x4());
    }

    // page 103 part1
    #[test]
    fn test_camera_ray_for_pixel_center() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.calc_pixel_size();

        let r = Camera::ray_for_pixel(&c, 100, 50);
        let origin_expected = Tuple4D::new_point(0.0, 0.0, 0.0);
        let direction_expected = Tuple4D::new_vector(0.0, 0.0, -1.0);

        assert_tuple(&r.get_origin(), &origin_expected);
        assert_tuple(&r.get_direction(), &direction_expected);
    }

    // page 103 part 2
    #[test]
    fn test_camera_ray_for_pixel_canvas_corner() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.calc_pixel_size();

        let r = Camera::ray_for_pixel(&c, 0, 0);
        let origin_expected = Tuple4D::new_point(0.0, 0.0, 0.0);
        let direction_expected = Tuple4D::new_vector(0.6651864, 0.33259323, -0.66851234);

        println!("origin            = {:?}", &r.get_origin());
        println!("origin_expected   = {:?}", origin_expected);

        println!("direction             = {:?}", &r.get_direction());
        println!("direction_expected    = {:?}", direction_expected);

        assert_tuple(&r.get_origin(), &origin_expected);
        assert_tuple(&r.get_direction(), &direction_expected);
    }

    // page 103 part 3
    #[test]
    fn test_camera_ray_for_pixel_transformed_camera() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.calc_pixel_size();

        let rot_y = Matrix::rotate_y(PI / 4.0);
        let trans = Matrix::translation(0.0, -2.0, 5.0);

        let transform = &rot_y * &trans;
        c.set_transformation(transform);

        let r = Camera::ray_for_pixel(&c, 100, 50);
        let expected_origin = Tuple4D::new_point(0.0, 2.0, -5.0);
        let expected_direction = Tuple4D::new_vector(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0);

        assert_tuple(&r.get_origin(), &expected_origin);
        assert_tuple(&r.get_direction(), &expected_direction);
    }

    // page 104
    #[test]
    fn test_camera_render() {
        let w = default_world();
        let mut c = Camera::new(11, 11, PI / 2.0);
        c.calc_pixel_size();

        let from = Tuple4D::new_point(0.0, 0.0, -5.0);
        let to = Tuple4D::new_point(0.0, 0.0, 0.0);
        let up = Tuple4D::new_vector(0.0, 1.0, 0.0);

        c.set_transformation(Matrix::view_transform(&from, &to, &up));

        let image = Camera::render(&c, &w);
        let pixel = image.pixel_at(5, 5);
        let c_expected = Color::new(0.38065884, 0.47582352, 0.28549412);

        println!("color          = {:?}", &pixel.color);
        println!("c_expected     = {:?}", c_expected);
        assert_color(&pixel.color, &c_expected);
    }
}
