use core::fmt;

use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ShapeEnum {
    SphereEnum(Sphere),
    PlaneEnum(Plane),
    CubeEnum(Cube),
    CylinderEnum(Cylinder),
    TriangleEnum(Triangle),
    SmoothTriangleEnum(SmoothTriangle),
    GroupEnum(Group),
    CsgEnum(Csg),
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

    // only relevant for Group
    fn get_children(&self) -> &Vec<ShapeIdx>;
    fn get_children_mut(&mut self) -> &mut Vec<ShapeIdx>;
}

pub trait ShapeIntersectOps<'a> {
    fn intersect_local(shape: &'a Shape, r: Ray, shapes: &'a ShapeArr) -> IntersectionList<'a>;
}

impl<'a> ShapeIntersectOps<'a> for Shape {
    fn intersect_local(shape: &'a Shape, r: Ray, shapes: &'a ShapeArr) -> IntersectionList<'a> {
        match shape.get_shape() {
            ShapeEnum::SphereEnum(ref _sphere) => Sphere::intersect_local(shape, r, shapes),
            ShapeEnum::PlaneEnum(ref _plane) => Plane::intersect_local(shape, r, shapes),
            ShapeEnum::CubeEnum(ref _cube) => Cube::intersect_local(shape, r, shapes),
            ShapeEnum::CylinderEnum(ref _cylinder) => Cylinder::intersect_local(shape, r, shapes),
            ShapeEnum::TriangleEnum(ref _triangle) => Triangle::intersect_local(shape, r, shapes),
            ShapeEnum::SmoothTriangleEnum(ref _triangle) => SmoothTriangle::intersect_local(shape, r, shapes),
            ShapeEnum::GroupEnum(ref _group) => {
                // println!("group intersect_local");
                Group::intersect_local(shape, r, shapes)
            }
            ShapeEnum::CsgEnum(ref _csg) => {
                println!("CSG intersect_local");
                Csg::intersect_local(shape, r, shapes)
            }
        }
    }
}

impl<'a> ShapeOps<'a> for Shape {
    fn set_transformation(&mut self, m: Matrix) {
        match self.shape {
            ShapeEnum::SphereEnum(ref mut sphere) => sphere.set_transformation(m),
            ShapeEnum::PlaneEnum(ref mut plane) => plane.set_transformation(m),
            ShapeEnum::CubeEnum(ref mut cube) => cube.set_transformation(m),
            ShapeEnum::CylinderEnum(ref mut cylinder) => cylinder.set_transformation(m),
            ShapeEnum::TriangleEnum(ref mut triangle) => triangle.set_transformation(m),
            ShapeEnum::SmoothTriangleEnum(ref mut triangle) => triangle.set_transformation(m),
            ShapeEnum::GroupEnum(ref mut group) => group.set_transformation(m),
            ShapeEnum::CsgEnum(ref mut csg) => csg.set_transformation(m),
        }
    }

    fn get_transformation(&self) -> &Matrix {
        match self.shape {
            ShapeEnum::SphereEnum(ref sphere) => sphere.get_transformation(),
            ShapeEnum::PlaneEnum(ref plane) => plane.get_transformation(),
            ShapeEnum::CubeEnum(ref cube) => cube.get_transformation(),
            ShapeEnum::CylinderEnum(ref cylinder) => cylinder.get_transformation(),
            ShapeEnum::TriangleEnum(ref triangle) => triangle.get_transformation(),
            ShapeEnum::SmoothTriangleEnum(ref triangle) => triangle.get_transformation(),
            ShapeEnum::GroupEnum(ref group) => group.get_transformation(),
            ShapeEnum::CsgEnum(ref csg) => csg.get_transformation(),
        }
    }

    fn get_inverse_transformation(&self) -> &Matrix {
        match self.shape {
            ShapeEnum::SphereEnum(ref sphere) => sphere.get_inverse_transformation(),
            ShapeEnum::PlaneEnum(ref plane) => plane.get_inverse_transformation(),
            ShapeEnum::CubeEnum(ref cube) => cube.get_inverse_transformation(),
            ShapeEnum::CylinderEnum(ref cylinder) => cylinder.get_inverse_transformation(),
            ShapeEnum::TriangleEnum(ref triangle) => triangle.get_inverse_transformation(),
            ShapeEnum::SmoothTriangleEnum(ref triangle) => triangle.get_inverse_transformation(),
            ShapeEnum::GroupEnum(ref group) => group.get_inverse_transformation(),
            ShapeEnum::CsgEnum(ref csg) => csg.get_inverse_transformation(),
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
            ShapeEnum::SphereEnum(ref sphere) => sphere.local_normal_at(local_point, i),
            ShapeEnum::PlaneEnum(ref plane) => plane.local_normal_at(local_point, i),
            ShapeEnum::CubeEnum(ref cube) => cube.local_normal_at(local_point, i),
            ShapeEnum::CylinderEnum(ref cylinder) => cylinder.local_normal_at(local_point, i),
            ShapeEnum::TriangleEnum(ref triangle) => triangle.local_normal_at(local_point, i),
            ShapeEnum::SmoothTriangleEnum(ref smooth_triangle) => smooth_triangle.local_normal_at(local_point, i),
            ShapeEnum::GroupEnum(ref group) => group.local_normal_at(local_point, i),
            ShapeEnum::CsgEnum(ref csg) => csg.local_normal_at(local_point, i),
        }
    }

    fn set_material(&mut self, m: Material) {
        match self.shape {
            ShapeEnum::SphereEnum(ref mut sphere) => sphere.set_material(m),
            ShapeEnum::PlaneEnum(ref mut plane) => plane.set_material(m),
            ShapeEnum::CubeEnum(ref mut c) => c.set_material(m),
            ShapeEnum::CylinderEnum(ref mut cylinder) => cylinder.set_material(m),
            ShapeEnum::TriangleEnum(ref mut triangle) => triangle.set_material(m),
            ShapeEnum::SmoothTriangleEnum(ref mut triangle) => triangle.set_material(m),
            ShapeEnum::GroupEnum(ref mut group) => group.set_material(m),
            ShapeEnum::CsgEnum(ref mut csg) => csg.set_material(m),
        }
    }

    fn get_material(&self) -> &Material {
        match self.shape {
            ShapeEnum::SphereEnum(ref sphere) => sphere.get_material(),
            ShapeEnum::PlaneEnum(ref plane) => plane.get_material(),
            ShapeEnum::CubeEnum(ref c) => c.get_material(),
            ShapeEnum::CylinderEnum(ref cylinder) => cylinder.get_material(),
            ShapeEnum::TriangleEnum(ref triangle) => triangle.get_material(),
            ShapeEnum::SmoothTriangleEnum(ref triangle) => triangle.get_material(),
            ShapeEnum::GroupEnum(_) => panic!("Group::get_material should never be called "),
            ShapeEnum::CsgEnum(_) => panic!("CSG::get_material should never be called "),
        }
    }

    fn get_material_mut(&mut self) -> &mut Material {
        match self.shape {
            ShapeEnum::SphereEnum(ref mut sphere) => sphere.get_material_mut(),
            ShapeEnum::PlaneEnum(ref mut plane) => plane.get_material_mut(),
            ShapeEnum::CubeEnum(ref mut c) => c.get_material_mut(),
            ShapeEnum::CylinderEnum(ref mut cylinder) => cylinder.get_material_mut(),
            ShapeEnum::TriangleEnum(ref mut triangle) => triangle.get_material_mut(),
            ShapeEnum::SmoothTriangleEnum(ref mut triangle) => triangle.get_material_mut(),
            ShapeEnum::GroupEnum(_) => panic!("Group::get_material should never be called "),
            ShapeEnum::CsgEnum(_) => panic!("CSG::get_material should never be called "),
        }
    }

    fn get_parent(&self) -> &Option<ShapeIdx> {
        &self.parent
    }

    fn get_children(&self) -> &Vec<ShapeIdx> {
        match self.shape {
            ShapeEnum::SphereEnum(ref _sphere) => unreachable!("tAhis should never be called"),
            ShapeEnum::PlaneEnum(ref _plane) => unreachable!("Bthis should never be called"),
            ShapeEnum::CubeEnum(ref _cube) => unreachable!("Cthis should never be called"),
            ShapeEnum::CylinderEnum(ref _cylinder) => unreachable!("Dthis should never be called"),
            ShapeEnum::TriangleEnum(ref _triangle) => unreachable!("Ethis should never be called"),
            ShapeEnum::SmoothTriangleEnum(ref _triangle) => unreachable!("Ethis should never be called"),
            ShapeEnum::GroupEnum(ref group) => group.get_children(),
            ShapeEnum::CsgEnum(ref _csg) => unreachable!("Ethis should never be called"),
        }
    }

    fn get_children_mut(&mut self) -> &mut Vec<ShapeIdx> {
        match self.shape {
            ShapeEnum::SphereEnum(ref mut _sphere) => unreachable!("Fthis should never be called"),
            ShapeEnum::PlaneEnum(ref mut _plane) => unreachable!("Gthis should never be called"),
            ShapeEnum::CubeEnum(ref mut _c) => unreachable!("Hthis should never be called"),
            ShapeEnum::CylinderEnum(ref mut _cylinder) => unreachable!("Ithis should never be called"),
            ShapeEnum::TriangleEnum(ref mut _triangle) => unreachable!("Jthis should never be called"),
            ShapeEnum::SmoothTriangleEnum(ref mut _triangle) => unreachable!("Jthis should never be called"),
            ShapeEnum::GroupEnum(ref mut group) => group.get_children_mut(),
            ShapeEnum::CsgEnum(ref mut _csg) => unreachable!("Jthis should never be called"),
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

    // only relevant for CSG
    pub fn get_left(&self) -> ShapeIdx {
        match self.shape {
            ShapeEnum::CsgEnum(ref csg) => csg.get_left(),
            _ => unreachable!("this should never be called"),
        }
    }

    pub fn get_right(&self) -> ShapeIdx {
        match self.shape {
            ShapeEnum::CsgEnum(ref csg) => csg.get_right(),
            _ => unreachable!("this should never be called"),
        }
    }

    pub(crate) fn set_left(&mut self, idx: ShapeIdx) {
        match self.shape {
            ShapeEnum::CsgEnum(ref mut csg) => csg.set_left(idx),
            _ => unreachable!("Jhis should never be called"),
        }
    }

    pub(crate) fn set_right(&mut self, idx: ShapeIdx) {
        match self.shape {
            ShapeEnum::CsgEnum(ref mut csg) => csg.set_right(idx),
            _ => unreachable!("this should never be called"),
        }
    }

    pub(crate) fn get_op(&self) -> &CsgOp {
        match self.shape {
            ShapeEnum::CsgEnum(ref csg) => csg.get_op(),
            _ => unreachable!("this should never be called"),
        }
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
            ShapeEnum::SphereEnum(_sphere) => parent_msg.push_str(format!("sphere").as_str()),
            ShapeEnum::PlaneEnum(_plane) => parent_msg.push_str(format!("plane").as_str()),
            ShapeEnum::CubeEnum(_cube) => parent_msg.push_str(format!("cube").as_str()),
            ShapeEnum::CylinderEnum(_cylinder) => parent_msg.push_str(format!("cylinder").as_str()),
            ShapeEnum::TriangleEnum(_trinagle) => parent_msg.push_str(format!("triangle").as_str()),
            ShapeEnum::SmoothTriangleEnum(_trinagle) => parent_msg.push_str(format!("smooth triangle").as_str()),
            ShapeEnum::GroupEnum(_group) => parent_msg.push_str(format!("group").as_str()),
            ShapeEnum::CsgEnum(_csg) => parent_msg.push_str(format!("csg").as_str()),
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
            ShapeEnum::SphereEnum(_sphere) => parent_msg.push_str(
                format!(
                    "sphere  name '{}'  parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
            ShapeEnum::PlaneEnum(_plane) => parent_msg.push_str(
                format!(
                    "plane    name '{}'  parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
            ShapeEnum::CubeEnum(_c) => parent_msg.push_str(
                format!(
                    "cube   name '{}'      parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
            ShapeEnum::CylinderEnum(_c) => parent_msg.push_str(
                format!(
                    "cylinder   name '{}'     parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
            ShapeEnum::TriangleEnum(_t) => parent_msg.push_str(
                format!(
                    "triangle   name '{}'      parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
            ShapeEnum::SmoothTriangleEnum(_t) => parent_msg.push_str(
                format!(
                    "smooth triangle   name '{}'      parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
            ShapeEnum::GroupEnum(_g) => parent_msg.push_str(
                format!(
                    "group    name '{}'      parent {:?}, part_of_group  {}   ",
                    n,
                    self.get_parent(),
                    self.part_of_group
                )
                .as_str(),
            ),
            ShapeEnum::CsgEnum(_g) => parent_msg.push_str(
                format!(
                    "CSG    name '{}'      parent {:?}, part_of_group  {}   ",
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
            ShapeEnum::SphereEnum(sphere) => {
                parent_msg.push_str(format!("sphere     {:?}", &sphere.get_transformation()).as_str())
            }
            ShapeEnum::PlaneEnum(plane) => {
                parent_msg.push_str(format!("plane    {:?}", &plane.get_transformation()).as_str())
            }
            ShapeEnum::CubeEnum(cube) => {
                parent_msg.push_str(format!("cube    {:?}", &cube.get_transformation()).as_str())
            }
            ShapeEnum::CylinderEnum(cylinder) => {
                parent_msg.push_str(format!("cylinder   {:?}", &cylinder.get_transformation()).as_str())
            }
            ShapeEnum::TriangleEnum(triangle) => {
                parent_msg.push_str(format!("triangle   {:?}", &triangle.get_transformation()).as_str())
            }
            ShapeEnum::SmoothTriangleEnum(triangle) => {
                parent_msg.push_str(format!("smooth triangle   {:?}", &triangle.get_transformation()).as_str())
            }
            ShapeEnum::GroupEnum(group) => {
                parent_msg.push_str(format!("group    {:?}", &group.get_transformation()).as_str())
            }
            ShapeEnum::CsgEnum(csg) => parent_msg.push_str(format!("CSG    {:?}", &csg.get_transformation()).as_str()),
        }
        write!(f, "ShapeEnum: {}   ", parent_msg)
    }
}
