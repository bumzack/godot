use crate::{Material, Matrix, Sphere, SphereOps, Tuple4D};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub enum ShapeEnum {
    Sphere(Sphere),
    //    Plane(Plane),
    //    Cube(Cube),
    //    Cylinder(Cylinder),
    //    Triangle(Triangle),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Shape {
    shape: ShapeEnum,
}

//
impl Shape {
    pub fn new(shape: ShapeEnum) -> Shape {
        Shape { shape }
    }

    pub fn normal_at(&self, p: &Tuple4D) -> Tuple4D {
        let res = match self.shape {
            ShapeEnum::Sphere(ref s) => s.normal_at(p),
            //            ShapeEnum::Plane(ref plane) => plane.normal_at(p),
            //            ShapeEnum::Cube(ref cube) => cube.normal_at(p),
            //            ShapeEnum::Cylinder(ref cylinder) => Cylinder::normal_at(cylinder, p),
            //            ShapeEnum::Triangle(ref triangle) => Triangle::normal_at(triangle, p),
        };
        res
    }

    pub fn get_material(&self) -> &Material {
        let res = match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_material(),
            //            ShapeEnum::Plane(ref p) => p.get_material(),
            //            ShapeEnum::Cube(ref c) => c.get_material(),
            //            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_material(),
            //            ShapeEnum::Triangle(ref triangle) => triangle.get_material(),
        };
        res
    }

    pub fn get_material_mut(&mut self) -> &mut Material {
        let res = match self.shape {
            ShapeEnum::Sphere(ref mut s) => s.get_material_mut(),
            //            ShapeEnum::Plane(ref mut p) => p.get_material_mut(),
            //            ShapeEnum::Cube(ref mut c) => c.get_material_mut(),
            //            ShapeEnum::Cylinder(ref mut cylinder) => cylinder.get_material_mut(),
            //            ShapeEnum::Triangle(ref mut triangle) => triangle.get_material_mut(),
        };
        res
    }

    pub fn get_transformation(&self) -> &Matrix {
        let res = match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_transformation(),
            //            ShapeEnum::Plane(ref p) => p.get_transformation(),
            //            ShapeEnum::Cube(ref c) => c.get_transformation(),
            //            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_transformation(),
            //            ShapeEnum::Triangle(ref triangle) => triangle.get_transformation(),
        };
        res
    }

    pub fn get_inverse_transformation(&self) -> &Matrix {
        let res = match self.shape {
            ShapeEnum::Sphere(ref s) => s.get_inverse_transformation(),
            //            ShapeEnum::Plane(ref p) => p.get_inverse_transformation(),
            //            ShapeEnum::Cube(ref c) => c.get_inverse_transformation(),
            //            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_inverse_transformation(),
            //            ShapeEnum::Triangle(ref triangle) => triangle.get_inverse_transformation(),
        };
        res
    }

    pub fn get_shape(&self) -> &ShapeEnum {
        &self.shape
    }
}
