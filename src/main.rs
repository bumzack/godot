use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;

mod math;

fn main() {
    let a = Matrix::new_matrix_4x4(-5.0, 2.0, 6.0, -8.0,
                                   1.0, -5.0, 1.0, 8.0,
                                   7.0, 7.0, -6.0, -7.0,
                                   1.0, -3.0, 7.0, 4.0);

    let b = Matrix::invert(&a).unwrap();
}
