#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

use math::prelude::*;

use crate::{Color, LightOps};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
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
        1
    }

    fn get_corner(&self) -> &Tuple4D {
        unimplemented!()
    }

    fn get_usteps(&self) -> usize {
        1
    }

    fn get_vsteps(&self) -> usize {
        1
    }

    // TODO: clone :-(
    fn point_on_light(&self, _u: usize, _v: usize) -> Tuple4D {
        self.position.clone()
    }
}

impl PointLight {
    pub fn new(position: Tuple4D, intensity: Color) -> PointLight {
        PointLight { position, intensity }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_color, assert_tuple, ColorOps, Tuple};

    use super::*;

    // page 84
    #[test]
    fn test_pointlight_new() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Tuple4D::new_point(0.0, 0.0, 0.0);

        let pl = PointLight::new(position, intensity);

        let intensity_expected: Color = Color::new(1.0, 1.0, 1.0);
        let position_expected = Tuple4D::new_point(0.0, 0.0, 0.0);

        assert_color(pl.get_intensity(), &intensity_expected);
        assert_tuple(pl.get_position(), &position_expected);
    }
}
