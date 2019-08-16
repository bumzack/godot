use crate::basics::ray::{Ray, RayOps};
use crate::material::material::Material;
use crate::material::material::MaterialOps;
use crate::math::common::EPSILON;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug, PartialEq)]
pub struct Cylinder {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    material: Material,
}

pub trait CylinderOps {
    fn new() -> Cylinder;
    fn intersect(r: &Ray) -> Option<Vec<f64>>;

    fn set_transformation(&mut self, m: Matrix);
    fn get_transformation(&self) -> &Matrix;
    fn get_inverse_transformation(&self) -> &Matrix;

    fn normal_at(&self, p: &Tuple4D) -> Tuple4D;

    fn set_material(&mut self, m: Material);
    fn get_material(&self) -> &Material;
    fn get_material_mut(&mut self) -> &mut Material;
}

impl CylinderOps for Cylinder {
    fn new() -> Cylinder {
        Cylinder {
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            material: Material::new(),
        }
    }

    fn intersect(r: &Ray) -> Option<Vec<f64>> {
        let a = r.get_direction().x.powi(2) + r.get_direction().z.powi(2);

        if a < EPSILON { return None; }
        let b = 2.0 * r.get_origin().x * r.get_direction().x + 2.0 * r.get_origin().z * r.get_direction().z;
        let c = r.get_origin().x.powi(2) + r.get_origin().z.powi(2) - 1.0;

        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return None;
        }
        let mut res = vec![0.0; 2];
        res[0] = (-b - disc.sqrt()) / (2.0 * a);
        res[1] = (-b + disc.sqrt()) / (2.0 * a);

        Some(res)
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix =
            Matrix::invert(&m).expect("Cube::set_transofrmation: cant unwrap inverse matrix");
        self.transformation_matrix = m;
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }

    fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }

    fn normal_at(&self, world_point: &Tuple4D) -> Tuple4D {
        Tuple4D::new_vector( world_point.x, 0.0, world_point.z)
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
    use crate::basics::ray::RayOps;
    use crate::math::common::{assert_float, assert_tuple};

    use super::*;

    // page 178
    fn test_ray_cylinder_intersection_miss_helper(origin: Tuple4D, mut direction: Tuple4D) {
        let cyl = Cylinder::new();

        direction = Tuple4D::normalize(&direction);
        let r = Ray::new(origin, direction);

        let xs = Cylinder::intersect(&r);

        assert_eq!(xs, None);
    }

    // page 178
    #[test]
    fn test_ray_cylinder_intersection_miss() {
        // 1
        let origin = Tuple4D::new_point(1.0, 0.0, 0.0);
        let direction = Tuple4D::new_vector(0.0, 1.0, 0.0);
        test_ray_cylinder_intersection_miss_helper(origin, direction);

        // 2
        let origin = Tuple4D::new_point(0.0, 0.0, 0.0);
        let direction = Tuple4D::new_vector(0.0, 1.0, 0.0);
        test_ray_cylinder_intersection_miss_helper(origin, direction);

        // 3
        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(1.0, 1.0, 1.0);
        test_ray_cylinder_intersection_miss_helper(origin, direction);
    }

    // page 180
    fn test_ray_cylinder_intersection_intersection_helper(origin: Tuple4D, mut direction: Tuple4D, t1: f64, t2: f64) {
        let cyl = Cylinder::new();

        direction = Tuple4D::normalize(&direction);
        let r = Ray::new(origin.clone(), direction.clone());
        let xs = Cylinder::intersect(&r);

        assert_eq!(xs.is_some(), true);
        let xs = xs.unwrap();
        assert_eq!(xs.len(), 2);

        println!("origin        = {:?} ", origin);
        println!("direction n   = {:?} ", direction);
        println!("expected  t1   = {:?}       actual t1 = {}", t1, xs[0]);
        println!("expected  t2   = {:?}       actual t1 = {}", t2, xs[1]);

        assert_float(t1, xs[0]);
        assert_float(t2, xs[1]);
    }

    // page 180
    #[test]
    fn test_ray_cylinder_intersection_intersection() {
        // 1
        let origin = Tuple4D::new_point(1.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_intersection_intersection_helper(origin, direction, 5.0, 5.0);

        // 2
        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_intersection_intersection_helper(origin, direction, 4.0, 6.0);

        // 3
        let origin = Tuple4D::new_point(0.5, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.1, 1.0, 1.0);
        test_ray_cylinder_intersection_intersection_helper(origin, direction, 6.80798, 7.08872);
    }

    // page 181
    fn test_ray_cylinder_normal_at_helper(point: Tuple4D, n_expected: Tuple4D) {
        let cyl = Cylinder::new();

        let n = cyl.normal_at(&point);

        println!("point        = {:?} ", point);
        println!("expected  n  = {:?} ", n_expected);
        println!("actual    n  = {:?} ", n);

        assert_tuple(&n, &n_expected);
    }

    // page 181
    #[test]
    fn test_ray_cylinder_normal_at() {
        // 1
        let point = Tuple4D::new_point(1.0, 0.0, 0.0);
        let normal = Tuple4D::new_vector(1.0, 0.0, 0.0);
        test_ray_cylinder_normal_at_helper(point, normal);

        // 2
        let point = Tuple4D::new_point(0.0, 5.0, -1.0);
        let normal = Tuple4D::new_vector(0.0, 0.0, -1.0);
        test_ray_cylinder_normal_at_helper(point, normal);

        // 3
        let point = Tuple4D::new_point(0.0, -2.0, 1.0);
        let normal = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_normal_at_helper(point, normal);

        // 4
        let point = Tuple4D::new_point(-1.0, 1.0, 0.0);
        let normal = Tuple4D::new_vector(-1.0, 0.0, 0.0);
        test_ray_cylinder_normal_at_helper(point, normal);
    }
}
