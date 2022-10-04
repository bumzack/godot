// rust impl of https://www.pbr-book.org/3ed-2018/Shapes/Curves

use std::ops::{Add, Mul};

use raytracer_challenge_reference_impl::basics::{IntersectionList, IntersectionListOps, Ray};
use raytracer_challenge_reference_impl::math::{Matrix, MatrixOps, Tuple, Tuple4D};
use raytracer_challenge_reference_impl::prelude::BoundingBox;

pub enum CurveType {
    FLAT,
    CYLINDER,
    RIBBON,
}

pub struct Curve {
    transform: Matrix,
    inv_transform: Matrix,
    reverse_orientation: bool,
    common: CurveCommon,
    u_min: f64,
    u_max: f64,
}

impl Curve {
    pub fn new(
        transform: Matrix,
        inv_transform: Matrix,
        reverse_orientation: bool,
        common: CurveCommon,
        u_min: f64,
        u_max: f64,
    ) -> Self {
        Curve {
            transform,
            inv_transform,
            reverse_orientation,
            common,
            u_min,
            u_max,
        }
    }

    pub fn bounds(&self) -> BoundingBox {
        let (p0, p1, p2, p3) = self.calc_blossom();

        let mut bounding_box1 = BoundingBox::new_from_min_max(p0, p1);
        let bounding_box2 = BoundingBox::new_from_min_max(p2, p3);

        bounding_box1.add(&bounding_box2);

        let width0 = lerp_float(self.u_min, self.common.width[0], self.common.width[1]);
        let width1 = lerp_float(self.u_max, self.common.width[0], self.common.width[1]);

        expand(&mut bounding_box1, width0.max(width1) * 0.5);

        bounding_box1
    }

    pub fn intersect(&self, r: &Ray) -> IntersectionList {
        // let r = &self.transform * r;
        //
        // let (cp_obj0, cp_obj1, cp_obj2.0, cp_obj3) = self.calc_blossom();
        //
        // let (dx, dy) = coordinate_system(&r);
        //
        // let (object_to_ray, object_to_ray_inv) = look_at(&r.origin, r.origin + r.direction, &dx);
        //
        // let cp0 = &object_to_ray * &cp_obj0;
        // let cp1 = &object_to_ray * &cp_obj0;
        // let cp2 = &object_to_ray * &cp_obj0;
        // let cp3 = &object_to_ray * &cp_obj0;
        //
        // let cp = vec![cp0, cp1, cp2.0, cp3];
        //
        // let mut l0 = 0.0;
        //
        // for i in 0..2 {
        //     let tmp_x = (cp.get(i).unwrap().x - 2 * cp.get(i + 1).unwrap().x + cp.get(i + 2).unwrap().x).abs();
        //     let tmp_y = (cp.get(i).unwrap().y - 2 * cp.get(i + 1).unwrap().y + cp.get(i + 2).unwrap().y).abs();
        //     let tmp: f64 = tmp_x.max(tmp_y);
        //     let tmp_z = (cp.get(i).unwrap().z - 2 * cp.get(i + 1).unwrap().z + cp.get(i + 2).unwrap().z).abs();
        //     let tmp = tmp.max(tmp_z);
        //     l0 = l0.max(tmp);
        // }
        // let eps = self
        //     .common
        //     .width
        //     .get(0)
        //     .unwrap()
        //     .max(*self.common.width.get(1).unwrap())
        //     * 0.05;
        //
        // let fr0 = (1.41421356237 * 12.0 * l0 / (8. * eps)).ln() * 0.7213475108;
        // let r0 = fr0.round() as i32;
        // let max_depth = clamp(r0, 0, 10);
        //
        // let res = self.recursive_intersect(r, cp, &object_to_ray_inv, self.u_min, self.u_max, max_depth);

        IntersectionList::new()
    }

    fn calc_blossom(&self) -> (Tuple4D, Tuple4D, Tuple4D, Tuple4D) {
        let p0 = blossom_bezier(&self.common.p, self.u_min, self.u_min, self.u_min);
        let p1 = blossom_bezier(&self.common.p, self.u_min, self.u_min, self.u_max);
        let p2 = blossom_bezier(&self.common.p, self.u_min, self.u_max, self.u_max);
        let p3 = blossom_bezier(&self.common.p, self.u_max, self.u_max, self.u_max);
        (p0, p1, p2, p3)
    }

    pub fn recursive_intersect(
        &self,
        r: &Ray,
        cp: Vec<Tuple4D>,
        object_to_ray_inv: &Matrix,
        u0: f64,
        u1: f64,
        depth: i32,
        intersection: &mut IntersectionList,
    ) -> bool {
        let mut curve_bounds = BoundingBox::new_from_min_max(cp[0], cp[1]);
        let bb2 = BoundingBox::new_from_min_max(cp[2], cp[3]);
        curve_bounds.add(&bb2);

        let tmp1 = lerp_float(u0, self.common.width[0], self.common.width[1]);
        let tmp2 = lerp_float(u1, self.common.width[0], self.common.width[1]);
        let max_width = tmp1.max(tmp2);
        expand(&mut curve_bounds, 0.5 * max_width);
        let ray_length = Tuple4D::magnitude(&r.direction);

        let z_max = ray_length; // there is t in the raytracer impl  ¯\_(ツ)_/¯
        let ray_bounds =
            BoundingBox::new_from_min_max(Tuple4D::new_point(0.0, 0.0, 0.0), Tuple4D::new_point(0.0, 0.0, z_max));
        if !curve_bounds.overlaps(&ray_bounds) {
            return false;
        }

        if depth > 0 {
            // let u_mid = 0.5 * (u0 + u1);
            // let csplit = subdivide_bezier(cp);
            // let c = csplit.as_slice().slice(0, 3).to_vec();
            // let r1 = self.recursive_intersect(r, c, object_to_ray_inv, u0, u_mid, depth - 1, intersection);
            // let c = csplit.as_slice().slice(3, 7).to_vec();
            // let r2 = self.recursive_intersect(r, c, object_to_ray_inv, u_mid, u1, depth - 1, intersection);
            // r1 || r2
            return false;
        }

        true
    }
}

pub struct CurveCommon {
    p: Vec<Tuple4D>,
    n: Vec<Tuple4D>,
    curve_type: CurveType,
    width: Vec<f64>,
    normal_angle: f64,
    inv_sin_normal_angle: f64,
}

impl CurveCommon {
    pub fn new(
        p0: Tuple4D,
        p1: Tuple4D,
        p2: Tuple4D,
        p3: Tuple4D,
        curve_type: CurveType,
        n0: Tuple4D,
        n1: Tuple4D,
        width0: f64,
        width1: f64,
    ) -> CurveCommon {
        let n0 = Tuple4D::normalize(&n0);
        let n1 = Tuple4D::normalize(&n1);
        let n = vec![n0, n1];

        let normal_angle = n0 ^ n1;
        let normal_angle = clamp(normal_angle, 0.0, 1.0);
        let normal_angle = normal_angle.acos();

        let inv_sin_normal_angle = 1.0 / normal_angle.sin();

        let width = vec![width0, width1];
        let p = vec![p0, p1, p2, p3];

        CurveCommon {
            p,
            curve_type,
            n,
            width,
            normal_angle,
            inv_sin_normal_angle,
        }
    }
}

pub fn clamp<T: PartialEq + std::cmp::PartialOrd>(val: T, low: T, high: T) -> T {
    if val < low {
        low
    } else if val > high {
        high
    } else {
        val
    }
}

// pub fn lerp<T>(t: f64, v1: T, v2: T) -> T
//     where
//         T: Mul<f64> + Add<f64>,
//         f64: Mul<T>
// // f64: Add<T>,
// // <f64 as Mul<T>>::Output: Add
// {
//     v1 * (1.0 - t) + v2 * t
// }

pub fn lerp(t: f64, v1: Tuple4D, v2: Tuple4D) -> Tuple4D {
    v1 * (1.0 - t) + v2 * t
}

pub fn lerp_float(t: f64, v1: f64, v2: f64) -> f64 {
    v1 * (1.0 - t) + v2 * t
}

pub fn expand(bb: &mut BoundingBox, delta: f64) {
    let v = Tuple4D::new_vector(delta, delta, delta);
    bb.min = bb.min - v;
    bb.max = bb.max + v;
}

pub fn blossom_bezier(p: &Vec<Tuple4D>, u0: f64, u1: f64, u2: f64) -> Tuple4D {
    let a0 = lerp(u0, p[0], p[1]);
    let a1 = lerp(u0, p[1], p[2]);
    let a2 = lerp(u0, p[2], p[3]);

    let b0 = lerp(u1, a0, a1);
    let b1 = lerp(u1, a1, a2);

    lerp(u2, b0, b1)
}

pub fn pbrt_cross(v1: &Tuple4D, v2: &Tuple4D) -> Tuple4D {
    Tuple4D::new_vector((v1.y * v2.z) - (v1.z * v2.y),
                        (v1.z * v2.x) - (v1.x * v2.z),
                        (v1.x * v2.y) - (v1.y * v2.x))
}

pub fn coordinate_system(v: &Tuple4D) -> (Tuple4D, Tuple4D) {
    let v2 = if v.x.abs() > v.y.abs() {
        Tuple4D::new_vector(-v.z, 0.0, v.x) / (v.x * v.x + v.z * v.z).sqrt()
    } else {
        Tuple4D::new_vector(0.0, v.z, -v.y) / (v.y * v.y + v.z * v.z).sqrt()
    };
    let v3 = v * &v2; // pbrt_cross(v, &v2);
    (v2, v3)
}

pub fn look_at(pos: &Tuple4D, look: &Tuple4D, up: &Tuple4D) -> (Matrix, Matrix) {
    let mut camera_to_world = Matrix::new(4, 4);
    camera_to_world.m[0][3] = pos.x;
    camera_to_world.m[1][3] = pos.y;
    camera_to_world.m[2][3] = pos.z;
    camera_to_world.m[3][3] = 1.0;

    let dir = Tuple4D::normalize(&(look - pos));
    let right = Tuple4D::normalize(&(Tuple4D::normalize(up) * dir));
    let new_up = dir * right;

    camera_to_world.m[0][0] = right.x;
    camera_to_world.m[1][0] = right.y;
    camera_to_world.m[2][0] = right.z;
    camera_to_world.m[3][0] = 0.0;

    camera_to_world.m[0][1] = new_up.x;
    camera_to_world.m[1][1] = new_up.y;
    camera_to_world.m[2][1] = new_up.z;
    camera_to_world.m[3][1] = 0.0;

    camera_to_world.m[0][2] = dir.x;
    camera_to_world.m[1][2] = dir.y;
    camera_to_world.m[2][2] = dir.z;
    camera_to_world.m[3][2] = 0.0;

    (camera_to_world.clone(), Matrix::invert(&camera_to_world).unwrap())
}

pub fn subdivide_bezier(cp: Vec<Tuple4D>) -> Vec<Tuple4D> {
    let csplit0 = cp[0];
    let csplit1 = (cp[0] + cp[1]) / 2.0;
    let csplit2 = (cp[0] + cp[1] * 2.0 + cp[2]) / 4.0;
    let csplit3 = (cp[0] + cp[1] * 3.0 + cp[2] * 3.0 + cp[3]) / 8.0;
    let csplit4 = (cp[1] + cp[2] * 2.0 + cp[3]) / 4.0;
    let csplit5 = (cp[2] + cp[3]) / 2.0;
    let csplit6 = cp[3];

    vec![csplit0, csplit1, csplit2, csplit3, csplit4, csplit5, csplit6]
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use raytracer_challenge_reference_impl::math::{Tuple, Tuple4D};

    use crate::curve::{blossom_bezier, coordinate_system, lerp, lerp_float};

    const EPSILON: f64 = 0.0000001;

    fn assert_two_float(a: f64, b: f64) -> bool {
        // println!("float_equal: a = {}, b = {}", a, b);
        if (a - b).abs() < EPSILON {
            return true;
        }
        false
    }

    fn assert_tuple(actual: &Tuple4D, expected: &Tuple4D) {
        assert!(assert_two_float(actual.x, expected.x));
        assert!(assert_two_float(actual.y, expected.y));
        assert!(assert_two_float(actual.z, expected.z));
        assert!(assert_two_float(actual.w, expected.w));
    }


    #[test]
    pub fn test_lerp_f64() {
        let data = vec![
            (0.000000, 2.000000, 0.000000, 0.000000),
            (0.000000, 2.000000, 0.250000, 0.500000),
            (0.000000, 2.000000, 0.750000, 1.500000),
            (0.000000, 2.000000, 0.500000, 1.000000),
            (0.000000, 2.000000, 1.000000, 2.000000),
            (0.000000, 2.000000, 1.500000, 3.000000),
            (0.000000, 2.000000, 1.750000, 3.500000),
            (0.000000, 2.000000, 2.000000, 4.000000),
        ];

        for d in data {
            let actual = lerp_float(d.2, d.0, d.1);
            let expected = d.3;
            assert_eq!(actual, expected);
        }
    }

    #[test]
    pub fn test_lerp_vec() {
        let data = vec![
            (Tuple4D::new_vector(1.0, 2.0, 3.0), Tuple4D::new_vector(14.0, -15.0, 16.0), 0.000000, Tuple4D::new_vector(1.0, 2.0, 3.0)),
            (Tuple4D::new_vector(1.0, 2.0, 3.0), Tuple4D::new_vector(14.0, -15.0, 16.0), 0.250000, Tuple4D::new_vector(4.25, -2.25, 6.25)),
            (Tuple4D::new_vector(1.0, 2.0, 3.0), Tuple4D::new_vector(14.0, -15.0, 16.0), 0.750000, Tuple4D::new_vector(10.75, -10.75, 12.75)),
            (Tuple4D::new_vector(1.0, 2.0, 3.0), Tuple4D::new_vector(14.0, -15.0, 16.0), 0.500000, Tuple4D::new_vector(7.5, -6.5, 9.5)),
            (Tuple4D::new_vector(1.0, 2.0, 3.0), Tuple4D::new_vector(14.0, -15.0, 16.0), 1.000000, Tuple4D::new_vector(14.0, -15.0, 16.0)),
            (Tuple4D::new_vector(1.0, 2.0, 3.0), Tuple4D::new_vector(14.0, -15.0, 16.0), 1.500000, Tuple4D::new_vector(20.5, -23.5, 22.5)),
            (Tuple4D::new_vector(1.0, 2.0, 3.0), Tuple4D::new_vector(14.0, -15.0, 16.0), 1.750000, Tuple4D::new_vector(23.75, -27.75, 25.75)),
            (Tuple4D::new_vector(1.0, 2.0, 3.0), Tuple4D::new_vector(14.0, -15.0, 16.0), 2.000000, Tuple4D::new_vector(27.0, -32.0, 29.0)),
        ];

        for d in data {
            let actual = lerp(d.2, d.0, d.1);
            let expected = d.3;
            assert_eq!(actual, expected);
        }
    }


    #[test]
    pub fn test_blossom_bezier() {
        let p = vec![
            Tuple4D::new_point(0.000000, 1.000000, 1.000000),
            Tuple4D::new_point(1.000000, 1.000000, 2.000000),
            Tuple4D::new_point(2.000000, 2.000000, 2.000000),
            Tuple4D::new_point(3.000000, 3.000000, 3.000000),
        ];

        let u0 = 0.000000;
        let u1 = 0.250000;
        let u2 = 1.000000;

        let data = vec![
            (blossom_bezier(&p, u0, u0, u0), Tuple4D::new_point(0., 1., 1.)),
            (blossom_bezier(&p, u0, u0, u1), Tuple4D::new_point(0.25, 1., 1.25)),
            (blossom_bezier(&p, u1, u1, u2), Tuple4D::new_point(1.5, 1.5, 2.0625)),
            (blossom_bezier(&p, u2, u2, u2), Tuple4D::new_point(3., 3., 3.)),
        ];

        for d in data {
            let actual = d.0;
            let expected = d.1;
            println!("actual        {:?}   ", actual);
            println!("expected      {:?} ", expected);
            assert_eq!(actual, expected);
        }
    }


    // TODO this crashes probably due to the coordinate system used in the PBRT and the left handed system from the Math Functions in this raytracer
    #[test]
    pub fn test_coordinate_system() {
        let data = vec![
            (Tuple4D::new_vector(1.0, 0.0, 0.0), Tuple4D::new_vector(0., 0., 1.), Tuple::new_vector(0., -1., 0.)),
            (Tuple4D::new_vector(1.0, 1.0, 0.0), Tuple4D::new_vector(0., 0., -1.), Tuple::new_vector(-1., 1., 0.)),
            (Tuple4D::new_vector(1.0, 0.0, 1.0), Tuple4D::new_vector(-SQRT_2 / 2.0, 0., SQRT_2 / 2.0), Tuple::new_vector(0., -SQRT_2, 0.)),
            (Tuple4D::new_vector(1.0, 1.0, 1.0), Tuple4D::new_vector(0., SQRT_2 / 2.0, -SQRT_2 / 2.0), Tuple::new_vector(-SQRT_2, SQRT_2 / 2.0, SQRT_2 / 2.0)),
        ];


        for d in data {
            let (actual1, actual2) = coordinate_system(&d.0);
            let expected1 = d.1;
            let expected2 = d.2;

            println!("-------------------------------------");
            println!("actual1        {:?}   ", actual1);
            println!("expected1      {:?} ", expected1);
            println!("actual2        {:?}   ", actual2);
            println!("expected2      {:?} ", expected2);

            assert_tuple(&actual1, &expected1);
            assert_tuple(&actual2, &expected2);
        }
    }
}
