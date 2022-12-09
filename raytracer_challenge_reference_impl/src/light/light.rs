use crate::basics::color::Color;
use crate::light::arealight::AreaLight;
use crate::light::pointlight::PointLight;
use crate::math::tuple4d::Tuple4D;
use crate::world::world::World;

#[derive(Clone, Debug)]
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

    fn intensity_at_point(&mut self, point: &Tuple4D, world: &World) -> f64;

    fn point_on_light(&mut self, u: usize, v: usize) -> Tuple4D;
}

impl LightOps for Light {
    fn get_intensity(&self) -> &Color {
        match self {
            Light::PointLight(ref pl) => pl.get_intensity(),
            Light::AreaLight(ref pl) => pl.get_intensity(),
        }
    }

    fn set_intensity(&mut self, intensity: Color) {
        match self {
            Light::PointLight(ref mut pl) => pl.set_intensity(intensity),
            Light::AreaLight(ref mut pl) => pl.set_intensity(intensity),
        };
    }

    fn get_position(&self) -> &Tuple4D {
        let res = match self {
            Light::PointLight(ref pl) => pl.get_position(),
            Light::AreaLight(ref pl) => pl.get_position(),
        };
        res
    }

    fn set_position(&mut self, pos: Tuple4D) {
        match self {
            Light::PointLight(ref mut pl) => pl.set_position(pos),
            Light::AreaLight(ref mut pl) => pl.set_position(pos),
        };
    }

    fn get_uvec(&self) -> &Tuple4D {
        match self {
            Light::PointLight(ref pl) => pl.get_uvec(),
            Light::AreaLight(ref pl) => pl.get_uvec(),
        }
    }

    fn get_vvec(&self) -> &Tuple4D {
        match self {
            Light::PointLight(ref pl) => pl.get_vvec(),
            Light::AreaLight(ref pl) => pl.get_vvec(),
        }
    }

    fn get_samples(&self) -> usize {
        match self {
            Light::PointLight(ref pl) => pl.get_samples(),
            Light::AreaLight(ref pl) => pl.get_samples(),
        }
    }

    fn get_corner(&self) -> &Tuple4D {
        match self {
            Light::PointLight(ref pl) => pl.get_corner(),
            Light::AreaLight(ref pl) => pl.get_corner(),
        }
    }

    fn get_usteps(&self) -> usize {
        match self {
            Light::PointLight(ref pl) => pl.get_usteps(),
            Light::AreaLight(ref pl) => pl.get_usteps(),
        }
    }

    fn get_vsteps(&self) -> usize {
        match self {
            Light::PointLight(ref pl) => pl.get_vsteps(),
            Light::AreaLight(ref pl) => pl.get_vsteps(),
        }
    }

    fn intensity_at_point(&mut self, point: &Tuple4D, world: &World) -> f64 {
        match self {
            Light::PointLight(ref mut point_light) => point_light.intensity_at_point(point, world),
            Light::AreaLight(ref mut pl) => pl.intensity_at_point(point, world),
        }
    }

    fn point_on_light(&mut self, u: usize, v: usize) -> Tuple4D {
        match self {
            Light::PointLight(ref mut point_light) => point_light.point_on_light(u, v),
            Light::AreaLight(ref mut area_light) => area_light.point_on_light(u, v),
        }
    }
}
