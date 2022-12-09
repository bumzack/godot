use crate::math::matrix::{Matrix, MatrixOps};
use crate::math::quaternion::Quaternion;
use crate::math::transform::Transform;
use crate::math::tuple4d::{Tuple, Tuple4D};
#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]

pub struct Camera {
    transform: Transform,
    projection: Matrix,
}

impl Camera {
    pub fn new(projection: Matrix) -> Camera {
        Camera {
            transform: Transform::new(),
            projection,
        }
    }

    pub fn view_projection(&self) -> &Matrix {
        &self.projection
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn rotate(&mut self, axis: &Tuple4D, angle: f32) {
        self.transform = self
            .transform
            .rotate(Quaternion::new_from_tuple_and_angle(*axis, angle))
    }

    pub fn mve(&mut self, dir: &Tuple4D, amt: f32) {
        self.transform = self.transform.set_pos(self.transform.pos() + &(dir * amt));
    }

    pub fn update(&mut self, _delta: f32) {
        //        let sensitivity_x = 2.66 * delta;
        //        let sensitivity_y = 2.0 * delta;
        //        let mov_amt = 5.0 * delta;

        // TODO: get input from piston and updte like in the java code
    }

    pub fn get_view_projection(&self) -> Matrix {
        let camera_rotation = self.transform.rot().conjugate().to_rotation_matrix();
        let camera_pos = self.transform.pos() * -1.0;
        let camera_translation = Matrix::init_translation(camera_pos.get_x(), camera_pos.get_y(), camera_pos.get_z());

        &self.projection * &(camera_rotation * camera_translation)
    }
}
