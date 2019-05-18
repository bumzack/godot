use std::ops::{Add, Sub};

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
fn test_sub_vec_vec() {
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
    let c = p-v;

    assert_eq!(c.x, -2.0);
    assert_eq!(c.y, -4.0);
    assert_eq!(c.z, -6.0);
    assert_eq!(Tuple4D::is_point(&c), true);
}

