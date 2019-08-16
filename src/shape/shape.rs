use crate::material::material::Material;
use crate::math::matrix::Matrix;
use crate::math::tuple4d::Tuple4D;
use crate::shape::cube::{Cube, CubeOps};
use crate::shape::plane::{Plane, PlaneOps};
use crate::shape::sphere::{Sphere, SphereOps};

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeEnum {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Shape<'a> {
    shape: ShapeEnum,
    name: &'a str,
}

impl<'a> Shape<'a> {
    pub fn new(shape: ShapeEnum, name: &'a str) -> Shape<'a> {
        Shape { shape, name }
    }

    pub fn normal_at(&self, p: &Tuple4D) -> Tuple4D {
        let res = match self.shape {
            ShapeEnum::Sphere(ref s) => s.normal_at(p),
            ShapeEnum::Plane(ref plane) => plane.normal_at(p),
            ShapeEnum::Cube(ref cube) => cube.normal_at(p),
        };
        res
    }

    pub fn get_material(&self) -> &Material {
        let res = match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_material(),
            ShapeEnum::Plane(ref p) => p.get_material(),
            ShapeEnum::Cube(ref c) => c.get_material(),
        };
        res
    }

    pub fn get_material_mut(&mut self) -> &mut Material {
        let res = match self.shape {
            ShapeEnum::Sphere(ref mut s) => s.get_material_mut(),
            ShapeEnum::Plane(ref mut p) => p.get_material_mut(),
            ShapeEnum::Cube(ref mut c) => c.get_material_mut(),
        };
        res
    }

    pub fn get_transformation(&self) -> &Matrix {
        let res = match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_transformation(),
            ShapeEnum::Plane(ref p) => p.get_transformation(),
            ShapeEnum::Cube(ref c) => c.get_transformation(),
        };
        res
    }

    pub fn get_inverse_transformation(&self) -> &Matrix {
        let res = match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_inverse_transformation(),
            ShapeEnum::Plane(ref p) => p.get_inverse_transformation(),
            ShapeEnum::Cube(ref c) => c.get_inverse_transformation(),
        };
        res
    }

    pub fn get_shape(&self) -> &ShapeEnum {
        &self.shape
    }

    pub fn get_name(&self) -> &'a str {
        &self.name
    }
}
