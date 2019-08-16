use crate::basics::color::Color;
use crate::math::matrix::Matrix;
use crate::math::tuple4d::Tuple4D;

pub const EPSILON: f64 = 0.00001;

pub fn assert_matrix(actual: &Matrix, expected: &Matrix) {
    assert_eq!(assert_two_float(actual.m[0][0], expected.m[0][0]), true);
    assert_eq!(assert_two_float(actual.m[0][1], expected.m[0][1]), true);
    assert_eq!(assert_two_float(actual.m[0][2], expected.m[0][2]), true);
    assert_eq!(assert_two_float(actual.m[0][3], expected.m[0][3]), true);

    assert_eq!(assert_two_float(actual.m[1][0], expected.m[1][0]), true);
    assert_eq!(assert_two_float(actual.m[1][1], expected.m[1][1]), true);
    assert_eq!(assert_two_float(actual.m[1][2], expected.m[1][2]), true);
    assert_eq!(assert_two_float(actual.m[1][3], expected.m[1][3]), true);

    assert_eq!(assert_two_float(actual.m[2][0], expected.m[2][0]), true);
    assert_eq!(assert_two_float(actual.m[2][1], expected.m[2][1]), true);
    assert_eq!(assert_two_float(actual.m[2][2], expected.m[2][2]), true);
    assert_eq!(assert_two_float(actual.m[2][3], expected.m[2][3]), true);

    assert_eq!(assert_two_float(actual.m[3][0], expected.m[3][0]), true);
    assert_eq!(assert_two_float(actual.m[3][1], expected.m[3][1]), true);
    assert_eq!(assert_two_float(actual.m[3][2], expected.m[3][2]), true);
    assert_eq!(assert_two_float(actual.m[3][3], expected.m[3][3]), true);
}

pub fn assert_tuple(actual: &Tuple4D, expected: &Tuple4D) {
    assert_eq!(assert_two_float(actual.x, expected.x), true);
    assert_eq!(assert_two_float(actual.y, expected.y), true);
    assert_eq!(assert_two_float(actual.z, expected.z), true);
    assert_eq!(assert_two_float(actual.w, expected.w), true);
}

pub fn assert_color(actual: &Color, expected: &Color) {
    assert_eq!(assert_two_float(actual.r, expected.r), true);
    assert_eq!(assert_two_float(actual.g, expected.g), true);
    assert_eq!(assert_two_float(actual.b, expected.b), true);
}

pub fn assert_two_float(a: f64, b: f64) -> bool {
    // println!("float_equal: a = {}, b = {}", a, b);
    if (a - b).abs() < EPSILON {
        return true;
    }
    false
}

pub fn assert_float(actual: f64, expected: f64) {
    assert_eq!(assert_two_float(actual, expected), true);
}


pub fn max_float(a: f64, b: f64, c: f64) -> f64 {
    let mut max = a;
    if b > max {
        max = b;
    }
    if c > max {
        max = c;
    }
    max
}

pub fn min_float(a: f64, b: f64, c: f64) -> f64 {
    let mut min = a;
    if b < min {
        min = b;
    }
    if c < min {
        min = c;
    }
    min
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_float_1() {
        let a = 1.0;
        let b = 2.0;
        let c = -2.0;

        let min = min_float(a, b, c);

        assert_float(min, c);
    }

    #[test]
    fn test_min_float_2() {
        let a = 1.0;
        let b = -2.0;
        let c = 2.0;

        let min = min_float(a, b, c);

        assert_float(min, b);
    }

    #[test]
    fn test_min_float_3() {
        let a = -111.0;
        let b = 2.0;
        let c = -2.0;

        let min = min_float(a, b, c);

        assert_float(min, a);
    }

    #[test]
    fn test_max_float_1() {
        let a = 1.0;
        let b = 2.0;
        let c = 23.0;

        let max = max_float(a, b, c);

        assert_float(max, c);
    }

    #[test]
    fn test_max_float_2() {
        let a = 1.0;
        let b = 99.0;
        let c = 2.0;

        let max = max_float(a, b, c);

        assert_float(max, b);
    }

    #[test]
    fn test_max_float_3() {
        let a = 111.0;
        let b = 2.0;
        let c = -2.0;

        let max = max_float(a, b, c);

        assert_float(max, a);
    }
}