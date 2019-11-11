#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

use math::prelude::*;
use math::prelude::math_ops::math_ops::*;

use crate::{Material, MaterialOps, Ray, RayOps, ShapeIntersectionResult, ShapeOps};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Plane {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    material: Material,
}

impl ShapeOps for Plane {
    fn intersect(&self, r: &Ray) -> ShapeIntersectionResult {
        let mut res = [0f32; 4];
        let mut res_cnt = 0;

        if intri_abs(r.get_direction().y) < EPSILON {
            return (res, res_cnt);
        }
        let t = -r.get_origin().y / r.get_direction().y;
        res[0] = t;
        res_cnt = 1;

        return (res, res_cnt);
    }

    fn normal_at(&self, world_point: &Tuple4D) -> Tuple4D {
        // TODO: its for the tests -remove and fix tests and add unreachable
        let object_point = self.get_inverse_transformation() * world_point;
        let local_normal = self.local_normal_at(&object_point);
        let mut world_normal = &Matrix::transpose(self.get_inverse_transformation()) * &local_normal;
        world_normal.w = 0.0;
        Tuple4D::normalize(&world_normal)
    }

    fn local_normal_at(&self, _local_point: &Tuple4D) -> Tuple4D {
        Tuple4D::new_vector(0.0, 1.0, 0.0)
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix =
            Matrix::invert(&m).expect("plane::set_transofrmation: cant unwrap inverse matrix");
        self.transformation_matrix = m;
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }

    fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }

    fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_material_mut(&mut self) -> &mut Material {
        &mut self.material
    }
}

impl Plane {
    pub fn new() -> Plane {
        Plane {
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            material: Material::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

    use crate::basics::ray::RayOps;
    use crate::math::common::assert_matrix;
    use crate::math::common::{assert_float, assert_tuple};
    use crate::shape::shape::{Shape, ShapeEnum};
    use crate::shape::sphere::Sphere;

    use super::*;

    // page 123 top
    #[test]
    fn test_ray_plane_intersection_parallel() {
        let o = Tuple4D::new_point(0.0, 10.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let p = Plane::new();

        let (intersections, cnt_hits) = p.intersect(&r);

        assert_eq!(cnt_hits, 0);
    }

    // page 123 top
    #[test]
    fn test_ray_plane_intersection_above_and_below() {
        // above
        let o = Tuple4D::new_point(0.0, 1.0, 0.0);
        let d = Tuple4D::new_vector(0.0, -1.0, 0.0);
        let r = Ray::new(o, d);

        let p = Plane::new();
        let (intersections, cnt_hits) = p.intersect(&r);

        assert_float(intersections[0], 1.0);

        // below
        let o = Tuple4D::new_point(0.0, -1.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(o, d);
        let (intersections, cnt_hits) = p.intersect(&r);

        assert_float(intersections[0], 1.0);
    }

    #[test]
    fn test_ray_sphere_intersection_no_hits() {
        let o = Tuple4D::new_point(0.0, 2.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let (intersections, cnt_hits) = s.intersect(&r);

        assert_eq!(cnt_hits, 0);
    }

    #[test]
    fn test_ray_sphere_intersection_origin_inside_sphere() {
        let o = Tuple4D::new_point(0.0, 0.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let (intersections, cnt_hits) = s.intersect(&r);

        assert_eq!(cnt_hits, 2);

        assert_float(intersections[0], -1.0);
        assert_float(intersections[1], 1.0);
    }

    #[test]
    fn test_ray_sphere_intersection_sphere_behind_origin() {
        let o = Tuple4D::new_point(0.0, 0.0, 5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let (intersections, cnt_hits) = s.intersect(&r);

        assert_eq!(cnt_hits, 2);

        assert_float(intersections[0], -6.0);
        assert_float(intersections[1], -4.0);
    }

    #[test]
    fn test_sphere_transformation() {
        let mut s = Sphere::new();
        let m = Matrix::translation(2.0, 3.0, 4.0);

        s.set_transformation(m);

        let m = Matrix::translation(2.0, 3.0, 4.0);

        assert_matrix(&s.get_transformation(), &m);
    }

    #[test]
    fn test_sphere_scale() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let mut s = Sphere::new();
        let m = Matrix::scale(2.0, 2.0, 2.0);
        s.set_transformation(m);

        let sphere_shape = Shape::new(ShapeEnum::Sphere(s));
        let (intersections, cnt_hits) = sphere_shape.intersect(&r);

        assert_eq!(cnt_hits, 2);

        println!("intersections[0].get_t() {}    expected 3.0", intersections[0]);
        println!("intersections[1].get_t() {}    expected 7.0", intersections[1]);
        assert_float(intersections[0], 3.0);
        assert_float(intersections[1], 7.0);
    }

    #[test]
    fn test_sphere_translated() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let mut s = Sphere::new();
        let m = Matrix::translation(5.0, 0.0, 0.0);
        s.set_transformation(m);

        let sphere_shape = Shape::new(ShapeEnum::Sphere(s));
        let (intersections, cnt_hits) = sphere_shape.intersect(&r);
        assert_eq!(cnt_hits, 0);
    }

    // page 122
    #[test]
    fn test_sphere_normal_at() {
        let p = Plane::new();
        let n_expected = Tuple4D::new_vector(0.0, 1.0, 0.0);

        let point = Tuple4D::new_point(0.0, 0.0, 0.0);
        let n = p.normal_at(&point);
        assert_tuple(&n, &n_expected);

        let point = Tuple4D::new_point(10.0, 0.0, -10.0);
        let n = p.normal_at(&point);
        assert_tuple(&n, &n_expected);

        let point = Tuple4D::new_point(-5.0, 0.0, 150.0);
        let n = p.normal_at(&point);
        assert_tuple(&n, &n_expected);
    }

    #[test]
    fn test_sphere_normal_at_transformed() {
        let mut s = Sphere::new();
        s.set_transformation(Matrix::translation(0.0, 1.0, 0.0));

        let p = Tuple4D::new_point(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let n = s.normal_at(&p);
        let n_expected = Tuple4D::new_vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        println!(
            "test_sphere_normal_at_transformed    n = {:#?}, n_expected = {:#?}",
            n, n_expected
        );
        assert_tuple(&n, &n_expected);
    }

    #[test]
    fn test_sphere_normal_at_scaled_rotated() {
        let mut s = Sphere::new();
        s.set_transformation(Matrix::scale(1.0, 0.5, 1.0) * Matrix::rotate_z(PI / 5.0));

        let p = Tuple4D::new_point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let n = s.normal_at(&p);
        let n_expected = Tuple4D::new_vector(0.0, 0.97014, -0.24254);
        println!(
            "test_sphere_normal_at_scaled_rotated    n = {:#?}, n_expected = {:#?}",
            n, n_expected
        );
        assert_tuple(&n, &n_expected);
    }
}
