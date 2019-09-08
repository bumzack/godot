extern crate rand;

use rand::Rng;

use crate::basics::color::Color;
use crate::light::light::LightOps;
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
    //    jitter_sequence: Sequence,
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

    fn intensity_at_point(&self, point: &Tuple4D, world: &World) -> f32 {
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

        total / self.get_samples() as f32
    }

    fn point_on_light(&self, u: usize, v: usize) -> Tuple4D {
        //TODO: can we store the points in the light and return a reference?
        // than we would not have to clone in PointLight

        // TODO: when the light is created, fill a Vec with jitter values and be done with it
        let mut rng = rand::thread_rng();
        let u_idx: f32 = u as f32 + rng.gen::<f32>();
        let v_idx: f32 = v as f32 + rng.gen::<f32>();

        let u_pos = self.get_uvec() * u_idx;
        let v_pos = self.get_vvec() * v_idx;

        self.get_corner() + &(u_pos + v_pos)
    }
}

impl AreaLight {
    pub fn new(corner: Tuple4D, v1: Tuple4D, usteps: usize, v2: Tuple4D, vsteps: usize, intensity: Color) -> AreaLight {
        let uvec = &v1 / usteps;
        let vvec = &v2 / vsteps;
        let position = &corner + &(&v1 / 2.0) + (&v2 / 2.0);

        AreaLight {
            position,
            corner,
            uvec,
            vvec,
            usteps,
            vsteps,
            intensity,
            //            jitter_sequence: Sequence::new(usteps * vsteps),
        }
    }

    //    pub fn set_test_sequence(&mut self, d: Vec<f32>) {
    //        self.jitter_sequence.clear();
    //        for &elem in d.iter() {
    //            self.jitter_sequence.add(elem);
    //        }
    //    }

    //    pub fn next(&self) -> f32 {
    //        self.jitter_sequence.next()
    //    }
}

// Scenario: A number generator returns a cyclic sequence of numbers
//
//#[derive(Clone, Debug)]
//struct Sequence {
//    data: Vec<f32>,
//    act_idx: usize,
//}
//
//impl Sequence {
//    fn new(cnt: usize) -> Sequence {
//        let mut s = Sequence {
//            data: Vec::new(),
//            act_idx: 0,
//        };
//
//        let mut rng = rand::thread_rng();
//
//        for &mut mut d in s.data.iter_mut() {
//            d = rng.gen();
//        }
//
//        s
//    }
//
//    fn add(&mut self, elem: f32) {
//        self.data.push(elem);
//    }
//
//    fn next(&mut self) -> f32 {
//        let elem = self.data[self.act_idx];
//        self.act_idx += 1;
//        if self.act_idx >= self.data.len() {
//            self.act_idx = 0;
//        }
//        elem
//    }
//
//    fn clear(&mut self) {
//        self.data.clear();
//    }
//}

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

        let arealight = AreaLight::new(corner, v1, usteps, v2, vsteps, intensity);

        let corner_expected = Tuple4D::new_point(0.0, 0.0, 0.0);
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
