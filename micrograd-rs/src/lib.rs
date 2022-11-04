extern crate core;

pub mod graph_v2;
pub mod graph_v3;
pub mod micrograd_rs_engine_v2;
pub mod micrograd_rs_engine_v3;
pub mod micrograd_rs_engine_v4_tensor;
pub mod micrograd_rs_v2;
pub mod micrograd_rs_v3;
pub mod micrograd_rs_v4_mathtensor;
pub mod micrograd_rs_v4_tensor;
pub mod micrograd_rs_v4_tensorinternal;

pub const EPS: f64 = 0.0001;
pub const EPS2: f64 = 0.3;

pub fn assert_two_float(a: f64, b: f64) -> bool {
    if (a - b).abs() < EPS {
        return true;
    }
    false
}

pub fn assert_float(actual: f64, expected: f64) {
    assert!(assert_two_float(actual, expected));
}

// pub mod prelude {
//     pub use super::graph_v2::*;
//     pub use super::micrograd_rs_engine_v2::*;
//     pub use super::micrograd_rs_engine_v3::*;
//     pub use super::micrograd_rs_v2::*;
// }
