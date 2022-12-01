// #![cfg_attr(feature = "cuda", no_std)]
// #![cfg_attr(feature = "wasm", no_std)]
//
//
// #[macro_use]
// extern crate rustacuda_derive;
//
// #[cfg(feature = "wasm")]
// extern crate libm;

#[cfg(feature = "use_serde")]
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod matrix_math;
pub const MAX_REFLECTION_RECURSION_DEPTH: i32 = 5;

pub use self::matrix_math::*;
pub mod prelude {
    pub use super::matrix_math::*;
    pub use super::MAX_REFLECTION_RECURSION_DEPTH;
}
