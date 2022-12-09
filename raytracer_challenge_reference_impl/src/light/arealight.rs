extern crate rand;

use crate::basics::color::Color;
use crate::light::light::LightOps;
use crate::light::Sequence;
use crate::math::tuple4d::Tuple4D;
use crate::world::world::{World, WorldOps};
use crate::DEBUG;

#[derive(Clone, Debug)]
pub struct AreaLight {
    position: Tuple4D,
    corner: Tuple4D,
    uvec: Tuple4D,
    vvec: Tuple4D,
    usteps: usize,
    vsteps: usize,
    intensity: Color,
    jitter_sequence: Sequence,
}

impl LightOps for AreaLight {
    fn get_intensity(&self) -> &Color {
        &self.intensity
    }

    fn set_intensity(&mut self, intensity: Color) {
        self.intensity = intensity;
    }

    fn get_position(&self) -> &Tuple4D {
        &self.position
    }

    fn set_position(&mut self, pos: Tuple4D) {
        self.position = pos;
    }

    fn get_uvec(&self) -> &Tuple4D {
        &self.uvec
    }
    fn get_vvec(&self) -> &Tuple4D {
        &self.vvec
    }

    fn get_samples(&self) -> usize {
        self.usteps * self.vsteps
    }

    fn get_corner(&self) -> &Tuple4D {
        &self.corner
    }

    fn get_usteps(&self) -> usize {
        self.usteps
    }

    fn get_vsteps(&self) -> usize {
        self.vsteps
    }

    fn intensity_at_point(&mut self, point: &Tuple4D, world: &World) -> f64 {
        let mut total = 0.0;

        if DEBUG {
            println!("light.get_usteps()  = {:?}", self.get_usteps());
            println!("light.get_vsteps()  = {:?}", self.get_vsteps());
        }
        for v in 0..self.get_vsteps() {
            for u in 0..self.get_usteps() {
                let light_position = self.point_on_light(u, v);
                if !World::is_shadowed(world, &light_position, point) {
                    total += 1.0;
                }
            }
        }

        total / self.get_samples() as f64
    }

    fn point_on_light(&mut self, u: usize, v: usize) -> Tuple4D {
        let jitter_by_x = self.jitter_sequence.next();
        let jitter_by_y = self.jitter_sequence.next();

        let u_pos = self.get_uvec() * (u as f64 + jitter_by_x);
        let v_pos = self.get_vvec() * (v as f64 + jitter_by_y);

        self.get_corner() + &(u_pos + v_pos)
    }
}

impl AreaLight {
    pub fn new(
        corner: Tuple4D,
        v1: Tuple4D,
        usteps: usize,
        v2: Tuple4D,
        vsteps: usize,
        intensity: Color,
        jitter_sequence: Sequence,
    ) -> Self {
        let uvec = v1 / usteps;
        let vvec = v2 / vsteps;
        let position = corner + (v1 / 2.0) + (v2 / 2.0);

        AreaLight {
            position,
            corner,
            uvec,
            vvec,
            usteps,
            vsteps,
            intensity,
            jitter_sequence,
        }
    }

    pub fn set_usteps(&mut self, usteps: usize) {
        self.usteps = usteps;
    }

    pub fn set_vsteps(&mut self, vsteps: usize) {
        self.vsteps = vsteps;
    }
}

#[cfg(test)]
mod tests {
    use crate::basics::color::ColorOps;
    use crate::math::common::assert_tuple;
    use crate::math::tuple4d::Tuple;

    use super::*;

    // bonus chapter:   Scenario: Creating an area light
    #[test]
    fn test_arealight_new() {
        let corner = Tuple4D::new_point(0.0, 0.0, 0.0);
        let v1 = Tuple4D::new_vector(2.0, 0.0, 0.0);
        let v2 = Tuple4D::new_vector(0.0, 0.0, 1.0);

        let usteps = 4;
        let vsteps = 2;

        let intensity = Color::new(1.0, 1.0, 1.0);

        let arealight = AreaLight::new(corner, v1, usteps, v2, vsteps, intensity, Sequence::new(vec![]));

        let _corner_expected = Tuple4D::new_point(0.0, 0.0, 0.0);
        let uvec_expected = Tuple4D::new_vector(0.5, 0.0, 0.0);
        let vvec_expected = Tuple4D::new_vector(0.0, 0.0, 0.5);

        let samples_expected = 8;
        let position_expected = Tuple4D::new_point(1.0, 0.0, 0.5);

        println!("vvec_expected = {:?}", vvec_expected);
        println!("get_vvec = {:?}", arealight.get_vvec());

        assert_tuple(&position_expected, arealight.get_position());
        assert_tuple(&uvec_expected, arealight.get_uvec());
        assert_tuple(&vvec_expected, arealight.get_vvec());
        assert_eq!(samples_expected, arealight.get_samples());
    }
}
