pub use self::backend::*;
pub use self::backend_cpu::*;

mod backend;
mod backend_cpu;

pub const MAX_REFLECTION_RECURSION_DEPTH: i32 = 10;

#[cfg(feature = "cuda")]
mod backend_cuda;

#[cfg(feature = "cuda")]
pub use self::backend_cuda::*;
