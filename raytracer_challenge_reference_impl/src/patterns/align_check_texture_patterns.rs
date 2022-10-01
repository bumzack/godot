use crate::basics::color::Color;
 use crate::math::tuple4d::Tuple4D;

#[derive(PartialEq, Debug, Clone)]
pub struct AlignCheckTexturePattern {
    checker: CubeChecker,
}

impl AlignCheckTexturePattern {
    pub fn new(checker: CubeChecker) -> AlignCheckTexturePattern {
        AlignCheckTexturePattern { checker }
    }

    pub fn pattern_at(&self, p: &Tuple4D) -> Color {
        let (u, v) = (p.x.rem_euclid(1.0), p.z.rem_euclid(1.0));
        *uv_align_check_pattern_at(&self.checker, u, v)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CubeChecker {
    main: Color,
    ul: Color,
    ur: Color,
    bl: Color,
    br: Color,
}

impl CubeChecker {
    pub fn new(main: Color, ul: Color, ur: Color, bl: Color, br: Color) -> CubeChecker {
        CubeChecker { main, ul, ur, bl, br }
    }
}

pub fn uv_align_check_pattern_at(pattern: &CubeChecker, u: f64, v: f64) -> &Color {
    if v > 0.8 {
        if u < 0.2 {
            return &pattern.ul;
        }
        if u > 0.8 {
            return &pattern.ur;
        }
    } else if v < 0.2 {
        if u < 0.2 {
            return &pattern.bl;
        }
        if u > 0.8 {
            return &pattern.br;
        }
    }
    &pattern.main
}

#[cfg(test)]
mod tests {
    use crate::basics::ColorOps;

    use crate::math::assert_color;

    use super::*;

    // bonus cube mapping  Scenario Outline: Layout of the "align check" pattern
    #[test]
    fn test_cube_mapping() {
        let main = Color::new(1.0, 1.0, 1.0);
        let ul = Color::new(1.0, 0.0, 0.0);
        let ur = Color::new(1.0, 1.0, 0.0);
        let bl = Color::new(0.0, 1.0, 0.0);
        let br = Color::new(0.0, 1.0, 1.0);

        let pattern = CubeChecker::new(main, ul, ur, bl, br);

        let (u, v) = (0.5, 0.5);
        let c = uv_align_check_pattern_at(&pattern, u, v);
        assert_color(&c, &main);

        let (u, v) = (0.1, 0.9);
        let c = uv_align_check_pattern_at(&pattern, u, v);
        assert_color(&c, &ul);

        let (u, v) = (0.9, 0.9);
        let c = uv_align_check_pattern_at(&pattern, u, v);
        assert_color(&c, &ur);

        let (u, v) = (0.1, 0.1);
        let c = uv_align_check_pattern_at(&pattern, u, v);
        assert_color(&c, &bl);

        let (u, v) = (0.9, 0.1);
        let c = uv_align_check_pattern_at(&pattern, u, v);
        assert_color(&c, &br);
    }
}
