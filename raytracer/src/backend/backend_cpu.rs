use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crossbeam_utils::thread;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use cpu_kernel_raytracer::CpuKernel;
use raytracer_lib_no_std::camera::{Camera, CameraOps};
use raytracer_lib_no_std::color::BLACK;
use raytracer_lib_no_std::ColorOps;
use raytracer_lib_std::{Canvas, CanvasOps, World, WorldOps};

use crate::backend::backend::Backend;
use crate::backend::MAX_REFLECTION_RECURSION_DEPTH;

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
                        let c = CpuKernel::color_at(
                            world.get_shapes(),
                            &lights,
                            &r,
                            MAX_REFLECTION_RECURSION_DEPTH,
                            c.get_calc_reflection(),
                            c.get_calc_refraction(),
                            c.get_calc_shadows(),
                            false,
                        );
                        color = c + color;
                    }
                    color = color / (n_samples * n_samples) as f32;
//                    color.clamp_color();
                    canvas.write_pixel(x, y, color);
                }
            }
        } else {
            for y in 0..c.get_vsize() {
                for x in 0..c.get_hsize() {
                    let r = Camera::ray_for_pixel(c, x, y);
                    let   color = CpuKernel::color_at(
                        world.get_shapes(),
                        &lights,
                        &r,
                        MAX_REFLECTION_RECURSION_DEPTH,
                        c.get_calc_reflection(),
                        c.get_calc_refraction(),
                        c.get_calc_shadows(),
                        false,
                    );
                    // color.clamp_color();
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
        println!("\n\ncpu multi core   started at : {:?} \n\n", start);

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
                    let c = CpuKernel::color_at(
                        world.get_shapes(),
                        &lights,
                        &r,
                        MAX_REFLECTION_RECURSION_DEPTH,
                        c.get_calc_reflection(),
                        c.get_calc_refraction(),
                        c.get_calc_shadows(),
                        false,
                    );
                    color = c + color;
                }
                color = color / (n_samples * n_samples) as f32;
                // color.clamp_color();
                p.color.r = color.r;
                p.color.g = color.g;
                p.color.b = color.b;
            } else {
                let r = Camera::ray_for_pixel(c, x, y);
                let mut color = CpuKernel::color_at(
                    world.get_shapes(),
                    &lights,
                    &r,
                    MAX_REFLECTION_RECURSION_DEPTH,
                    c.get_calc_reflection(),
                    c.get_calc_refraction(),
                    c.get_calc_shadows(),
                    false,
                );
                color.clamp_color();

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

    fn render_world_multi_core_crossbeam(&self, world: &mut World, c: &Camera) -> Result<Canvas, Box<dyn Error>> {
        let start = Instant::now();
        println!("\n\ncpu multi core   started at : {:?} \n\n", start);

        // TODO: remove, when World has lights vector
        let mut lights = Vec::new();
        lights.push(world.get_light().clone());

        let canvas = Canvas::new(c.get_hsize(), c.get_vsize());
        let data = Arc::new(Mutex::new(canvas));


        let act_y: usize = 0;
        let act_y_mutex = Arc::new(Mutex::new(act_y));
        let cloned_data = Arc::clone(&data);

        let num_cores = num_cpus::get();


        thread::scope( |scope| {


            for _ in 0..num_cores {
                let n_samples = c.get_antialiasing_size();
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


//
                let c_clone = c.clone();
             let w_clone = world.clone();
                let cloned_act_y = Arc::clone(&act_y_mutex);
                let lights_clone = lights.clone();

                scope.spawn(move |_| {
                    let mut y: usize = 0;

                    let height = c.get_vsize();
                    let width = c.get_hsize();

                    println!(
                        "camera height / width  {}/{}    ",
                        height,
                        width,
                    );

                    while *cloned_act_y.lock().unwrap() < height {
                        if y < height {
                            let mut acty = cloned_act_y.lock().unwrap();
                            y = *acty;
                            *acty = *acty + 1;
                            println!("y = {}, height = {}", y,height);
                        }
                        for x in 0..width {
                            let mut color = BLACK;
                            if c_clone.get_antialiasing() {
                                // Accumulate light for N samples.
                                for sample in 0..n_samples {
                                    let delta_x = jitter_matrix[2 * sample] * c_clone.get_pixel_size();
                                    let delta_y = jitter_matrix[2 * sample + 1] * c_clone.get_pixel_size();

                                    let r = Camera::ray_for_pixel_anti_aliasing(&c_clone, x, y, delta_x, delta_y);

                                    color = color + CpuKernel::color_at(
                                        w_clone.get_shapes(),
                                        &lights_clone,
                                        &r,
                                        MAX_REFLECTION_RECURSION_DEPTH,
                                        c.get_calc_reflection(),
                                        c.get_calc_refraction(),
                                        c.get_calc_shadows(),
                                        false,
                                    );
                                }
                                color = color / n_samples as f32;
                                // println!("with AA    color at ({}/{}): {:?}", x, y, color);
                            } else {
                                let r = Camera::ray_for_pixel(&c_clone, x, y);
                                color =CpuKernel::color_at(
                                    w_clone.get_shapes(),
                                    &lights,
                                    &r,
                                    MAX_REFLECTION_RECURSION_DEPTH,
                                    c.get_calc_reflection(),
                                    c.get_calc_refraction(),
                                    c.get_calc_shadows(),
                                    false,
                                );
                                // println!("no AA    color at ({}/{}): {:?}", x, y, color);
                            }

                            let mut canvas = cloned_data.lock().unwrap();
                            canvas.write_pixel(x, y, color);
                        }
                    }
                    // thread::current().id()
                });
            }
        });


        let stopped = Instant::now();
        println!(
            "\n\ncpu multicore crossbeam      duration  {:?}      \n\n",
            stopped.duration_since(start)
        );
        // Ok(canvas)
        Ok(Canvas::new(1,1))
    }
}

impl BackendCpu {
    pub fn new() -> BackendCpu {
        BackendCpu {}
    }

    pub fn render_world_debug(
        &self,
        world: &mut World,
        c: &Camera,
        x: usize,
        y: usize,
    ) -> Result<Canvas, Box<dyn Error>> {
        let mut canvas = Canvas::new(c.get_hsize(), c.get_vsize());

        // TODO: remove, when WOrld has lights vector
        let mut lights = Vec::new();
        lights.push(world.get_light().clone());

        let r = Camera::ray_for_pixel(c, x, y);
        let mut color = CpuKernel::color_at(
            world.get_shapes(),
            &lights,
            &r,
            MAX_REFLECTION_RECURSION_DEPTH,
            c.get_calc_reflection(),
            c.get_calc_refraction(),
            c.get_calc_shadows(),
            true,
        );
        println!("'render_world_debug'   color   = {:?}", color);
        color.clamp_color();
        println!("'render_world_debug'    after color clamp   color   = {:?}   ({}/{})", color, x, y);

        canvas.write_pixel(x, y, color);

        Ok(canvas)
    }
}
