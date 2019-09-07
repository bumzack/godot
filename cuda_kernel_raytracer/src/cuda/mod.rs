pub mod cuda_kernel;
pub mod intersection;
pub mod intersection_list;

pub const MAX_REFLECTION_RECURSION_DEPTH: i32 = 10;
pub const MAX_LIGHT_SAMPLES :usize = 50;