use core::fmt;

use crate::{Cube, Cylinder, Material, Plane, Ray, Sphere, Triangle};
use math::prelude::*;

pub type ShapeIdx = usize;
pub type ShapeIntersectionResult = ([f32; 4], usize);

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

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Shape {
    shape: ShapeEnum,
    parent: Option<ShapeIdx>,
    casts_shadow: bool,
}

pub trait ShapeOps {
    fn intersect(&self, r: &Ray) -> ShapeIntersectionResult;
    fn normal_at(&self, world_point: &Tuple4D) -> Tuple4D;
    fn local_normal_at(&self, local_point: &Tuple4D) -> Tuple4D;

    // TODO: intersect and normal_at are individual implementatiosn for each shape
    // but the setters / getters are all identical for all shapes, groups, CSG (if ever implemented)
    // move to a "BaseShape" and make a compose struct of a BaseShape and the individual componentes
    fn set_transformation(&mut self, m: Matrix);
    fn get_transformation(&self) -> &Matrix;
    fn get_inverse_transformation(&self) -> &Matrix;

    fn set_material(&mut self, m: Material);
    fn get_material(&self) -> &Material;
    fn get_material_mut(&mut self) -> &mut Material;
}

impl ShapeOps for Shape {
    fn intersect(&self, r: &Ray) -> ShapeIntersectionResult {
        match self.shape {
            ShapeEnum::Sphere(ref s) => s.intersect(r),
            ShapeEnum::Plane(ref plane) => plane.intersect(r),
            ShapeEnum::Cube(ref cube) => cube.intersect(r),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.intersect(r),
            ShapeEnum::Triangle(ref triangle) => triangle.intersect(r),
            // ShapeEnum::Group(_) => panic!("Group::normal_at should never be called "),
        }
    }

    fn normal_at(&self, world_point: &Tuple4D) -> Tuple4D {
        let object_point = self.get_inverse_transformation() * world_point;
        let local_normal = match self.shape {
            ShapeEnum::Sphere(ref s) => s.local_normal_at(&object_point),
            ShapeEnum::Plane(ref plane) => plane.local_normal_at(&object_point),
            ShapeEnum::Cube(ref cube) => cube.local_normal_at(&object_point),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.local_normal_at(&object_point),
            ShapeEnum::Triangle(ref triangle) => triangle.local_normal_at(&object_point),
            // ShapeEnum::Group(_) => panic!("Group::normal_at should never be called "),
        };
        let mut world_normal = &Matrix::transpose(self.get_inverse_transformation()) * &local_normal;
        world_normal.w = 0.0;
        Tuple4D::normalize(&world_normal)
    }

    fn local_normal_at(&self, _local_point: &Tuple4D) -> Tuple4D {
        unreachable!("should never get here ");
    }

    fn set_transformation(&mut self, m: Matrix) {
        match self.shape {
            ShapeEnum::Sphere(ref mut sphere) => sphere.set_transformation(m),
            ShapeEnum::Plane(ref mut plane) => plane.set_transformation(m),
            ShapeEnum::Cube(ref mut cube) => cube.set_transformation(m),
            ShapeEnum::Cylinder(ref mut cylinder) => cylinder.set_transformation(m),
            ShapeEnum::Triangle(ref mut triangle) => triangle.set_transformation(m),
            // ShapeEnum::Group(ref mut group) => group.set_transformation(m),
        };
    }

    fn get_transformation(&self) -> &Matrix {
        match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_transformation(),
            ShapeEnum::Plane(ref p) => p.get_transformation(),
            ShapeEnum::Cube(ref c) => c.get_transformation(),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_transformation(),
            ShapeEnum::Triangle(ref triangle) => triangle.get_transformation(),
            // ShapeEnum::Group(ref group) => group.get_transformation(),
        }
    }

    fn get_inverse_transformation(&self) -> &Matrix {
        match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_inverse_transformation(),
            ShapeEnum::Plane(ref p) => p.get_inverse_transformation(),
            ShapeEnum::Cube(ref c) => c.get_inverse_transformation(),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_inverse_transformation(),
            ShapeEnum::Triangle(ref triangle) => triangle.get_inverse_transformation(),
            // ShapeEnum::Group(ref group) => group.get_inverse_transformation(),
        }
    }

    fn set_material(&mut self, m: Material) {
        match self.shape {
            ShapeEnum::Sphere(ref mut s) => s.set_material(m),
            ShapeEnum::Plane(ref mut p) => p.set_material(m),
            ShapeEnum::Cube(ref mut c) => c.set_material(m),
            ShapeEnum::Cylinder(ref mut cylinder) => cylinder.set_material(m),
            ShapeEnum::Triangle(ref mut triangle) => triangle.set_material(m),
            // ShapeEnum::Group(ref mut group) => group.set_material(m),
        };
    }

    fn get_material(&self) -> &Material {
        match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_material(),
            ShapeEnum::Plane(ref p) => p.get_material(),
            ShapeEnum::Cube(ref c) => c.get_material(),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_material(),
            ShapeEnum::Triangle(ref triangle) => triangle.get_material(),
            // ShapeEnum::Group(_) => panic!("Group::get_material should never be called "),
        }
    }

    fn get_material_mut(&mut self) -> &mut Material {
        match self.shape {
            ShapeEnum::Sphere(ref mut s) => s.get_material_mut(),
            ShapeEnum::Plane(ref mut p) => p.get_material_mut(),
            ShapeEnum::Cube(ref mut c) => c.get_material_mut(),
            ShapeEnum::Cylinder(ref mut cylinder) => cylinder.get_material_mut(),
            ShapeEnum::Triangle(ref mut triangle) => triangle.get_material_mut(),
            // ShapeEnum::Group(_) => panic!("Group::get_material should never be called "),
        }
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
        write!(f, "shape type = {:?} ", self.shape)
    }
}
