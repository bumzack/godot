use std::f32::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

use crate::basics::intersection::IntersectionOps;
use crate::basics::intersection::{Intersection, IntersectionListOps};
use crate::basics::ray::Ray;
use crate::basics::ray::RayOps;
use crate::material::material::Material;
use crate::material::material::MaterialOps;
use crate::math::common::{assert_float, assert_matrix, assert_tuple};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple4D;
use crate::math::tuple4d::{Tuple, ORIGIN};
use crate::shape::shape::Shape;

#[derive(Clone, Debug)]
pub struct Sphere {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    material: Material,
}

pub trait SphereOps {
    fn new() -> Sphere;
    fn intersect(s: &Sphere, r: &Ray) -> Option<Vec<f32>>;

    fn set_transformation(&mut self, m: Matrix);
    fn get_transformation(&self) -> &Matrix;
    fn get_inverse_transformation(&self) -> &Matrix;

    fn normal_at(&self, p: &Tuple4D) -> Tuple4D;

    fn set_material(&mut self, m: Material);
    fn get_material(&self) -> &Material;
    fn get_material_mut(&mut self) -> &mut Material;
}

impl SphereOps for Sphere {
    fn new() -> Sphere {
        Sphere {
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            material: Material::new(),
        }
    }

    fn intersect(s: &Sphere, r: &Ray) -> Option<Vec<f32>> {
        let sphere_to_ray = &r.origin - &ORIGIN;
        let a = &r.direction ^ &r.direction;
        let b = 2.0 * (&r.direction ^ &sphere_to_ray);
        let c = (&sphere_to_ray ^ &sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }
        let mut res = vec![0.0; 2];
        res[0] = (-b + discriminant.sqrt()) / (2.0 * a);
        res[1] = (-b - discriminant.sqrt()) / (2.0 * a);

        res.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // println!("res in intersect: {:?}", res);
        Some(res)
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix = Matrix::invert(&m).unwrap();
        self.transformation_matrix = m;
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }
    fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }

    fn normal_at(&self, world_point: &Tuple4D) -> Tuple4D {
        let p_shape = &self.inverse_transformation_matrix * world_point;
        let object_normal = &(p_shape - ORIGIN);
        let mut world_normal = &Matrix::transpose(&self.inverse_transformation_matrix) * object_normal;
        world_normal.w = 0.0;
        Tuple4D::normalize(&world_normal)
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

#[cfg(test)]
mod tests {
    use crate::math::common::{assert_color, assert_float, assert_matrix, assert_tuple, assert_two_float};

    use super::*;

    #[test]
    fn test_ray_sphere_intersection() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let intersections = Sphere::intersect(&s, &r).unwrap();

        assert_eq!(intersections.len(), 2);

        assert_float(intersections[0], 4.0);
        assert_float(intersections[1], 6.0);

        let o = Tuple4D::new_point(0.0, 1.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let intersections = Sphere::intersect(&s, &r).unwrap();

        assert_eq!(intersections.len(), 2);

        assert_float(intersections[0], 5.0);
        assert_float(intersections[1], 5.0);
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

        assert_float(intersections[0], -1.0);
        assert_float(intersections[1], 1.0);
    }

    #[test]
    fn test_ray_sphere_intersection_sphere_behind_origin() {
        let o = Tuple4D::new_point(0.0, 0.0, 5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let intersections = Sphere::intersect(&s, &r).unwrap();

        assert_eq!(intersections.len(), 2);

        assert_float(intersections[0], -6.0);
        assert_float(intersections[1], -4.0);
    }

    #[test]
    fn test_sphere_transformation() {
        let mut s = Sphere::new();
        let m = Matrix::translation(2.0, 3.0, 4.0);

        s.set_transformation(m);

        let m = Matrix::translation(2.0, 3.0, 4.0);

        assert_matrix(&s.transformation_matrix, &m);
    }

    #[test]
    fn test_sphere_scale() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let mut s = Sphere::new();
        let m = Matrix::scale(2.0, 2.0, 2.0);
        s.set_transformation(m);

        let sphere_shape = Shape::Sphere(s);
        let is = Intersection::intersect(&sphere_shape, &r);

        let intersections = is.get_intersections();

        assert_eq!(intersections.len(), 2);

        // println!("intersections[0].get_t() {}", intersections[0].get_t());
        // println!("intersections[1].get_t() {}", intersections[1].get_t());
        assert_float(intersections[0].get_t(), 3.0);
        assert_float(intersections[1].get_t(), 7.0);
    }

    #[test]
    fn test_sphere_translated() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let mut s = Sphere::new();
        let m = Matrix::translation(5.0, 0.0, 0.0);
        s.set_transformation(m);

        let sphere_shape = Shape::Sphere(s);
        let is = Intersection::intersect(&sphere_shape, &r);

        let intersections = is.get_intersections();
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_sphere_normal_at() {
        let s = Sphere::new();

        let p = Tuple4D::new_point(1.0, 0.0, 0.0);
        let n = s.normal_at(&p);
        let n_expected = Tuple4D::new_vector(1.0, 0.0, 0.0);
        assert_tuple(&n, &n_expected);

        let p = Tuple4D::new_point(0.0, 1.0, 0.0);
        let n = s.normal_at(&p);
        let n_expected = Tuple4D::new_vector(0.0, 1.0, 0.0);
        assert_tuple(&n, &n_expected);

        let p = Tuple4D::new_point(0.0, 0.0, 1.0);
        let n = s.normal_at(&p);
        let n_expected = Tuple4D::new_vector(0.0, 0.0, 1.0);
        assert_tuple(&n, &n_expected);

        let a = 3_f32.sqrt() / 3.0;
        let p = Tuple4D::new_point(a, a, a);
        let n = s.normal_at(&p);
        let n_expected = Tuple4D::new_vector(a, a, a);
        assert_tuple(&n, &n_expected);

        let a = 3_f32.sqrt() / 3.0;
        let p = Tuple4D::new_point(a, a, a);
        let n = Tuple4D::normalize(&s.normal_at(&p));
        let n_expected = Tuple4D::new_vector(a, a, a);
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
