use std::fmt;

use crate::basics::{Intersection, IntersectionList, IntersectionListOps, IntersectionOps, Ray, RayOps};
use crate::material::Material;
use crate::math::{Matrix, MatrixOps, Tuple4D};
use crate::prelude::{Shape, ShapeArr, ShapeEnum, ShapeIdx, ShapeIntersectOps, ShapeOps};

#[derive(Clone, PartialEq, Debug)]
pub enum CsgOp {
    UNION,
    INTERSECTION,
    DIFFERENCE,
}

#[derive(Clone, PartialEq)]
pub struct Csg {
    shape_idx: ShapeIdx,
    left: Option<ShapeIdx>,
    right: Option<ShapeIdx>,
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    op: CsgOp,
}

impl<'a> ShapeOps<'a> for Csg {
    fn set_transformation(&mut self, m: Matrix) {
        println!("setting new transformation matrix {:?}", &m);
        println!("old transofrmation matrix  {:?}", self.get_transformation());
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
        unreachable!("this should never be called");
    }

    fn get_children_mut(&mut self) -> &mut Vec<ShapeIdx> {
        unreachable!("this should never be called");
    }
}

impl<'a> ShapeIntersectOps<'a> for Csg {
    fn intersect_local(shape: &'a Shape, r: Ray, shapes: &'a ShapeArr) -> IntersectionList<'a> {
        let mut intersection_list = IntersectionList::new();
        let root_idx = match shape.get_shape() {
            ShapeEnum::CsgEnum(g) => Some(g.shape_idx),
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
            let mut xs = match shape.get_shape() {
                ShapeEnum::SphereEnum(ref _sphere) => Shape::intersects(shape, r.clone(), shapes),
                ShapeEnum::PlaneEnum(ref _plane) => Shape::intersects(shape, r.clone(), shapes),
                ShapeEnum::CylinderEnum(ref _cylinder) => Shape::intersects(shape, r.clone(), shapes),
                ShapeEnum::CubeEnum(ref _cube) => Shape::intersects(shape, r.clone(), shapes),
                ShapeEnum::TriangleEnum(ref _triangle) => Shape::intersects(shape, r.clone(), shapes),
                ShapeEnum::SmoothTriangleEnum(ref _triangle) => Shape::intersects(shape, r.clone(), shapes),
                ShapeEnum::GroupEnum(ref _group) => Shape::intersects(shape, r.clone(), shapes),
                ShapeEnum::CsgEnum(ref _csg) => Shape::intersects(shape, r.clone(), shapes),
            };

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

impl<'a> Csg {
    pub fn new(shapes: &mut ShapeArr, name: String, op: CsgOp) -> ShapeIdx {
        let idx = shapes.len();
        let g = Csg {
            shape_idx: idx,
            left: None,
            right: None,
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            op,
        };
        let g = ShapeEnum::CsgEnum(g);
        let shape = Shape::new_with_name(g, name);
        shapes.push(shape);
        assert_eq!(idx, shapes.len() - 1);
        idx
    }

    pub fn add_child(
        shapes: &mut ShapeArr,
        parent_idx: ShapeIdx,
        mut shape_left: Shape,
        mut shape_right: Shape,
    ) -> (ShapeIdx, ShapeIdx) {
        shape_right.set_parent(parent_idx);
        shape_left.set_parent(parent_idx);
        shapes.push(shape_left);
        let shape_idx_left = shapes.len() - 1;
        shapes.push(shape_right);
        let shape_idx_right = shapes.len() - 1;
        let parent = shapes.get_mut(parent_idx).unwrap();
        parent.set_left(shape_idx_left);
        parent.set_right(shape_idx_right);
        (shape_idx_left, shape_idx_right)
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

    pub fn get_left(&self) -> ShapeIdx {
        self.left.unwrap()
    }

    pub fn get_right(&self) -> ShapeIdx {
        self.right.unwrap()
    }

    pub fn set_left(&mut self, idx: ShapeIdx) {
        self.left = Some(idx);
    }

    pub fn set_right(&mut self, idx: ShapeIdx) {
        self.right = Some(idx);
    }

    pub fn get_op(&self) -> &CsgOp {
        &self.op
    }
}

pub fn intersection_allowed(op: &CsgOp, lhit: bool, inl: bool, inr: bool) -> bool {
    match op {
        CsgOp::UNION => (lhit && !inr) || (!lhit && !inl),
        CsgOp::INTERSECTION => (lhit && inr) || (!lhit && inl),
        CsgOp::DIFFERENCE => (lhit && !inr) || (!lhit && inl),
    }
}

pub fn filter_intersections<'a>(csg: &Shape, xs: &IntersectionList<'a>, shapes: &ShapeArr) -> IntersectionList<'a> {
    let mut inl = false;
    let mut inr = false;

    let mut result = IntersectionList::new();

    for i in xs.get_intersections() {
        let left = shapes.get(csg.get_left() as usize).unwrap();
        let lhit = a_includes_b(i.get_shape(), left, shapes);

        if intersection_allowed(csg.get_op(), lhit, inl, inr) {
            result.add((*i).clone());
        }

        if lhit {
            inl = !inl;
        } else {
            inr = !inr;
        }
    }

    result
}

fn a_includes_b(a: &Shape, b: &Shape, shapes: &ShapeArr) -> bool {
    return match a.get_shape() {
        ShapeEnum::GroupEnum(g) => {
            for child in g.get_children() {
                let c = shapes.get(*child as usize).unwrap();
                let res = a_includes_b(a, c, shapes);
                if res {
                    return true;
                }
            }
            false
        }
        ShapeEnum::CsgEnum(csg) => {
            let left = shapes.get(csg.get_left() as usize).unwrap();
            let r = a_includes_b(a, left, shapes);
            if r {
                return true;
            }
            let right = shapes.get(csg.get_right() as usize).unwrap();
            return a_includes_b(a, right, shapes);
        }
        _ => {
            return a_includes_b(a, b, shapes);
        }
    };
}

impl fmt::Debug for Csg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Csg").field("value", &"TODO Csg".to_string()).finish()
    }
}

impl fmt::Display for Csg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent_msg = String::new();
        // match self.parent {
        //     Some(p_idx) => parent_msg.push_str(format!("parent {}", p_idx).as_str()),
        //     None => parent_msg.push_str(format!("no parent ").as_str()),
        // }
        write!(f, "Csg: {}   ", parent_msg)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Sphere;
    use crate::shape::shape::{Shape, ShapeEnum};
    use crate::shape::Cube;

    use super::*;

    // page 230
    // CSG is created with an operation and two shapes
    #[test]
    fn test_creating_new_csg_with_2_shapes() {
        let mut shapes: ShapeArr = vec![];

        let s1 = Shape::new_with_name(ShapeEnum::SphereEnum(Sphere::new()), "sphere".to_string());
        let s2 = Shape::new_with_name(ShapeEnum::CubeEnum(Cube::new()), "cube".to_string());

        let csg_idx = Csg::new(&mut shapes, "csg".to_string(), CsgOp::UNION);
        let (shape_idx_left, shape_idx_right) = Csg::add_child(&mut shapes, csg_idx, s1, s2);

        let csg = shapes.get(csg_idx as usize).unwrap();
        assert_eq!(*csg.get_op(), CsgOp::UNION);

        let s1 = shapes.get(shape_idx_left as usize).unwrap();
        assert_eq!(s1.get_name().as_ref().unwrap(), &"sphere".to_string());

        let s2 = shapes.get(shape_idx_right as usize).unwrap();
        assert_eq!(s2.get_name().as_ref().unwrap(), &"cube".to_string());

        // let m = group.get_transformation();
        // assert_matrix(&m, &Matrix::new_identity_4x4());
    }

    // page 231
    // Evaluating the rule for a CSG operation
    #[test]
    fn test_evaluating_the_rule_for_a_csg_operation() {
        assert_eq!(false, intersection_allowed(&CsgOp::UNION, true, true, true));
        assert_eq!(true, intersection_allowed(&CsgOp::UNION, true, true, false));
        assert_eq!(false, intersection_allowed(&CsgOp::UNION, true, false, true));
        assert_eq!(true, intersection_allowed(&CsgOp::UNION, true, false, false));
        assert_eq!(false, intersection_allowed(&CsgOp::UNION, false, true, true));
        assert_eq!(false, intersection_allowed(&CsgOp::UNION, false, true, false));
        assert_eq!(true, intersection_allowed(&CsgOp::UNION, false, false, true));
        assert_eq!(true, intersection_allowed(&CsgOp::UNION, false, false, false));

        assert_eq!(true, intersection_allowed(&CsgOp::INTERSECTION, true, true, true));
        assert_eq!(false, intersection_allowed(&CsgOp::INTERSECTION, true, true, false));
        assert_eq!(true, intersection_allowed(&CsgOp::INTERSECTION, true, false, true));
        assert_eq!(false, intersection_allowed(&CsgOp::INTERSECTION, true, false, false));
        assert_eq!(true, intersection_allowed(&CsgOp::INTERSECTION, false, true, true));
        assert_eq!(true, intersection_allowed(&CsgOp::INTERSECTION, false, true, false));
        assert_eq!(false, intersection_allowed(&CsgOp::INTERSECTION, false, false, true));
        assert_eq!(false, intersection_allowed(&CsgOp::INTERSECTION, false, false, false));

        assert_eq!(false, intersection_allowed(&CsgOp::DIFFERENCE, true, true, true));
        assert_eq!(true, intersection_allowed(&CsgOp::DIFFERENCE, true, true, false));
        assert_eq!(false, intersection_allowed(&CsgOp::DIFFERENCE, true, false, true));
        assert_eq!(true, intersection_allowed(&CsgOp::DIFFERENCE, true, false, false));
        assert_eq!(true, intersection_allowed(&CsgOp::DIFFERENCE, false, true, true));
        assert_eq!(true, intersection_allowed(&CsgOp::DIFFERENCE, false, true, false));
        assert_eq!(false, intersection_allowed(&CsgOp::DIFFERENCE, false, false, true));
        assert_eq!(false, intersection_allowed(&CsgOp::DIFFERENCE, false, false, false));
    }

    // page 234
    // Filtering a list of intertsections
    #[test]
    fn test_filtering_list_of_intersections() {
        let mut shapes: ShapeArr = vec![];

        let s1 = Shape::new_with_name(ShapeEnum::SphereEnum(Sphere::new()), "sphere".to_string());
        let s2 = Shape::new_with_name(ShapeEnum::CubeEnum(Cube::new()), "cube".to_string());

        let s1_clone = s1.clone();
        let s2_clone = s2.clone();

        let i1 = Intersection::new(1.0, &s1);
        let i2 = Intersection::new(2.0, &s2);
        let i3 = Intersection::new(3.0, &s1);
        let i4 = Intersection::new(4.0, &s2);

        let mut xs = IntersectionList::new();
        xs.add(i1);
        xs.add(i2);
        xs.add(i3);
        xs.add(i4);

        let csg_idx = Csg::new(&mut shapes, "csg".to_string(), CsgOp::UNION);
        let (shape_idx_left, shape_idx_right) = Csg::add_child(&mut shapes, csg_idx, s1_clone, s2_clone);

        let csg = shapes.get(csg_idx as usize).unwrap();

        test_filtering_list_of_intersections_helper(csg, CsgOp::UNION, &xs, 0, 3, &shapes);
        test_filtering_list_of_intersections_helper(csg, CsgOp::INTERSECTION, &xs, 1, 2, &shapes);
        test_filtering_list_of_intersections_helper(csg, CsgOp::DIFFERENCE, &xs, 0, 1, &shapes);
    }

    fn test_filtering_list_of_intersections_helper(
        csg: &Shape,
        op: CsgOp,
        xs: &IntersectionList,
        expected_x0_idx: usize,
        expected_x1_idx: usize,
        shapes: &ShapeArr,
    ) {
        let res = filter_intersections(csg, xs, shapes);

        assert_eq!(res.get_intersections().len(), 2);

        assert_eq!(
            res.get_intersections().get(0).unwrap().get_t(),
            xs.get_intersections().get(expected_x0_idx).unwrap().get_t()
        );
        assert_eq!(
            res.get_intersections().get(1).unwrap().get_t(),
            xs.get_intersections().get(expected_x1_idx).unwrap().get_t()
        );
    }
}
