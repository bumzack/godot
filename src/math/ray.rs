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
    fn position(r: &Ray, t: f32) -> Tuple4D;
}

impl RayOps for Ray {
    fn new(origin: Tuple4D, direction: Tuple4D) -> Ray {
        assert!(Tuple4D::is_point(&origin));
        assert!(Tuple4D::is_vector(&direction));
        Ray {
            origin,
            direction,
        }
    }

    fn position(r: &Ray, t: f32) -> Tuple4D {
        &r.origin + &(&r.direction * t)
    }
}


#[test]
fn test_ray_new() {
    let o = Tuple4D::new_point(1.0, 2.0, 3.0);
    let d = Tuple4D::new_vector(4.0, 5.0, 6.0);

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

#[test]
fn test_ray_position() {
    let o = Tuple4D::new_point(2.0, 3.0, 4.0);
    let d = Tuple4D::new_vector(1.0, 0.0, 0.0);

    let r = Ray::new(o, d);

    let p1 = Ray::position(&r, 0.0);
    let p2 = Ray::position(&r, 1.0);
    let p3 = Ray::position(&r, -1.0);
    let p4 = Ray::position(&r, 2.5);

    assert_eq!(Tuple4D::is_point(&p1), true);
    assert_eq!(Tuple4D::is_point(&p2), true);
    assert_eq!(Tuple4D::is_point(&p3), true);
    assert_eq!(Tuple4D::is_point(&p4), true);

    assert_eq!(float_equal(p1.x, 2.0), true);
    assert_eq!(float_equal(p1.y, 3.0), true);
    assert_eq!(float_equal(p1.z, 4.0), true);

    assert_eq!(float_equal(p2.x, 3.0), true);
    assert_eq!(float_equal(p2.y, 3.0), true);
    assert_eq!(float_equal(p2.z, 4.0), true);

    assert_eq!(float_equal(p3.x, 1.0), true);
    assert_eq!(float_equal(p3.y, 3.0), true);
    assert_eq!(float_equal(p3.z, 4.0), true);

    assert_eq!(float_equal(p4.x, 4.5), true);
    assert_eq!(float_equal(p4.y, 3.0), true);
    assert_eq!(float_equal(p4.z, 4.0), true);
}









