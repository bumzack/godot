#![feature(stmt_expr_attributes)]

extern crate cpu_kernel_raytracer;
extern crate raytracer_lib_std;
#[cfg(feature = "cuda")]
extern crate rustacuda_core;

#[cfg_attr(feature = "cuda", macro_use)]

#[cfg(feature = "cuda")]
extern crate rustacuda;

#[cfg(feature = "cuda")]
extern crate rustacuda_derive;

pub use self::backend::*;
pub use self::cpu_kernel_raytracer::*;
pub use self::raytracer_lib_no_std::*;
pub use self::raytracer_lib_std::*;
#[cfg(feature = "cuda")]
pub use self::BackendCuda;

mod backend;

pub mod prelude {
    pub use super::cpu_kernel_raytracer::*;
    pub use super::raytracer_lib_no_std::*;
    pub use super::raytracer_lib_std::*;

    pub use super::backend::*;

    #[cfg(feature = "cuda")]
    pub use super::BackendCuda;
}
