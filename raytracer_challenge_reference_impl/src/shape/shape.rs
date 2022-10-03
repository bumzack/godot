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
    bounding_box: Option<BoundingBox>,
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    material: Material,
}

pub trait ShapeOps<'a> {
    fn set_transformation(&mut self, m: Matrix);
    fn get_transformation(&self) -> &Matrix;
    fn get_inverse_transformation(&self) -> &Matrix;

    fn normal_at(&self, p: &Tuple4D, i: &Intersection<'a>) -> Tuple4D;

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
    fn local_normal_at(&self, local_point: &Tuple4D, i: &Intersection<'a>) -> Tuple4D;
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
            ShapeEnum::GroupEnum(ref _group) => Group::intersect_local(shape, r, shapes),
            ShapeEnum::CsgEnum(ref _csg) => Csg::intersect_local(shape, r, shapes),
        }
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
}

impl<'a> ShapeOps<'a> for Shape {
    fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix =
            Matrix::invert(&m).expect("Cube::set_transformation: cant unwrap inverse matrix");
        self.transformation_matrix = m;
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }

    fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }

    fn normal_at(&self, world_point: &Tuple4D, i: &Intersection<'a>) -> Tuple4D {
        let object_point = self.get_inverse_transformation() * world_point;
        let local_normal = self.local_normal_at(&object_point, i);
        let mut world_normal = &Matrix::transpose(self.get_inverse_transformation()) * &local_normal;
        world_normal.w = 0.0;
        Tuple4D::normalize(&world_normal)
    }

    fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn get_parent(&self) -> &Option<ShapeIdx> {
        &self.parent
    }

    fn get_children(&self) -> &Vec<ShapeIdx> {
        match self.shape {
            ShapeEnum::GroupEnum(ref group) => group.get_children(),
            _ => unreachable!("this should never be called"),
        }
    }

    fn get_children_mut(&mut self) -> &mut Vec<ShapeIdx> {
        match self.shape {
            ShapeEnum::GroupEnum(ref mut group) => group.get_children_mut(),
            _ => unreachable!("Jthis should never be called"),
        }
    }
}

impl<'a> Shape {
    pub fn new_with_name(shape: ShapeEnum, name: String) -> Shape {
        let mut s = Shape {
            shape,
            casts_shadow: true,
            parent: None,
            part_of_group: false,
            name: Some(name),
            bounding_box: None,
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            material: Material::new(),
        };
        // it makes sad and sick - this has to change
        s.add_bounding_box(&vec![]);
        s
    }

    pub fn new(shape: ShapeEnum) -> Shape {
        let mut s = Shape {
            shape,
            casts_shadow: true,
            parent: None,
            part_of_group: false,
            name: None,
            bounding_box: None,
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            material: Material::new(),
        };
        // it makes sad and sick - this has to change
        s.add_bounding_box(&vec![]);
        s
    }

    // pub fn new_part_of_group(shape: Shape , name: String) -> Shape {
    //     let mut s = Shape {
    //         shape,
    //         casts_shadow: true,
    //         parent: None,
    //         part_of_group: true,
    //         name: Some(name),
    //         bounding_box: None,
    //         transformation_matrix: Matrix::new_identity_4x4(),
    //         inverse_transformation_matrix: Matrix::new_identity_4x4(),
    //         material: Material::new(),
    //     };
    //     // it makes sad and sick - this has to change
    //     s.add_bounding_box(&vec![]);
    //     s
    // }

    pub fn get_shape(&self) -> &ShapeEnum {
        &self.shape
    }

    pub fn get_casts_shadow(&self) -> bool {
        self.casts_shadow
    }

    pub fn set_casts_shadow(&mut self, casts_shadow: bool) {
        self.casts_shadow = casts_shadow;
    }

    pub fn intersects(shape: &'a Shape, r: Ray, shapes: &'a ShapeArr) -> IntersectionList<'a> {
        let r = Ray::transform(&r, shape.get_inverse_transformation());
        Self::intersect_local(shape, r, shapes)
    }

    pub fn set_parent(&mut self, parent_idx: ShapeIdx) {
        self.parent = Some(parent_idx);
    }

    pub fn get_part_of_group(&self) -> bool {
        self.part_of_group
    }

    pub fn set_part_of_group(&mut self, part_of_group: bool) {
        self.part_of_group = part_of_group;
    }

    pub fn get_name(&self) -> &Option<String> {
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

    pub fn set_left(&mut self, idx: ShapeIdx) {
        match self.shape {
            ShapeEnum::CsgEnum(ref mut csg) => csg.set_left(idx),
            _ => unreachable!("Jhis should never be called"),
        }
    }

    pub fn set_right(&mut self, idx: ShapeIdx) {
        match self.shape {
            ShapeEnum::CsgEnum(ref mut csg) => csg.set_right(idx),
            _ => unreachable!("this should never be called"),
        }
    }

    pub fn get_op(&self) -> &CsgOp {
        match self.shape {
            ShapeEnum::CsgEnum(ref csg) => csg.get_op(),
            _ => unreachable!("this should never be called"),
        }
    }

    pub fn get_parent_space_bounds_of(shape: &Shape, shapes: &ShapeArr) -> BoundingBox {
        BoundingBox::transform(&shape.get_bounds_of(shapes), shape.get_transformation())
    }

    pub fn add_bounding_box(&mut self, shapes: &ShapeArr) {
        let bounding_box = self.get_bounds_of(shapes);
        self.bounding_box = Some(bounding_box);
    }

    pub fn get_bounding_box(&self) -> &BoundingBox {
        &self.bounding_box.as_ref().unwrap()
    }

    pub fn get_bounds_of(&self, shapes: &ShapeArr) -> BoundingBox {
        match self.shape {
            ShapeEnum::SphereEnum(ref sphere) => sphere.get_bounds_of(),
            ShapeEnum::PlaneEnum(ref plane) => plane.get_bounds_of(),
            ShapeEnum::CubeEnum(ref cube) => cube.get_bounds_of(),
            ShapeEnum::CylinderEnum(ref cylinder) => cylinder.get_bounds_of(),
            ShapeEnum::TriangleEnum(ref triangle) => triangle.get_bounds_of(),
            ShapeEnum::SmoothTriangleEnum(ref triangle) => triangle.get_bounds_of(),
            ShapeEnum::GroupEnum(ref group) => group.get_bounds_of(shapes),
            ShapeEnum::CsgEnum(ref csg) => csg.get_bounds_of(shapes),
        }
    }
}

impl fmt::Debug for Shape {
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
            ShapeEnum::SphereEnum(_sphere) => parent_msg.push_str("sphere"),
            ShapeEnum::PlaneEnum(_plane) => parent_msg.push_str("plane"),
            ShapeEnum::CubeEnum(_cube) => parent_msg.push_str("cube"),
            ShapeEnum::CylinderEnum(_cylinder) => parent_msg.push_str("cylinder"),
            ShapeEnum::TriangleEnum(_triangle) => parent_msg.push_str("triangle"),
            ShapeEnum::SmoothTriangleEnum(_smooth_triangle) => parent_msg.push_str("smooth triangle"),
            ShapeEnum::GroupEnum(_group) => parent_msg.push_str("group"),
            ShapeEnum::CsgEnum(_csg) => parent_msg.push_str("csg"),
        }

        write!(f, "ShapeEnum {}", &parent_msg)
    }
}

impl fmt::Display for Shape {
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
            ShapeEnum::SphereEnum(_sphere) => parent_msg.push_str(format!("sphere ").as_str()),
            ShapeEnum::PlaneEnum(_plane) => parent_msg.push_str(format!("plane").as_str()),
            ShapeEnum::CubeEnum(_cube) => parent_msg.push_str(format!("cube").as_str()),
            ShapeEnum::CylinderEnum(_cylinder) => parent_msg.push_str(format!("cylinder").as_str()),
            ShapeEnum::TriangleEnum(_triangle) => parent_msg.push_str(format!("triangle").as_str()),
            ShapeEnum::SmoothTriangleEnum(_triangle) => parent_msg.push_str(format!("smooth triangle").as_str()),
            ShapeEnum::GroupEnum(_group) => parent_msg.push_str(format!("group").as_str()),
            ShapeEnum::CsgEnum(_csg) => parent_msg.push_str(format!("CSG").as_str()),
        }
        write!(f, "ShapeEnum: {}   ", parent_msg)
    }
}
