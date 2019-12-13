#![feature(core_intrinsics)]
#![cfg_attr(feature = "cuda", no_std)]
#![cfg_attr(feature = "wasm", no_std)]

extern crate math;
#[cfg(feature = "cuda")]
#[macro_use]
extern crate rustacuda_derive;
#[cfg(feature = "use_serde")]
#[macro_use]
extern crate serde_derive;

mod vertex;

pub mod prelude {
    pub use super::vertex::*;
}
