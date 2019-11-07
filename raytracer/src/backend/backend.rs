use core::fmt;
use std::error::Error;

#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

use raytracer_lib_no_std::Camera;
use raytracer_lib_std::{Canvas, World};

#[cfg(feature = "cpu_multi_core")]
use crate::BackendCpuMultiCore;
#[cfg(any(feature = "cpu_single_core"))]
use crate::BackendCpuSingleCore;
#[cfg(feature = "cuda")]
use crate::BackendCuda;
#[cfg(feature = "wasm")]
use crate::BackendWasm;

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

// TODO: error handling =!=! the display trait ?!
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
