// rust impl of https://www.pbr-book.org/3ed-2018/Shapes/Curves

use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

use raytracer_challenge_reference_impl::basics::{IntersectionList, IntersectionListOps, Ray};
use raytracer_challenge_reference_impl::math::{Matrix, MatrixOps, Tuple, Tuple4D};
use raytracer_challenge_reference_impl::prelude::{BoundingBox, RayOps};

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

    fn calc_blossom(&self) -> (Tuple4D, Tuple4D, Tuple4D, Tuple4D) {
        let p0 = blossom_bezier(&self.common.p, self.u_min, self.u_min, self.u_min);
        let p1 = blossom_bezier(&self.common.p, self.u_min, self.u_min, self.u_max);
        let p2 = blossom_bezier(&self.common.p, self.u_min, self.u_max, self.u_max);
        let p3 = blossom_bezier(&self.common.p, self.u_max, self.u_max, self.u_max);
        (p0, p1, p2, p3)
    }

    pub fn intersect(&self, r: &Ray, is: &mut IntersectionList) -> bool {
        // Transform _Ray_ to object space
        // TODO how ?
        //  let r = &self.transform * r;

        // Compute object-space control points for curve segment, _cpObj_
        let (cp_obj0, cp_obj1, cp_obj2, cp_obj3) = self.calc_blossom();

        let mut dx = r.get_direction() * &(cp_obj3 - cp_obj0);
        let dx_length_squared = Tuple4D::magnitude_squared(&dx);
        if dx_length_squared == 0.0 {
            let (dx_new, _) = coordinate_system(r.get_origin());
            dx = dx_new;
        }
        // let (dx, dy) = coordinate_system(&r.get_origin());

        let (object_to_ray, object_to_ray_inv) = look_at(&r.origin, &(r.get_origin() + r.get_direction()), &dx);

        let cp0 = &object_to_ray * &cp_obj0;
        let cp1 = &object_to_ray * &cp_obj1;
        let cp2 = &object_to_ray * &cp_obj2;
        let cp3 = &object_to_ray * &cp_obj3;

        let cp = vec![cp0, cp1, cp2, cp3];

        let tmp1 = lerp_float(self.u_min, self.common.width[0], self.common.width[1]);
        let tmp2 = lerp_float(self.u_max, self.common.width[0], self.common.width[1]);
        let max_width = tmp1.max(tmp2);

        let tmp_y1 = cp0.y.max(cp1.y);
        let tmp_y2 = cp2.y.max(cp3.y);
        let y1 = tmp_y1.max(tmp_y2) + 0.5 * max_width;
        let y2 = tmp_y1.max(tmp_y2) - 0.5 * max_width;

        if y1 < 0.0 || y2 > 0.0 {
            return false;
        }

        // Check for non-overlap in x
        let tmp_x1 = cp0.x.max(cp1.x);
        let tmp_x2 = cp2.x.max(cp3.x);
        let x1 = tmp_x1.max(tmp_x2) + 0.5 * max_width;
        let x2 = tmp_x1.max(tmp_x2) - 0.5 * max_width;

        if x1 < 0.0 || x2 > 0.0 {
            return false;
        }

        // Check for non-overlap in z.
        let ray_length = Tuple4D::magnitude(r.get_direction());
        // TODO
        // tmax and stuff
        let z_max = ray_length;

        let tmp_z1 = cp0.z.max(cp1.z);
        let tmp_z2 = cp2.z.max(cp3.z);
        let z1 = tmp_z1.max(tmp_z2) + 0.5 * max_width;
        let z2 = tmp_z1.max(tmp_z2) - 0.5 * max_width;

        if z1 < 0.0 || z2 > z_max {
            return false;
        }

        let mut l0: f64 = 0.0;

        for i in 0..2 {
            let tmp_x = (cp[i].x - 2.0 * cp[i + 1].x + cp[i + 2].x).abs();
            let tmp_y = (cp[i].y - 2.0 * cp[i + 1].y + cp[i + 2].y).abs();
            let tmp: f64 = tmp_x.max(tmp_y);
            let tmp_z = (cp[i].z - 2.0 * cp[i + 1].z + cp[i + 2].z).abs();
            let tmp = tmp.max(tmp_z);
            l0 = l0.max(tmp);
        }
        let eps = self.common.width[0].max(self.common.width[1]) * 0.05;

        let r0 = log2((SQRT_2 * 6.0 * l0 / (8.0 * eps)) as f32) / 2;
        let max_depth = clamp(r0, 0, 10);

        let mut is = IntersectionList::new();

        self.recursive_intersect(r, &cp, &object_to_ray_inv, self.u_min, self.u_max, max_depth, &mut is)
    }

    pub fn recursive_intersect(
        &self,
        r: &Ray,
        cp: &Vec<Tuple4D>,
        object_to_ray_inv: &Matrix,
        u0: f64,
        u1: f64,
        depth: i32,
        is: &mut IntersectionList,
    ) -> bool {
        let ray_length = Tuple4D::magnitude(r.get_direction());

        if depth > 0 {
            let csplits = subdivide_bezier(cp);

            let mut hit = false;
            let u = vec![u0, (u0 + u1) / 2.0, u1];

            let increment = 3;
            for seg in 0..2 {
                let from = seg * increment;
                let to = seg * increment + increment;
                let cps = &csplits[from..to];

                // loop iteration seg = 0
                let tmp1 = lerp_float(u[0], self.common.width[0], self.common.width[1]);
                let tmp2 = lerp_float(u[1], self.common.width[0], self.common.width[1]);

                let max_width = tmp1.max(tmp2);

                // y coord
                let tmp_y1 = cps[0].y.max(cps[1].y);
                let tmp_y2 = cps[2].y.max(cps[3].y);
                let y1 = tmp_y1.max(tmp_y2) + 0.5 * max_width;
                let y2 = tmp_y1.max(tmp_y2) - 0.5 * max_width;

                if y1 < 0.0 || y2 > 0.0 {
                    continue;
                }

                // x coord
                let tmp_x1 = cps[0].x.max(cps[1].x);
                let tmp_x2 = cps[2].x.max(cps[3].x);
                let x1 = tmp_x1.max(tmp_x2) + 0.5 * max_width;
                let x2 = tmp_x1.max(tmp_x2) - 0.5 * max_width;

                if x1 < 0.0 || x2 > 0.0 {
                    continue;
                }

                // z coord
                // TODO
                // tmax and stuff
                let z_max = ray_length;

                let tmp_z1 = cps[0].z.max(cps[1].z);
                let tmp_z2 = cps[2].z.max(cps[3].z);
                let z1 = tmp_z1.max(tmp_z2) + 0.5 * max_width;
                let z2 = tmp_z1.max(tmp_z2) - 0.5 * max_width;

                if z1 < 0.0 || z2 > z_max {
                    continue;
                }

                hit = hit | self.recursive_intersect(r, &cp, &object_to_ray_inv, u0, u1, depth, is);

                // TODO
                // early exit
            }
            return hit;
        } else {
            let edge = (cp[1].y - cp[0].y) * (-cp[0].y) + (cp[0].x) * (cp[0].x - cp[1].x);
            if edge < 0.0 {
                return false;
            }

            let edge = (cp[2].y - cp[3].y) * (-cp[3].y) + (cp[3].x) * (cp[3].x - cp[2].x);
            if edge < 0.0 {
                return false;
            }

            let segment_direction =
                Tuple4D::new_point(cp[3].x, cp[3].y, 0.0) - Tuple4D::new_point(cp[0].x, cp[0].y, 0.0);
            let denom = Tuple4D::magnitude(&segment_direction);
            if denom == 0.0 {
                return false;
            }
            let p_2d = Tuple4D::new_point(cp[0].x, cp[0].y, 0.0);
            let w = (&(p_2d * (-1.0)) ^ &segment_direction) / denom;
            let u = clamp(lerp_float(w, u0, u1), u0, u1);
            let hit_width = lerp_float(u, self.common.width[0], self.common.width[1]);

            match self.common.curve_type {
                CurveType::RIBBON => {}
                CurveType::FLAT => {}
                CurveType::CYLINDER => {}
            }
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

pub fn ray_bounds(bb: &mut BoundingBox, delta: f64) {
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

// pub fn pbrt_cross(v1: &Tuple4D, v2: &Tuple4D) -> Tuple4D {
//     Tuple4D::new_vector(
//         (v1.y * v2.z) - (v1.z * v2.y),
//         (v1.z * v2.x) - (v1.x * v2.z),
//         (v1.x * v2.y) - (v1.y * v2.x),
//     )
// }

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

    (Matrix::invert(&camera_to_world).unwrap(), camera_to_world.clone())
}

pub fn subdivide_bezier(cp: &Vec<Tuple4D>) -> Vec<Tuple4D> {
    let csplit0 = cp[0];
    let csplit1 = (cp[0] + cp[1]) / 2.0;
    let csplit2 = (cp[0] + cp[1] * 2.0 + cp[2]) / 4.0;
    let csplit3 = (cp[0] + cp[1] * 3.0 + cp[2] * 3.0 + cp[3]) / 8.0;
    let csplit4 = (cp[1] + cp[2] * 2.0 + cp[3]) / 4.0;
    let csplit5 = (cp[2] + cp[3]) / 2.0;
    let csplit6 = cp[3];

    vec![csplit0, csplit1, csplit2, csplit3, csplit4, csplit5, csplit6]
}

fn log2(x: f32) -> i32 {
    if x < 0.0 {
        0
    } else {
        let x_bits = x.to_bits() as i32;
        let tmp = 1i32 << 22;
        // println!("  (x_bits >> 23)    {}", (x_bits >> 23));
        // println!("x_bits & tmp   {}", x_bits & tmp);
        // println!("(x_bits & tmp)   {}", (x_bits & tmp));
        let add = if (x_bits & tmp) == 0 { 0 } else { 1 };
        // println!(" (bits & (1 << 22) ? 1 : 0)   {}", add);

        (x_bits >> 23) - 127 + add
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use raytracer_challenge_reference_impl::math::{assert_matrix, Matrix, Tuple, Tuple4D};
    use raytracer_challenge_reference_impl::prelude::{MatrixOps, Ray, RayOps};
    use raytracer_challenge_reference_impl::shape::BoundingBox;

    use crate::curve::{blossom_bezier, coordinate_system, expand, lerp, lerp_float, log2, look_at};

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
            (
                Tuple4D::new_vector(1.0, 2.0, 3.0),
                Tuple4D::new_vector(14.0, -15.0, 16.0),
                0.000000,
                Tuple4D::new_vector(1.0, 2.0, 3.0),
            ),
            (
                Tuple4D::new_vector(1.0, 2.0, 3.0),
                Tuple4D::new_vector(14.0, -15.0, 16.0),
                0.250000,
                Tuple4D::new_vector(4.25, -2.25, 6.25),
            ),
            (
                Tuple4D::new_vector(1.0, 2.0, 3.0),
                Tuple4D::new_vector(14.0, -15.0, 16.0),
                0.750000,
                Tuple4D::new_vector(10.75, -10.75, 12.75),
            ),
            (
                Tuple4D::new_vector(1.0, 2.0, 3.0),
                Tuple4D::new_vector(14.0, -15.0, 16.0),
                0.500000,
                Tuple4D::new_vector(7.5, -6.5, 9.5),
            ),
            (
                Tuple4D::new_vector(1.0, 2.0, 3.0),
                Tuple4D::new_vector(14.0, -15.0, 16.0),
                1.000000,
                Tuple4D::new_vector(14.0, -15.0, 16.0),
            ),
            (
                Tuple4D::new_vector(1.0, 2.0, 3.0),
                Tuple4D::new_vector(14.0, -15.0, 16.0),
                1.500000,
                Tuple4D::new_vector(20.5, -23.5, 22.5),
            ),
            (
                Tuple4D::new_vector(1.0, 2.0, 3.0),
                Tuple4D::new_vector(14.0, -15.0, 16.0),
                1.750000,
                Tuple4D::new_vector(23.75, -27.75, 25.75),
            ),
            (
                Tuple4D::new_vector(1.0, 2.0, 3.0),
                Tuple4D::new_vector(14.0, -15.0, 16.0),
                2.000000,
                Tuple4D::new_vector(27.0, -32.0, 29.0),
            ),
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

    #[test]
    pub fn test_coordinate_system() {
        let data = vec![
            (
                Tuple4D::new_vector(1.0, 0.0, 0.0),
                Tuple4D::new_vector(0., 0., 1.),
                Tuple::new_vector(0., -1., 0.),
            ),
            (
                Tuple4D::new_vector(1.0, 1.0, 0.0),
                Tuple4D::new_vector(0., 0., -1.),
                Tuple::new_vector(-1., 1., 0.),
            ),
            (
                Tuple4D::new_vector(1.0, 0.0, 1.0),
                Tuple4D::new_vector(-SQRT_2 / 2.0, 0., SQRT_2 / 2.0),
                Tuple::new_vector(0., -SQRT_2, 0.),
            ),
            (
                Tuple4D::new_vector(1.0, 1.0, 1.0),
                Tuple4D::new_vector(0., SQRT_2 / 2.0, -SQRT_2 / 2.0),
                Tuple::new_vector(-SQRT_2, SQRT_2 / 2.0, SQRT_2 / 2.0),
            ),
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

    #[test]
    pub fn test_look_at() {
        let pos = Tuple4D::new_point(1.0, 2.0, 3.0);
        let look = Tuple4D::new_point(5.0, 6.0, 7.0);

        let m_expected1 = Matrix::new_matrix_4x4(
            2.23517418e-08,
            -0.707106769,
            0.707106829,
            -0.707106769,
            0.816496551,
            -0.408248276,
            -0.408248276,
            1.2247448,
            0.577350318,
            0.577350318,
            0.577350318,
            -3.46410179,
            0.0,
            0.0,
            0.0,
            1.0,
        );
        let m_inv_expected1 = Matrix::new_matrix_4x4(
            0.0,
            0.816496551,
            0.577350259,
            1.0,
            -0.707106769,
            -0.408248276,
            0.577350259,
            2.0,
            0.707106769,
            -0.408248276,
            0.577350259,
            3.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let m_expected2 = Matrix::new_matrix_4x4(
            0.707106829,
            -2.98023224e-08,
            -0.707106829,
            1.41421354,
            -0.408248276,
            0.816496551,
            -0.408248246,
            0.0,
            0.577350378,
            0.577350318,
            0.577350259,
            -3.46410179,
            -1.49011612e-08,
            0.0,
            0.0,
            1.0,
        );
        let m_inv_expected2 = Matrix::new_matrix_4x4(
            0.707106769,
            -0.408248276,
            0.577350259,
            1.0,
            0.0,
            0.816496551,
            0.577350259,
            2.0,
            -0.707106769,
            -0.408248276,
            0.577350259,
            3.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let m_expected3 = Matrix::new_matrix_4x4(
            -0.707106829,
            0.707106829,
            -4.47034836e-08,
            -0.707106769,
            -0.408248305,
            -0.408248246,
            0.816496611,
            -1.2247448,
            0.577350318,
            0.577350318,
            0.577350318,
            -3.46410179,
            0.0,
            0.0,
            0.0,
            1.0,
        );
        let m_inv_expected3 = Matrix::new_matrix_4x4(
            -0.707106769,
            -0.408248276,
            0.577350259,
            1.0,
            0.707106769,
            -0.408248276,
            0.577350259,
            2.0,
            0.0,
            0.816496551,
            0.577350259,
            3.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let data = vec![
            (Tuple4D::new_vector(1.0, 0.0, 0.0), m_expected1, m_inv_expected1),
            (Tuple4D::new_vector(0.0, 1.0, 0.0), m_expected2, m_inv_expected2),
            (Tuple::new_vector(0.0, 0.0, 1.0), m_expected3, m_inv_expected3),
        ];

        for d in data {
            let (m, n) = look_at(&pos, &look, &d.0);

            println!("-------------------------------------");
            println!("pos           {:?}   ", &pos);
            println!("look          {:?} ", &look);
            println!("up            {:?}   ", d);

            println!("m expected            {:?}   ", &d.1);
            println!("m actual            {:?}   ", &m);

            println!("m_inv expected            {:?}   ", &d.2);
            println!("m_inv actual            {:?}   ", &n);

            assert_matrix(&m, &d.1);
            assert_matrix(&n, &d.2);
        }
    }

    #[test]
    pub fn test_union() {
        let p0 = Tuple4D::new_point(0.0, 0.0, 0.0);
        let p1 = Tuple4D::new_point(2.0, 3.0, 4.0);

        let p2 = Tuple4D::new_point(0.5, -0.5, -1.5);
        let p3 = Tuple4D::new_point(1.5, 13.0, 3.0);

        let mut bb1 = BoundingBox::new_from_min_max(p0, p1);
        let bb2 = BoundingBox::new_from_min_max(p2, p3);

        let union_min = Tuple4D::new_point(0.000000, -0.500000, -1.500000);
        let union_max = Tuple4D::new_point(2.000000, 13.000000, 4.000000);

        bb1.add(&bb2);

        println!("bb.min            {:?}", &bb1.min);
        println!("union_min         {:?}", &union_min);

        println!("bb.max            {:?}", &bb1.max);
        println!("union_max         {:?}", &union_max);

        assert_tuple(&bb1.min, &union_min);
        assert_tuple(&bb1.max, &union_max);
    }

    #[test]
    pub fn test_expand() {
        let p0 = Tuple4D::new_point(0.000000, 0.000000, 0.000000);
        let p1 = Tuple4D::new_point(2.000000, 3.000000, 4.000000);

        let p2 = Tuple4D::new_point(-0.750000, -0.750000, -0.750000);
        let p3 = Tuple4D::new_point(2.750000, 3.750000, 4.750000);

        let mut actual = BoundingBox::new_from_min_max(p0, p1);
        expand(&mut actual, 0.75);

        let bb_expected = BoundingBox::new_from_min_max(p2, p3);

        println!("actual.min            {:?}", &actual.min);
        println!("actual.max            {:?}", &actual.max);

        println!("bb_expected.min           {:?}", &bb_expected.min);
        println!("bb_expected.max           {:?}", &bb_expected.max);

        assert_tuple(&actual.min, &bb_expected.min);
        assert_tuple(&actual.max, &bb_expected.max);
    }

    #[test]
    pub fn test_ray_bounds() {
        let p = Tuple4D::new_point(1.000000, 2.000000, 3.000000);
        let d = Tuple4D::new_vector(-2.000000, -2.000000, -1.500000);

        let r = Ray::new(p, d);
        let p3 = Tuple4D::new_point(2.750000, 3.750000, 4.750000);

        // let mut actual = BoundingBox::new_from_min_max(p0, p1);
        // expand(&mut actual, 0.75);
        //
        // let bb_expected = BoundingBox::new_from_min_max(p2, p3);
        //
        //
        // println!("actual.min            {:?}", &actual.min);
        // println!("actual.max            {:?}", &actual.max);
        //
        // println!("bb_expected.min           {:?}", &bb_expected.min);
        // println!("bb_expected.max           {:?}", &bb_expected.max);
        //
        // assert_tuple(&actual.min, &bb_expected.min);
        // assert_tuple(&actual.max, &bb_expected.max);
    }

    #[test]
    pub fn test_log2() {
        let data: Vec<(f64, i32)> = vec![
            (7.000000, 3),
            (19.000000, 4),
            (16.000000, 4),
            (79.000000, 6),
            (1.000000, 0),
            (83.000000, 6),
            (22.000000, 4),
            (76.000000, 6),
            (71.000000, 6),
            (58.000000, 6),
            (31.000000, 5),
            (71.000000, 6),
            (3.000000, 2),
            (55.000000, 6),
            (73.000000, 6),
            (100.000000, 7),
            (46.000000, 5),
            (94.000000, 6),
            (47.000000, 5),
            (76.000000, 6),
            (70.000000, 6),
            (31.000000, 5),
            (60.000000, 6),
            (55.000000, 6),
            (49.000000, 6),
            (1.000000, 0),
            (86.000000, 6),
            (32.000000, 5),
            (51.000000, 6),
            (31.000000, 5),
            (30.000000, 5),
            (59.000000, 6),
            (10.000000, 3),
            (87.000000, 6),
            (29.000000, 5),
            (8.000000, 3),
            (27.000000, 5),
            (7.000000, 3),
            (85.000000, 6),
            (46.000000, 5),
            (26.000000, 5),
            (85.000000, 6),
            (41.000000, 5),
            (87.000000, 6),
            (6.000000, 3),
            (67.000000, 6),
            (83.000000, 6),
            (95.000000, 6),
            (47.000000, 5),
            (82.000000, 6),
            (10.000000, 3),
            (60.000000, 6),
            (54.000000, 6),
            (85.000000, 6),
            (35.000000, 5),
            (77.000000, 6),
            (61.000000, 6),
            (49.000000, 6),
            (16.000000, 4),
            (92.000000, 6),
            (23.000000, 4),
            (34.000000, 5),
            (41.000000, 5),
            (15.000000, 4),
            (5.000000, 2),
            (100.000000, 7),
            (22.000000, 4),
            (94.000000, 6),
            (14.000000, 4),
            (70.000000, 6),
            (41.000000, 5),
            (88.000000, 6),
            (85.000000, 6),
            (21.000000, 4),
            (21.000000, 4),
            (22.000000, 4),
            (92.000000, 6),
            (68.000000, 6),
            (84.000000, 6),
            (75.000000, 6),
            (23.000000, 4),
            (100.000000, 7),
            (97.000000, 7),
            (7.000000, 3),
            (65.000000, 6),
            (54.000000, 6),
            (46.000000, 5),
            (19.000000, 4),
            (40.000000, 5),
            (5.000000, 2),
            (68.000000, 6),
            (90.000000, 6),
            (61.000000, 6),
            (67.000000, 6),
            (64.000000, 6),
            (53.000000, 6),
            (10.000000, 3),
            (91.000000, 6),
            (84.000000, 6),
            (68.000000, 6),
        ];

        for (idx, d) in data.iter().enumerate() {
            let l2 = log2(d.0 as f32);
            println!("asserting {}. entry actual {} == {}", idx + 1, l2, d.1);
            assert_eq!(l2, d.1);
        }
    }
}
