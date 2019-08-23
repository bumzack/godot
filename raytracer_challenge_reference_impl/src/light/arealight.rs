use crate::basics::color::Color;
use crate::light::light::LightOps;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug)]
pub struct AreaLight {
    position: Tuple4D,
    corner: Tuple4D,
    uvec: Tuple4D,
    vvec: Tuple4D,
    usteps: usize,
    vsteps: usize,
    intensity: Color,
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
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::basics::color::ColorOps;
    use crate::math::common::{assert_color, assert_tuple};
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
