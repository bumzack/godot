use crate::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    children: Vec<ShapeIdx>,
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
}

impl ShapeOps for Group {
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

    fn local_normal_at(&self, local_point: &Tuple4D) -> Tuple4D {
        unimplemented!()
    }

    fn set_material(&mut self, m: Material) {
        unimplemented!()
    }

    fn get_material(&self) -> &Material {
        unimplemented!()
    }

    fn get_material_mut(&mut self) -> &mut Material {
        unimplemented!()
    }
}

impl Group {
    pub fn new() -> Group {
        Group {
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            children: Vec::new(),
        }
    }

    pub fn intersect(r: &Ray) -> Option<Vec<f32>> {
        let res = vec![0.0; 2];

        Some(res)
    }

    pub fn get_children(&self) -> &Vec<usize> {
        &self.children
    }

    pub fn add_child(&mut self, idx: usize, shapes: &mut Vec<ShapeEnum>) {
        // shapes[idx]
    }
}

#[cfg(test)]
mod tests {
    use crate::math::common::assert_matrix;

    use super::*;

    // page 195
    fn test_group_new() {
        let g = Group::new();

        let identity_matrix = Matrix::new_identity_4x4();

        assert_matrix(&g.get_transformation(), &identity_matrix);
        //assert_eq!(&g.get_children().len(), 0);
    }
}
