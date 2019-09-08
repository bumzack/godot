use core::f32::{INFINITY, NAN};

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Cube {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    material: Material,
}

impl ShapeOps for Cube {
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
        // TODO: its for the tests -remove and fix tests and add unreachable
        let object_point = self.get_inverse_transformation() * world_point;
        let local_normal = self.local_normal_at(&object_point);
        let mut world_normal = &Matrix::transpose(self.get_inverse_transformation()) * &local_normal;
        world_normal.w = 0.0;
        Tuple4D::normalize(&world_normal)
    }

    fn local_normal_at(&self, local_point: &Tuple4D) -> Tuple4D {
        let maxc = max_float(local_point.x.abs(), local_point.y.abs(), local_point.z.abs());
        if (maxc - local_point.x.abs()) < EPSILON {
            return Tuple4D::new_vector(local_point.x, 0.0, 0.0);
        } else if (maxc - local_point.y.abs()) < EPSILON {
            return Tuple4D::new_vector(0.0, local_point.y, 0.0);
        }
        Tuple4D::new_vector(0.0, 0.0, local_point.z)
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

impl Cube {
    pub fn new() -> Cube {
        Cube {
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            material: Material::new(),
        }
    }

    pub fn intersect(r: &Ray) -> Option<Vec<f32>> {
        let (xt_min, xt_max) = Self::check_axis(r.get_origin().x, r.get_direction().x);
        let (yt_min, yt_max) = Self::check_axis(r.get_origin().y, r.get_direction().y);
        let (zt_min, zt_max) = Self::check_axis(r.get_origin().z, r.get_direction().z);

        let tmin = max_float(xt_min, yt_min, zt_min);
        let tmax = min_float(xt_max, yt_max, zt_max);

        if tmin > tmax {
            return None;
        }
        let mut res = vec![0.0; 2];

        if tmin == NAN {
            println!("CUBE: here we have a NAN tmin is {}", tmin);
        }

        if tmax == NAN {
            println!("CUBE:  here we have a NAN tmax is {}", tmax);
        }

        res[0] = tmin;
        res[1] = tmax;

        Some(res)
    }

    fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
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
    use crate::prelude::ShapeOps;

    // page 168 helper
    fn test_ray_cube_intersection_helper(origin: Tuple4D, direction: Tuple4D, t1: f32, t2: f32) {
        let r = Ray::new(origin, direction);

        let c = Cube::new();

        let xs = Cube::intersect(&r).unwrap();

        assert_eq!(xs.len(), 2);

        println!("expected t1   = {} ", t1);
        println!("actual  t1    = {} ", xs[0]);
        println!("expected t2   = {} ", t2);
        println!("actual  t2    = {} ", xs[1]);

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

        let xs = Cube::intersect(&r);

        assert_eq!(xs.is_none(), true);
    }

    // page 172
    #[test]
    fn test_ray_cube_miss() {
        // 1
        let o = Tuple4D::new_point(-2.0, 0.0, 0.0);
        let d = Tuple4D::new_vector(0.2673, 0.5345, 0.8018);
        test_ray_cube_miss_helper(o, d);

        // 2
        let o = Tuple4D::new_point(0.0, -2.0, 0.0);
        let d = Tuple4D::new_vector(0.8018, 0.2673, 0.5345);
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

        println!("point        = {:?} ", point);
        println!("expected n   = {:?} ", n_expected);
        println!("actual   n   = {:?} ", n);

        assert_tuple(&n, &n_expected);
    }

    // page 173/174
    #[test]
    fn test_cube_normal() {
        // 1
        let point = Tuple4D::new_point(1.0, 0.5, -0.8);
        let n = Tuple4D::new_vector(1.0, 0.0, 0.0);
        test_cube_normal_helper(point, n);

        // 2
        let point = Tuple4D::new_point(-1.0, -0.2, 0.9);
        let n_expected = Tuple4D::new_vector(-1.0, 0.0, 0.0);
        test_cube_normal_helper(point, n_expected);

        // 3
        let point = Tuple4D::new_point(-0.4, 1., -0.1);
        let n_expected = Tuple4D::new_vector(0.0, 1.0, 0.0);
        test_cube_normal_helper(point, n_expected);

        // 4
        let point = Tuple4D::new_point(0.3, -1., -0.7);
        let n_expected = Tuple4D::new_vector(0.0, -1.0, 0.0);
        test_cube_normal_helper(point, n_expected);

        // 5
        let point = Tuple4D::new_point(-0.6, 0.3, 1.0);
        let n_expected = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_cube_normal_helper(point, n_expected);

        // 6
        let point = Tuple4D::new_point(0.4, 0.4, -1.0);
        let n_expected = Tuple4D::new_vector(0.0, 0.0, -1.0);
        test_cube_normal_helper(point, n_expected);

        // 7
        let point = Tuple4D::new_point(1., 1., 1.);
        let n_expected = Tuple4D::new_vector(1.0, 0.0, 0.0);
        test_cube_normal_helper(point, n_expected);

        // 8
        let point = Tuple4D::new_point(-1., -1., -1.);
        let n_expected = Tuple4D::new_vector(-1.0, 0.0, 0.0);
        test_cube_normal_helper(point, n_expected);
    }
}
