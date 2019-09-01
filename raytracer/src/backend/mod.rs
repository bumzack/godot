pub use self::backend::*;
pub use self::backend_cpu::*;

mod backend;
mod backend_cpu;

pub const MAX_REFLECTION_RECURSION_DEPTH: i32 = 10;

#[cfg(feature = "cuda")]
pub mod backend_cuda;
