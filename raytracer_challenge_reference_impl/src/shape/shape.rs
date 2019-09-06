use core::fmt;

use crate::material::material::Material;
use crate::math::matrix::Matrix;
use crate::math::tuple4d::Tuple4D;
use crate::shape::cube::{Cube, CubeOps};
use crate::shape::cylinder::{Cylinder, CylinderOps};
use crate::shape::groups::{Group, GroupOps};
use crate::shape::plane::{Plane, PlaneOps};
use crate::shape::sphere::{Sphere, SphereOps};
use crate::shape::triangle::{Triangle, TriangleOps};

pub type ShapeIdx = usize;

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeEnum {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Triangle(Triangle),
    Group(Group),
}

#[derive(Clone, PartialEq)]
pub struct Shape<'a> {
    shape: ShapeEnum,
    name: &'a str,
    parent: Option<ShapeIdx>,
    casts_shadow: bool,
}

impl<'a> Shape<'a> {
    pub fn new(shape: ShapeEnum, name: &'a str) -> Shape<'a> {
        Shape {
            shape,
            name,
            parent: None,
            casts_shadow: true,
        }
    }

    pub fn normal_at(&self, p: &Tuple4D) -> Tuple4D {
        let res = match self.shape {
            ShapeEnum::Sphere(ref s) => s.normal_at(p),
            ShapeEnum::Plane(ref plane) => plane.normal_at(p),
            ShapeEnum::Cube(ref cube) => cube.normal_at(p),
            ShapeEnum::Cylinder(ref cylinder) => Cylinder::normal_at(cylinder, p),
            ShapeEnum::Triangle(ref triangle) => Triangle::normal_at(triangle, p),
            ShapeEnum::Group(_) => panic!("Group::normal_at should never be called "),
        };
        res
    }

    pub fn get_material(&self) -> &Material {
        let res = match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_material(),
            ShapeEnum::Plane(ref p) => p.get_material(),
            ShapeEnum::Cube(ref c) => c.get_material(),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_material(),
            ShapeEnum::Triangle(ref triangle) => triangle.get_material(),
            ShapeEnum::Group(_) => panic!("Group::get_material should never be called "),
        };
        res
    }

    pub fn get_material_mut(&mut self) -> &mut Material {
        let res = match self.shape {
            ShapeEnum::Sphere(ref mut s) => s.get_material_mut(),
            ShapeEnum::Plane(ref mut p) => p.get_material_mut(),
            ShapeEnum::Cube(ref mut c) => c.get_material_mut(),
            ShapeEnum::Cylinder(ref mut cylinder) => cylinder.get_material_mut(),
            ShapeEnum::Triangle(ref mut triangle) => triangle.get_material_mut(),
            ShapeEnum::Group(_) => panic!("Group::get_material should never be called "),
        };
        res
    }

    pub fn get_transformation(&self) -> &Matrix {
        let res = match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_transformation(),
            ShapeEnum::Plane(ref p) => p.get_transformation(),
            ShapeEnum::Cube(ref c) => c.get_transformation(),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_transformation(),
            ShapeEnum::Triangle(ref triangle) => triangle.get_transformation(),
            ShapeEnum::Group(ref group) => group.get_transformation(),
        };
        res
    }

    pub fn get_inverse_transformation(&self) -> &Matrix {
        let res = match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_inverse_transformation(),
            ShapeEnum::Plane(ref p) => p.get_inverse_transformation(),
            ShapeEnum::Cube(ref c) => c.get_inverse_transformation(),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_inverse_transformation(),
            ShapeEnum::Triangle(ref triangle) => triangle.get_inverse_transformation(),
            ShapeEnum::Group(ref group) => group.get_inverse_transformation(),
        };
        res
    }

    pub fn get_shape(&self) -> &ShapeEnum {
        &self.shape
    }

    pub fn get_name(&self) -> &'a str {
        &self.name
    }

    pub fn get_casts_shadow(&self) -> bool {
        self.casts_shadow
    }

    pub fn set_casts_shadow(&mut self, casts_shadow: bool) {
        self.casts_shadow = casts_shadow;
    }
}

impl<'a> fmt::Debug for Shape<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "shape type = {:?},   name = {:?}", self.shape, self.name)
    }
}
