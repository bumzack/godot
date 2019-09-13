// TODO
// THATS FOR A LEFT HANDED COORDINATE SYSTEM ...
// where are modifications needed

use std::f32::consts::SQRT_2;
use std::ops::{Add, BitXor, Div, Mul, Neg, Sub};

use crate::math::common::{assert_float, assert_Tuple4D};

pub const ORIGIN: Tuple4D = Tuple4D {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 1.0,
};

#[derive(Clone, Debug, Copy)]
pub struct Tuple4D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait Tuple4DOps {
    fn mag(a: &Tuple4D) -> f32;
    fn norm(a: &Tuple4D) -> Tuple4D;

    fn magnitude(&self) -> f32;
    fn normalize(&mut self);

    fn new_vector(x: f32, y: f32, z: f32) -> Tuple4D;
    fn new_vector_from(v: &Tuple4D) -> Tuple4D;

    fn new_point(x: f32, y: f32, z: f32) -> Tuple4D;
    fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple4D;
    fn empty() -> Tuple4D;

    fn is_point(a: &Tuple4D) -> bool;
    fn is_Tuple4D(a: &Tuple4D) -> bool;

    fn reflect(v: &Tuple4D, n: &Tuple4D) -> Tuple4D;

    fn get_x(self) -> f32;
    fn get_y(self) -> f32;
    fn get_z(self) -> f32;

    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn set_z(&mut self, z: f32);
}

impl Tuple4DOps for Tuple4D {
    fn mag(a: &Tuple4D) -> f32 {
        (a.x * a.x + a.y * a.y + a.z * a.z + a.w * a.w).sqrt()
    }

    fn norm(a: &Tuple4D) -> Tuple4D {
        let m = Tuple4D::mag(a);
        Tuple4D {
            x: a.x / m,
            y: a.y / m,
            z: a.z / m,
            w: a.w / m,
        }
    }

    fn magnitude(&self) -> f32 {
        Self::mag(self)
    }

    fn normalize(&mut self) {
        let m = Tuple4D::mag(self);
        self.x = self.x / m;
        self.y = self.y / m;
        self.z = self.z / m;
        self.w = self.w / m;
    }

    fn new_vector(x: f32, y: f32, z: f32) -> Tuple4D {
        Tuple4D {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }

    fn new_vector_from(v: &Tuple4D) -> Tuple4D {
        Tuple4D {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        }
    }

    fn new_point(x: f32, y: f32, z: f32) -> Tuple4D {
        Tuple4D { x, y, z, w: 1.0 }
    }

    fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple4D {
        Tuple4D { x, y, z, w }
    }

    fn empty() -> Tuple4D {
        Tuple4D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    fn is_point(a: &Tuple4D) -> bool {
        a.w == 1.0
    }

    fn is_Tuple4D(a: &Tuple4D) -> bool {
        a.w == 0.0
    }

    fn reflect(v: &Tuple4D, n: &Tuple4D) -> Tuple4D {
        v - &((n * 2.0) * (v ^ n))
    }

    fn get_x(self) -> f32 {
        self.x
    }

    fn get_y(self) -> f32 {
        self.y
    }

    fn get_z(self) -> f32 {
        self.z
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn set_z(&mut self, z: f32) {
        self.z = z;
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

impl Neg for Tuple4D {
    type Output = Tuple4D;

    fn neg(self) -> Tuple4D {
        Tuple4D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

//TODO: this returns a new Tuple!, but we want it to modify the reference?
// or do we?
impl<'a> Neg for &'a Tuple4D {
    type Output = Tuple4D;

    fn neg(self) -> Tuple4D {
        Tuple4D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
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
            w: self.w,
        }
    }
}

impl Mul<Tuple4D> for f32 {
    type Output = Tuple4D;

    fn mul(self, rhs: Tuple4D) -> Tuple4D {
        Tuple4D {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: rhs.w,
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
            w: self.w,
        }
    }
}

impl<'a> Mul<&'a Tuple4D> for f32 {
    type Output = Tuple4D;

    fn mul(self, rhs: &'a Tuple4D) -> Tuple4D {
        Tuple4D {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: rhs.w,
        }
    }
}

//impl<'a> Mul<&f32> for &'a Tuple4D {
//    type Output = Tuple4D;
//
//    fn mul(self, rhs: &f32) -> Tuple4D {
//        Tuple4D {
//            x: self.x * *rhs,
//            y: self.y * *rhs,
//            z: self.z * *rhs,
//            w: self.w * *rhs,
//        }
//    }
//}

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
fn test_is_Tuple4D() {
    let a = Tuple4D::new_vector(4.3, -4.2, 3.1);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 0.0);
    assert!(Tuple4D::is_Tuple4D(&a));
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
    assert!(Tuple4D::is_Tuple4D(&c));
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
    assert!(Tuple4D::is_Tuple4D(&c));
}

#[test]
fn test_neg_tuple() {
    let v1 = Tuple4D::new(1., -2., 3., 4.);
    let v2 = -v1;

    assert_eq!(v2.x, -1.0);
    assert_eq!(v2.y, 2.0);
    assert_eq!(v2.z, -3.0);
    assert_eq!(v2.w, -4.0);
}

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
    let m = Tuple4D::mag(&v);
    assert_eq!(m, 1.);

    let v = Tuple4D::new_vector(0., 1., 0.);
    let m = Tuple4D::mag(&v);
    assert_eq!(m, 1.);

    let v = Tuple4D::new_vector(0., 0., 1.);
    let m = Tuple4D::mag(&v);
    assert_eq!(m, 1.);

    let expected: f32 = 14.0;

    let v = Tuple4D::new_vector(1., 2., 3.);
    let m = Tuple4D::mag(&v);
    assert_float(m, expected.sqrt());

    let v = Tuple4D::new_vector(-1., -2., -3.);
    let m = Tuple4D::mag(&v);
    assert_float(m, expected.sqrt());
}

#[test]
fn test_normalize() {
    let v = Tuple4D::new_vector(4., 0., 0.);
    let n = Tuple4D::norm(&v);
    assert_float(n.x, 1.);
    assert_float(n.y, 0.);
    assert_float(n.z, 0.);
    Tuple4D::is_Tuple4D(&n);

    let expected: f32 = 14.0;

    let v = Tuple4D::new_vector(1., 2., 3.);
    let n = Tuple4D::norm(&v);
    assert_float(n.x, 1. / expected.sqrt());
    assert_float(n.y, 2. / expected.sqrt());
    assert_float(n.z, 3. / expected.sqrt());
    assert!(Tuple4D::is_Tuple4D(&n));

    let v = Tuple4D::new_vector(1., 2., 3.);
    let n = Tuple4D::norm(&v);
    let m = Tuple4D::mag(&n);
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

#[test]
fn test_tuple_reflecting_45() {
    let v = Tuple4D::new_vector(1., -1., 0.);
    let n = Tuple4D::new_vector(0., 1., 0.);

    let r = Tuple4D::reflect(&v, &n);

    let r_expected = Tuple4D::new_vector(1., 1., 0.);

    assert_Tuple4D(&r, &r_expected);
}

#[test]
fn test_tuple_reflecting() {
    let v = Tuple4D::new_vector(0.0, -1.0, 0.);
    let n = Tuple4D::new_vector(SQRT_2 / 2.0, SQRT_2 / 2.0, 0.);

    let r = Tuple4D::reflect(&v, &n);

    let r_expected = Tuple4D::new_vector(1.0, 0.0, 0.0);

    assert_Tuple4D(&r, &r_expected);
}
