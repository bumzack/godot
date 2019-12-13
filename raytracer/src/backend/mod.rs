pub use self::backend::*;
#[cfg(feature = "cpu_multi_core")]
pub use self::backend_cpu_multi_core::*;
#[cfg(feature = "cpu_single_core")]
pub use self::backend_cpu_single_core::*;
#[cfg(feature = "cuda")]
pub use self::backend_cuda::*;
#[cfg(feature = "wasm")]
pub use self::backend_wasm::*;

#[cfg(any(feature = "cpu_single_core", feature = "wasm"))]
mod backend_cpu_single_core;

#[cfg(feature = "cpu_multi_core")]
mod backend_cpu_multi_core;

#[cfg(feature = "cuda")]
mod backend_cuda;

#[cfg(feature = "wasm")]
mod backend_wasm;

mod backend;
mod backend_helper;
