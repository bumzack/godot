use std::f32::consts::FRAC_1_SQRT_2;

use crate::math::common::assert_tuple;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::sphere::{Sphere, SphereOps};
//use crate::math::sphere::{Sphere, SphereOps};
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;
//use crate::math::common::assert_tuple;
//use crate::math::matrix::Matrix;
//use crate::math::matrix::MatrixOps;
use crate::math::world::{World, WorldOps};

mod math;

fn main() {
    let mut w = World::new(500, 500);
    w.render_scene();
    w.write_ppm("blupp.ppm");

    let mut s = Sphere::new();
    s.set_transformation(Matrix::translation(0.0, 1.0, 0.0));

    let p = Tuple4D::new_point(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
    let n = s.normal_at(&p);
    let n_expected = Tuple4D::new_vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
    println!("test_sphere_normal_at_transformed    n = {:#?}, n_expected = {:#?}", n, n_expected);
    assert_tuple(&n, &n_expected);
}
