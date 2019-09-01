#![feature(stmt_expr_attributes)]

extern crate cpu_kernel_raytracer;
extern crate raytracer_lib_std;
#[cfg(feature = "cuda")]
extern crate rustacuda;
#[cfg(feature = "cuda")]
extern crate rustacuda_core;
#[cfg(feature = "cuda")]
#[macro_use]
extern crate rustacuda_derive;

pub use self::backend::*;
#[cfg(feature = "cuda")]
pub use self::backend_cuda::*;
pub use self::cpu_kernel_raytracer::*;
pub use self::raytracer_lib_std::*;

mod backend;
