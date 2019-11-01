use crate::backend::MAX_REFLECTION_RECURSION_DEPTH;
use crate::BackendOps;
use cpu_kernel_raytracer::CpuKernel;
use raytracer_lib_no_std::{Camera, CameraOps, ColorOps, BLACK};
use raytracer_lib_std::{Canvas, CanvasOps, World, WorldOps};
use std::error::Error;

pub struct BackendWasm {}

impl BackendOps for BackendWasm {
    fn render_world(&self, world: &mut World, c: &Camera) -> Result<Canvas, Box<dyn Error>> {
        // let start = Instant::now();
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
                    color.clamp_color();
                    canvas.write_pixel(x, y, color);
                }
            }
        } else {
            for y in 0..c.get_vsize() {
                for x in 0..c.get_hsize() {
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
                    canvas.write_pixel(x, y, color);
                }
            }
        }
        // let stopped = Instant::now();
        Ok(canvas)
    }
}

impl BackendWasm {
    pub fn new() -> BackendWasm {
        BackendWasm {}
    }
}
