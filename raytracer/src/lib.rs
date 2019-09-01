#![feature(stmt_expr_attributes)]

// #[macro_use]
//extern crate rustacuda;

extern crate cpu_kernel_raytracer;
extern crate raytracer_lib_std;

pub use self::backend::*;
pub use self::cpu_kernel_raytracer::*;
pub use self::raytracer_lib_std::*;

mod backend;

