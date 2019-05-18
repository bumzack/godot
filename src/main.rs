use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;

mod math;

fn main() {
    let a = Matrix::new_matrix_4x4(-2.0, -8.0, 3.0, 5.0,
                                   -3.0, 1.0, 7.0, 3.0,
                                   1.0, 2.0, -9.0, 6.0,
                                   -6.0, 7.0, 7.0, -9.0);

    let cofactor_a1 = Matrix::cofactor(&a, 0, 0);
    let cofactor_a2 = Matrix::cofactor(&a, 0, 1);
    let cofactor_a3 = Matrix::cofactor(&a, 0, 2);
    let cofactor_a4 = Matrix::cofactor(&a, 0, 3);

    let det_a = Matrix::determinant(&a);
}
