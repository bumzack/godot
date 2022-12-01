extern crate math;
extern crate utils;

mod camera;
mod canvas;
mod color;
mod edge;
mod gradient;
mod mesh;
mod pixel;
mod render_context;
mod vertex;

pub mod prelude {
    pub use super::camera::*;
    pub use super::canvas::*;
    pub use super::color::*;
    pub use super::edge::*;
    pub use super::gradient::*;
    pub use super::mesh::*;
    pub use super::render_context::*;
    pub use super::vertex::*;
}
