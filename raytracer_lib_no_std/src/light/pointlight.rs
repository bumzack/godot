use crate::{Color, LightOps, Tuple4D};

#[derive(Clone, Debug)]
pub struct PointLight {
    pub position: Tuple4D,
    pub intensity: Color,
}

impl LightOps for PointLight {
    fn get_intensity(&self) -> &Color {
        &self.intensity
    }

    fn get_position(&self) -> &Tuple4D {
        &self.position
    }
}

impl PointLight {
    pub fn new(position: Tuple4D, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
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
        let position_expected = Tuple4D::new_point(
            0.0,
            0.0,
            0.0
        );

        assert_color(pl.get_intensity(), &intensity_expected);
        assert_tuple(pl.get_position(), &position_expected);
    }
}
