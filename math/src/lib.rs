#![feature(core_intrinsics)]
//
//#![cfg(feature = "cuda")]
//#![no_std]
//
//#![cfg(feature = "cuda")]
//extern crate rustacuda_core;
//
//#[cfg(test)]
//extern crate std;
pub mod matrix_math;
pub use self::matrix_math::*;

pub mod prelude {
    pub use super::matrix_math::*;
}

