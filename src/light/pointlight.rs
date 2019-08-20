use crate::basics::color::Color;
use crate::light::light::LightOps;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug)]
pub struct PointLight {
    pub position: Tuple4D,
    pub intensity: Color,
}

impl LightOps for PointLight {
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
        unimplemented!()
    }

    fn get_vvec(&self) -> &Tuple4D {
        unimplemented!()
    }

    fn get_samples(&self) -> usize {
        unimplemented!()
    }

    fn get_corner(&self) -> &Tuple4D {
        unimplemented!()
    }

    fn get_usteps(&self) -> usize {
        unimplemented!()
    }

    fn get_vsteps(&self) -> usize {
        unimplemented!()
    }


}

impl PointLight {
    pub fn new(position: Tuple4D, intensity: Color) -> PointLight {
        PointLight { position, intensity }
    }
}

#[cfg(test)]
mod tests {
    use crate::basics::color::ColorOps;
    use crate::math::common::{assert_color, assert_tuple};
    use crate::math::tuple4d::Tuple;

    use super::*;

    // page 84
    #[test]
    fn test_pointlight_new() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Tuple4D::new_point(0.0, 0.0, 0.0);

        let pl = PointLight::new(position, intensity);

        let intensity_expected = Color::new(1.0, 1.0, 1.0);
        let position_expected = Tuple4D::new_point(0.0, 0.0, 0.0);

        assert_color(pl.get_intensity(), &intensity_expected);
        assert_tuple(pl.get_position(), &position_expected);
    }
}
