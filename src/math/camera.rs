use std::f32::consts::{PI, SQRT_2};

use crate::math::common::{assert_float, assert_matrix};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;

#[derive(Clone, Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    transform: Matrix,
}

pub trait CameraOps {
    fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera;

    fn get_hsize(&self) -> usize;
    fn get_vsize(&self) -> usize;
    fn get_field_of_view(&self) -> f32;
    fn get_transform(&self) -> &Matrix;
}

impl CameraOps for Camera {
    fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera {
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::new_identity_4x4(),
        }
    }

    fn get_hsize(&self) -> usize {
        self.hsize
    }

    fn get_vsize(&self) -> usize {
        self.vsize
    }

    fn get_field_of_view(&self) -> f32 {
        self.field_of_view
    }

    fn get_transform(&self) -> &Matrix {
        &self.transform
    }
}

#[test]
fn test_camera_new() {
    let c = Camera::new(160, 120, PI / SQRT_2);

    assert_eq!(c.get_hsize(), 160);
    assert_eq!(c.get_vsize(), 120);

    assert_float(c.get_field_of_view(), PI / SQRT_2);
    assert_matrix(c.get_transform(), &Matrix::new_identity_4x4());
}

