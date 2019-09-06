#![cfg_attr(target_os = "cuda", feature(abi_ptx, proc_macro_hygiene))]
#![cfg_attr(target_os = "cuda", no_std)]
#![feature(stmt_expr_attributes)]

extern crate raytracer_lib_no_std;

use cuda::cuda_kernel::CudaKernel;
use raytracer_lib_no_std::basics::camera::{Camera, CameraOps};
use raytracer_lib_no_std::basics::color::{BLACK, Color};
use raytracer_lib_no_std::light::light::Light;
use raytracer_lib_no_std::shape::shape::{Shape, ShapeEnum};

pub mod cuda;

// TODO: there are multiple definitions of this constant
pub const MAX_REFLECTION_RECURSION_DEPTH: i32 = 10;

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
    let x_idx =
        (Context::thread().index().x + Context::block().index().x * block_dim_x as u64) as isize;
    let y_idx = Context::block().index().y as isize;

    //     cuda_printf!("calc_pixel:   w = %f,  h = %f             \n", *width as f64, *height as f64);
    if x_idx < w && y_idx < h {
        let x = x_idx as f32;
        let y = y_idx as f32;

        let c = camera
            .offset(0)
            .as_ref()
            .expect("camera expect in 'calc_pixel' ");

        // TODO: add antialising parameters to method parameter list
        // assume aa of 3x
        let n_samples = 3;

        let two_over_six = 2.0 / 6.0;
        #[rustfmt::skip]
            let jitter_matrix = [-two_over_six, two_over_six, 0.0, two_over_six, two_over_six, two_over_six,
            -two_over_six, 0.0, 0.0, 0.0, two_over_six, 0.0,
            -two_over_six, -two_over_six, 0.0, -two_over_six, two_over_six, -two_over_six,
        ];

        let mut color = BLACK;
        for sample in 0..n_samples {
            let delta_x = jitter_matrix[2 * sample] * c.get_pixel_size();
            let delta_y = jitter_matrix[2 * sample + 1] * c.get_pixel_size();

            let r = Camera::ray_for_pixel_anti_aliasing(
                c,
                x_idx as usize,
                y_idx as usize,
                delta_x,
                delta_y,
            );

            color = color
                + CudaKernel::color_at(
                    shapes,
                    cnt_shapes,
                    lights,
                    cnt_lights,
                    &r,
                    MAX_REFLECTION_RECURSION_DEPTH,
                );
        }
        color = color / n_samples as f32;

        let idx = y_idx * w + x_idx;

        *pixels.offset(idx) = color;
    }
}

fn main() {}
