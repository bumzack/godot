use core::fmt;

use crate::prelude::*;

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
pub struct Shape {
    shape: ShapeEnum,
    parent: Option<ShapeIdx>,
    casts_shadow: bool,
}

pub trait ShapeOps {
    fn set_transformation(&mut self, m: Matrix);
    fn get_transformation(&self) -> &Matrix;
    fn get_inverse_transformation(&self) -> &Matrix;

    fn normal_at(&self, p: &Tuple4D) -> Tuple4D;
    fn local_normal_at(&self, local_point: &Tuple4D) -> Tuple4D;

    fn set_material(&mut self, m: Material);
    fn get_material(&self) -> &Material;
    fn get_material_mut(&mut self) -> &mut Material;
}

impl ShapeOps for Shape {
    fn normal_at(&self, world_point: &Tuple4D) -> Tuple4D {
        let object_point = self.get_inverse_transformation() * world_point;
        let local_normal = match self.shape {
            ShapeEnum::Sphere(ref s) => s.local_normal_at(&object_point),
            ShapeEnum::Plane(ref plane) => plane.local_normal_at(&object_point),
            ShapeEnum::Cube(ref cube) => cube.local_normal_at(&object_point),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.local_normal_at(&object_point),
            ShapeEnum::Triangle(ref triangle) => triangle.local_normal_at(&object_point),
            ShapeEnum::Group(_) => panic!("Group::normal_at should never be called "),
        };
        let mut world_normal = &Matrix::transpose(self.get_inverse_transformation()) * &local_normal;
        world_normal.w = 0.0;
        Tuple4D::normalize(&world_normal)
    }

    fn get_material(&self) -> &Material {
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

    fn get_material_mut(&mut self) -> &mut Material {
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

    fn get_transformation(&self) -> &Matrix {
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

    fn get_inverse_transformation(&self) -> &Matrix {
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

    fn set_transformation(&mut self, m: Matrix) {
        unimplemented!()
    }

    fn local_normal_at(&self, local_point: &Tuple4D) -> Tuple4D {
        unreachable!("shoudl never get here ");
    }

    fn set_material(&mut self, m: Material) {
        unimplemented!()
    }
}

impl Shape {
    pub fn new(shape: ShapeEnum) -> Shape {
        Shape {
            shape,
            parent: None,
            casts_shadow: true,
        }
    }
    pub fn get_shape(&self) -> &ShapeEnum {
        &self.shape
    }

    pub fn get_casts_shadow(&self) -> bool {
        self.casts_shadow
    }

    pub fn set_casts_shadow(&mut self, casts_shadow: bool) {
        self.casts_shadow = casts_shadow;
    }
}

impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "shape type = {:?}", self.shape)
    }
}
