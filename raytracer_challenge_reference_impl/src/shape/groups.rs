use crate::basics::ray::{Ray};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple4D;
use crate::shape::shape::{ShapeEnum, ShapeIdx};

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    children: Vec<ShapeIdx>,
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
}

pub trait GroupOps {
    fn new() -> Group;
    fn intersect(r: &Ray) -> Option<Vec<f32>>;

    fn set_transformation(&mut self, m: Matrix);
    fn get_transformation(&self) -> &Matrix;
    fn get_inverse_transformation(&self) -> &Matrix;

    fn normal_at(&self, p: &Tuple4D) -> Tuple4D;

    fn get_children(&self) -> &Vec<ShapeIdx>;

    fn add_child(&mut self, idx: ShapeIdx, shapes: &mut Vec<ShapeEnum>);
    //
    //    fn set_material(&mut self, m: Material);
    //    fn get_material(&self) -> &Material;
    //    fn get_material_mut(&mut self) -> &mut Material;
    //
    //    fn check_axis(origin: f32, direction: f32) -> (f32, f32);
}

impl GroupOps for Group {
    fn new() -> Group {
        Group {
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            children: Vec::new(),
        }
    }

    fn intersect(r: &Ray) -> Option<Vec<f32>> {
        let res = vec![0.0; 2];

        Some(res)
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix =
            Matrix::invert(&m).expect("Group::set_transofrmation: cant unwrap inverse matrix");
        self.transformation_matrix = m;
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }

    fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }

    fn normal_at(&self, world_point: &Tuple4D) -> Tuple4D {
        panic!("Group::normal_at() should never be called!");
    }

    fn get_children(&self) -> &Vec<usize> {
        &self.children
    }

    fn add_child(&mut self, idx: usize, shapes: &mut Vec<ShapeEnum>) {
        // shapes[idx]
    }

    //    fn set_material(&mut self, m: Material) {
    //        self.material = m;
    //    }
    //
    //    fn get_material(&self) -> &Material {
    //        &self.material
    //    }
    //
    //    fn get_material_mut(&mut self) -> &mut Material {
    //        &mut self.material
    //    }
    //
    //    fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
    //        let tmin_numerator = -1.0 - origin;
    //        let tmax_numerator = 1.0 - origin;
    //
    //        let mut tmin;
    //        let mut tmax;
    //
    //        if direction.abs() >= EPSILON {
    //            tmin = tmin_numerator / direction;
    //            tmax = tmax_numerator / direction;
    //        } else {
    //            tmin = tmin_numerator * INFINITY;
    //            tmax = tmax_numerator * INFINITY;
    //        }
    //        if tmin > tmax {
    //            let tmp = tmin;
    //            tmin = tmax;
    //            tmax = tmp;
    //        }
    //        (tmin, tmax)
    //    }
}

#[cfg(test)]
mod tests {
    use crate::math::common::{ assert_matrix};

    use super::*;

    // page 195
    fn test_group_new() {
        let g = Group::new();

        let identity_matrix = Matrix::new_identity_4x4();

        assert_matrix(&g.get_transformation(), &identity_matrix);
        //assert_eq!(&g.get_children().len(), 0);
    }
}
