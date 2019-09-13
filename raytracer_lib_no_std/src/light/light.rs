use crate::{AreaLight, Color, PointLight};
use math::prelude::*;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub enum Light {
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

    fn point_on_light(&self, u: usize, v: usize) -> Tuple4D;

    // moved to Cpu/Cuda Kernel
    //    fn intensity_at_point(&self, point: &Tuple4D, world: &World) -> f32;
    //
}

impl LightOps for Light {
    fn get_intensity(&self) -> &Color {
        match self {
            Light::PointLight(ref pl) => pl.get_intensity(),
            Light::AreaLight(ref al) => al.get_intensity(),
        }
    }

    fn set_intensity(&mut self, intensity: Color) {
        match self {
            Light::PointLight(ref mut pl) => pl.set_intensity(intensity),
            Light::AreaLight(ref mut al) => al.set_intensity(intensity),
        };
    }

    fn get_position(&self) -> &Tuple4D {
        match self {
            Light::PointLight(ref pl) => pl.get_position(),
            Light::AreaLight(ref al) => al.get_position(),
        }
    }

    fn set_position(&mut self, pos: Tuple4D) {
        match self {
            Light::PointLight(ref mut pl) => pl.set_position(pos),
            Light::AreaLight(ref mut al) => al.set_position(pos),
        };
    }

    fn get_uvec(&self) -> &Tuple4D {
        let res = match self {
            Light::PointLight(ref pl) => pl.get_uvec(),
            Light::AreaLight(ref al) => al.get_uvec(),
        };
        res
    }

    fn get_vvec(&self) -> &Tuple4D {
        let res = match self {
            Light::PointLight(ref pl) => pl.get_vvec(),
            Light::AreaLight(ref al) => al.get_vvec(),
        };
        res
    }

    fn get_samples(&self) -> usize {
        let res = match self {
            Light::PointLight(ref pl) => pl.get_samples(),
            Light::AreaLight(ref al) => al.get_samples(),
        };
        res
    }

    fn get_corner(&self) -> &Tuple4D {
        let res = match self {
            Light::PointLight(ref pl) => pl.get_corner(),
            Light::AreaLight(ref al) => al.get_corner(),
        };
        res
    }

    fn get_usteps(&self) -> usize {
        let res = match self {
            Light::PointLight(ref pl) => pl.get_usteps(),
            Light::AreaLight(ref al) => al.get_usteps(),
        };
        res
    }

    fn get_vsteps(&self) -> usize {
        let res = match self {
            Light::PointLight(ref pl) => pl.get_vsteps(),
            Light::AreaLight(ref al) => al.get_vsteps(),
        };
        res
    }

    fn point_on_light(&self, u: usize, v: usize) -> Tuple4D {
        match self {
            Light::PointLight(ref pl) => pl.point_on_light(u, v),
            Light::AreaLight(ref al) => al.point_on_light(u, v),
        }
    }
}
