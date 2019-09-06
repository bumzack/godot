use crate::{Color, PointLight, Tuple4D};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
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

    fn intensity_at_point(&self, point: &Tuple4D, world: &World) -> f32;

    fn point_on_light(&self, u: usize, v: usize) -> Tuple4D;
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

    fn intensity_at_point(&self, point: &Tuple4D, world: &World) -> f32 {
        let res = match self {
            LightEnum::PointLight(ref point_light) => point_light.intensity_at_point(point, world),
            LightEnum::AreaLight(ref pl) => pl.intensity_at_point(point, world),
        };
        res
    }

    fn point_on_light(&self, u: usize, v: usize) -> Tuple4D {
        let res = match self {
            LightEnum::PointLight(ref point_light) => point_light.point_on_light(u, v),
            LightEnum::AreaLight(ref area_light) => area_light.point_on_light(u, v),
        };
        res
    }
}

