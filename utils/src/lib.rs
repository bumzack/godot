extern crate math;

mod indexed_model;
mod obj_file;

pub mod prelude {
    pub use super::indexed_model::*;
    pub use super::obj_file::*;
}
