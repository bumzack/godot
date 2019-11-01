pub use self::backend::*;

#[cfg(not(feature = "wasm"))]
pub use self::backend_cpu_single_core::*;

#[cfg(not(feature = "wasm"))]
pub use self::backend_cpu_multi_core::*;


#[cfg(feature = "cuda")]
pub use self::backend_cuda::*;

#[cfg(feature = "wasm")]
pub use self::backend_wasm::*;

mod backend;

#[cfg(not(feature = "wasm"))]
mod backend_cpu_single_core;

#[cfg(not(feature = "wasm"))]
mod backend_cpu_multi_core;

pub const MAX_REFLECTION_RECURSION_DEPTH: i32 = 5;

#[cfg(feature = "cuda")]
mod backend_cuda;

#[cfg(feature = "wasm")]
mod backend_wasm;