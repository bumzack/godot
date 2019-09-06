#![feature(stmt_expr_attributes)]

#[cfg(feature = "cuda")]
extern crate rustacuda;
#[cfg(feature = "cuda")]
extern crate rustacuda_core;
#[cfg(feature = "cuda")]


// #[cfg(feature = "cuda")]
// #[macro_use]

#[cfg_attr(cuda, macro_use)]
extern crate rustacuda_derive;

extern crate cpu_kernel_raytracer;
extern crate raytracer_lib_std;

pub use self::backend::*;
pub use self::cpu_kernel_raytracer::*;
pub use self::raytracer_lib_std::*;
pub use self::raytracer_lib_no_std::*;

#[cfg(feature = "cuda")]
pub use self::BackendCuda;

mod backend;
