use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    material: Material,
}

impl ShapeOps for Plane {
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

    fn normal_at(&self, _world_point: &Tuple4D) -> Tuple4D {
        // // TODO: its for the tests -remove and fix tests and add unreachable
        // let object_point = self.get_inverse_transformation() * world_point;
        // let local_normal = self.local_normal_at(&object_point);
        // let mut world_normal = &Matrix::transpose(self.get_inverse_transformation()) * &local_normal;
        // world_normal.w = 0.0;
        // Tuple4D::normalize(&world_normal)
        unimplemented!()
    }

    fn local_normal_at(&self, _local_point: &Tuple4D) -> Tuple4D {
        Tuple4D::new_vector(0.0, 1.0, 0.0)
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

    pub fn intersect(r: &Ray) -> Option<Vec<f32>> {
        if r.direction.y.abs() < EPSILON {
            return None;
        }
        let t = -r.origin.y / r.direction.y;
        let mut res = vec![0.0; 1];

        res[0] = t;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

    use crate::basics::intersection::{Intersection, IntersectionListOps, IntersectionOps};
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

        let intersections = Plane::intersect(&r);

        assert_eq!(intersections.is_none(), true);
    }

    // page 123 top
    #[test]
    fn test_ray_plane_intersection_above_and_below() {
        // above
        let o = Tuple4D::new_point(0.0, 1.0, 0.0);
        let d = Tuple4D::new_vector(0.0, -1.0, 0.0);
        let r = Ray::new(o, d);

        let intersections = Plane::intersect(&r);

        assert_eq!(intersections.is_some(), true);
        assert_float(intersections.unwrap()[0], 1.0);

        // below
        let o = Tuple4D::new_point(0.0, -1.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(o, d);
        let intersections = Plane::intersect(&&r);

        assert_eq!(intersections.is_some(), true);
        assert_float(intersections.unwrap()[0], 1.0);
    }

    #[test]
    fn test_ray_sphere_intersection_no_hits() {
        let o = Tuple4D::new_point(0.0, 2.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let intersections = Sphere::intersect(&r);

        assert_eq!(intersections, None);
    }

    #[test]
    fn test_ray_sphere_intersection_origin_inside_sphere() {
        let o = Tuple4D::new_point(0.0, 0.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let intersections = Sphere::intersect(&r).unwrap();

        assert_eq!(intersections.len(), 2);

        assert_float(intersections[0], -1.0);
        assert_float(intersections[1], 1.0);
    }

    #[test]
    fn test_ray_sphere_intersection_sphere_behind_origin() {
        let o = Tuple4D::new_point(0.0, 0.0, 5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let intersections = Sphere::intersect(&r).unwrap();

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

        let sphere_shape = Shape::new(ShapeEnum::Sphere(s));
        let is = Intersection::intersect(&sphere_shape, &r);

        let intersections = is.get_intersections();
        assert_eq!(intersections.len(), 0);
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
