pub mod cuda_kernel;
pub mod intersection;
pub mod intersection_list;
pub mod shape_idx_list;

pub const MAX_REFLECTION_RECURSION_DEPTH: i32 = 5;
pub const MAX_LIGHT_SAMPLES: usize = 100;
