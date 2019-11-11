pub mod intersection;
pub mod intersection_list;
pub mod raytracer_kernel;
pub mod shape_idx_list;

pub const MAX_LIGHT_SAMPLES: usize = 100;

pub mod prelude {
    pub use super::intersection_list::*;
    pub use super::intersection::*;
    pub use super::raytracer_kernel::*;
    pub use super::shape_idx_list::*;
}
