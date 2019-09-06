use core::fmt;

use crate::{Material, Matrix, Ray, Sphere, SphereOps, Tuple4D};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub enum ShapeEnum {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Triangle(Triangle),
    //  Group(Group),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Shape<'a> {
    shape: ShapeEnum,
    name: &'a str,
    parent: Option<ShapeIdx>,
    casts_shadow: bool,
}


pub trait ShapeOps {
    fn intersect(r: &Ray) -> ([f32; 2], usize);

    fn set_transformation(&mut self, m: Matrix);
    fn get_transformation(&self) -> &Matrix;
    fn get_inverse_transformation(&self) -> &Matrix;

    fn normal_at(&self, p: &Tuple4D) -> Tuple4D;

    fn set_material(&mut self, m: Material);
    fn get_material(&self) -> &Material;
    fn get_material_mut(&mut self) -> &mut Material;
}

impl<'a> ShapeOps for Shape<'a> {
    fn normal_at(&self, p: &Tuple4D) -> Tuple4D {
       match self.shape {
            ShapeEnum::Sphere(ref s) => s.normal_at(p),
            ShapeEnum::Plane(ref plane) => plane.normal_at(p),
            ShapeEnum::Cube(ref cube) => cube.normal_at(p),
            ShapeEnum::Cylinder(ref cylinder) => Cylinder::normal_at(cylinder, p),
            ShapeEnum::Triangle(ref triangle) => Triangle::normal_at(triangle, p),
            ShapeEnum::Group(_) => panic!("Group::normal_at should never be called "),
        }
    }

    fn get_material(&self) -> &Material {
          match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_material(),
            ShapeEnum::Plane(ref p) => p.get_material(),
            ShapeEnum::Cube(ref c) => c.get_material(),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_material(),
            ShapeEnum::Triangle(ref triangle) => triangle.get_material(),
            ShapeEnum::Group(_) => panic!("Group::get_material should never be called "),
        }
    }

    fn get_material_mut(&mut self) -> &mut Material {
        match self.shape {
            ShapeEnum::Sphere(ref mut s) => s.get_material_mut(),
            ShapeEnum::Plane(ref mut p) => p.get_material_mut(),
            ShapeEnum::Cube(ref mut c) => c.get_material_mut(),
            ShapeEnum::Cylinder(ref mut cylinder) => cylinder.get_material_mut(),
            ShapeEnum::Triangle(ref mut triangle) => triangle.get_material_mut(),
            ShapeEnum::Group(_) => panic!("Group::get_material should never be called "),
        }
    }

    fn get_transformation(&self) -> &Matrix {
        match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_transformation(),
            ShapeEnum::Plane(ref p) => p.get_transformation(),
            ShapeEnum::Cube(ref c) => c.get_transformation(),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_transformation(),
            ShapeEnum::Triangle(ref triangle) => triangle.get_transformation(),
            ShapeEnum::Group(ref group) => group.get_transformation(),
        }
    }

    fn get_inverse_transformation(&self) -> &Matrix {
        match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_inverse_transformation(),
            ShapeEnum::Plane(ref p) => p.get_inverse_transformation(),
            ShapeEnum::Cube(ref c) => c.get_inverse_transformation(),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_inverse_transformation(),
            ShapeEnum::Triangle(ref triangle) => triangle.get_inverse_transformation(),
            ShapeEnum::Group(ref group) => group.get_inverse_transformation(),
        }
    }

    fn set_transformation(&mut self, m: Matrix) {
        match self.shape {
            ShapeEnum::Sphere(ref s) => s.set_transformation(m),
            ShapeEnum::Plane(ref p) => p.set_transformation(m),
            ShapeEnum::Cube(ref c) => c.get_inverse_transformation(m),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.set_transformation(m),
            ShapeEnum::Triangle(ref triangle) => triangle.set_transformation(m),
            ShapeEnum::Group(ref group) => group.set_transformation(m),
        };
    }

    fn set_material(&mut self, m: Material) {
        match self.shape {
            ShapeEnum::Sphere(ref s) => s.set_material(m),
            ShapeEnum::Plane(ref p) => p.set_material(m),
            ShapeEnum::Cube(ref c) => c.set_material(m),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.set_material(m),
            ShapeEnum::Triangle(ref triangle) => triangle.set_material(m),
            ShapeEnum::Group(ref group) => group.set_material(m),
        };
    }
}

impl<'a> Shape<'a> {
    fn new(shape: ShapeEnum, name: &'a str) -> Shape<'a> {
        Shape {
            shape,
            name,
            parent: None,
            casts_shadow: true,
        }
    }

    fn get_shape(&self) -> &ShapeEnum {
        &self.shape
    }

    fn get_name(&self) -> &'a str {
        self.name
    }

    fn get_casts_shadow(&self) -> bool {
        self.casts_shadow
    }

    fn set_casts_shadow(&mut self, casts_shadow: bool) {
        self.casts_shadow = casts_shadow;
    }
}

impl<'a> fmt::Debug for Shape<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "shape type = {:?},   name = {:?}", self.shape, self.name)
    }
}

