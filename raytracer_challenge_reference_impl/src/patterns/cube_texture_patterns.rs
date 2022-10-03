use std::collections::HashMap;

use crate::basics::color::Color;
use crate::math::tuple4d::Tuple4D;
use crate::patterns::{uv_align_check_pattern_at, CubeChecker};
use crate::prelude::{Cube, CubeFace};

#[derive(PartialEq, Debug, Clone)]
pub struct CubeTexturePattern {
    cube_map: HashMap<CubeFace, CubeChecker>,
}

impl CubeTexturePattern {
    pub fn new(cube_map: HashMap<CubeFace, CubeChecker>) -> CubeTexturePattern {
        CubeTexturePattern { cube_map }
    }

    pub fn pattern_at(&self, p: &Tuple4D) -> Color {
        let face = Cube::face_from_point(p);
        let (u, v) = match face {
            CubeFace::LEFT => cube_uv_left(p),
            CubeFace::RIGHT => cube_uv_right(p),
            CubeFace::UP => cube_uv_up(p),
            CubeFace::DOWN => cube_uv_down(p),
            CubeFace::FRONT => cube_uv_front(p),
            CubeFace::BACK => cube_uv_back(p),
        };
        *uv_align_check_pattern_at(self.cube_map.get(&face).unwrap(), u, v)
    }
}

pub fn cube_uv_front(p: &Tuple4D) -> (f64, f64) {
    (((p.x + 1.0).rem_euclid(2.0)) / 2.0, ((p.y + 1.0).rem_euclid(2.0)) / 2.0)
}

pub fn cube_uv_back(p: &Tuple4D) -> (f64, f64) {
    (((1.0 - p.x).rem_euclid(2.0)) / 2.0, ((p.y + 1.0).rem_euclid(2.0)) / 2.0)
}

pub fn cube_uv_left(p: &Tuple4D) -> (f64, f64) {
    let (u, v) = (((p.z - 1.0).rem_euclid(2.0)) / 2.0, ((p.y + 1.0).rem_euclid(2.0)) / 2.0);
    (u, v)
}

pub fn cube_uv_right(p: &Tuple4D) -> (f64, f64) {
    (((1.0 - p.z).rem_euclid(2.0)) / 2.0, ((p.y + 1.0).rem_euclid(2.0)) / 2.0)
}

pub fn cube_uv_up(p: &Tuple4D) -> (f64, f64) {
    (((p.x + 1.0).rem_euclid(2.0)) / 2.0, ((1.0 - p.z).rem_euclid(2.0)) / 2.0)
}

pub fn cube_uv_down(p: &Tuple4D) -> (f64, f64) {
    (((p.x - 1.0).rem_euclid(2.0)) / 2.0, ((p.z + 1.0).rem_euclid(2.0)) / 2.0)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::basics::{Color, ColorOps};
    use crate::math::{assert_color, assert_two_float, Tuple, Tuple4D};
    use crate::patterns::{
        cube_uv_back, cube_uv_down, cube_uv_front, cube_uv_left, cube_uv_right, cube_uv_up, CubeTexturePattern,
    };
    use crate::prelude::{CubeChecker, CubeFace};

    // bonus cube mapping Scenario Outline: UV mapping the front face of a cube
    #[test]
    fn test_cube_mapping_front_face() {
        let p = Tuple4D::new_point(-0.5, 0.5, 1.0);
        let actual = cube_uv_front(&p);
        assert_tuple(actual, (0.25, 0.75));

        let p = Tuple4D::new_point(0.5, -0.5, 1.0);
        let actual = cube_uv_front(&p);
        assert_tuple(actual, (0.75, 0.25));
    }

    // bonus cube mapping Scenario Outline: UV mapping the back face of a cube
    #[test]
    fn test_cube_mapping_back_face() {
        let p = Tuple4D::new_point(0.5, 0.5, -1.0);
        let actual = cube_uv_back(&p);
        assert_tuple(actual, (0.25, 0.75));

        let p = Tuple4D::new_point(-0.5, -0.5, 1.0);
        let actual = cube_uv_back(&p);
        assert_tuple(actual, (0.75, 0.25));
    }

    // bonus cube mapping Scenario Outline: UV mapping the left face of a cube
    #[test]
    fn test_cube_mapping_left_face() {
        let p = Tuple4D::new_point(-1., 0.5, -0.5);
        let actual = cube_uv_left(&p);
        assert_tuple(actual, (0.25, 0.75));

        let p = Tuple4D::new_point(-1., -0.5, 0.5);
        let actual = cube_uv_left(&p);
        assert_tuple(actual, (0.75, 0.25));
    }

    // bonus cube mapping Scenario Outline: UV mapping the right face of a cube
    #[test]
    fn test_cube_mapping_right_face() {
        let p = Tuple4D::new_point(1., 0.5, 0.5);
        let actual = cube_uv_right(&p);
        assert_tuple(actual, (0.25, 0.75));

        let p = Tuple4D::new_point(1., -0.5, -0.5);
        let actual = cube_uv_right(&p);
        assert_tuple(actual, (0.75, 0.25));
    }

    // bonus cube mapping Scenario Outline: UV mapping the up face of a cube
    #[test]
    fn test_cube_mapping_up_face() {
        let p = Tuple4D::new_point(-0.5, 1.0, -0.5);
        let actual = cube_uv_up(&p);
        assert_tuple(actual, (0.25, 0.75));

        let p = Tuple4D::new_point(0.5, 1., 0.5);
        let actual = cube_uv_up(&p);
        assert_tuple(actual, (0.75, 0.25));
    }

    // bonus cube mapping Scenario Outline: UV mapping the down face of a cube
    #[test]
    fn test_cube_mapping_down_face() {
        let p = Tuple4D::new_point(-0.5, -1., 0.5);
        let actual = cube_uv_down(&p);
        assert_tuple(actual, (0.25, 0.75));

        let p = Tuple4D::new_point(0.5, -1.0, -0.5);
        let actual = cube_uv_down(&p);
        assert_tuple(actual, (0.75, 0.25));
    }

    // bonus cube mapping Scenario Outline: Finding the colors on a mapped cube
    #[test]
    fn test_cube_find_colors_on_mapped_cube() {
        let red = Color::new(1.0, 0.0, 0.0);
        let yellow = Color::new(1.0, 1.0, 0.0);
        let brown = Color::new(1.0, 0.5, 0.0);
        let green = Color::new(0.0, 1.0, 0.0);
        let cyan = Color::new(0.0, 1.0, 1.0);
        let blue = Color::new(0.0, 0.0, 1.0);
        let purple = Color::new(1.0, 0.0, 1.0);
        let white = Color::new(1.0, 1.0, 1.0);

        let left = CubeChecker::new(yellow, cyan, red, blue, brown);
        let front = CubeChecker::new(cyan, red, yellow, brown, green);
        let right = CubeChecker::new(red, yellow, purple, green, white);
        let back = CubeChecker::new(green, purple, cyan, white, blue);
        let up = CubeChecker::new(brown, cyan, purple, red, yellow);
        let down = CubeChecker::new(purple, brown, green, blue, white);

        let mut cube_map: HashMap<CubeFace, CubeChecker> = HashMap::new();
        cube_map.insert(CubeFace::LEFT, left);
        cube_map.insert(CubeFace::RIGHT, right);
        cube_map.insert(CubeFace::UP, up);
        cube_map.insert(CubeFace::DOWN, down);
        cube_map.insert(CubeFace::FRONT, front);
        cube_map.insert(CubeFace::BACK, back);

        let cube_map = CubeTexturePattern::new(cube_map);
        let p = Tuple4D::new_point(-1.0, 0.0, 0.0);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &yellow);

        let p = Tuple4D::new_point(-1.0, 0.9, -0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &cyan);

        let p = Tuple4D::new_point(-1.0, 0.9, 0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &red);

        let p = Tuple4D::new_point(-1.0, -0.9, -0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &blue);

        let p = Tuple4D::new_point(-1.0, -0.9, 0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &brown);

        let p = Tuple4D::new_point(0.0, 0.0, 1.0);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &cyan);

        let p = Tuple4D::new_point(-0.9, 0.9, 1.0);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &red);

        let p = Tuple4D::new_point(0.9, 0.9, 1.0);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &yellow);

        let p = Tuple4D::new_point(-0.9, -0.9, 1.0);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &brown);

        let p = Tuple4D::new_point(0.9, -0.9, 1.0);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &green);

        let p = Tuple4D::new_point(1., 0., 0.0);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &red);

        let p = Tuple4D::new_point(1., 0.9, 0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &yellow);

        let p = Tuple4D::new_point(1., 0.9, -0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &purple);

        let p = Tuple4D::new_point(1., -0.9, 0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &green);

        let p = Tuple4D::new_point(1., -0.9, -0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &white);

        let p = Tuple4D::new_point(0., 0., -1.);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &green);

        let p = Tuple4D::new_point(0.9, 0.9, -1.);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &purple);

        let p = Tuple4D::new_point(-0.9, 0.9, -1.);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &cyan);

        let p = Tuple4D::new_point(0.9, -0.9, -1.);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &white);

        let p = Tuple4D::new_point(-0.9, -0.9, -1.0);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &blue);

        let p = Tuple4D::new_point(0., 1., 0.);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &brown);

        let p = Tuple4D::new_point(-0.9, 1., -0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &cyan);

        let p = Tuple4D::new_point(0.9, 1., -0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &purple);

        let p = Tuple4D::new_point(-0.9, 1., 0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &red);

        let p = Tuple4D::new_point(0.9, 1., 0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &yellow);

        let p = Tuple4D::new_point(0., -1., 0.);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &purple);

        let p = Tuple4D::new_point(-0.9, -1., 0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &brown);

        let p = Tuple4D::new_point(0.9, -1., 0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &green);

        let p = Tuple4D::new_point(-0.9, -1., -0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &blue);

        let p = Tuple4D::new_point(0.9, -1., -0.9);
        let actual = cube_map.pattern_at(&p);
        assert_color(&actual, &white);
    }

    fn assert_tuple(actual: (f64, f64), expected: (f64, f64)) {
        assert_two_float(actual.0, expected.0);
        assert_two_float(actual.0, expected.0);
    }
}
