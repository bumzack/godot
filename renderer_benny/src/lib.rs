extern crate image;

mod game;
mod math;
mod renderer;
mod utils;

pub mod prelude {
    pub use super::game::game::*;
    pub use super::math::matrix::*;
    pub use super::math::quaternion::*;
    pub use super::math::transform::*;
    pub use super::math::tuple4d::*;
    pub use super::renderer::camera::*;
    pub use super::renderer::canvas::*;
    pub use super::renderer::color::*;
    pub use super::renderer::edge::*;
    pub use super::renderer::gradient::*;
    pub use super::renderer::mesh::*;
    pub use super::renderer::pixel::*;
    pub use super::renderer::render_context::*;
    pub use super::renderer::vertex::*;
}
