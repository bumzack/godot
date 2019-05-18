use std::ops::{Add, Mul, Neg, Sub};

struct Tuple4D {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}


fn float_equal(a: f32, b: f32) -> bool {
    let EPSILON = 0.00001;

    if (a - b).abs() < EPSILON {
        return true;
    }
    false
}

trait Tuple {
    fn add(a: &Tuple4D, b: &Tuple4D) -> Tuple4D;
    fn mul_by_scalar(a: &Tuple4D, f: f32) -> Tuple4D;

    fn new_vector(x: f32, y: f32, z: f32) -> Tuple4D;
    fn new_point(x: f32, y: f32, z: f32) -> Tuple4D;
    fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple4D;

    fn is_point(a: &Tuple4D) -> bool;
    fn is_vector(a: &Tuple4D) -> bool;
}

impl Tuple for Tuple4D {
    fn add(a: &Tuple4D, b: &Tuple4D) -> Tuple4D {
        Tuple4D {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
            w: 0.0,
        }
    }

    fn mul_by_scalar(a: &Tuple4D, f: f32) -> Tuple4D {
        Tuple4D {
            x: a.x * f,
            y: a.y * f,
            z: a.z * f,
            w: 0.0,
        }
    }

    fn new_vector(x: f32, y: f32, z: f32) -> Tuple4D {
        Tuple4D {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }

    fn new_point(x: f32, y: f32, z: f32) -> Tuple4D {
        Tuple4D {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        }
    }

    fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple4D {
        Tuple4D {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    fn is_point(a: &Tuple4D) -> bool {
        a.w == 1.0
    }

    fn is_vector(a: &Tuple4D) -> bool {
        a.w == 0.0
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

#[test]
fn test_blaaaaaaa() {
    let a = Tuple4D::new_vector(1.0, 2.0, 3.0);
    let b = Tuple4D::mul_by_scalar(&a, 2.0);

    assert_eq!(b.x, 2.0);
    assert_eq!(b.y, 4.0);
    assert_eq!(b.z, 6.0);
}

#[test]
fn test_is_point() {
    let a = Tuple4D::new_point(4.3, -4.2, 3.1);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 1.0);
    assert_eq!(Tuple4D::is_point(&a), true);
}

#[test]
fn test_is_vector() {
    let a = Tuple4D::new_vector(4.3, -4.2, 3.1);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 0.0);
    assert_eq!(Tuple4D::is_vector(&a), true);
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
    assert_eq!(Tuple4D::is_vector(&c), true);
}

#[test]
fn test_sub_vec_point() {
    let p = Tuple4D::new_point(3., 2., 1.);
    let v = Tuple4D::new_vector(5., 6., 7.);
    let c = p - v;

    assert_eq!(c.x, -2.0);
    assert_eq!(c.y, -4.0);
    assert_eq!(c.z, -6.0);
    assert_eq!(Tuple4D::is_point(&c), true);
}


#[test]
fn test_sub_vec_vec() {
    let v1 = Tuple4D::new_vector(3., 2., 1.);
    let v2 = Tuple4D::new_vector(5., 6., 7.);
    let c = v1 - v2;

    assert_eq!(c.x, -2.0);
    assert_eq!(c.y, -4.0);
    assert_eq!(c.z, -6.0);
    assert_eq!(Tuple4D::is_vector(&c), true);
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
    let v2 = v1  * 3.5;

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
