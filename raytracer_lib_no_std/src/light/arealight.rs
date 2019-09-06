// TODO
// use rand::{Rng};
// use rand::rngs::SmallRng;

use crate::basics::color::Color;
use crate::light::light::LightOps;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
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

    //TODO: into kernel


    fn point_on_light(&self, u: usize, v: usize) -> Tuple4D {
        //TODO: can we store the points in the light and return a reference?
        // than we would not have to clone in PointLight

        // TODO: when the light is created, fill a Vec with jitter values and be done with it
        // let mut small_rng = SmallRng::from_entropy();
        let u_idx: f32 = u as f32 ;// + small_rng.gen::<f32>();
        let v_idx: f32 = v as f32; //  + small_rng.gen::<f32>();

        let u_pos = self.get_uvec() * u_idx;
        let v_pos = self.get_vvec() * v_idx;

        self.get_corner() + &(u_pos + v_pos)
    }
}

impl AreaLight {
    pub fn new(corner: Tuple4D, v1: Tuple4D, usteps: usize, v2: Tuple4D, vsteps: usize, intensity: Color) -> AreaLight {
        let uvec = &v1 / usteps ;
        let vvec = &v2 / vsteps ;
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
