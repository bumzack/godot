#[cfg(feature = "cuda")]
extern crate rustacuda_core;

use math::Matrix;

use crate::math::{Tuple, Tuple4D};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Vertex {
    pos: Tuple4D,
    tex_coords: Tuple4D,
    normal: Tuple4D,
}

impl Vertex {
    fn new(pos: Tuple4D, tex_coords: Tuple4D, normal: Tuple4D) -> Vertex {
        let v = Vertex {
            pos,
            tex_coords,
            normal,
        };
        v
    }

    fn x(&self) -> f32 {
        self.pos.get_x()
    }
    fn y(&self) -> f32 {
        self.pos.get_y()
    }

    fn z(&self) -> f32 {
        self.pos.get_z()
    }

    fn w(&self) -> f32 {
        self.pos.get_w()
    }

    fn pos(&self) -> &Tuple4D {
        &self.pos
    }

    fn tex_coords(&self) -> &Tuple4D {
        &self.tex_coords
    }

    fn normal(&self) -> &Tuple4D {
        &self.normal
    }

    fn transform(&self, transform_matrix: &Matrix, normal_transform_matrix: &Matrix) -> Vertex {
        let pos_transformed = transform_matrix * self.pos;
        let tex_coords = tex_coords.clone();
        let normal_transformed = normal_transform_matrix * self.normal;

        Vertex::new(pos_transformed, tex_coords, normal_transformed)
    }
}
