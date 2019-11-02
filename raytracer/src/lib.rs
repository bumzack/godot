#![feature(stmt_expr_attributes)]

#[cfg_attr(feature = "cuda", macro_use)]
#[cfg(feature = "cuda")]
extern crate rustacuda;
#[cfg(feature = "cuda")]
extern crate rustacuda_core;
#[cfg(feature = "cuda")]
extern crate rustacuda_derive;

#[cfg(feature = "use_serde")]
extern crate serde;

extern crate cpu_kernel_raytracer;
extern crate raytracer_lib_std;

pub use self::backend::*;

pub use self::cpu_kernel_raytracer::*;
pub use self::raytracer_lib_no_std::*;
pub use self::raytracer_lib_std::*;
pub use self::utils::*;

mod backend;
mod utils;

pub mod prelude {
    #[cfg(feature = "cuda")]
    pub use super::BackendCuda;

    #[cfg(feature = "wasm")]
    pub use super::BackendWasm;

    #[cfg(feature = "cpu_single_core")]
    pub use super::BackendCpuSingleCore;

    #[cfg(feature = "cpu_multi_core")]
    pub use super::BackendCpuMultiCore;

    pub use super::backend::Backend;
    pub use super::backend::BackendOps;
    pub use super::cpu_kernel_raytracer::*;
    pub use super::raytracer_lib_no_std::*;
    pub use super::raytracer_lib_std::*;
    pub use super::utils::*;
}
