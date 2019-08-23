#![no_std]


use crate::math::math::intri_sqrt;
use core::ops::{Add, BitXor, Div, Mul, Sub};

#[derive(Clone, Debug, PartialEq, DeviceCopy)]
pub struct Tuple4D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait Tuple {
    fn magnitude(a: &Tuple4D) -> f32;
    fn normalize(a: &Tuple4D) -> Tuple4D;

    fn new_vector(x: f32, y: f32, z: f32) -> Tuple4D;
    fn new_vector_from(v: &Tuple4D) -> Tuple4D;
    fn new_point(x: f32, y: f32, z: f32) -> Tuple4D;
    fn new_point_from(v: &Tuple4D) -> Tuple4D;
    fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple4D;
    fn empty() -> Tuple4D;

    fn is_point(a: &Tuple4D) -> bool;
    fn is_vector(a: &Tuple4D) -> bool;

    fn reflect(v: &Tuple4D, n: &Tuple4D) -> Tuple4D;
}

impl Tuple for Tuple4D {
    #[inline]
    fn magnitude(a: &Tuple4D) -> f32 {
        intri_sqrt(a.x * a.x + a.y * a.y + a.z * a.z + a.w * a.w)
    }
    #[inline]
    fn normalize(a: &Tuple4D) -> Tuple4D {
        let m = Tuple4D::magnitude(a);
        Tuple4D {
            x: a.x / m,
            y: a.y / m,
            z: a.z / m,
            w: a.w,
        }
    }
    #[inline]
    fn new_vector(x: f32, y: f32, z: f32) -> Tuple4D {
        Tuple4D {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }
    #[inline]
    fn new_vector_from(v: &Tuple4D) -> Tuple4D {
        Tuple4D {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        }
    }
    #[inline]
    fn new_point(x: f32, y: f32, z: f32) -> Tuple4D {
        Tuple4D { x, y, z, w: 1.0 }
    }
    #[inline]
    fn new_point_from(v: &Tuple4D) -> Tuple4D {
        Tuple4D {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 1.0,
        }
    }
    #[inline]
    fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple4D {
        Tuple4D { x, y, z, w }
    }
    #[inline]
    fn empty() -> Tuple4D {
        Tuple4D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
    #[inline]
    fn is_point(a: &Tuple4D) -> bool {
        a.w == 1.0
    }
    #[inline]
    fn is_vector(a: &Tuple4D) -> bool {
        a.w == 0.0
    }
    #[inline]
    fn reflect(v: &Tuple4D, n: &Tuple4D) -> Tuple4D {
        v - &((n * 2.0) * (v ^ n))
    }
}

impl Add for Tuple4D {
    type Output = Tuple4D;

    fn add(self, other: Tuple4D) -> Tuple4D {
        Tuple4D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<'a, 'b> Add<&'b Tuple4D> for &'a Tuple4D {
    type Output = Tuple4D;

    fn add(self, rhs: &'b Tuple4D) -> Tuple4D {
        Tuple4D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Tuple4D {
    type Output = Tuple4D;

    fn sub(self, other: Tuple4D) -> Tuple4D {
        Tuple4D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<'a, 'b> Sub<&'b Tuple4D> for &'a Tuple4D {
    type Output = Tuple4D;

    fn sub(self, rhs: &'b Tuple4D) -> Tuple4D {
        Tuple4D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Mul<f32> for Tuple4D {
    type Output = Tuple4D;

    fn mul(self, rhs: f32) -> Tuple4D {
        Tuple4D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<'a> Mul<f32> for &'a Tuple4D {
    type Output = Tuple4D;

    fn mul(self, rhs: f32) -> Tuple4D {
        Tuple4D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul for Tuple4D {
    type Output = Tuple4D;

    fn mul(self, rhs: Tuple4D) -> Tuple4D {
        Tuple4D::new_vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl<'a, 'b> Mul<&'b Tuple4D> for &'a Tuple4D {
    type Output = Tuple4D;

    fn mul(self, rhs: &'b Tuple4D) -> Tuple4D {
        Tuple4D::new_vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

// a ^ b
impl BitXor for Tuple4D {
    type Output = f32;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl<'a, 'b> BitXor<&'b Tuple4D> for &'a Tuple4D {
    type Output = f32;

    fn bitxor(self, rhs: &'b Tuple4D) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl Div<f32> for Tuple4D {
    type Output = Tuple4D;

    fn div(self, rhs: f32) -> Tuple4D {
        Tuple4D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use core::f32::consts::SQRT_2;

    use crate::math::common::{assert_float, assert_tuple};

    use super::*;

    #[test]
    fn test_is_point() {
        let a = Tuple4D::new_point(4.3, -4.2, 3.1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(Tuple4D::is_point(&a));
    }

    #[test]
    fn test_is_vector() {
        let a = Tuple4D::new_vector(4.3, -4.2, 3.1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(Tuple4D::is_vector(&a));
    }

    #[test]
    fn test_add_tuple4d() {
        let a = Tuple4D::new(3., -2., 5., 1.);
        let b = Tuple4D::new(-2., 3., 1., 0.);
        let c = a + b;

        assert_eq!(c.x, 1.0);
        assert_eq!(c.y, 1.0);
        assert_eq!(c.z, 6.0);
        assert_eq!(c.w, 1.0);
    }

    #[test]
    fn test_sub_point_point() {
        let a = Tuple4D::new_point(3., 2., 1.);
        let b = Tuple4D::new_point(5., 6., 7.);
        let c = a - b;

        assert_eq!(c.x, -2.0);
        assert_eq!(c.y, -4.0);
        assert_eq!(c.z, -6.0);
        assert!(Tuple4D::is_vector(&c));
    }

    #[test]
    fn test_sub_vec_point() {
        let p = Tuple4D::new_point(3., 2., 1.);
        let v = Tuple4D::new_vector(5., 6., 7.);
        let c = p - v;

        assert_eq!(c.x, -2.0);
        assert_eq!(c.y, -4.0);
        assert_eq!(c.z, -6.0);
        assert!(Tuple4D::is_point(&c));
    }

    #[test]
    fn test_sub_vec_vec() {
        let v1 = Tuple4D::new_vector(3., 2., 1.);
        let v2 = Tuple4D::new_vector(5., 6., 7.);
        let c = v1 - v2;

        assert_eq!(c.x, -2.0);
        assert_eq!(c.y, -4.0);
        assert_eq!(c.z, -6.0);
        assert!(Tuple4D::is_vector(&c));
    }

    //    #[test]
    //    fn test_neg_tuple() {
    //        let v1 = Tuple4D::new(1., -2., 3., 4.);
    //        let v2 = -v1;
    //
    //        assert_eq!(v2.x, -1.0);
    //        assert_eq!(v2.y, 2.0);
    //        assert_eq!(v2.z, -3.0);
    //        assert_eq!(v2.w, -4.0);
    //    }

    #[test]
    fn test_mul_tuple_scalar() {
        let v1 = Tuple4D::new(1., -2., 3., -4.);
        let v2 = v1 * 3.5;

        assert_eq!(v2.x, 3.5);
        assert_eq!(v2.y, -7.0);
        assert_eq!(v2.z, 10.5);
        assert_eq!(v2.w, -14.0);

        let v1 = Tuple4D::new(1., -2., 3., -4.);
        let v2 = v1 * 0.5;

        assert_eq!(v2.x, 0.5);
        assert_eq!(v2.y, -1.0);
        assert_eq!(v2.z, 1.5);
        assert_eq!(v2.w, -2.0);
    }

    #[test]
    fn test_div_tuple_scalar() {
        let v1 = Tuple4D::new(1., -2., 3., -4.);
        let v2 = v1 / 2.0;

        assert_eq!(v2.x, 0.5);
        assert_eq!(v2.y, -1.0);
        assert_eq!(v2.z, 1.5);
        assert_eq!(v2.w, -2.0);
    }

    #[test]
    fn test_magnitude() {
        let v = Tuple4D::new_vector(1., 0., 0.);
        let m = Tuple4D::magnitude(&v);
        assert_eq!(m, 1.);

        let v = Tuple4D::new_vector(0., 1., 0.);
        let m = Tuple4D::magnitude(&v);
        assert_eq!(m, 1.);

        let v = Tuple4D::new_vector(0., 0., 1.);
        let m = Tuple4D::magnitude(&v);
        assert_eq!(m, 1.);

        let expected: f32 = 14.0;

        let v = Tuple4D::new_vector(1., 2., 3.);
        let m = Tuple4D::magnitude(&v);
        assert_float(m, expected.sqrt());

        let v = Tuple4D::new_vector(-1., -2., -3.);
        let m = Tuple4D::magnitude(&v);
        assert_float(m, expected.sqrt());
    }

    #[test]
    fn test_normalize() {
        let v = Tuple4D::new_vector(4., 0., 0.);
        let n = Tuple4D::normalize(&v);
        assert_float(n.x, 1.);
        assert_float(n.y, 0.);
        assert_float(n.z, 0.);
        Tuple4D::is_vector(&n);

        let expected: f32 = 14.0;

        let v = Tuple4D::new_vector(1., 2., 3.);
        let n = Tuple4D::normalize(&v);
        assert_float(n.x, 1. / expected.sqrt());
        assert_float(n.y, 2. / expected.sqrt());
        assert_float(n.z, 3. / expected.sqrt());
        assert!(Tuple4D::is_vector(&n));

        let v = Tuple4D::new_vector(1., 2., 3.);
        let n = Tuple4D::normalize(&v);
        let m = Tuple4D::magnitude(&n);
        assert_float(m, 1.);
    }

    #[test]
    fn test_dot_product() {
        let a = Tuple4D::new_vector(1., 2., 3.);
        let b = Tuple4D::new_vector(2., 3., 4.);
        let c = a ^ b;
        assert_float(c, 20.);
    }

    #[test]
    fn test_cross_product() {
        let a = Tuple4D::new_vector(1., 2., 3.);
        let b = Tuple4D::new_vector(2., 3., 4.);

        let c = a * b;
        assert_eq!(c.x, -1.0);
        assert_eq!(c.y, 2.0);
        assert_eq!(c.z, -1.0);

        let a = Tuple4D::new_vector(1., 2., 3.);
        let b = Tuple4D::new_vector(2., 3., 4.);
        let c = b * a;
        assert_eq!(c.x, 1.0);
        assert_eq!(c.y, -2.0);
        assert_eq!(c.z, 1.0);
    }

    // page 83
    #[test]
    fn test_tuple_reflecting_45() {
        let v = Tuple4D::new_vector(1., -1., 0.);
        let n = Tuple4D::new_vector(0., 1., 0.);

        let r = Tuple4D::reflect(&v, &n);
        let r_expected = Tuple4D::new_vector(1., 1., 0.);
        assert_tuple(&r, &r_expected);
    }

    // page 83
    #[test]
    fn test_tuple_reflecting_slanted_surface() {
        let v = Tuple4D::new_vector(0.0, -1.0, 0.);
        let n = Tuple4D::new_vector(SQRT_2 / 2.0, SQRT_2 / 2.0, 0.);

        let r = Tuple4D::reflect(&v, &n);
        let r_expected = Tuple4D::new_vector(1.0, 0.0, 0.0);
        assert_tuple(&r, &r_expected);
    }
}
