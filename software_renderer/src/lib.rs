#![feature(core_intrinsics)]
#![cfg_attr(feature = "cuda", no_std)]
#![cfg_attr(feature = "wasm", no_std)]

extern crate math;
extern crate raytracer_lib_std;
#[cfg(feature = "cuda")]
#[macro_use]
extern crate rustacuda_derive;
#[cfg(feature = "use_serde")]
#[macro_use]
extern crate serde_derive;

mod edge;
mod gradient;
mod render_context;
mod vertex;

pub mod prelude {
    pub use super::edge::*;
    pub use super::gradient::*;
    pub use super::render_context::*;
    pub use super::vertex::*;
}
