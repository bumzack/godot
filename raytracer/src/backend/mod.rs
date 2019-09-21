pub use self::backend::*;
pub use self::backend_cpu::*;
#[cfg(feature = "cuda")]
pub use self::backend_cuda::*;

mod backend;
mod backend_cpu;

pub const MAX_REFLECTION_RECURSION_DEPTH: i32 = 5;

#[cfg(feature = "cuda")]
mod backend_cuda;
