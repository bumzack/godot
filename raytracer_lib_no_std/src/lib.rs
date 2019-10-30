// #![cfg_attr(target_os = "cuda", feature(abi_ptx, proc_macro_hygiene))]
//  #![cfg_attr(target_os = "cuda", no_std)]

#![feature(core_intrinsics)]
#![no_std]

#[cfg(test)]
extern crate cpu_kernel_raytracer;
extern crate math;
#[cfg(feature = "cuda")]
#[macro_use]
extern crate rustacuda_derive;
#[macro_use]
extern crate serde_derive;
// enable std for tests :-)
#[cfg(test)]
#[macro_use]
extern crate std;

pub use self::basics::*;
pub use self::light::*;
pub use self::material::*;
pub use self::math::*;
pub use self::patterns::*;
pub use self::shape::*;

pub mod basics;
pub mod light;
pub mod material;
pub mod patterns;
pub mod shape;

pub mod prelude {
    pub use super::basics::*;
    pub use super::light::*;
    pub use super::material::*;
    pub use super::math::*;
    pub use super::patterns::*;
    pub use super::shape::*;
}

pub const DEBUG: bool = false;
