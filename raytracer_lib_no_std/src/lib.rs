#![cfg_attr(target_os = "cuda", feature(abi_ptx, proc_macro_hygiene))]
#![cfg_attr(target_os = "cuda", no_std)]
#![feature(core_intrinsics)]
#![no_std]


// enable std for tests :-)
#[cfg(test)]
#[macro_use]
extern crate std;


#[cfg(test)]
extern crate cpu_kernel_raytracer;

#[macro_use]
extern crate rustacuda_derive;
extern crate rustacuda_core;

pub mod basics;
pub mod light;
pub mod material;
pub mod math;
pub mod patterns;
pub mod shape;
