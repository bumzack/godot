use std::f32::consts::PI;
use std::ops::Mul;

use crate::math::common::float_equal;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;

pub struct Ray {
    pub origin: Tuple4D,
    pub direction: Tuple4D,
}


pub trait RayOps {
    fn new(origin: Tuple4D, direction: Tuple4D) -> Ray;
}

impl RayOps for Ray {
    fn new(origin: Tuple4D, direction: Tuple4D) -> Ray {
        assert!(Tuple4D::is_point(&origin));
        assert!(Tuple4D::is_vector(&direction));
        Ray {
            origin,
            direction
        }
    }
}


#[test]
fn test_ray_new() {
    let o = Tuple4D::new_point(1.0,2.0,3.0);
    let d = Tuple4D::new_vector(4.0,5.0,6.0);

    let r = Ray::new(o, d);

    assert_eq!(Tuple4D::is_point(&r.origin), true);
    assert_eq!(Tuple4D::is_vector(&r.direction), true);

    assert_eq!(float_equal(r.origin.x, 1.0), true);
    assert_eq!(float_equal(r.origin.y, 2.0), true);
    assert_eq!(float_equal(r.origin.z, 3.0), true);

    assert_eq!(float_equal(r.direction.x, 4.0), true);
    assert_eq!(float_equal(r.direction.y, 5.0), true);
    assert_eq!(float_equal(r.direction.z, 6.0), true);
}











