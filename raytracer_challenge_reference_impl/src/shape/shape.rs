use core::fmt;

use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ShapeEnum {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Triangle(Triangle),
    SmoothTriangle(SmoothTriangle),
    Group(Group),
}

#[derive(Clone, PartialEq)]
pub struct Shape {
    shape: ShapeEnum,
    casts_shadow: bool,
    part_of_group: bool,
    parent: Option<ShapeIdx>,
    name: Option<String>,
}

pub trait ShapeOps<'a> {
    fn set_transformation(&mut self, m: Matrix);
    fn get_transformation(&self) -> &Matrix;
    fn get_inverse_transformation(&self) -> &Matrix;

    fn normal_at(&self, p: &Tuple4D, shapes: &ShapeArr, i: &Intersection<'a>) -> Tuple4D;
    fn local_normal_at(&self, local_point: &Tuple4D, i: &Intersection<'a>) -> Tuple4D;

    fn set_material(&mut self, m: Material);
    fn get_material(&self) -> &Material;
    fn get_material_mut(&mut self) -> &mut Material;

    fn get_parent(&self) -> &Option<ShapeIdx>;
    fn get_children(&self) -> &Vec<ShapeIdx>;
    fn get_children_mut(&mut self) -> &mut Vec<ShapeIdx>;
}

pub trait ShapeIntersectOps<'a> {
    fn intersect_local(shape: &'a Shape, r: Ray, shapes: &'a ShapeArr) -> IntersectionList<'a>;
}

impl<'a> ShapeIntersectOps<'a> for Shape {
    fn intersect_local(shape: &'a Shape, r: Ray, shapes: &'a ShapeArr) -> IntersectionList<'a> {
        match shape.get_shape() {
            ShapeEnum::Sphere(ref _sphere) => Sphere::intersect_local(shape, r, shapes),
            ShapeEnum::Plane(ref _plane) => Plane::intersect_local(shape, r, shapes),
            ShapeEnum::Cube(ref _cube) => Cube::intersect_local(shape, r, shapes),
            ShapeEnum::Cylinder(ref _cylinder) => Cylinder::intersect_local(shape, r, shapes),
            ShapeEnum::Triangle(ref _triangle) => Triangle::intersect_local(shape, r, shapes),
            ShapeEnum::SmoothTriangle(ref _triangle) => SmoothTriangle::intersect_local(shape, r, shapes),
            ShapeEnum::Group(ref _group) => {
                // println!("group intersect_local");
                Group::intersect_local(shape, r, shapes)
            }
        }
    }
}

impl<'a> ShapeOps<'a> for Shape {
    fn set_transformation(&mut self, m: Matrix) {
        match self.shape {
            ShapeEnum::Sphere(ref mut sphere) => sphere.set_transformation(m),
            ShapeEnum::Plane(ref mut plane) => plane.set_transformation(m),
            ShapeEnum::Cube(ref mut cube) => cube.set_transformation(m),
            ShapeEnum::Cylinder(ref mut cylinder) => cylinder.set_transformation(m),
            ShapeEnum::Triangle(ref mut triangle) => triangle.set_transformation(m),
            ShapeEnum::SmoothTriangle(ref mut triangle) => triangle.set_transformation(m),
            ShapeEnum::Group(ref mut group) => group.set_transformation(m),
        }
    }

    fn get_transformation(&self) -> &Matrix {
        match self.shape {
            ShapeEnum::Sphere(ref sphere) => sphere.get_transformation(),
            ShapeEnum::Plane(ref plane) => plane.get_transformation(),
            ShapeEnum::Cube(ref cube) => cube.get_transformation(),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_transformation(),
            ShapeEnum::Triangle(ref triangle) => triangle.get_transformation(),
            ShapeEnum::SmoothTriangle(ref triangle) => triangle.get_transformation(),
            ShapeEnum::Group(ref group) => group.get_transformation(),
        }
    }

    fn get_inverse_transformation(&self) -> &Matrix {
        match self.shape {
            ShapeEnum::Sphere(ref sphere) => sphere.get_inverse_transformation(),
            ShapeEnum::Plane(ref plane) => plane.get_inverse_transformation(),
            ShapeEnum::Cube(ref cube) => cube.get_inverse_transformation(),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_inverse_transformation(),
            ShapeEnum::Triangle(ref triangle) => triangle.get_inverse_transformation(),
            ShapeEnum::SmoothTriangle(ref triangle) => triangle.get_inverse_transformation(),
            ShapeEnum::Group(ref group) => group.get_inverse_transformation(),
        }
    }

    fn normal_at(&self, world_point: &Tuple4D, shapes: &ShapeArr, i: &Intersection<'a>) -> Tuple4D {
        let local_point = world_to_object(self, world_point, shapes);
        let local_normal = Self::local_normal_at(self, &local_point, i);
        normal_to_world(self, &local_normal, shapes)

        // let object_point = self.get_inverse_transformation() * world_point;
        // let local_normal = match self.shape {
        //     ShapeEnum::Sphere(ref sphere) => sphere.local_normal_at(&object_point),
        //     ShapeEnum::Plane(ref plane) => plane.local_normal_at(&object_point),
        //     ShapeEnum::Cube(ref cube) => cube.local_normal_at(&object_point),
        //     ShapeEnum::Cylinder(ref cylinder) => cylinder.local_normal_at(&object_point),
        //     ShapeEnum::Triangle(ref triangle) => triangle.local_normal_at(&object_point),
        //     ShapeEnum::Group(_) => panic!("Group::normal_at should never be called "),
        // };
        // let mut world_normal = &Matrix::transpose(self.get_inverse_transformation()) * &local_normal;
        // world_normal.w = 0.0;
        // Tuple4D::normalize(&world_normal)
    }

    fn local_normal_at(&self, local_point: &Tuple4D, i: &Intersection<'a>) -> Tuple4D {
        match self.shape {
            ShapeEnum::Sphere(ref sphere) => sphere.local_normal_at(local_point, i),
            ShapeEnum::Plane(ref plane) => plane.local_normal_at(local_point, i),
            ShapeEnum::Cube(ref cube) => cube.local_normal_at(local_point, i),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.local_normal_at(local_point, i),
            ShapeEnum::Triangle(ref triangle) => triangle.local_normal_at(local_point, i),
            ShapeEnum::SmoothTriangle(ref smooth_triangle) => smooth_triangle.local_normal_at(local_point, i),
            ShapeEnum::Group(ref group) => group.local_normal_at(local_point, i),
        }
    }

    fn set_material(&mut self, m: Material) {
        match self.shape {
            ShapeEnum::Sphere(ref mut sphere) => sphere.set_material(m),
            ShapeEnum::Plane(ref mut plane) => plane.set_material(m),
            ShapeEnum::Cube(ref mut c) => c.set_material(m),
            ShapeEnum::Cylinder(ref mut cylinder) => cylinder.set_material(m),
            ShapeEnum::Triangle(ref mut triangle) => triangle.set_material(m),
            ShapeEnum::SmoothTriangle(ref mut triangle) => triangle.set_material(m),
            ShapeEnum::Group(ref mut group) => group.set_material(m),
        }
    }

    fn get_material(&self) -> &Material {
        match self.shape {
            ShapeEnum::Sphere(ref sphere) => sphere.get_material(),
            ShapeEnum::Plane(ref plane) => plane.get_material(),
            ShapeEnum::Cube(ref c) => c.get_material(),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.get_material(),
            ShapeEnum::Triangle(ref triangle) => triangle.get_material(),
            ShapeEnum::SmoothTriangle(ref triangle) => triangle.get_material(),
            ShapeEnum::Group(_) => panic!("Group::get_material should never be called "),
        }
    }

    fn get_material_mut(&mut self) -> &mut Material {
        match self.shape {
            ShapeEnum::Sphere(ref mut sphere) => sphere.get_material_mut(),
            ShapeEnum::Plane(ref mut plane) => plane.get_material_mut(),
            ShapeEnum::Cube(ref mut c) => c.get_material_mut(),
            ShapeEnum::Cylinder(ref mut cylinder) => cylinder.get_material_mut(),
            ShapeEnum::Triangle(ref mut triangle) => triangle.get_material_mut(),
            ShapeEnum::SmoothTriangle(ref mut triangle) => triangle.get_material_mut(),
            ShapeEnum::Group(_) => panic!("Group::get_material should never be called "),
        }
    }

    fn get_parent(&self) -> &Option<ShapeIdx> {
        &self.parent
    }

    fn get_children(&self) -> &Vec<ShapeIdx> {
        match self.shape {
            ShapeEnum::Sphere(ref _sphere) => unreachable!("tAhis should never be called"),
            ShapeEnum::Plane(ref _plane) => unreachable!("Bthis should never be called"),
            ShapeEnum::Cube(ref _cube) => unreachable!("Cthis should never be called"),
            ShapeEnum::Cylinder(ref _cylinder) => unreachable!("Dthis should never be called"),
            ShapeEnum::Triangle(ref _triangle) => unreachable!("Ethis should never be called"),
            ShapeEnum::SmoothTriangle(ref _triangle) => unreachable!("Ethis should never be called"),
            ShapeEnum::Group(ref group) => group.get_children(),
        }
    }

    fn get_children_mut(&mut self) -> &mut Vec<ShapeIdx> {
        match self.shape {
            ShapeEnum::Sphere(ref mut _sphere) => unreachable!("Fthis should never be called"),
            ShapeEnum::Plane(ref mut _plane) => unreachable!("Gthis should never be called"),
            ShapeEnum::Cube(ref mut _c) => unreachable!("Hthis should never be called"),
            ShapeEnum::Cylinder(ref mut _cylinder) => unreachable!("Ithis should never be called"),
            ShapeEnum::Triangle(ref mut _triangle) => unreachable!("Jthis should never be called"),
            ShapeEnum::SmoothTriangle(ref mut _triangle) => unreachable!("Jthis should never be called"),
            ShapeEnum::Group(ref mut group) => group.get_children_mut(),
        }
    }
}

impl<'a> Shape {
    pub fn new_with_name(shape: ShapeEnum, name: String) -> Shape {
        Shape {
            shape,
            casts_shadow: true,
            parent: None,
            part_of_group: false,
            name: Some(name),
        }
    }

    pub fn new(shape: ShapeEnum) -> Shape {
        Shape {
            shape,
            casts_shadow: true,
            parent: None,
            part_of_group: false,
            name: None,
        }
    }

    pub fn new_part_of_group(shape: ShapeEnum, name: String) -> Shape {
        Shape {
            shape,
            casts_shadow: true,
            parent: None,
            part_of_group: true,
            name: Some(name),
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

    pub(crate) fn intersects(shape: &'a Shape, r: Ray, shapes: &'a ShapeArr) -> IntersectionList<'a> {
        let r = Ray::transform(&r, shape.get_inverse_transformation());
        Self::intersect_local(shape, r, shapes)
    }

    pub(crate) fn set_parent(&mut self, parent_idx: ShapeIdx) {
        self.parent = Some(parent_idx);
    }

    pub(crate) fn get_part_of_group(&self) -> bool {
        self.part_of_group
    }

    pub(crate) fn get_name(&self) -> &Option<String> {
        &self.name
    }
}

impl<'a> fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = match self.get_name() {
            Some(n) => n,
            None => "",
        };
        write!(f, "shape type = {}, name = '{}'", self.shape, n)
    }
}

impl fmt::Debug for ShapeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parent_msg = String::new();
        match &self {
            ShapeEnum::Sphere(_sphere) => parent_msg.push_str(format!("sphere").as_str()),
            ShapeEnum::Plane(_plane) => parent_msg.push_str(format!("plane").as_str()),
            ShapeEnum::Cube(_cube) => parent_msg.push_str(format!("cube").as_str()),
            ShapeEnum::Cylinder(_cylinder) => parent_msg.push_str(format!("cylinder").as_str()),
            ShapeEnum::Triangle(_trinagle) => parent_msg.push_str(format!("triangle").as_str()),
            ShapeEnum::SmoothTriangle(_trinagle) => parent_msg.push_str(format!("smooth triangle").as_str()),
            ShapeEnum::Group(_group) => parent_msg.push_str(format!("group").as_str()),
        }

        write!(f, "ShapeEnum {}", &parent_msg)
    }
}

impl<'a> fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parent_msg = String::new();
        let n = match self.get_name() {
            Some(n) => n,
            None => "",
        };
        match &self.shape {
            ShapeEnum::Sphere(_sphere) => parent_msg.push_str(
                format!(
                    "sphere  name '{}'  parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
            ShapeEnum::Plane(_plane) => parent_msg.push_str(
                format!(
                    "plane    name '{}'  parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
            ShapeEnum::Cube(_c) => parent_msg.push_str(
                format!(
                    "cube   name '{}'      parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
            ShapeEnum::Cylinder(_c) => parent_msg.push_str(
                format!(
                    "cylinder   name '{}'     parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
            ShapeEnum::Triangle(_t) => parent_msg.push_str(
                format!(
                    "triangle   name '{}'      parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
            ShapeEnum::SmoothTriangle(_t) => parent_msg.push_str(
                format!(
                    "smooth triangle   name '{}'      parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
            ShapeEnum::Group(_g) => parent_msg.push_str(
                format!(
                    "group    name '{}'      parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
        }
        write!(f, "Shape: {}   ", parent_msg)
    }
}

impl fmt::Display for ShapeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parent_msg = String::new();

        match &self {
            ShapeEnum::Sphere(sphere) => {
                parent_msg.push_str(format!("sphere     {:?}", &sphere.get_transformation()).as_str())
            }
            ShapeEnum::Plane(plane) => {
                parent_msg.push_str(format!("plane    {:?}", &plane.get_transformation()).as_str())
            }
            ShapeEnum::Cube(cube) => parent_msg.push_str(format!("cube    {:?}", &cube.get_transformation()).as_str()),
            ShapeEnum::Cylinder(cylinder) => {
                parent_msg.push_str(format!("cylinder   {:?}", &cylinder.get_transformation()).as_str())
            }
            ShapeEnum::Triangle(triangle) => {
                parent_msg.push_str(format!("triangle   {:?}", &triangle.get_transformation()).as_str())
            }
            ShapeEnum::SmoothTriangle(triangle) => {
                parent_msg.push_str(format!("smooth triangle   {:?}", &triangle.get_transformation()).as_str())
            }
            ShapeEnum::Group(group) => {
                parent_msg.push_str(format!("group    {:?}", &group.get_transformation()).as_str())
            }
        }
        write!(f, "ShapeEnum: {}   ", parent_msg)
    }
}
