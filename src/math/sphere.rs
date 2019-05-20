use std::f32::consts::PI;
use std::ops::Mul;

use crate::math::common::{assert_matrices, float_equal};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::ray::Ray;
use crate::math::ray::RayOps;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;

pub struct Sphere {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
}

pub trait SphereOps {
    fn new() -> Sphere;
    fn intersect(s: &Sphere, r: &Ray) -> Option<Vec<f32>>;

    fn set_transformation(&mut self, m: Matrix);
}

impl SphereOps for Sphere {
    fn new() -> Sphere {
        Sphere {
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
        }
    }

    fn intersect(s: &Sphere, r: &Ray) -> Option<Vec<f32>> {
        let sphere_to_ray = &r.origin - &Tuple4D::new_point(0.0, 0.0, 0.0);
        let a = &r.direction ^ &r.direction;
        let b = 2.0 * (&r.direction ^ &sphere_to_ray);
        let c = (&sphere_to_ray ^ &sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if (discriminant < 0.0) {
            return None;
        }
        let mut res = vec![0.0; 2];
        res[0] = (-b + discriminant.sqrt()) / (2.0 * a);
        res[1] = (-b - discriminant.sqrt()) / (2.0 * a);

        res.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Some(res)
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix = Matrix::invert(&m).unwrap();
        self.transformation_matrix = m;
    }
}


#[test]
fn test_ray_sphere_intersection() {
    let o = Tuple4D::new_point(0.0, 0.0, -5.0);
    let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
    let r = Ray::new(o, d);

    let s = Sphere::new();

    let intersections = Sphere::intersect(&s, &r).unwrap();

    assert_eq!(intersections.len(), 2);

    assert_eq!(float_equal(intersections[0], 4.0), true);
    assert_eq!(float_equal(intersections[1], 6.0), true);

    let o = Tuple4D::new_point(0.0, 1.0, -5.0);
    let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
    let r = Ray::new(o, d);

    let s = Sphere::new();

    let intersections = Sphere::intersect(&s, &r).unwrap();

    assert_eq!(intersections.len(), 2);

    assert_eq!(float_equal(intersections[0], 5.0), true);
    assert_eq!(float_equal(intersections[1], 5.0), true);
}

#[test]
fn test_ray_sphere_intersection_no_hits() {
    let o = Tuple4D::new_point(0.0, 2.0, -5.0);
    let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
    let r = Ray::new(o, d);

    let s = Sphere::new();

    let intersections = Sphere::intersect(&s, &r);

    assert_eq!(intersections, None);
}

#[test]
fn test_ray_sphere_intersection_origin_inside_sphere() {
    let o = Tuple4D::new_point(0.0, 0.0, 0.0);
    let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
    let r = Ray::new(o, d);

    let s = Sphere::new();

    let intersections = Sphere::intersect(&s, &r).unwrap();

    assert_eq!(intersections.len(), 2);

    assert_eq!(float_equal(intersections[0], -1.0), true);
    assert_eq!(float_equal(intersections[1], 1.0), true);
}

#[test]
fn test_ray_sphere_intersection_sphere_behind_origin() {
    let o = Tuple4D::new_point(0.0, 0.0, 5.0);
    let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
    let r = Ray::new(o, d);

    let s = Sphere::new();

    let intersections = Sphere::intersect(&s, &r).unwrap();

    assert_eq!(intersections.len(), 2);

    assert_eq!(float_equal(intersections[0], -6.0), true);
    assert_eq!(float_equal(intersections[1], -4.0), true);
}

#[test]
fn test_sphere_transformation() {
    let mut s = Sphere::new();
    let m = Matrix::translation(2.0, 3.0, 4.0);

    s.set_transformation(m);

    let m = Matrix::translation(2.0, 3.0, 4.0);

    assert_matrices(&s.transformation_matrix, &m);
}

