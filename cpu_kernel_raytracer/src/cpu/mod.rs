pub use self::cpu_kernel::*;

mod cpu_kernel;
mod intersection;
mod intersection_list;

pub const MAX_REFLECTION_RECURSION_DEPTH: i32 = 10;
