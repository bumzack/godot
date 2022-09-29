use crate::basics::{Ray, RayOps};
use std::fmt;

use crate::math::{max_float, min_float, Matrix, Tuple, Tuple4D, EPSILON};

#[derive(Clone, PartialEq)]
pub struct BoundingBox {
    pub min: Tuple4D,
    pub max: Tuple4D,
}

impl BoundingBox {
    pub fn new() -> BoundingBox {
        BoundingBox {
            min: Tuple4D::new_point(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: Tuple4D::new_point(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY),
        }
    }

    pub fn new_from_min_max(min: Tuple4D, max: Tuple4D) -> BoundingBox {
        BoundingBox { min, max }
    }

    pub fn add_point(&mut self, point: &Tuple4D) {
        if point.x < self.min.x {
            self.min.x = point.x;
        }
        if point.y < self.min.y {
            self.min.y = point.y;
        }
        if point.z < self.min.z {
            self.min.z = point.z;
        }

        if point.x > self.max.x {
            self.max.x = point.x;
        }
        if point.y > self.max.y {
            self.max.y = point.y;
        }
        if point.z > self.max.z {
            self.max.z = point.z;
        }
    }

    pub fn contains_point(&self, point: &Tuple4D) -> bool {
        (self.min.x <= point.x)
            & (point.x <= self.max.x)
            & (self.min.y <= point.y)
            & (point.y <= self.max.y)
            & (self.min.z <= point.z)
            & (point.z <= self.max.z)
    }

    pub fn contains_box(&self, bb: &BoundingBox) -> bool {
        self.contains_point(bb.get_min()) & (self.contains_point(bb.get_max()))
    }

    pub fn transform(bb: &BoundingBox, m: &Matrix) -> BoundingBox {
        let points = vec![
            bb.min.clone(),
            Tuple4D::new_point(bb.min.x, bb.min.y, bb.max.z),
            Tuple4D::new_point(bb.min.x, bb.max.y, bb.min.z),
            Tuple4D::new_point(bb.min.x, bb.max.y, bb.max.z),
            Tuple4D::new_point(bb.max.x, bb.min.y, bb.min.z),
            Tuple4D::new_point(bb.max.x, bb.min.y, bb.max.z),
            Tuple4D::new_point(bb.max.x, bb.max.y, bb.min.z),
            bb.max.clone(),
        ];
        let mut bb = BoundingBox::new();
        points.iter().for_each(|p| {
            let p1 = m * p;
            bb.add_point(&p1);
        });
        bb
    }

    pub fn intersects(&self, r: &Ray) -> bool {
        let (xt_min, xt_max) = Self::check_axis(r.get_origin().x, r.get_direction().x, self.min.x, self.max.x);
        let (yt_min, yt_max) = Self::check_axis(r.get_origin().y, r.get_direction().y, self.min.y, self.max.y);
        let (zt_min, zt_max) = Self::check_axis(r.get_origin().z, r.get_direction().z, self.min.z, self.max.z);

        let tmin = max_float(xt_min, yt_min, zt_min);
        let tmax = min_float(xt_max, yt_max, zt_max);

        if tmin > tmax {
            return false;
        }

        if tmin == f64::NAN {
            println!("BoundingBox: here we have a NAN tmin is {}", tmin);
        }

        if tmax == f64::NAN {
            println!("BoundingBox:  here we have a NAN tmax is {}", tmax);
        }

        true
    }

    pub fn check_axis(origin: f64, direction: f64, min: f64, max: f64) -> (f64, f64) {
        let tmin_numerator = min - origin;
        let tmax_numerator = max - origin;

        let mut tmin;
        let mut tmax;

        if direction.abs() >= EPSILON {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        } else {
            tmin = tmin_numerator * f64::INFINITY;
            tmax = tmax_numerator * f64::INFINITY;
        }
        if tmin > tmax {
            let tmp = tmin;
            tmin = tmax;
            tmax = tmp;
        }
        (tmin, tmax)
    }

    pub fn add(&mut self, bb: &BoundingBox) {
        self.add_point(bb.get_min());
        self.add_point(bb.get_max());
    }

    pub fn get_min(&self) -> &Tuple4D {
        &self.min
    }

    pub fn get_max(&self) -> &Tuple4D {
        &self.max
    }
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent_msg = String::new();
        // match self.parent {
        //     Some(p_idx) => parent_msg.push_str(format!("parent {}", p_idx).as_str()),
        //     None => parent_msg.push_str(format!("no parent ").as_str()),
        // }
        write!(f, "Csg: {}   ", parent_msg)
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::basics::{Ray, RayOps};
    use crate::math::{assert_tuple, Matrix, MatrixOps, Tuple};
    use crate::prelude::{Csg, Cylinder, Shape, ShapeOps, Sphere};
    use crate::shape::{CsgOp, Cube, Group, Plane, ShapeEnum, SmoothTriangle, Triangle};

    use super::*;

    // bonus bounding box
    // Create an empty bounding box
    #[test]
    fn test_creating_empty_bounding_box() {
        let bb = BoundingBox::new();
        let min = Tuple4D::new_point(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let max = Tuple4D::new_point(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        assert_eq!(bb.get_min().x, min.x);
        assert_eq!(bb.get_min().y, min.y);
        assert_eq!(bb.get_min().z, min.z);

        assert_eq!(bb.get_max().x, max.x);
        assert_eq!(bb.get_max().y, max.y);
        assert_eq!(bb.get_max().z, max.z);
    }

    // bonus bounding box
    // Create a bounding box with volume
    #[test]
    fn test_creating_bounding_box_with_volume() {
        let min = Tuple4D::new_point(-1.0, -2.0, -3.0);
        let max = Tuple4D::new_point(1.0, 2.0, 3.0);
        let min_c = min.clone();
        let max_c = max.clone();

        let bb = BoundingBox::new_from_min_max(min, max);

        assert_tuple(bb.get_min(), &min_c);
        assert_tuple(bb.get_max(), &max_c);
    }

    // bonus bounding box
    // Adding Points to a an empty bounding box
    #[test]
    fn test_adding_points_to_empty_bounding_box() {
        let mut bb = BoundingBox::new();

        let p1 = Tuple4D::new_point(-5.0, 2.0, 0.0);
        let p2 = Tuple4D::new_point(7.0, 0.0, -3.0);

        bb.add_point(&p1);
        bb.add_point(&p2);

        let min = Tuple4D::new_point(-5.0, 0.0, -3.0);
        let max = Tuple4D::new_point(7.0, 2.0, 0.0);

        assert_tuple(bb.get_min(), &min);
        assert_tuple(bb.get_max(), &max);
    }

    // bonus bounding box
    // A sphere has a bounding box
    #[test]
    fn test_bounding_box_sphere() {
        let sphere = Sphere::new();
        let shape = Shape::new(ShapeEnum::SphereEnum(sphere));

        let shapes = vec![];
        let bb = shape.get_bounds_of(&shapes);

        let p1 = Tuple4D::new_point(-1.0, -1.0, -1.0);
        let p2 = Tuple4D::new_point(1.0, 1.0, 1.0);

        assert_tuple(bb.get_min(), &p1);
        assert_tuple(bb.get_max(), &p2);
    }

    // bonus bounding box
    // A plane has a bounding box
    #[test]
    fn test_bounding_box_plane() {
        let plane = Plane::new();
        let shape = Shape::new(ShapeEnum::PlaneEnum(plane));

        let shapes = vec![];
        let bb = shape.get_bounds_of(&shapes);

        let p1 = Tuple4D::new_point(-f64::INFINITY, 0.0, -f64::INFINITY);
        let p2 = Tuple4D::new_point(f64::INFINITY, 0.0, f64::INFINITY);

        assert_eq!(bb.get_min().x, p1.x);
        assert_eq!(bb.get_min().y, p1.y);
        assert_eq!(bb.get_min().z, p1.z);

        assert_eq!(bb.get_max().x, p2.x);
        assert_eq!(bb.get_max().y, p2.y);
        assert_eq!(bb.get_max().z, p2.z);
    }

    // bonus bounding box
    // A cube has a bounding box
    #[test]
    fn test_bounding_box_cube() {
        let cube = Cube::new();
        let shape = Shape::new(ShapeEnum::CubeEnum(cube));

        let shapes = vec![];
        let bb = shape.get_bounds_of(&shapes);

        let p1 = Tuple4D::new_point(-1.0, -1.0, -1.0);
        let p2 = Tuple4D::new_point(1.0, 1.0, 1.0);

        assert_tuple(bb.get_min(), &p1);
        assert_tuple(bb.get_max(), &p2);
    }

    // bonus bounding box
    // An unbounded cylinder has a bounding box
    #[test]
    fn test_bounding_box_unbounded_cylinder() {
        let cylinder = Cylinder::new();
        let shape = Shape::new(ShapeEnum::CylinderEnum(cylinder));

        let shapes = vec![];
        let bb = shape.get_bounds_of(&shapes);

        let p1 = Tuple4D::new_point(-1.0, -f64::INFINITY, -1.0);
        let p2 = Tuple4D::new_point(1.0, f64::INFINITY, 1.0);

        assert_eq!(bb.get_min().x, p1.x);
        assert_eq!(bb.get_min().y, p1.y);
        assert_eq!(bb.get_min().z, p1.z);

        assert_eq!(bb.get_max().x, p2.x);
        assert_eq!(bb.get_max().y, p2.y);
        assert_eq!(bb.get_max().z, p2.z);
    }

    // bonus bounding box
    // A bounded cylinder has a bounding box
    #[test]
    fn test_bounding_box_bounded_cylinder() {
        let mut cylinder = Cylinder::new();
        cylinder.set_minimum(-5.0);
        cylinder.set_maximum(3.0);
        let shape = Shape::new(ShapeEnum::CylinderEnum(cylinder));

        let shapes = vec![];
        let bb = shape.get_bounds_of(&shapes);

        let p1 = Tuple4D::new_point(-1.0, -5.0, -1.0);
        let p2 = Tuple4D::new_point(1.0, 3.0, 1.0);

        assert_eq!(bb.get_min().x, p1.x);
        assert_eq!(bb.get_min().y, p1.y);
        assert_eq!(bb.get_min().z, p1.z);

        assert_eq!(bb.get_max().x, p2.x);
        assert_eq!(bb.get_max().y, p2.y);
        assert_eq!(bb.get_max().z, p2.z);
    }

    // // bonus bounding box
    // // An unbounded cone has a bounding box
    // #[test]
    // fn test_unbounded_box_bounded_cone() {
    //     let mut cone = Cone::new();
    //     let shape = Shape::new(ShapeEnum::ConeEnum(cone));
    //
    //     let bb = shape.get_bounds_of();
    //
    //     let p1 = Tuple4D::new_point(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);
    //     let p2 = Tuple4D::new_point(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    //
    //
    //     assert_eq!(bb.get_min().x, p1.x);
    //     assert_eq!(bb.get_min().y, p1.y);
    //     assert_eq!(bb.get_min().z, p1.z);
    //
    //     assert_eq!(bb.get_max().x, p2.x);
    //     assert_eq!(bb.get_max().y, p2.y);
    //     assert_eq!(bb.get_max().z, p2.z);
    // }
    //
    // // bonus bounding box
    // // A bounded cone has a bounding box
    // #[test]
    // fn test_bounding_box_bounded_cone() {
    //     let mut cone = Cone::new();
    //     cone.set_minimum(-5.0);
    //     cone.set_maximum(3.0);
    //     let shape = Shape::new(ShapeEnum::ConeEnum(cone));
    //
    //     let bb = shape.get_bounds_of();
    //
    //     let p1 = Tuple4D::new_point(-5.0, -5.0, -5.0);
    //     let p2 = Tuple4D::new_point(3.0, 3.0, 3.0);
    //
    //     assert_eq!(bb.get_min().x, p1.x);
    //     assert_eq!(bb.get_min().y, p1.y);
    //     assert_eq!(bb.get_min().z, p1.z);
    //
    //     assert_eq!(bb.get_max().x, p2.x);
    //     assert_eq!(bb.get_max().y, p2.y);
    //     assert_eq!(bb.get_max().z, p2.z);
    // }

    // bonus bounding box
    // An triangle has a bounding box
    #[test]
    fn test_bounding_box_triangle() {
        let p1 = Tuple4D::new_point(-3.0, 7.0, 2.0);
        let p2 = Tuple4D::new_point(6.0, 2.0, -4.0);
        let p3 = Tuple4D::new_point(2.0, -1.0, -1.0);

        let triangle = Triangle::new(p1, p2, p3);
        let shape = Shape::new(ShapeEnum::TriangleEnum(triangle));

        let shapes = vec![];
        let bb = shape.get_bounds_of(&shapes);

        let p1 = Tuple4D::new_point(-3.0, -1.0, -4.0);
        let p2 = Tuple4D::new_point(6.0, 7.0, 2.0);

        assert_eq!(bb.get_min().x, p1.x);
        assert_eq!(bb.get_min().y, p1.y);
        assert_eq!(bb.get_min().z, p1.z);

        assert_eq!(bb.get_max().x, p2.x);
        assert_eq!(bb.get_max().y, p2.y);
        assert_eq!(bb.get_max().z, p2.z);
    }

    // bonus bounding box
    // An smooth_triangle has a bounding box
    #[test]
    fn test_bounding_box_smooth_triangle() {
        let p1 = Tuple4D::new_point(-3.0, 7.0, 2.0);
        let p2 = Tuple4D::new_point(6.0, 2.0, -4.0);
        let p3 = Tuple4D::new_point(2.0, -1.0, -1.0);

        let n = Tuple4D::new_vector(1.0, 1.0, 1.0);

        let triangle = SmoothTriangle::new(p1, p2, p3, n.clone(), n.clone(), n);
        let shape = Shape::new(ShapeEnum::SmoothTriangleEnum(triangle));

        let shapes = vec![];
        let bb = shape.get_bounds_of(&shapes);

        let p1 = Tuple4D::new_point(-3.0, -1.0, -4.0);
        let p2 = Tuple4D::new_point(6.0, 7.0, 2.0);

        assert_eq!(bb.get_min().x, p1.x);
        assert_eq!(bb.get_min().y, p1.y);
        assert_eq!(bb.get_min().z, p1.z);

        assert_eq!(bb.get_max().x, p2.x);
        assert_eq!(bb.get_max().y, p2.y);
        assert_eq!(bb.get_max().z, p2.z);
    }

    // bonus bounding box
    // Adding one box to another
    #[test]
    fn test_bounding_box_adding_to_another() {
        let min = Tuple4D::new_point(-5.0, -2.0, 0.0);
        let max = Tuple4D::new_point(7.0, 4.0, 4.0);
        let mut box1 = BoundingBox::new_from_min_max(min, max);

        let min = Tuple4D::new_point(8.0, -7.0, -2.0);
        let max = Tuple4D::new_point(14.0, 2.0, 8.0);
        let box2 = BoundingBox::new_from_min_max(min, max);

        box1.add(&box2);

        let min = Tuple4D::new_point(-5.0, -7.0, -2.0);
        let max = Tuple4D::new_point(14.0, 4.0, 8.0);

        assert_eq!(box1.get_min().x, min.x);
        assert_eq!(box1.get_min().y, min.y);
        assert_eq!(box1.get_min().z, min.z);

        assert_eq!(box1.get_max().x, max.x);
        assert_eq!(box1.get_max().y, max.y);
        assert_eq!(box1.get_max().z, max.z);
    }

    // bonus bounding box
    // Checking to see if a box contains a given point
    #[test]
    fn test_bounding_box_check_if_box_contains_point() {
        let min = Tuple4D::new_point(5.0, -2.0, 0.0);
        let max = Tuple4D::new_point(11.0, 4.0, 7.0);
        let b = BoundingBox::new_from_min_max(min, max);

        let data = vec![
            (Tuple4D::new_point(5.0, -2.0, 0.0), true),
            (Tuple4D::new_point(11.0, 4.0, 7.0), true),
            (Tuple4D::new_point(8.0, 1.0, 3.0), true),
            (Tuple4D::new_point(3.0, 0.0, 3.0), false),
            (Tuple4D::new_point(8.0, -4.0, 3.0), false),
            (Tuple4D::new_point(8.0, 1.0, -1.0), false),
            (Tuple4D::new_point(13.0, 1.0, 3.0), false),
            (Tuple4D::new_point(8.0, 5.0, 3.0), false),
            (Tuple4D::new_point(8.0, 1.0, 8.0), false),
        ];

        data.iter().for_each(|d| {
            println!("comparing {:?} -> {}", &d.0, d.1);
            assert_eq!(b.contains_point(&d.0), d.1)
        });
    }

    // bonus bounding box
    // Checking to see if a box contains a given box
    #[test]
    fn test_bounding_box_check_if_box_contains_box() {
        let min = Tuple4D::new_point(5.0, -2.0, 0.0);
        let max = Tuple4D::new_point(11.0, 4.0, 7.0);
        let b = BoundingBox::new_from_min_max(min, max);

        let data = vec![
            // (Tuple4D::new_point(5.0, -2.0, 0.0), Tuple4D::new_point(11.0, 4.0, 7.0), true),
            // (Tuple4D::new_point(6.0, -1.0, 1.0), Tuple4D::new_point(10.0, 4.0, 6.0), true),
            // (Tuple4D::new_point(4.0, -3.0, -1.0), Tuple4D::new_point(10.0, 3.0, 6.0), false),
            (
                Tuple4D::new_point(6.0, -1.0, 1.0),
                Tuple4D::new_point(12.0, 5.0, 8.0),
                false,
            ),
        ];

        data.iter().for_each(|d| {
            println!("comparing {:?}  / {:?}   ==>   {}", &d.0, d.1, d.2);
            let box2 = BoundingBox::new_from_min_max(d.0.clone(), d.1.clone());
            let x = b.contains_box(&box2);
            println!("x = {}", x);
            assert_eq!(x, d.2)
        });
    }

    // bonus bounding box
    // Transforming bounding boxes
    #[test]
    fn test_bounding_box_transforming_a_box() {
        let min = Tuple4D::new_point(-1.0, -1.0, -1.0);
        let max = Tuple4D::new_point(1.0, 1.0, 1.0);
        let b = BoundingBox::new_from_min_max(min, max);

        let matrix = &Matrix::rotate_x(PI / 4.0) * &Matrix::rotate_y(PI / 4.0);
        let b2 = BoundingBox::transform(&b, &matrix);

        let min = Tuple4D::new_point(-1.414213562373095, -1.7071067811865475, -1.7071067811865475);
        let max = Tuple4D::new_point(1.414213562373095, 1.7071067811865475, 1.7071067811865475);

        assert_eq!(b2.get_min().x, min.x);
        assert_eq!(b2.get_min().y, min.y);
        assert_eq!(b2.get_min().z, min.z);

        assert_eq!(b2.get_max().x, max.x);
        assert_eq!(b2.get_max().y, max.y);
        assert_eq!(b2.get_max().z, max.z);
    }

    // bonus bounding box
    // Querying a shpaes bounding box in its parents space
    #[test]
    fn test_bounding_box_query_in_parent_space() {
        let mut s = Sphere::new();
        let trans = &Matrix::translation(1.0, -3.0, 5.0) * &Matrix::scale(0.5, 2.0, 4.0);
        s.set_transformation(trans);
        let shape = Shape::new(ShapeEnum::SphereEnum(s));
        let shapes = vec![];
        let b2 = Shape::get_parent_space_bounds_of(&shape, &shapes);

        let min = Tuple4D::new_point(0.5, -5.0, 1.0);
        let max = Tuple4D::new_point(1.5, -1.0, 9.0);

        assert_eq!(b2.get_min().x, min.x);
        assert_eq!(b2.get_min().y, min.y);
        assert_eq!(b2.get_min().z, min.z);

        assert_eq!(b2.get_max().x, max.x);
        assert_eq!(b2.get_max().y, max.y);
        assert_eq!(b2.get_max().z, max.z);
    }

    // bonus bounding box
    // A group has a bounding box that contains its children
    #[test]
    fn test_bounding_box_that_contains_its_children() {
        let mut s = Sphere::new();
        let trans = &Matrix::translation(2.0, 5.0, -3.0) * &Matrix::scale(2.0, 2.0, 2.0);
        s.set_transformation(trans);
        let sphere = Shape::new(ShapeEnum::SphereEnum(s));

        let mut c = Cylinder::new();
        c.set_minimum(-2.0);
        c.set_maximum(2.0);
        let trans = &Matrix::translation(-4.0, -1.0, 4.0) * &Matrix::scale(0.5, 1.0, 0.5);
        c.set_transformation(trans);
        let cylinder = Shape::new(ShapeEnum::CylinderEnum(c));

        let mut shapes = vec![];
        let group_idx = Group::new(&mut shapes, "group".to_string());

        let _child1_idx = Group::add_child(&mut shapes, group_idx, sphere);
        let _child2_idx = Group::add_child(&mut shapes, group_idx, cylinder);
        let group = shapes.get(group_idx as usize).unwrap();

        let bb = group.get_bounds_of(&shapes);

        let min = Tuple4D::new_point(-4.5, -3.0, -5.0);
        let max = Tuple4D::new_point(4., 7.0, 4.5);

        assert_eq!(bb.get_min().x, min.x);
        assert_eq!(bb.get_min().y, min.y);
        assert_eq!(bb.get_min().z, min.z);

        assert_eq!(bb.get_max().x, max.x);
        assert_eq!(bb.get_max().y, max.y);
        assert_eq!(bb.get_max().z, max.z);
    }

    // bonus bounding box
    // A CSG shape  has a bounding box that contains its children
    #[test]
    fn test_bounding_csg_that_contains_its_children() {
        let s = Sphere::new();
        let left = Shape::new(ShapeEnum::SphereEnum(s));

        let mut s = Sphere::new();
        let trans = Matrix::translation(2.0, 3.0, 4.0);
        s.set_transformation(trans);
        let right = Shape::new(ShapeEnum::SphereEnum(s));

        let mut shapes = vec![];
        let csg = Csg::new(&mut shapes, "csg".to_string(), CsgOp::DIFFERENCE);
        Csg::add_child(&mut shapes, csg, left, right);

        let csg = shapes.get(csg as usize).unwrap();

        let bb = csg.get_bounds_of(&shapes);

        let min = Tuple4D::new_point(-1.0, -1.0, -1.0);
        let max = Tuple4D::new_point(3.0, 4.0, 5.0);

        assert_eq!(bb.get_min().x, min.x);
        assert_eq!(bb.get_min().y, min.y);
        assert_eq!(bb.get_min().z, min.z);

        assert_eq!(bb.get_max().x, max.x);
        assert_eq!(bb.get_max().y, max.y);
        assert_eq!(bb.get_max().z, max.z);
    }

    // bonus bounding box
    // Intersecting  a bounding box with a ray at the origin
    #[test]
    fn test_bounding_intersecting_with_ray_at_orign() {
        let min = Tuple4D::new_point(-1.0, -1.0, -1.0);
        let max = Tuple4D::new_point(1.0, 1.0, 1.0);
        let bb = BoundingBox::new_from_min_max(min, max);

        let data = vec![
            (
                Tuple4D::new_point(5.0, 0.5, 0.0),
                Tuple4D::new_vector(-1.0, 0., 0.0),
                true,
            ),
            (
                Tuple4D::new_point(-5.0, 0.5, 0.0),
                Tuple4D::new_vector(1.0, 0., 0.0),
                true,
            ),
            (
                Tuple4D::new_point(0.5, 5.0, 0.0),
                Tuple4D::new_vector(0.0, -1.0, 0.0),
                true,
            ),
            (
                Tuple4D::new_point(0.5, -5., 0.0),
                Tuple4D::new_vector(0.0, 1., 0.0),
                true,
            ),
            (
                Tuple4D::new_point(0.5, 0., 5.0),
                Tuple4D::new_vector(0.0, 0., -1.0),
                true,
            ),
            (
                Tuple4D::new_point(0.5, 0.5, -5.0),
                Tuple4D::new_vector(0.0, 0., 1.0),
                true,
            ),
            (
                Tuple4D::new_point(0.0, 0.5, 0.0),
                Tuple4D::new_vector(0.0, 0., 1.0),
                true,
            ),
            (
                Tuple4D::new_point(-2.0, 0.0, 0.0),
                Tuple4D::new_vector(2.0, 4., 6.0),
                false,
            ),
            (
                Tuple4D::new_point(0.0, -2.0, 0.0),
                Tuple4D::new_vector(6.0, 2., 4.0),
                false,
            ),
            (
                Tuple4D::new_point(0.0, 0.0, -2.0),
                Tuple4D::new_vector(4.0, 6., 2.0),
                false,
            ),
            (
                Tuple4D::new_point(2.0, 0.0, 2.0),
                Tuple4D::new_vector(0.0, 0., -1.0),
                false,
            ),
            (
                Tuple4D::new_point(0.0, 2.0, 2.0),
                Tuple4D::new_vector(0.0, -1., 0.0),
                false,
            ),
            (
                Tuple4D::new_point(2.0, 2.0, 0.0),
                Tuple4D::new_vector(-1.0, 0., 0.0),
                false,
            ),
        ];

        data.iter().for_each(|d| {
            let direction = Tuple4D::normalize(&d.1);
            let r = Ray::new(d.0, direction);
            let result = bb.intersects(&r);

            println!("testing p {:?}, dir {:?}, result {}", &d.0, &d.1, d.2);
            assert_eq!(result, d.2);
        });
    }

    // bonus bounding box
    // Intersecting a non-cubic bounding box with a ray
    #[test]
    fn test_bounding_intersecting_non_cubic_bounding_box_with_ray() {
        let min = Tuple4D::new_point(5.0, -2.0, 0.0);
        let max = Tuple4D::new_point(11.0, 4.0, 7.0);
        let bb = BoundingBox::new_from_min_max(min, max);

        let data = vec![
            (
                Tuple4D::new_point(15.0, 1., 2.0),
                Tuple4D::new_vector(-1.0, 0., 0.0),
                true,
            ),
            (
                Tuple4D::new_point(-5.0, -1.0, 4.0),
                Tuple4D::new_vector(1.0, 0., 0.0),
                true,
            ),
            (
                Tuple4D::new_point(7.0, 6.0, 5.0),
                Tuple4D::new_vector(0.0, -1.0, 0.0),
                true,
            ),
            (
                Tuple4D::new_point(9.0, -5., 6.0),
                Tuple4D::new_vector(0.0, 1., 0.0),
                true,
            ),
            (
                Tuple4D::new_point(8., 2., 12.0),
                Tuple4D::new_vector(0.0, 0., -1.0),
                true,
            ),
            (
                Tuple4D::new_point(6., 0., -5.0),
                Tuple4D::new_vector(0.0, 0., 1.0),
                true,
            ),
            (
                Tuple4D::new_point(8.0, 1.0, 3.5),
                Tuple4D::new_vector(0.0, 0., 1.0),
                true,
            ),
            (
                Tuple4D::new_point(9.0, -1.0, -8.0),
                Tuple4D::new_vector(2.0, 4., 6.0),
                false,
            ),
            (
                Tuple4D::new_point(8.0, 3.0, -4.0),
                Tuple4D::new_vector(6.0, 2., 4.0),
                false,
            ),
            (
                Tuple4D::new_point(9.0, -1.0, -2.0),
                Tuple4D::new_vector(4.0, 6., 2.0),
                false,
            ),
            (
                Tuple4D::new_point(4.0, 0.0, 9.0),
                Tuple4D::new_vector(0.0, 0., -1.0),
                false,
            ),
            (
                Tuple4D::new_point(8.0, 6.0, -1.0),
                Tuple4D::new_vector(0.0, -1., 0.0),
                false,
            ),
            (
                Tuple4D::new_point(12.0, 5.0, 4.0),
                Tuple4D::new_vector(-1.0, 0., 0.0),
                false,
            ),
        ];

        data.iter().for_each(|d| {
            let direction = Tuple4D::normalize(&d.1);
            let r = Ray::new(d.0, direction);
            let result = bb.intersects(&r);

            println!("testing p {:?}, dir {:?}, result {}", &d.0, &d.1, d.2);
            assert_eq!(result, d.2);
        });
    }

    // bonus bounding box
    // Intersecting ray + group doesn't test children if box is missed
    // TODO: dont have test shape

    // bonus bounding box
    // Intersecting ray + group  tests children if box is hit
    // TODO: dont have test shape
}
