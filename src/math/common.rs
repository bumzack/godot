use crate::math::matrix::Matrix;
use crate::math::tuple4d::{Tuple, Tuple4D};

pub fn float_equal(a: f32, b: f32) -> bool {
    let EPSILON = 0.00001;

    println!("float_equal: a = {}, b = {}", a, b);
    if (a - b).abs() < EPSILON {
        return true;
    }
    false
}


pub fn assert_matrices(a: &Matrix, b: &Matrix) {
    assert_eq!(float_equal(a.m[0][0], b.m[0][0]), true);
    assert_eq!(float_equal(a.m[0][1], b.m[0][1]), true);
    assert_eq!(float_equal(a.m[0][2], b.m[0][2]), true);
    assert_eq!(float_equal(a.m[0][3], b.m[0][3]), true);

    assert_eq!(float_equal(a.m[1][0], b.m[1][0]), true);
    assert_eq!(float_equal(a.m[1][1], b.m[1][1]), true);
    assert_eq!(float_equal(a.m[1][2], b.m[1][2]), true);
    assert_eq!(float_equal(a.m[1][3], b.m[1][3]), true);

    assert_eq!(float_equal(a.m[2][0], b.m[2][0]), true);
    assert_eq!(float_equal(a.m[2][1], b.m[2][1]), true);
    assert_eq!(float_equal(a.m[2][2], b.m[2][2]), true);
    assert_eq!(float_equal(a.m[2][3], b.m[2][3]), true);

    assert_eq!(float_equal(a.m[3][0], b.m[3][0]), true);
    assert_eq!(float_equal(a.m[3][1], b.m[3][1]), true);
    assert_eq!(float_equal(a.m[3][2], b.m[3][2]), true);
    assert_eq!(float_equal(a.m[3][3], b.m[3][3]), true);
}

pub fn assert_tuple(a: &Tuple4D, b: &Tuple4D) {
    assert_eq!(float_equal(a.x, b.x), true);
    assert_eq!(float_equal(a.y, b.y), true);
    assert_eq!(float_equal(b.z, b.z), true);
    assert_eq!(float_equal(b.w, b.w), true);
}

pub fn assert_float(a: f32, b: f32) {
    assert_eq!(float_equal(a, b), true);
}

