use crate::math::tuple4d::{Tuple, Tuple4D};

#[derive(Debug)]
pub struct IndexedModel {
    positions: Vec<Tuple4D>,
    tex_coords: Vec<Tuple4D>,
    normals: Vec<Tuple4D>,
    tangents: Vec<Tuple4D>,
    indices: Vec<usize>,
}

impl IndexedModel {
    pub fn new() -> IndexedModel {
        IndexedModel {
            positions: vec![],
            tex_coords: vec![],
            normals: vec![],
            tangents: vec![],
            indices: vec![],
        }
    }

    pub fn calc_normals(&mut self) {
        for x in (0..self.indices.len() - 3).step_by(3) {
            let i0 = *self.indices.get(x).unwrap();
            let i1 = *self.indices.get(x + 1).unwrap();
            let i2 = *self.indices.get(x + 2).unwrap();

            let v1 = self.positions.get(i1).unwrap() - self.positions.get(i0).unwrap();
            let v2 = self.positions.get(i2).unwrap() - self.positions.get(i0).unwrap();

            let normal = Tuple4D::normalize(&(&v1 * &v2));

            let n = self.normals.get(i0).unwrap() + &normal;
            self.normals[i0] = n;

            let n = self.normals.get(i1).unwrap() + &normal;
            self.normals[i1] = n;

            let n = self.normals.get(i2).unwrap() + &normal;
            self.normals[i2] = n;
        }

        self.normals.iter_mut().for_each(|n| *n = Tuple4D::normalize(n));
    }

    pub fn calc_tangents(&mut self) {
        // self.indices.iter().for_each(|index| // println!("index = {:?}", index));
        // self.positions.iter().for_each(|pos| // println!("position = {:?}", pos));

        // println!(
        //     "indices.len() = {}, positions.len() = {} ",
        //     self.indices.len(),
        //     self.positions.len()
        // );

        for x in (0..self.indices.len() - 3).step_by(3) {
            let i0 = *self.indices.get(x).unwrap();
            let i1 = *self.indices.get(x + 1).unwrap();
            let i2 = *self.indices.get(x + 2).unwrap();

            //            // println!(
            //                "i0 = {}, i1 = {}, i2 = {},    x = {},   positions.len() = {}",
            //                i0,
            //                i1,
            //                i2,
            //                x,
            //                self.positions.len()
            //            );

            let edge1 = self.positions.get(i1).unwrap() - self.positions.get(i0).unwrap();
            let edge2 = self.positions.get(i2).unwrap() - self.positions.get(i0).unwrap();

            let delta_u1 = self.tex_coords.get(i1).unwrap().get_x() - self.tex_coords.get(i0).unwrap().get_x();
            let delta_v1 = self.tex_coords.get(i1).unwrap().get_y() - self.tex_coords.get(i0).unwrap().get_y();

            let delta_u2 = self.tex_coords.get(i2).unwrap().get_x() - self.tex_coords.get(i0).unwrap().get_x();
            let delta_v2 = self.tex_coords.get(i2).unwrap().get_y() - self.tex_coords.get(i0).unwrap().get_y();

            let dividend = delta_u1 * delta_v2 - delta_u2 * delta_v1;

            let f = if dividend == 0.0 { 0.0 } else { 1.0 / dividend };

            let tangent = Tuple4D::new_vector(
                f * (delta_v2 * edge1.get_x() - delta_v1 * edge2.get_x()),
                f * (delta_v2 * edge1.get_y() - delta_v1 * edge2.get_y()),
                f * (delta_v2 * edge1.get_z() - delta_v1 * edge2.get_z()),
            );

            let t = self.tangents.get(i0).unwrap() + &tangent;
            self.tangents[i0] = t;

            let t = self.tangents.get(i1).unwrap() + &tangent;
            self.tangents[i1] = t;

            let t = self.tangents.get(i2).unwrap() + &tangent;
            self.tangents[i2] = t;
        }

        self.tangents.iter_mut().for_each(|t| *t = Tuple4D::normalize(t));
    }

    pub fn positions(&self) -> &Vec<Tuple4D> {
        &self.positions
    }

    pub fn positions_mut(&mut self) -> &mut Vec<Tuple4D> {
        &mut self.positions
    }

    pub fn tex_coords(&self) -> &Vec<Tuple4D> {
        &self.tex_coords
    }

    pub fn tex_coords_mut(&mut self) -> &mut Vec<Tuple4D> {
        &mut self.tex_coords
    }

    pub fn normals(&self) -> &Vec<Tuple4D> {
        &self.normals
    }

    pub fn normals_mut(&mut self) -> &mut Vec<Tuple4D> {
        &mut self.normals
    }

    pub fn tangents(&self) -> &Vec<Tuple4D> {
        &self.tangents
    }

    pub fn tangents_mut(&mut self) -> &mut Vec<Tuple4D> {
        &mut self.tangents
    }

    pub fn indices_mut(&mut self) -> &mut Vec<usize> {
        &mut self.indices
    }

    // move on purpose ...
    pub fn indices(self) -> Vec<usize> {
        self.indices
    }

    pub fn indices_borrow(&self) -> &Vec<usize> {
        &self.indices
    }
}
