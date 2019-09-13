extern crate math;

use math::prelude::*;

pub mod collision;
pub mod force;
pub mod particle;

pub use self::collision::*;
pub use self::force::*;
pub use self::particle::*;

pub mod prelude {
    pub use super::collision::*;
    pub use super::common::*;
    pub use super::force::*;
    pub use super::particle::*;
}
