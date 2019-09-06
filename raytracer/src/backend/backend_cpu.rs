// extern crate crossbeam;

use std::error::Error;
use std::time::Instant;

use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use cpu_kernel_raytracer::camera::{Camera, CameraOps};
use cpu_kernel_raytracer::color::BLACK;
use cpu_kernel_raytracer::CpuKernel;
use cpu_kernel_raytracer::ray::RayOps;
use raytracer_lib_std::{Canvas, CanvasOps, World, WorldOps};

use crate::backend::backend::Backend;
use crate::backend::MAX_REFLECTION_RECURSION_DEPTH;
use raytracer_lib_no_std::ColorOps;

pub struct BackendCpu {}

impl Backend for BackendCpu {
    fn render_world(&self, world: &mut World, c: &Camera) -> Result<Canvas, Box<dyn Error>> {
        let start = Instant::now();
        let n_samples = c.get_antialiasing_size();
        let mut jitter_matrix = Vec::new();
        if n_samples == 2 {
            #[rustfmt::skip]
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
            #[rustfmt::skip]
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
        let mut canvas = Canvas::new(c.get_hsize(), c.get_vsize());
        println!("single core      jitter_matrix  {:?}", jitter_matrix);
        println!("single core      n_smaples  {:?}", n_samples);

        // TODO: remove, when WOrld has lights vector
        let mut lights = Vec::new();
        lights.push(world.get_light().clone());

        if c.get_antialiasing() {
            for y in 0..c.get_vsize() {
                for x in 0..c.get_hsize() {
                    let mut color = BLACK;

                    // Accumulate light for N samples.
                    for sample in 0..(n_samples * n_samples) {
                        let delta_x = jitter_matrix[2 * sample] * c.get_pixel_size();
                        let delta_y = jitter_matrix[2 * sample + 1] * c.get_pixel_size();

                        let r = Camera::ray_for_pixel_anti_aliasing(c, x, y, delta_x, delta_y);
                        let c = CpuKernel::color_at(world.get_shapes(), &lights, &r, MAX_REFLECTION_RECURSION_DEPTH);
                        color = c + color;
                        if x == 163 && y == 67 {
                            println!("single core   with AA  sample: {},             c  {:?}", sample, c);
                            println!("single core  with AA   sample: {},         color  {:?}", sample, color);
                        }
                    }
                    color = color / (n_samples*n_samples) as f32;
                    if x == 163 && y == 67 {
                        println!("single core  with AA    BEFORE clamp    color at ({}/{}): {:?}", x, y, color);
                    }
                    color.clamp_color();
                    if x == 163 && y == 67 {
                        println!("single core  with AA     AFTER  clamp    color at ({}/{}): {:?}", x, y, color);
                    }
                    canvas.write_pixel(x, y, color);
                }
            }
        } else {
            for y in 0..c.get_vsize() {
                for x in 0..c.get_hsize() {
                    let r = Camera::ray_for_pixel(c, x, y);

                    let mut color = CpuKernel::color_at(world.get_shapes(), &lights, &r, MAX_REFLECTION_RECURSION_DEPTH);
                    if x == 163 && y == 67 {
                        println!("single core    no AA   BEFORE clamp color at ({}/{}): {:?}", x, y, color);
                    }
                    color.clamp_color();
                    if x == 163 && y == 67 {
                        println!("single core    no AA   AFTER  clamp color at ({}/{}): {:?}", x, y, color);
                    }
                    canvas.write_pixel(x, y, color);
                }
            }
        }
        let stopped = Instant::now();
        println!(
            "\n\ncpu single core     duration: {:?} \n\n",
            stopped.duration_since(start)
        );
        Ok(canvas)
    }

    fn render_world_multi_core(&self, world: &mut World, c: &Camera) -> Result<Canvas, Box<dyn Error>> {
        let start = Instant::now();
        let n_samples = c.get_antialiasing_size();
        let mut jitter_matrix = Vec::new();
        if n_samples == 2 {
            #[rustfmt::skip]
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
            #[rustfmt::skip]
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
        let mut canvas = Canvas::new(c.get_hsize(), c.get_vsize());

        println!("multicore     jitter_matrix  {:?}", jitter_matrix);
        println!("multicore     n_smaples  {:?}", n_samples);


        // TODO: remove, when World has lights vector
        let mut lights = Vec::new();
        lights.push(world.get_light().clone());

        canvas.get_pixels_mut().into_par_iter().for_each(|p| {
            let x = p.x;
            let y = p.y;
            if c.get_antialiasing() {
                let mut color = BLACK;

                // Accumulate light for N samples.
                for sample in 0..(n_samples * n_samples) {
                    let delta_x = jitter_matrix[2 * sample] * c.get_pixel_size();
                    let delta_y = jitter_matrix[2 * sample + 1] * c.get_pixel_size();

                    let r = Camera::ray_for_pixel_anti_aliasing(c, x, y, delta_x, delta_y);

                    let c = CpuKernel::color_at(world.get_shapes(), &lights, &r, MAX_REFLECTION_RECURSION_DEPTH);
                    color = c + color;

                    if x == 163 && y == 67 {
                        println!("multicore  with AA  sample: {},             c  {:?}", sample, c);
                        println!("multicore  with AA   sample: {},         color  {:?}", sample, color);
                    }
                }
                color = color / (n_samples*n_samples) as f32;
                if x == 163 && y == 67 {
                    println!("multicore  with AA   BEFORE CLAMP color at ({}/{}): {:?}", x, y, color);
                } color.clamp_color();
                if x == 163 && y == 67 {
                    println!("multicore  with AA    AFTER CLAMP color at ({}/{}): {:?}", x, y, color);
                }
                p.color.r = color.r;
                p.color.g = color.g;
                p.color.b = color.b;
            } else {
                let r = Camera::ray_for_pixel(c, x, y);

                let mut color = CpuKernel::color_at(world.get_shapes(), &lights, &r, MAX_REFLECTION_RECURSION_DEPTH);
                if x == 163 && y == 67 {
                    println!("multicore  no AA   BEFORE CLAMP   color at ({}/{}): {:?}", x, y, color);
                } color.clamp_color();

                if x == 163 && y == 67 {
                    println!("multicore  no AA   AFTER CLAMP    color at ({}/{}): {:?}", x, y, color);
                }
                p.color.r = color.r;
                p.color.g = color.g;
                p.color.b = color.b;
            }
        });

        let stopped = Instant::now();
        println!(
            "\n\ncpu multicore       duration  {:?}      \n\n",
            stopped.duration_since(start)
        );
        Ok(canvas)
    }
}

impl BackendCpu {
    pub fn new() -> BackendCpu {
        BackendCpu {}
    }
}

