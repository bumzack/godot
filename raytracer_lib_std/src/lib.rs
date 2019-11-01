#[cfg(feature = "use_serde")]
#[macro_use]
extern crate serde_derive;

extern crate math;

pub use raytracer_lib_no_std;

pub use self::canvas::*;
pub use self::scene::*;
pub use self::world::*;

pub mod canvas;
pub mod scene;
pub mod world;

#[cfg(not(feature = "wasm"))]
pub mod canvas_std;

#[cfg(not(feature = "wasm"))]
pub use self::canvas_std::*;

#[cfg(not(feature = "wasm"))]
extern crate image;
