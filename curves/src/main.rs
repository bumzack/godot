use raytracer_challenge_reference_impl::math::{Matrix, MatrixOps};
use raytracer_challenge_reference_impl::prelude::{Tuple, Tuple4D};

use crate::curve::{Curve, CurveCommon, CurveType};

mod curve;

fn main() {
    let trans = Matrix::new_identity_4x4();
    let trans_inv = Matrix::new_identity_4x4();

    let p0 = Tuple4D::new_point(0.0, 0.0, 0.0);
    let p1 = Tuple4D::new_point(1.0, 0.0, 0.0);
    let p2 = Tuple4D::new_point(1.0, 1.0, 0.0);
    let p3 = Tuple4D::new_point(0.0, 1.0, 0.0);
    let n1 = Tuple4D::new_vector(0.0, 1.0, 0.0);
    let n2 = Tuple4D::new_vector(0.0, 1.0, 0.0);

    let width0 = 1.0;
    let width1 = 2.0;

    let c = CurveCommon::new(p0, p1, p2, p3, CurveType::RIBBON, n1, n2, width0, width1);
    let u_min = 0.0;
    let u_max = 1.0;
    let reverse_orientation = false;
    let c = Curve::new(trans, trans_inv, reverse_orientation, c, u_min, u_max);
}
