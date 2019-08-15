use std::cmp::{max, min};
use std::f64::INFINITY;

use crate::basics::ray::{Ray, RayOps};
use crate::material::material::Material;
use crate::material::material::MaterialOps;
use crate::math::common::EPSILON;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug, PartialEq)]
pub struct Cube {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    material: Material,
}

pub trait CubeOps {
    fn new() -> Cube;
    fn intersect(s: &Cube, r: &Ray) -> Option<Vec<f64>>;

    fn set_transformation(&mut self, m: Matrix);
    fn get_transformation(&self) -> &Matrix;
    fn get_inverse_transformation(&self) -> &Matrix;

    fn normal_at(&self, p: &Tuple4D) -> Tuple4D;

    fn set_material(&mut self, m: Material);
    fn get_material(&self) -> &Material;
    fn get_material_mut(&mut self) -> &mut Material;

    fn check_axis(origin: f64, direction: f64) -> (f64, f64);
}

impl CubeOps for Cube {
    fn new() -> Cube {
        Cube {
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            material: Material::new(),
        }
    }

    fn intersect(_c: &Cube, r: &Ray) -> Option<Vec<f64>> {
        let (xt_min, xt_max) = Self::check_axis(r.get_origin().x, r.get_direction().x);
        let (yt_min, yt_max) = Self::check_axis(r.get_origin().y, r.get_direction().y);
        let (zt_min, zt_max) = Self::check_axis(r.get_origin().z, r.get_direction().z);

        let tmin = xt_min.max(yt_min).max(zt_min);
        let tmax = xt_max.min(yt_max).min(zt_max);

        if tmin > tmax {
            return None;
        }

        let mut res = vec![0.0; 2];
        res[0] = tmin;
        res[0] = tmax;

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
        let maxc = world_point.x.abs().max(world_point.y.abs()).max(world_point.z.abs());
        if maxc == world_point.x {
            return Tuple4D::new_vector(world_point.x, 0.0, 0.0);
        } else if maxc == world_point.y {
            return Tuple4D::new_vector(0.0, world_point.y, 0.0);
        }
        Tuple4D::new_vector(0.0, 0.0, world_point.z)
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

    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let mut tmin;
        let mut tmax;

        if direction.abs() >= EPSILON {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        } else {
            tmin = tmin_numerator * INFINITY;
            tmax = tmax_numerator * INFINITY;
        }
        if tmin > tmax {
            let tmp = tmin;
            tmin = tmax;
            tmax = tmp;
        }
        (tmin, tmax)
    }
}

#[cfg(test)]
mod tests {
    use crate::basics::ray::RayOps;
    use crate::math::common::{assert_float, assert_tuple};

    use super::*;

    // page 168 helper
    fn test_ray_cube_intersection_helper(origin: Tuple4D, direction: Tuple4D, t1: f64, t2: f64) {
        let r = Ray::new(origin, direction);

        let c = Cube::new();

        let xs = Cube::intersect(&c, &r).unwrap();

        assert_eq!(xs.len(), 2);

        assert_float(xs[0], t1);
        assert_float(xs[1], t2);
    }

    // page 168
    #[test]
    fn test_ray_cube_intersection() {
        // +x
        let o = Tuple4D::new_point(5.0, 0.5, 0.0);
        let d = Tuple4D::new_vector(-1.0, 0.0, 0.0);
        test_ray_cube_intersection_helper(o, d, 4.0, 6.0);

        // -x
        let o = Tuple4D::new_point(-5.0, 0.5, 0.0);
        let d = Tuple4D::new_vector(1.0, 0.0, 0.0);
        test_ray_cube_intersection_helper(o, d, 4.0, 6.0);

        // +y
        let o = Tuple4D::new_point(0.5, 5.0, 0.0);
        let d = Tuple4D::new_vector(0.0, -1.0, 0.0);
        test_ray_cube_intersection_helper(o, d, 4.0, 6.0);

        // -y
        let o = Tuple4D::new_point(0.5, -5.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 1.0, 0.0);
        test_ray_cube_intersection_helper(o, d, 4.0, 6.0);

        // +z
        let o = Tuple4D::new_point(0.5, 0.0, 5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, -1.0);
        test_ray_cube_intersection_helper(o, d, 4.0, 6.0);

        // -z
        let o = Tuple4D::new_point(0.5, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cube_intersection_helper(o, d, 4.0, 6.0);

        // inside
        let o = Tuple4D::new_point(0.0, 0.5, 0.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cube_intersection_helper(o, d, -1.0, 1.0);
    }

    // page 172 helper
    fn test_ray_cube_miss_helper(origin: Tuple4D, direction: Tuple4D) {
        let r = Ray::new(origin, direction);

        let c = Cube::new();

        let xs = Cube::intersect(&c, &r);

        assert_eq!(xs.is_none(), true);
    }

    // page 172
    #[test]
    fn test_ray_cube_miss() {
        // 1
        let o = Tuple4D::new_point(-2.0, 0.0, 0.8018);
        let d = Tuple4D::new_vector(0.2673, 0.5345, 0.8018);
        test_ray_cube_miss_helper(o, d);

        // 2
        let o = Tuple4D::new_point(0.0, -2.0, 0.0);
        let d = Tuple4D::new_vector(0.8018, 0.2873, 0.5345);
        test_ray_cube_miss_helper(o, d);

        // 3
        let o = Tuple4D::new_point(0.0, 0.0, -2.0);
        let d = Tuple4D::new_vector(0.5345, 0.8018, 0.2673);
        test_ray_cube_miss_helper(o, d);

        // 4
        let o = Tuple4D::new_point(2.0, 0.0, 2.0);
        let d = Tuple4D::new_vector(0.0, 0.0, -1.0);
        test_ray_cube_miss_helper(o, d);

        // 5
        let o = Tuple4D::new_point(0.0, 2.0, 2.0);
        let d = Tuple4D::new_vector(0.0, -1.0, 0.0);
        test_ray_cube_miss_helper(o, d);
        // -z
        let o = Tuple4D::new_point(2.0, 2.0, 0.0);
        let d = Tuple4D::new_vector(-1.0, 0.0, 0.0);
        test_ray_cube_miss_helper(o, d);
    }

    // page 173 helper
    fn test_cube_normal_helper(point: Tuple4D, n_expected: Tuple4D) {
        let c = Cube::new();
        let n = c.normal_at(&point);

        assert_tuple(&n, &n_expected);
    }

    // page 173/174
    #[test]
    fn test_cube_normal() {
        // 1
        let o = Tuple4D::new_point(1.0, 0.5, -0.8);
        let n = Tuple4D::new_vector(1.0, 0.0, 0.0);
        test_cube_normal_helper(o, n);

        // 2
        let o = Tuple4D::new_point(-1.0, -0.2, 0.9);
        let n = Tuple4D::new_vector(-1.0, 0.0, 0.0);
        test_cube_normal_helper(o, n);

        // 3
        let o = Tuple4D::new_point(-0.4, 1., -0.1);
        let d = Tuple4D::new_vector(0.0, 1.0, 0.0);
        test_cube_normal_helper(o, d);

        // 4
        let o = Tuple4D::new_point(0.3, -1., -0.7);
        let d = Tuple4D::new_vector(0.0, -1.0, 0.0);
        test_cube_normal_helper(o, d);

        // 5
        let o = Tuple4D::new_point(-0.6, 0.3, 1.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_cube_normal_helper(o, d);

        // 6
        let o = Tuple4D::new_point(0.4, 0.4, -1.0);
        let d = Tuple4D::new_vector(0.0, 0.0, -1.0);
        test_cube_normal_helper(o, d);

        // 7
        let o = Tuple4D::new_point(1., 1., 1.);
        let d = Tuple4D::new_vector(1.0, 0.0, 0.0);
        test_cube_normal_helper(o, d);

        // 8
        let o = Tuple4D::new_point(-1., -1., -1.);
        let d = Tuple4D::new_vector(-1.0, 0.0, 0.0);
        test_cube_normal_helper(o, d);
    }
}
