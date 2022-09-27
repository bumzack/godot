use std::fmt;

use crate::basics::{Intersection, IntersectionList, IntersectionListOps, IntersectionOps, Ray, RayOps};
use crate::material::Material;
use crate::math::{Matrix, MatrixOps, Tuple, Tuple4D};
use crate::prelude::{BoundingBox, Shape, ShapeArr, ShapeEnum, ShapeIdx, ShapeIntersectOps, ShapeOps};

#[derive(Clone, PartialEq)]
pub struct Group {
    shape_idx: ShapeIdx,
    children: Vec<ShapeIdx>,
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
}

impl<'a> ShapeOps<'a> for Group {
    fn set_transformation(&mut self, m: Matrix) {
        println!("setting new transformation matrix {:?}", &m);
        println!("old transformation matrix  {:?}", self.get_transformation());
        self.inverse_transformation_matrix =
            Matrix::invert(&m).expect("Group::set_transformation:  can't unwrap inverted matrix ");
        self.transformation_matrix = m;
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }

    fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }

    fn normal_at(&self, _world_point: &Tuple4D, _shapes: &ShapeArr, _i: &Intersection<'a>) -> Tuple4D {
        unreachable!("this should never be called");
    }

    fn local_normal_at(&self, _local_point: &Tuple4D, _i: &Intersection<'a>) -> Tuple4D {
        unreachable!("this should never be called");
    }

    fn set_material(&mut self, _m: Material) {
        unreachable!("this should never be called");
    }

    fn get_material(&self) -> &Material {
        unreachable!("this should never be called");
    }

    fn get_material_mut(&mut self) -> &mut Material {
        unreachable!("this should never be called");
    }

    fn get_parent(&self) -> &Option<ShapeIdx> {
        unreachable!("this should never be called");
    }

    fn get_children(&self) -> &Vec<ShapeIdx> {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut Vec<ShapeIdx> {
        &mut self.children
    }
}

impl<'a> ShapeIntersectOps<'a> for Group {
    fn intersect_local(shape: &'a Shape, r: Ray, shapes: &'a ShapeArr) -> IntersectionList<'a> {
        // let bb = shape.get_bounds_of(shapes);
        // if !bb.intersects(&r) {
        if !shape.get_bounding_box().intersects(&r) {
            println!("group boundbox has NO hit");
            return IntersectionList::new();
        }
        println!("group boundbox has A hit");
        let mut intersection_list = IntersectionList::new();
        let root_idx = match shape.get_shape() {
            ShapeEnum::GroupEnum(g) => Some(g.shape_idx),
            _ => None,
        };
        if root_idx.is_none() {
            return intersection_list;
        }
        let root_idx = root_idx.unwrap();

        let group = shapes.get(root_idx as usize).unwrap();
        let children = group.get_children();

        let r = Ray::transform(&r, group.get_inverse_transformation());

        for child in children {
            let shape = shapes.get(*child as usize).unwrap();
            let mut xs = Shape::intersects(shape, r.clone(), shapes);
            // println!("shape {}  has  {} intersections ", shape, xs.get_intersections().len());
            for is in xs
                .get_intersections_mut()
                .drain(..)
                .filter(|i| !i.get_t().is_infinite())
            {
                intersection_list.add(is);
            }
        }
        intersection_list
            .get_intersections_mut()
            .sort_by(|a, b| a.get_t().partial_cmp(&b.get_t()).unwrap_or(std::cmp::Ordering::Equal));

        intersection_list
    }
}

impl Group {
    pub fn new(shapes: &mut ShapeArr, name: String) -> ShapeIdx {
        let idx = shapes.len();
        let g = Group {
            shape_idx: idx,
            children: vec![],
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
        };
        let g = ShapeEnum::GroupEnum(g);
        let shape = Shape::new_with_name(g, name);
        shapes.push(shape);
        assert_eq!(idx, shapes.len() - 1);
        idx
    }

    pub fn new_part_of_group(shapes: &mut ShapeArr, name: String) -> ShapeIdx {
        let idx = shapes.len();
        let g = Group {
            shape_idx: idx,
            children: vec![],
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
        };
        let g = ShapeEnum::GroupEnum(g);
        let shape = Shape::new_part_of_group(g, name);
        shapes.push(shape);
        assert_eq!(idx, shapes.len() - 1);
        idx
    }

    pub fn add_child(shapes: &mut ShapeArr, parent_idx: ShapeIdx, mut shape: Shape) -> ShapeIdx {
        shape.set_parent(parent_idx);
        shapes.push(shape);
        let shape_idx = shapes.len() - 1;
        let parent = shapes.get_mut(parent_idx).unwrap();
        parent.get_children_mut().push(shape_idx);
        shape_idx
    }

    pub fn add_child_idx(shapes: &mut ShapeArr, parent_idx: ShapeIdx, shape_idx: ShapeIdx) {
        let parent = shapes.get_mut(parent_idx).unwrap();
        parent.get_children_mut().push(shape_idx);

        let child = shapes.get_mut(shape_idx).unwrap();
        child.set_parent(parent_idx);
    }

    pub fn print_children(shapes: &ShapeArr, node_idx: ShapeIdx) {
        let parent = shapes.get(node_idx).unwrap();
        for c in parent.get_children() {
            let n = shapes.get(*c).unwrap();
            println!("child: {:?}", n);
        }
    }

    pub fn print_tree(shapes: &ShapeArr, root_idx: ShapeIdx, depth: usize) {
        let node = shapes.get(root_idx).unwrap();
        let spaces = " ".repeat(depth);
        println!("{:?}  {:?}", spaces, &node);

        if let ShapeEnum::GroupEnum(_) = node.get_shape() {
            for c in node.get_children() {
                // let n = shapes.get(*c).unwrap();
                Self::print_tree(shapes, *c, depth + 2);
            }
        }
    }

    pub(crate) fn get_bounds_of(&self, shapes: &ShapeArr) -> BoundingBox {
        println!("get_bounds_of group");
        let mut bb = BoundingBox::new();
        for c in self.get_children() {
            let child = shapes.get(*c as usize).unwrap();
            let b = Shape::get_parent_space_bounds_of(child, shapes);
            bb.add(&b);
        }
        bb
    }
}

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Group")
            .field("value", &"TODO Group".to_string())
            .field("parent", &"TODO parent")
            .field("children", &self.children)
            .finish()
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent_msg = String::new();
        // match self.parent {
        //     Some(p_idx) => parent_msg.push_str(format!("parent {}", p_idx).as_str()),
        //     None => parent_msg.push_str(format!("no parent ").as_str()),
        // }
        write!(f, "Group: {}   ", parent_msg)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::math::common::assert_matrix;
    use crate::math::Tuple;
    use crate::prelude::{assert_tuple, normal_to_world, world_to_object, Sphere};
    use crate::shape::shape::{Shape, ShapeEnum};

    use super::*;

    // page 195
    // Creating a new group
    #[test]
    fn test_creating_new_group() {
        let mut shapes: ShapeArr = vec![];
        let group_idx = Group::new(&mut shapes, "group".to_string());
        let group = shapes.get(group_idx).unwrap();
        let m = group.get_transformation();
        assert_matrix(&m, &Matrix::new_identity_4x4());
    }

    // page 195
    // A shape has a parent attribute
    #[test]
    fn test_shape_has_empty_parent() {
        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        assert!(shape.get_parent().is_none());
    }

    // page 195
    // Adding a child to a group
    #[test]
    fn test_adding_child_to_group() {
        let mut shapes: ShapeArr = vec![];
        let group_idx = Group::new(&mut shapes, "group".to_string());
        let group = shapes.get(group_idx).unwrap();
        assert!(group.get_parent().is_none());

        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        let _child_idx = Group::add_child(&mut shapes, group_idx, shape);
        // let child = shapes.get(child_idx).unwrap();

        let group = shapes.get(group_idx).unwrap();
        let children = group.get_children();
        assert_eq!(children.len(), 1);
    }

    // page 196
    // Intersecting a ray with an empty group
    #[test]
    fn test_intersecting_ray_with_empty_group() {
        let mut shapes: ShapeArr = vec![];
        let group_idx = Group::new(&mut shapes, "group".to_string());
        let shape = shapes.get(group_idx).unwrap();
        let group = match shape.get_shape() {
            ShapeEnum::GroupEnum(g) => Some(g),
            _ => None,
        };

        if group.is_some() {
            let r = Ray::new(Tuple4D::new_point(0.0, 0.0, 0.0), Tuple4D::new_vector(0.0, 0.0, 1.0));
            let is = Group::intersect_local(shape, r, &shapes);
            assert_eq!(is.get_intersections().len(), 0);
        } else {
            unreachable!("should never be here");
        }
    }

    // page 196
    //    test spheres one by one
    #[test]
    fn test_intersecting_ray_with_non_empty_group_sphere1_alone() {
        // sphere 1
        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        let r = Ray::new(Tuple4D::new_point(0.0, 0.0, -5.0), Tuple4D::new_vector(0.0, 0.0, 1.0));

        let shapes = vec![];
        let is = Shape::intersects(&shape, r, &shapes);
        println!("sphere 1  intersections .len  {:?}", is.get_intersections().len());

        assert_eq!(is.get_intersections().len(), 2);
    }

    #[test]
    fn test_intersecting_ray_with_non_empty_group_sphere2_alone() {
        // sphere 2
        let translation2 = Matrix::translation(0.0, 0.0, -3.0);
        let mut s2 = Sphere::new();
        s2.set_transformation(translation2);
        let shape = Shape::new(ShapeEnum::SphereEnum(s2));

        let r = Ray::new(Tuple4D::new_point(0.0, 0.0, -5.0), Tuple4D::new_vector(0.0, 0.0, 1.0));
        let shapes = vec![];
        let is = Shape::intersects(&shape, r, &shapes);
        println!("sphere 1  intersections {:?}", is);

        assert_eq!(is.get_intersections().len(), 2);
    }

    #[test]
    fn test_intersecting_ray_with_non_empty_group_sphere3_alone() {
        // sphere 3
        let translation3 = Matrix::translation(5.0, 0.0, 0.0);
        let mut s3 = Sphere::new();
        s3.set_transformation(translation3);
        let shape = Shape::new(ShapeEnum::SphereEnum(s3));

        let r = Ray::new(Tuple4D::new_point(0.0, 0.0, -5.0), Tuple4D::new_vector(0.0, 0.0, 1.0));
        let shapes = vec![];
        let is = Shape::intersects(&shape, r, &shapes);
        println!("sphere 1  intersections {:?}", is);

        assert_eq!(is.get_intersections().len(), 0);
    }

    // page 196
    //  Intersecting a ray with a non-empty group
    #[test]
    fn test_intersecting_ray_with_non_empty_group() {
        // sphere 1
        let s1 = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));

        // sphere 2
        let translation2 = Matrix::translation(0.0, 0.0, -3.0);
        let mut s2 = Sphere::new();
        s2.set_transformation(translation2);
        let s2 = Shape::new(ShapeEnum::SphereEnum(s2));

        // sphere 3
        let translation3 = Matrix::translation(5.0, 0.0, 0.0);
        let mut s3 = Sphere::new();
        s3.set_transformation(translation3);
        let s3 = Shape::new(ShapeEnum::SphereEnum(s3));

        let mut shapes: ShapeArr = vec![];
        let group_idx = Group::new(&mut shapes, "group".to_string());

        let _s1_idx = Group::add_child(&mut shapes, group_idx, s1);
        let _s2_idx = Group::add_child(&mut shapes, group_idx, s2);
        let _s3_idx = Group::add_child(&mut shapes, group_idx, s3);

        Group::print_tree(&shapes, group_idx, 0);

        let shape = shapes.get(group_idx).unwrap();

        let r = Ray::new(Tuple4D::new_point(0.0, 0.0, -5.0), Tuple4D::new_vector(0.0, 0.0, 1.0));
        let is = Group::intersect_local(shape, r, &shapes);
        assert_eq!(is.get_intersections().len(), 4);
    }

    // page 197
    //  Intersecting a ray with a transformed group
    #[test]
    fn test_intersecting_ray_with_transformed_group() {
        let mut shapes: ShapeArr = vec![];
        let group_idx = Group::new(&mut shapes, "group".to_string());
        let group = shapes.get_mut(group_idx).unwrap();

        let scale = Matrix::scale(2.0, 2.0, 2.0);
        group.set_transformation(scale);

        // sphere 1
        let translation = Matrix::translation(5.0, 0.0, 0.0);
        let mut sphere = Sphere::new();
        sphere.set_transformation(translation);
        let sphere = Shape::new(ShapeEnum::SphereEnum(sphere));

        let _s1_idx = Group::add_child(&mut shapes, group_idx, sphere);

        Group::print_tree(&shapes, group_idx, 0);

        let group = shapes.get(group_idx).unwrap();

        let r = Ray::new(Tuple4D::new_point(10.0, 0.0, -10.0), Tuple4D::new_vector(0.0, 0.0, 1.0));
        let is = Group::intersect_local(group, r, &shapes);
        assert_eq!(is.get_intersections().len(), 2);

        println!("intersection 1 {}", is.get_intersections().get(0).unwrap().get_t());
        println!("intersection 2 {}", is.get_intersections().get(1).unwrap().get_t());
    }

    // page 198
    // Converting a point from world to object space
    #[test]
    fn test_converting_point_from_world_to_object_space() {
        let mut shapes: ShapeArr = vec![];

        // group 1
        let group1_idx = Group::new(&mut shapes, "group".to_string());
        let group1 = shapes.get_mut(group1_idx).unwrap();
        let rot_y = Matrix::rotate_y(PI / 2.0);
        group1.set_transformation(rot_y);

        // group 2
        let group2_idx = Group::new(&mut shapes, "group".to_string());
        let group2 = shapes.get_mut(group2_idx).unwrap();
        let scale = Matrix::scale(2.0, 2.0, 2.0);
        group2.set_transformation(scale);
        Group::add_child_idx(&mut shapes, group1_idx, group2_idx);

        // sphere
        let translation = Matrix::translation(5.0, 0.0, 0.0);
        let mut sphere = Sphere::new();
        sphere.set_transformation(translation);
        let sphere = Shape::new(ShapeEnum::SphereEnum(sphere));

        // add sphere as child to group2
        let sphere_idx = Group::add_child(&mut shapes, group2_idx, sphere);

        let sphere = shapes.get(sphere_idx).unwrap();
        let p = world_to_object(&sphere, &Tuple4D::new_point(-2.0, 0.0, -10.0), &shapes);
        let expected = Tuple4D::new_point(0.0, 0.0, -1.0);
        println!("actual {:?}        expected {:?}", &p, &expected);

        assert_tuple(&p, &expected);
    }

    // page 198
    // Converting a normal from  object to world  space
    #[test]
    fn test_converting_normal_from_object_to_world_space() {
        let mut shapes: ShapeArr = vec![];

        // group 1
        let group1_idx = Group::new(&mut shapes, "group".to_string());
        let group1 = shapes.get_mut(group1_idx).unwrap();
        let rot_y = Matrix::rotate_y(PI / 2.0);
        group1.set_transformation(rot_y);

        // group 2
        let group2_idx = Group::new(&mut shapes, "group".to_string());
        let group2 = shapes.get_mut(group2_idx).unwrap();
        let scale = Matrix::scale(1.0, 2.0, 3.0);
        group2.set_transformation(scale);
        Group::add_child_idx(&mut shapes, group1_idx, group2_idx);

        // sphere
        let translation = Matrix::translation(5.0, 0.0, 0.0);
        let mut sphere = Sphere::new();
        sphere.set_transformation(translation);
        let sphere = Shape::new(ShapeEnum::SphereEnum(sphere));

        // add sphere as child to group2
        let sphere_idx = Group::add_child(&mut shapes, group2_idx, sphere);

        let sphere = shapes.get(sphere_idx).unwrap();
        let sqrt3_3 = (3.0 as f64).sqrt() / 3.0;
        let vec = normal_to_world(&sphere, &Tuple4D::new_vector(sqrt3_3, sqrt3_3, sqrt3_3), &shapes);
        let expected = Tuple4D::new_vector(0.28571427, 0.42857146, -0.8571429);
        println!("actual {:?}        expected {:?}", &vec, &expected);

        assert_tuple(&vec, &expected);
    }

    // page 199
    // finding a normal on a child object
    #[test]
    fn test_finding_normal_on_child_object() {
        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        let intersection = Intersection::new(1.0, &shape);
        let mut shapes: ShapeArr = vec![];

        // group 1
        let group1_idx = Group::new(&mut shapes, "group".to_string());
        let group1 = shapes.get_mut(group1_idx).unwrap();
        let rot_y = Matrix::rotate_y(PI / 2.0);
        group1.set_transformation(rot_y);

        // group 2
        let group2_idx = Group::new(&mut shapes, "group".to_string());
        let group2 = shapes.get_mut(group2_idx).unwrap();
        let scale = Matrix::scale(1.0, 2.0, 3.0);
        group2.set_transformation(scale);
        Group::add_child_idx(&mut shapes, group1_idx, group2_idx);

        // sphere
        let translation = Matrix::translation(5.0, 0.0, 0.0);
        let mut sphere = Sphere::new();
        sphere.set_transformation(translation);
        let sphere = Shape::new(ShapeEnum::SphereEnum(sphere));

        // add sphere as child to group2
        let sphere_idx = Group::add_child(&mut shapes, group2_idx, sphere);

        let sphere = shapes.get(sphere_idx).unwrap();
        let n = sphere.normal_at(&Tuple4D::new_point(1.7321, 1.1547, -5.5774), &shapes, &intersection);
        let expected = Tuple4D::new_vector(0.28570366, 0.42854306, -0.8571606);
        println!("actual {:?}        expected {:?}", &n, &expected);

        assert_tuple(&n, &expected);
    }
}
