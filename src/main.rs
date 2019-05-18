use crate::math::common::float_equal;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::ray::Ray;
use crate::math::ray::RayOps;
use crate::math::sphere::Sphere;
use crate::math::sphere::SphereOps;
use crate::math::tuple4d::{Tuple, Tuple4D};

mod math;

fn main() {
    let o = Tuple4D::new_point(0.0, 1.0, -5.0);
    let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
    let r = Ray::new(o, d);

    let s = Sphere::new();

    let intersections = Sphere::intersect(&s, &r).unwrap();

    assert_eq!(intersections.len(), 2);

    assert_eq!(float_equal(intersections[0], 5.0), true);
    assert_eq!(float_equal(intersections[1], 5.0), true);
}
