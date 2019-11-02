use std::error::Error;

use core::fmt;
use raytracer_lib_no_std::{Camera, CameraOps, Color, ColorOps, Light, Ray, Shape, BLACK, Pixel};
use raytracer_lib_std::{Canvas, World, WorldOps};

#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "cpu_single_core")]
use crate::BackendCpuSingleCore;

#[cfg(feature = "cpu_multi_core")]
use crate::BackendCpuMultiCore;

#[cfg(feature = "cuda")]
use crate::BackendCuda;

#[cfg(feature = "wasm")]
use crate::BackendWasm;
use crate::MAX_REFLECTION_RECURSION_DEPTH;

// TODO: use Vec<Light> if multiple light sources should be supported

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
pub enum BackendEnum {
    #[cfg(feature = "cpu_single_core")]
    CpuSingleCore,

    #[cfg(feature = "cpu_multi_core")]
    CpuMultiCore,

    #[cfg(feature = "cuda")]
    Cuda,

    #[cfg(feature = "wasm")]
    Wasm,
}

type BackendVec = Vec<BackendEnum>;

pub struct Backend {
    available_backends: BackendVec,
}

pub trait BackendOps {
    fn render_world(&self, world: &mut World, c: &Camera) -> Result<Canvas, Box<dyn Error>>;
}

impl Backend {
    pub fn new() -> Backend {
        let mut list = Vec::new();

        #[cfg(feature = "cpu_single_core")]
            list.push(BackendEnum::CpuSingleCore);

        #[cfg(feature = "cpu_multi_core")]
            list.push(BackendEnum::CpuMultiCore);

        #[cfg(feature = "cuda")]
            list.push(BackendEnum::Cuda);

        #[cfg(feature = "wasm")]
            list.push(BackendEnum::Wasm);

        Backend {
            available_backends: list,
        }
    }

    pub fn get_available_backends(&self) -> &BackendVec {
        &self.available_backends
    }

    pub fn get_backend(&self, backend_type: &BackendEnum) -> Result<Box<dyn BackendOps>, Box<dyn Error>> {
        if !self.available_backends.contains(&backend_type) {
            // TODO: erro from string or custom error ?!
            return Err(Box::new(BackendError::BackendNotAvailable));
        }
        match backend_type {
            #[cfg(feature = "cpu_single_core")]
            BackendEnum::CpuSingleCore => Ok(Box::new(BackendCpuSingleCore::new())),

            #[cfg(feature = "cpu_multi_core")]
            BackendEnum::CpuMultiCore => Ok(Box::new(BackendCpuMultiCore::new())),

            #[cfg(feature = "cuda")]
            BackendEnum::Cuda => Ok(Box::new(BackendCuda::new())),

            #[cfg(feature = "wasm")]
            BackendEnum::Wasm => Ok(Box::new(BackendWasm::new())),
        }
    }
}

#[derive(Debug)]
enum BackendError {
    BackendNotAvailable,
}

//TODO: error handling =!=! the display trait ?!
impl Error for BackendError {
    fn description(&self) -> &str {
        "I'm the superhero of errors"
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BackendError!")
    }
}

impl fmt::Display for BackendEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "cpu_single_core")]
            BackendEnum::CpuSingleCore => write!(f, "CPU single core "),

            #[cfg(feature = "cpu_multi_core")]
            BackendEnum::CpuMultiCore => write!(f, "CPU multi core "),

            #[cfg(feature = "cuda")]
            BackendEnum::Cuda => write!(f, "CUDA "),

            #[cfg(feature = "wasm")]
            BackendEnum::Wasm => write!(f, "WASM"),
        }
    }
}

pub fn calc_pixel_single<F>(
    world: &mut World,
    c: &Camera,
    f: &F,
    n_samples: usize,
    jitter_matrix: &Vec<f32>,
    lights: &Vec<Light>,
    p: &mut Pixel,
) -> ()
    where
        F: Fn(&Vec<Shape>, &Vec<Light>, &Ray, i32, bool, bool, bool, bool)  -> Color,
{
    let x = p.x;
    let y = p.y;
    if c.get_antialiasing() {
        let mut color = BLACK;

        // Accumulate light for N samples.
        for sample in 0..(n_samples * n_samples) {
            let delta_x = jitter_matrix[2 * sample] * c.get_pixel_size();
            let delta_y = jitter_matrix[2 * sample + 1] * c.get_pixel_size();
            let r = Camera::ray_for_pixel_anti_aliasing(c, x, y, delta_x, delta_y);
            let c = f(
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
        p.color.r = color.r;
        p.color.g = color.g;
        p.color.b = color.b;
    } else {
        let r = Camera::ray_for_pixel(c, x, y);
        let mut color = f(
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
}

pub fn get_antialiasing_params(c: &Camera) -> (usize, Vec<f32>) {
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

    (n_samples, jitter_matrix)
}
