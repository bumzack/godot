use std::ops::{Add, Mul};

use raytracer_challenge_reference_impl::math::{Matrix, Tuple, Tuple4D};
use raytracer_challenge_reference_impl::prelude::{BoundingBox, Ray};

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
        let p0 = blossom_bezier(&self.common.p, self.u_min, self.u_min, self.u_min);
        let p1 = blossom_bezier(&self.common.p, self.u_min, self.u_min, self.u_max);
        let p2 = blossom_bezier(&self.common.p, self.u_min, self.u_max, self.u_max);
        let p3 = blossom_bezier(&self.common.p, self.u_max, self.u_max, self.u_max);

        let mut bounding_box1 = BoundingBox::new_from_min_max(p0, p1);
        let bounding_box2 = BoundingBox::new_from_min_max(p2, p3);

        bounding_box1.add(&bounding_box2);

        let width0 = lerp(self.u_min, self.common.width[0], self.common.width[1]);
        let width1 = lerp(self.u_max, self.common.width[0], self.common.width[1]);

        expand(&mut bounding_box1, width0.max(width1) * 0.5);

        bounding_box1
    }

    pub fn intersect(&self, r: &Ray) -> BoundingBox {
        let p0 = blossom_bezier(&self.common.p, self.u_min, self.u_min, self.u_min);
        let p1 = blossom_bezier(&self.common.p, self.u_min, self.u_min, self.u_max);
        let p2 = blossom_bezier(&self.common.p, self.u_min, self.u_max, self.u_max);
        let p3 = blossom_bezier(&self.common.p, self.u_max, self.u_max, self.u_max);

        let mut bounding_box1 = BoundingBox::new_from_min_max(p0, p1);
        let bounding_box2 = BoundingBox::new_from_min_max(p2, p3);

        bounding_box1.add(&bounding_box2);

        let width0 = lerp(self.u_min, self.common.width[0], self.common.width[1]);
        let width1 = lerp(self.u_max, self.common.width[0], self.common.width[1]);

        expand(&mut bounding_box1, width0.max(width1) * 0.5);

        bounding_box1
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
    pub fn new(p0: Tuple4D, p1: Tuple4D, p2: Tuple4D, p3: Tuple4D, curve_type: CurveType, n0: Tuple4D, n1: Tuple4D, width0: f64, width1: f64) -> CurveCommon {
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


pub fn clamp(val: f64, low: f64, high: f64) -> f64 {
    if val < low {
        low
    } else if val > high {
        high
    } else { val }
}

pub fn lerp<T>(t: f64, v1: T, v2: T) -> T
    where
        T: Mul<f64> + Add <f64> ,
        f64: Mul<T>,
        <f64 as Mul<T>>::Output: Add
        // f64: Mul<T>,
        // f64: Add<T>,
        // <f64 as Mul<T>>::Output: Add
{
    (1.0 - t) * v1 + t * v2
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
