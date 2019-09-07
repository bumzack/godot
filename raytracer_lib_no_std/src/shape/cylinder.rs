use core::f32::INFINITY;

use crate::{
    intri_abs, intri_powi, intri_sqrt, Material, MaterialOps, Matrix, MatrixOps, Ray, RayOps, ShapeIntersectionResult,
    ShapeOps, Tuple, Tuple4D, EPSILON,
};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Cylinder {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    material: Material,
    minimum: f32,
    maximum: f32,
    closed: bool,
}

impl ShapeOps for Cylinder {
    fn intersect(&self, r: &Ray) -> ShapeIntersectionResult {
        let mut res = [0f32; 4];
        let mut res_cnt = 0;

        let a = intri_powi(r.get_direction().x, 2) + intri_powi(r.get_direction().z, 2);
        if !(intri_abs(a) < EPSILON) {
            let b = 2.0 * r.get_origin().x * r.get_direction().x + 2.0 * r.get_origin().z * r.get_direction().z;
            let c = intri_powi(r.get_origin().x, 2) + intri_powi(r.get_origin().z, 2) - 1.0;

            let disc = b * b - 4.0 * a * c;
            if disc < 0.0 {
                return (res, res_cnt);
            }
            let mut t0 = (-b - intri_sqrt(disc)) / (2.0 * a);
            let mut t1 = (-b + intri_sqrt(disc)) / (2.0 * a);

            if t0 > t1 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }
            let y0 = r.get_origin().y + t0 * r.get_direction().y;
            if self.get_minimum() < y0 && y0 < self.get_maximum() {
                res[res_cnt] = t0;
                res_cnt += 1;
            }

            let y1 = r.get_origin().y + t1 * r.get_direction().y;
            if self.get_minimum() < y1 && y1 < self.get_maximum() {
                res[res_cnt] = t0;
                res_cnt += 1;
            }
        }
        let (res_caps, res_cnt_caps) = self.intersect_caps(r);
        for i in 0..res_cnt_caps {
            res[res_cnt] = res_caps[i];
            res_cnt += 1;
        }
        (res, res_cnt)
    }

    fn normal_at(&self, world_point: &Tuple4D) -> Tuple4D {
        let dist = intri_powi(world_point.x, 2) + intri_powi(world_point.z, 2);
        if dist < 1.0 && world_point.y >= self.get_maximum() - EPSILON {
            return Tuple4D::new_vector(0.0, 1.0, 0.0);
        } else if dist < 1.0 && world_point.y <= self.get_maximum() + EPSILON {
            return Tuple4D::new_vector(0.0, -1.0, 0.0);
        }
        Tuple4D::new_vector(world_point.x, 0.0, world_point.z)
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

impl Cylinder {
    pub fn new() -> Cylinder {
        Cylinder {
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            material: Material::new(),
            minimum: -INFINITY,
            maximum: INFINITY,
            closed: false,
        }
    }

    pub fn get_minimum(&self) -> f32 {
        self.minimum
    }

    pub fn get_maximum(&self) -> f32 {
        self.maximum
    }

    pub fn set_minimum(&mut self, min: f32) {
        self.minimum = min;
    }

    pub fn set_maximum(&mut self, max: f32) {
        self.maximum = max;
    }

    pub fn get_closed(&self) -> bool {
        self.closed
    }

    pub fn set_closed(&mut self, closed: bool) {
        self.closed = closed;
    }

    fn check_cap(&self, r: &Ray, t: f32) -> bool {
        let x = r.get_origin().x + t * r.get_direction().x;
        let z = r.get_origin().z + t * r.get_direction().z;
        (intri_powi(x, 2) + intri_powi(z, 2)) - 1.0 < EPSILON
    }

    fn intersect_caps(&self, r: &Ray) -> ShapeIntersectionResult {
        let mut res = [0f32; 4];
        let mut res_cnt = 0;

        if !self.get_closed() || intri_abs(r.get_direction().y) < EPSILON {
            return (res, res_cnt);
        }
        let t = (self.get_minimum() - r.get_origin().y) / r.get_direction().y;
        if self.check_cap(r, t) {
            res[res_cnt] = t;
            res_cnt += 1;
        }
        let t = (self.get_maximum() - r.get_origin().y) / r.get_direction().y;
        if self.check_cap(r, t) {
            res[res_cnt] = t;
            res_cnt += 1;
        };
        (res, res_cnt)
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

        let xs = cyl.intersect(&r);

        assert_eq!(xs.unwrap().len(), 0);
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
    fn test_ray_cylinder_intersection_intersection_helper(origin: Tuple4D, mut direction: Tuple4D, t1: f32, t2: f32) {
        let cyl = Cylinder::new();

        direction = Tuple4D::normalize(&direction);
        let r = Ray::new(origin.clone(), direction.clone());
        let xs = cyl.intersect(&r);

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
        test_ray_cylinder_intersection_intersection_helper(origin, direction, 6.808006, 7.0886984);
    }

    // page 181
    fn test_ray_cylinder_normal_at_helper(point: Tuple4D, n_expected: Tuple4D) {
        let cyl = Cylinder::new();

        let n = Cylinder::normal_at(&cyl, &point);

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

    // page 182
    #[test]
    fn test_ray_cylinder_new() {
        let c = Cylinder::new();

        println!("c.getminimum() = {},    -INFINITY = {}", c.get_minimum(), -INFINITY);
        println!("c.get_maximum() = {},    INFINITY = {}", c.get_maximum(), INFINITY);
        assert_eq!(c.get_minimum(), -INFINITY);
        assert_eq!(c.get_maximum(), INFINITY);
    }

    // page 182
    fn test_ray_cylinder_truncate_helper(point: Tuple4D, mut direction: Tuple4D, count: usize) {
        let mut cyl = Cylinder::new();
        cyl.set_minimum(1.0);
        cyl.set_maximum(2.0);
        direction = Tuple4D::normalize(&direction);

        let r = Ray::new(point.clone(), direction.clone());

        let xs = Cylinder::intersect(&cyl, &r);
        let xs_clone = xs.clone();

        //        println!("point        = {:?} ", point);
        //        println!("direction     = {:?} ", direction);
        //        println!("expected  count  = {:?} ", count);
        //        if xs_clone.is_some() {
        //            println!("xs is some        = {:?} ", &xs_clone.unwrap());
        //        } else {
        //            println!("xs is none  ");
        //        }

        if count == 0 {
            if xs.is_some() {
                assert_eq!(&xs.unwrap().len(), &count);
            } else {
                // hmmmm redundant test ?
                assert_eq!(&xs.is_none(), &true);
            }
        } else {
            assert_eq!(&xs.unwrap().len(), &count);
        }
    }

    // page 182
    #[test]
    fn test_ray_cylinder_truncate() {
        // 1
        let point = Tuple4D::new_point(0.0, 1.5, 0.0);
        let direction = Tuple4D::new_vector(0.1, 1.0, 0.0);
        test_ray_cylinder_truncate_helper(point, direction, 0);

        // 2
        let point = Tuple4D::new_point(0.0, 3., -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_truncate_helper(point, direction, 0);

        // 3
        let point = Tuple4D::new_point(0.0, 0., -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_truncate_helper(point, direction, 0);

        // 4
        let point = Tuple4D::new_point(0.0, 2., -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_truncate_helper(point, direction, 0);

        // 5
        let point = Tuple4D::new_point(0.0, 1., -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_truncate_helper(point, direction, 0);

        // 6
        let point = Tuple4D::new_point(0.0, 1.5, -2.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_truncate_helper(point, direction, 2);
    }

    // page 185
    #[test]
    fn test_ray_cylinder_closed_cylinder() {
        let c = Cylinder::new();

        assert!(c.get_closed() == false);
    }

    // page 185
    fn test_ray_cylinder_capped_helper(point: Tuple4D, mut direction: Tuple4D, count: usize) {
        let mut cyl = Cylinder::new();
        cyl.set_minimum(1.0);
        cyl.set_maximum(2.0);
        cyl.set_closed(true);
        direction = Tuple4D::normalize(&direction);

        let r = Ray::new(point.clone(), direction.clone());

        let xs = Cylinder::intersect(&cyl, &r);
        let xs_clone = xs.clone();

        println!("point        = {:?} ", point);
        println!("direction     = {:?} ", direction);
        println!("expected  count  = {:?} ", count);
        if xs_clone.is_some() {
            println!("xs is some        = {:?} ", &xs_clone.unwrap());
        } else {
            println!("xs is none  ");
        }

        if count == 0 {
            if xs.is_some() {
                assert_eq!(&xs.unwrap().len(), &count);
            } else {
                // hmmmm redundant test ?
                assert_eq!(&xs.is_none(), &true);
            }
        } else {
            assert_eq!(&xs.unwrap().len(), &count);
        }
    }

    // page 185
    #[test]
    fn test_ray_cylinder_capped() {
        // 1
        //        let point = Tuple4D::new_point(0.0, 3.0, 0.0);
        //        let direction = Tuple4D::new_vector(0.0, -1.0, 0.0);
        //        test_ray_cylinder_capped_helper(point, direction, 2);
        //
        //        // 2
        //        let point = Tuple4D::new_point(0.0, 3.0, -2.0);
        //        let direction = Tuple4D::new_vector(0.0, -1.0, 2.0);
        //        test_ray_cylinder_capped_helper(point, direction, 2);

        // 3
        let point = Tuple4D::new_point(0.0, 4.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, -1.0, 1.0);
        test_ray_cylinder_capped_helper(point, direction, 2);

        // 4
        let point = Tuple4D::new_point(0.0, 0.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, 1.0, 2.0);
        test_ray_cylinder_capped_helper(point, direction, 2);

        // 5
        let point = Tuple4D::new_point(0.0, -1.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, 1.0, 1.0);
        test_ray_cylinder_capped_helper(point, direction, 2);
    }

    // page 186
    fn test_ray_cylinder_capped_normal_at_helper(point: Tuple4D, normal: Tuple4D) {
        let mut cyl = Cylinder::new();
        cyl.set_minimum(1.0);
        cyl.set_maximum(2.0);
        cyl.set_closed(true);
        let n = Cylinder::normal_at(&cyl, &point);

        println!("point        = {:?} ", point);
        println!("direction     = {:?} ", normal);

        assert_tuple(&n, &normal);
    }

    // page 185
    #[test]
    fn test_ray_cylinder_capped_normal_at() {
        // 1
        let point = Tuple4D::new_point(0.0, 1.0, 0.0);
        let normal = Tuple4D::new_vector(0.0, -1.0, 0.0);
        test_ray_cylinder_capped_normal_at_helper(point, normal);

        // 2
        let point = Tuple4D::new_point(0.5, 1.0, 0.0);
        let normal = Tuple4D::new_vector(0.0, -1.0, 0.0);
        test_ray_cylinder_capped_normal_at_helper(point, normal);

        // 3
        let point = Tuple4D::new_point(0.0, 1.0, 0.5);
        let normal = Tuple4D::new_vector(0.0, -1.0, 0.0);
        test_ray_cylinder_capped_normal_at_helper(point, normal);

        // 4
        let point = Tuple4D::new_point(0.0, 2.0, 0.0);
        let normal = Tuple4D::new_vector(0.0, 1.0, 0.0);
        test_ray_cylinder_capped_normal_at_helper(point, normal);

        // 5
        let point = Tuple4D::new_point(0.5, 2.0, 0.0);
        let normal = Tuple4D::new_vector(0.0, 1.0, 0.0);
        test_ray_cylinder_capped_normal_at_helper(point, normal);

        // 6
        let point = Tuple4D::new_point(0.0, 2.0, 0.5);
        let normal = Tuple4D::new_vector(0.0, 1.0, 0.0);
        test_ray_cylinder_capped_normal_at_helper(point, normal);
    }
}
