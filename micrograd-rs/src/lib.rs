extern crate core;

pub mod graph_v2;
pub mod graph_v3;
pub mod micrograd_rs_engine_v2;
pub mod micrograd_rs_engine_v3;
pub mod micrograd_rs_v2;
pub mod micrograd_rs_v3;

pub mod prelude {
    pub use super::graph_v2::*;
    pub use super::micrograd_rs_engine_v2::*;
    pub use super::micrograd_rs_engine_v3::*;
    pub use super::micrograd_rs_v2::*;
}
