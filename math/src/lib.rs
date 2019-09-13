#![feature(core_intrinsics)]
#![cfg_attr(feature = "cuda", no_std)]

#[cfg(feature = "cuda")]
#[macro_use]
extern crate rustacuda_derive;

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod matrix_math;
pub use self::matrix_math::*;

pub mod prelude {
    pub use super::matrix_math::*;
}
