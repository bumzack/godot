use crate::material::material::Material;
use crate::math::matrix::Matrix;
use crate::math::tuple4d::Tuple4D;
use crate::shape::cube::{Cube, CubeOps};
use crate::shape::plane::{Plane, PlaneOps};
use crate::shape::sphere::{Sphere, SphereOps};

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
}

impl Shape {
    pub fn normal_at(&self, p: &Tuple4D) -> Tuple4D {
        let res = match self {
            Shape::Sphere(ref s) => s.normal_at(p),
            Shape::Plane(ref plane) => plane.normal_at(p),
            Shape::Cube(ref cube) => cube.normal_at(p),
        };
        res
    }

    pub fn get_material(&self) -> &Material {
        let res = match self {
            Shape::Sphere(ref s) => s.get_material(),
            Shape::Plane(ref p) => p.get_material(),
            Shape::Cube(ref c) => c.get_material(),
        };
        res
    }

    pub fn get_material_mut(&mut self) -> &mut Material {
        let res = match self {
            Shape::Sphere(ref mut s) => s.get_material_mut(),
            Shape::Plane(ref mut p) => p.get_material_mut(),
            Shape::Cube(ref mut c) => c.get_material_mut(),
        };
        res
    }

    pub fn get_transformation(&self) -> &Matrix {
        let res = match self {
            Shape::Sphere(ref s) => s.get_transformation(),
            Shape::Plane(ref p) => p.get_transformation(),
            Shape::Cube(ref c) => c.get_transformation(),
        };
        res
    }

    pub fn get_inverse_transformation(&self) -> &Matrix {
        let res = match self {
            Shape::Sphere(ref s) => s.get_inverse_transformation(),
            Shape::Plane(ref p) => p.get_inverse_transformation(),
            Shape::Cube(ref c) => c.get_inverse_transformation(),
        };
        res
    }
}
