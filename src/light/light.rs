use crate::basics::color::Color;
use crate::light::arealight::AreaLight;
use crate::light::pointlight::PointLight;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug)]
pub enum LightEnum {
    PointLight(PointLight),
    AreaLight(AreaLight),
}

pub trait LightOps {
    fn get_intensity(&self) -> &Color;
    fn set_intensity(&mut self, intensity: Color);

    fn get_position(&self) -> &Tuple4D;
    fn set_position(&mut self, pos: Tuple4D);

    fn get_uvec(&self) -> &Tuple4D;
    fn get_vvec(&self) -> &Tuple4D;
    fn get_samples(&self) -> usize;
    fn get_corner(&self) -> &Tuple4D;

    fn get_usteps(&self) -> usize;
    fn get_vsteps(&self) -> usize;
}

impl LightOps for LightEnum {
    fn get_intensity(&self) -> &Color {
        let res = match self {
            LightEnum::PointLight(ref pl) => pl.get_intensity(),
            LightEnum::AreaLight(ref pl) => pl.get_intensity(),
        };
        res
    }

    fn set_intensity(&mut self, intensity: Color) {
        let res = match self {
            LightEnum::PointLight(ref mut pl) => pl.set_intensity(intensity),
            LightEnum::AreaLight(ref mut pl) => pl.set_intensity(intensity),
        };
    }

    fn get_position(&self) -> &Tuple4D {
        let res = match self {
            LightEnum::PointLight(ref pl) => pl.get_position(),
            LightEnum::AreaLight(ref pl) => pl.get_position(),
        };
        res
    }

    fn set_position(&mut self, pos: Tuple4D) {
        let res = match self {
            LightEnum::PointLight(ref mut pl) => pl.set_position(pos),
            LightEnum::AreaLight(ref mut pl) => pl.set_position(pos),
        };
    }

    fn get_uvec(&self) -> &Tuple4D {
        let res = match self {
            LightEnum::PointLight(ref pl) => pl.get_uvec(),
            LightEnum::AreaLight(ref pl) => pl.get_uvec(),
        };
        res
    }

    fn get_vvec(&self) -> &Tuple4D {
        let res = match self {
            LightEnum::PointLight(ref pl) => pl.get_vvec(),
            LightEnum::AreaLight(ref pl) => pl.get_vvec(),
        };
        res
    }

    fn get_samples(&self) -> usize {
        let res = match self {
            LightEnum::PointLight(ref pl) => pl.get_samples(),
            LightEnum::AreaLight(ref pl) => pl.get_samples(),
        };
        res
    }

    fn get_corner(&self) -> &Tuple4D {
        let res = match self {
            LightEnum::PointLight(ref pl) => pl.get_corner(),
            LightEnum::AreaLight(ref pl) => pl.get_corner(),
        };
        res
    }

    fn get_usteps(&self) -> usize {
        let res = match self {
            LightEnum::PointLight(ref pl) => pl.get_usteps(),
            LightEnum::AreaLight(ref pl) => pl.get_usteps(),
        };
        res
    }

    fn get_vsteps(&self) -> usize {
        let res = match self {
            LightEnum::PointLight(ref pl) => pl.get_vsteps(),
            LightEnum::AreaLight(ref pl) => pl.get_vsteps(),
        };
        res
    }
}
