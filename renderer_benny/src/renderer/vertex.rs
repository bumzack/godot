use crate::math::matrix::Matrix;
use crate::math::tuple4d::{Tuple, Tuple4D};

#[derive(Clone, Debug)]

pub struct Vertex {
    pos: Tuple4D,
    tex_coords: Tuple4D,
    normal: Tuple4D,
}

impl Vertex {
    pub fn new(pos: Tuple4D, tex_coords: Tuple4D, normal: Tuple4D) -> Vertex {
        Vertex {
            pos,
            tex_coords,
            normal,
        }
    }

    pub fn x(&self) -> f32 {
        self.pos.get_x()
    }

    pub fn y(&self) -> f32 {
        self.pos.get_y()
    }

    pub fn z(&self) -> f32 {
        self.pos.get_z()
    }

    pub fn w(&self) -> f32 {
        self.pos.get_w()
    }

    pub fn pos(&self) -> &Tuple4D {
        &self.pos
    }

    pub fn tex_coords(&self) -> &Tuple4D {
        &self.tex_coords
    }

    pub fn normal(&self) -> &Tuple4D {
        &self.normal
    }

    pub fn transform(&self, transform_matrix: &Matrix, normal_transform_matrix: &Matrix) -> Vertex {
        let pos_transformed = transform_matrix * &self.pos;
        let tex_coords = self.tex_coords;
        let normal_transformed = normal_transform_matrix * &self.normal;

        Vertex::new(pos_transformed, tex_coords, normal_transformed)
    }

    pub fn perspective_divide(&self) -> Vertex {
        Vertex::new(
            Tuple4D::new(self.x() / self.w(), self.y() / self.w(), self.z() / self.w(), self.w()),
            self.tex_coords,
            self.normal,
        )
    }

    pub fn triangle_area_times_two(&self, a: &Vertex, b: &Vertex) -> f32 {
        let x1 = a.x() - self.x();
        let y1 = a.y() - self.y();

        let x2 = b.x() - self.x();
        let y2 = b.y() - self.y();

        x1 * y2 - x2 * y1
    }

    pub fn lerp(&self, other: &Vertex, lerp_amt: f32) -> Vertex {
        Vertex {
            pos: self.pos.lerp(&other.pos, lerp_amt),
            tex_coords: self.tex_coords.lerp(&other.tex_coords, lerp_amt),
            normal: self.normal.lerp(&other.normal, lerp_amt),
        }
    }

    pub fn is_inside_view_frustum(&self) -> bool {
        self.x().abs() <= self.w().abs() && self.y().abs() <= self.w().abs() && self.z().abs() <= self.w().abs()
    }

    pub fn get(&self, index: usize) -> f32 {
        match index {
            0 => self.x(),
            1 => self.y(),
            2 => self.z(),
            3 => self.w(),
            _ => panic!("index out of bounds - check your code"),
        }
    }
}

impl core::fmt::Display for Vertex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Vertex").unwrap();
        writeln!(f, "     pos             = {} ", self.pos).unwrap();
        writeln!(f, "     tex_coords      = {} ", self.tex_coords).unwrap();
        writeln!(f, "     normal          = {} ", self.normal)
    }
}
