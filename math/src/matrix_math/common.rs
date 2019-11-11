use crate::prelude::*;

pub const EPSILON: f32 = 0.00001;
pub const EPSILON_OVER_UNDER: f32 = 0.005;

use crate::prelude::math_ops::math_ops::{intri_abs};

pub fn assert_matrix(actual: &Matrix, expected: &Matrix) {
    assert_eq!(actual.rows, expected.rows);
    assert_eq!(actual.cols, expected.cols);
    for c in 0..expected.cols {
        for r in 0..expected.rows {
            assert_eq!(assert_two_float(actual[r][c], expected[r][c]), true);
        }
    }
}

pub fn assert_tuple(actual: &Tuple4D, expected: &Tuple4D) {
    assert_eq!(assert_two_float(actual.x, expected.x), true);
    assert_eq!(assert_two_float(actual.y, expected.y), true);
    assert_eq!(assert_two_float(actual.z, expected.z), true);
    assert_eq!(assert_two_float(actual.w, expected.w), true);
}

pub fn assert_two_float(a: f32, b: f32) -> bool {
    // println!("float_equal: a = {}, b = {}", a, b);
    // println!("float_equal: a = {}, b = {}", a, b);
    if intri_abs(a - b) < EPSILON {
        return true;
    }
    false
}

pub fn assert_float(actual: f32, expected: f32) {
    assert_eq!(assert_two_float(actual, expected), true);
}

pub fn max_float(a: f32, b: f32, c: f32) -> f32 {
    let mut max = a;
    if b > max {
        max = b;
    }
    if c > max {
        max = c;
    }
    max
}

pub fn min_float(a: f32, b: f32, c: f32) -> f32 {
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
