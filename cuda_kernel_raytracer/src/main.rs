#![cfg_attr(target_os = "cuda", feature(abi_ptx, proc_macro_hygiene))]
#![cfg_attr(target_os = "cuda", no_std)]
#![feature(stmt_expr_attributes)]

extern crate raytracer_lib_no_std;

use cuda::cuda_kernel::CudaKernel;
use raytracer_lib_no_std::basics::camera::{Camera, CameraOps};
use raytracer_lib_no_std::basics::color::{Color, BLACK};
use raytracer_lib_no_std::light::light::Light;
use raytracer_lib_no_std::shape::shape::Shape;
use raytracer_lib_no_std::ColorOps;
use raytracer_lib_no_std::MAX_REFLECTION_RECURSION_DEPTH;

pub mod cuda;

#[no_mangle]
#[cfg(target_os = "cuda")]
pub unsafe extern "ptx-kernel" fn calc_pixel(
    pixels: *mut Color,
    shapes: *mut Shape,
    cnt_shapes: usize,
    lights: *const Light,
    cnt_lights: usize,
    camera: *const Camera,
    width: *const f32,
    height: *const f32,
    block_dim_x: u32,
    block_dim_y: u32,
) {
    use ptx_support::prelude::*;

    let w = *width as isize;
    let h = *height as isize;

    // pixel coordinates
    let x_idx = (Context::thread().index().x + Context::block().index().x * block_dim_x as u64) as isize;
    let y_idx = Context::block().index().y as isize;

    if x_idx < w && y_idx < h {
        let x = x_idx as f32;
        let y = y_idx as f32;
        //  cuda_printf!("calc_pixel:   w = %f,  h = %f     x = %f  y= %f     \n", *width as f64, *height as f64, x as f64, y as f64);

        let c = camera.offset(0).as_ref().expect("camera expect in 'calc_pixel' ");

        if c.get_antialiasing() {
            let n_samples = c.get_antialiasing_size();

            let two_over_six = 2.0 / 6.0;
            let mut jitter_matrix = [0f32; 18];

            if n_samples == 2 {
                jitter_matrix[0] = -1.0 / 4.0;
                jitter_matrix[1] = 1.0 / 4.0;
                jitter_matrix[2] = 1.0 / 4.;
                jitter_matrix[3] = 1.0 / 4.0;
                jitter_matrix[4] = -1.0 / 4.0;
                jitter_matrix[5] = -1.0 / 4.0;
                jitter_matrix[6] = 1.0 / 4.0;
                jitter_matrix[7] = -3.0 / 4.0;
            }
            if n_samples == 3 {
                let two_over_six = 2.0 / 6.0;
                jitter_matrix[0] = -two_over_six;
                jitter_matrix[1] = two_over_six;
                jitter_matrix[2] = 0.0;
                jitter_matrix[3] = two_over_six;
                jitter_matrix[4] = two_over_six;
                jitter_matrix[5] = two_over_six;
                jitter_matrix[6] = -two_over_six;
                jitter_matrix[7] = 0.0;
                jitter_matrix[8] = 0.0;
                jitter_matrix[9] = 0.0;
                jitter_matrix[10] = two_over_six;
                jitter_matrix[11] = 0.0;
                jitter_matrix[12] = -two_over_six;
                jitter_matrix[13] = -two_over_six;
                jitter_matrix[14] = 0.0;
                jitter_matrix[15] = -two_over_six;
                jitter_matrix[16] = two_over_six;
                jitter_matrix[17] = -two_over_six;
            }

            let mut color = BLACK;
            for sample in 0..(n_samples * n_samples) {
                let delta_x = jitter_matrix[2 * sample] * c.get_pixel_size();
                let delta_y = jitter_matrix[2 * sample + 1] * c.get_pixel_size();

                let r = Camera::ray_for_pixel_anti_aliasing(c, x_idx as usize, y_idx as usize, delta_x, delta_y);

                color = color
                    + CudaKernel::color_at(
                        shapes,
                        cnt_shapes,
                        lights,
                        cnt_lights,
                        &r,
                        MAX_REFLECTION_RECURSION_DEPTH,
                        c.get_calc_reflection(),
                        c.get_calc_refraction(),
                        c.get_calc_shadows(),
                    );
            }
            color = color / (n_samples * n_samples) as f32;
            color.clamp_color();
            let idx = y_idx * w + x_idx;

            *pixels.offset(idx) = color;
        } else {
            let r = Camera::ray_for_pixel(c, x_idx as usize, y_idx as usize);
            let mut color = CudaKernel::color_at(
                shapes,
                cnt_shapes,
                lights,
                cnt_lights,
                &r,
                MAX_REFLECTION_RECURSION_DEPTH,
                c.get_calc_reflection(),
                c.get_calc_refraction(),
                c.get_calc_shadows(),
            );
            let idx = y_idx * w + x_idx;
            color.clamp_color();

            *pixels.offset(idx) = color;
        }
    }
}

fn main() {}
