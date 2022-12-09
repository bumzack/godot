extern crate core;

pub mod basics;
pub mod example_scenes;
pub mod light;
pub mod material;
pub mod math;
pub mod patterns;
pub mod shape;
pub mod world;

pub const DEBUG: bool = false;

pub mod prelude {
    pub use super::basics::*;
    pub use super::light::*;
    pub use super::material::*;
    pub use super::math::*;
    pub use super::patterns::*;
    pub use super::shape::*;
    pub use super::world::*;
}
