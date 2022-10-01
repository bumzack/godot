use crate::basics::color::Color;
 use crate::math::tuple4d::Tuple4D;
use crate::patterns::{uv_pattern_at, Checker};

#[derive(PartialEq, Debug, Clone)]
pub struct PlaneTexturePattern {
    checker: Checker,
}

impl PlaneTexturePattern {
    pub fn new(checker: Checker) -> PlaneTexturePattern {
        PlaneTexturePattern { checker }
    }

    pub fn pattern_at(&self, p: &Tuple4D) -> Color {
        let (u, v) = planar_map(p);
        uv_pattern_at(&self.checker, u, v)
    }
}

pub fn planar_map(p: &Tuple4D) -> (f64, f64) {
    (p.x.rem_euclid(1.0), p.z.rem_euclid(1.0))
}

#[cfg(test)]
mod tests {
    use crate::math::{assert_two_float, Tuple};

    use super::*;

    // bonus planar mapping  Scenario Outline: Using a planar mapping on a 3D point
    #[test]
    fn test_planar_mapping() {
        let p = Tuple4D::new_point(0.25, 0.0, 0.5);
        let actual = planar_map(&p);
        assert_tuple(actual, (0.25, 0.5));

        let p = Tuple4D::new_point(0.25, 0.0, -0.25);
        let actual = planar_map(&p);
        assert_tuple(actual, (0.25, 0.75));

        let p = Tuple4D::new_point(0.25, 0.5, -0.25);
        let actual = planar_map(&p);
        assert_tuple(actual, (0.25, 0.75));

        let p = Tuple4D::new_point(1.25, 0.0, 0.5);
        let actual = planar_map(&p);
        assert_tuple(actual, (0.25, 0.5));

        let p = Tuple4D::new_point(1.0, 0.0, -1.0);
        let actual = planar_map(&p);
        assert_tuple(actual, (0.0, 0.0));

        let p = Tuple4D::new_point(0.0, 0.0, 0.0);
        let actual = planar_map(&p);
        assert_tuple(actual, (0.0, 0.0));
    }

    fn assert_tuple(actual: (f64, f64), expected: (f64, f64)) {
        assert_two_float(actual.0, expected.0);
        assert_two_float(actual.0, expected.0);
    }
}
