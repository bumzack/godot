#[cfg(feature = "cuda")]
extern crate rustacuda_core;

use crate::math::{Tuple, Tuple4D};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Vertex {
    m_pos: Tuple4D,
    m_tex_coord: Tuple4D,
    m_normal: Tuple4D,
}

pub trait VertexOps {
    fn new() -> Vertex;
}

impl VertexOps for Vertex {
    fn new() -> Vertex {
        let v = Vertex {
            m_pos: Tuple4D::empty(),
            m_tex_coord: Tuple4D::empty(),
            m_normal: Tuple4D::empty(),
        };
        v
    }
}
