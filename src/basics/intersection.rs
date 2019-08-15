use std::fmt;

use crate::basics::precomputed_component::PrecomputedComponent;
use crate::basics::ray::Ray;
use crate::basics::ray::RayOps;
use crate::math::common::assert_tuple;
use crate::math::common::EPSILON;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;
use crate::shape::plane::{Plane, PlaneOps};
use crate::shape::shape::Shape;
use crate::shape::sphere::{Sphere, SphereOps};
use crate::world::world::World;
use crate::world::world::WorldOps;

pub struct Intersection<'a> {
    t: f64,
    shape: &'a Shape,
}

pub trait IntersectionOps<'a> {
    fn new(t: f64, shape: &Shape) -> Intersection;
    fn intersect(shape: &'a Shape, r: &Ray) -> IntersectionList<'a>;
    fn intersect_world(w: &'a World, r: &'a Ray) -> IntersectionList<'a>;
    fn get_t(&self) -> f64;
    fn get_shape(&self) -> &'a Shape;
    fn prepare_computations(&self, r: &Ray) -> PrecomputedComponent;
}

impl<'a> IntersectionOps<'a> for Intersection<'a> {
    fn new(t: f64, shape: &Shape) -> Intersection {
        Intersection { t, shape }
    }

    fn intersect(shape: &'a Shape, r: &Ray) -> IntersectionList<'a> {
        let mut intersection_list = IntersectionList::new();
        let r2 = Ray::transform(r, shape.get_inverse_transformation());

        let res = match shape {
            Shape::Sphere(ref s) => {
                let res = Sphere::intersect(s, &r2);
                match res {
                    Some(r) => {
                        let i1 = Intersection::new(r[0], shape);
                        let i2 = Intersection::new(r[1], shape);
                        intersection_list.add(i1);
                        intersection_list.add(i2);
                    }
                    None => {}
                }
                intersection_list
            }
            Shape::Plane(ref p) => {
                let res = Plane::intersect(p, &r2);
                match res {
                    Some(r) => {
                        let i1 = Intersection::new(r[0], shape);
                        intersection_list.add(i1);
                    }
                    None => {}
                }
                intersection_list
            }
        };
        res
    }

    fn intersect_world(w: &'a World, r: &'a Ray) -> IntersectionList<'a> {
        let mut res = IntersectionList::new();
        for shape in w.get_shapes().iter() {
            let mut tmp = Intersection::intersect(shape, r);
            for is in tmp.get_intersections_mut().drain(..) {
                res.add(is);
            }
        }
        res.get_intersections_mut()
            .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        res
    }

    fn get_t(&self) -> f64 {
        self.t
    }

    fn get_shape(&self) -> &'a Shape {
        self.shape
    }

    fn prepare_computations(&self, r: &Ray) -> PrecomputedComponent {
        let p = Ray::position(r, self.t);
        let mut normal_vector = self.shape.normal_at(&p);
        let eye_vector = r.get_direction() * (-1.0);
        let mut inside = true;
        if (&normal_vector ^ &eye_vector) < 0.0 {
            normal_vector = normal_vector * (-1.0);
        } else {
            inside = false;
        }
        let over_point = &p + &(&normal_vector * EPSILON);
        PrecomputedComponent::new(self.get_t(), self.get_shape(), p, over_point, eye_vector, normal_vector, inside)
    }
}

impl<'a> fmt::Debug for Intersection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "is shape t = {}", self.t)
    }
}

pub struct IntersectionList<'a> {
    l: Vec<Intersection<'a>>,
}

pub trait IntersectionListOps<'a> {
    fn new() -> IntersectionList<'a>;
    fn add(&mut self, i: Intersection<'a>);

    fn hit(&self) -> Option<&Intersection<'a>>;

    fn get_intersections(&self) -> &Vec<Intersection<'a>>;
    fn get_intersections_mut(&mut self) -> &mut Vec<Intersection<'a>>;
}

impl<'a> IntersectionListOps<'a> for IntersectionList<'a> {
    fn new() -> IntersectionList<'a> {
        IntersectionList { l: Vec::new() }
    }

    fn add(&mut self, i: Intersection<'a>) {
        self.l.push(i);
        self.l.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }

    fn hit(&self) -> Option<&Intersection<'a>> {
        self.l.iter().find(|&i| i.t >= 0.0)
    }

    fn get_intersections(&self) -> &Vec<Intersection<'a>> {
        &self.l
    }

    fn get_intersections_mut(&mut self) -> &mut Vec<Intersection<'a>> {
        &mut self.l
    }
}

impl<'a> fmt::Debug for IntersectionList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.l.iter() {
            writeln!(f, "isl  {:?}", i);
        }
        writeln!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use crate::math::common::{assert_color, assert_float, assert_matrix, assert_tuple, assert_two_float};

    use super::*;

    #[test]
    fn test_new_intersection() {
        let s = Sphere::new();
        let t: f64 = 3.5;
        let o = Shape::Sphere(s);
        let i = Intersection::new(t, &o);
        assert_eq!(i.t, 3.5);
    }

    #[test]
    fn test_new_intersectionlist() {
        let s1 = Sphere::new();
        let t1: f64 = 3.5;
        let o1 = Shape::Sphere(s1);
        let i1 = Intersection::new(t1, &o1);

        let s2 = Sphere::new();
        let t2: f64 = 4.5;
        let o2 = Shape::Sphere(s2);
        let i2 = Intersection::new(t2, &o2);

        let i_list = IntersectionList::new();

        let mut il = IntersectionList::new();
        il.add(i1);
        il.add(i2);

        // TODO: test ???
    }

    #[test]
    fn test_intersection_hit() {
        let s = Sphere::new();
        let o = Shape::Sphere(s);

        let t1: f64 = 1.0;
        let i1 = Intersection::new(t1, &o);

        let t2: f64 = 2.0;
        let i2 = Intersection::new(t2, &o);

        let mut il = IntersectionList::new();
        il.add(i2);
        il.add(i1);

        let i = il.hit().unwrap();

        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn test_intersection_hit_neg() {
        let s = Sphere::new();
        let o = Shape::Sphere(s);

        let t1: f64 = -1.0;
        let i1 = Intersection::new(t1, &o);

        let t2: f64 = 1.0;
        let i2 = Intersection::new(t2, &o);

        let mut il = IntersectionList::new();
        il.add(i2);
        il.add(i1);

        let i = il.hit().unwrap();

        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn test_intersection_no_hit() {
        let s = Sphere::new();
        let o = Shape::Sphere(s);

        let t1: f64 = -1.0;
        let i1 = Intersection::new(t1, &o);

        let t2: f64 = -2.0;
        let i2 = Intersection::new(t2, &o);

        let mut il = IntersectionList::new();
        il.add(i2);
        il.add(i1);

        let i = il.hit();

        // TODO - how to assert????
        assert_eq!(i.is_none(), true);
    }

    #[test]
    fn test_intersection_hit_from_list() {
        let s = Sphere::new();
        let o = Shape::Sphere(s);

        let t1: f64 = 5.0;
        let i1 = Intersection::new(t1, &o);

        let t2: f64 = 7.0;
        let i2 = Intersection::new(t2, &o);

        let t3: f64 = -3.0;
        let i3 = Intersection::new(t3, &o);

        let t4: f64 = 2.0;
        let i4 = Intersection::new(t4, &o);

        let mut il = IntersectionList::new();
        il.add(i1);
        il.add(i2);
        il.add(i3);
        il.add(i4);

        let i = il.hit().unwrap();

        assert_eq!(i.t, 2.0);
    }

    #[test]
    fn test_intersect() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();
        let o = Shape::Sphere(s);

        let i = Intersection::intersect(&o, &r);
        let intersections = i.get_intersections();
        assert_eq!(intersections.len(), 2);
    }

    // page 93
    #[test]
    fn test_prepare_computations() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();
        let o = Shape::Sphere(s);

        let i = Intersection::new(4.0, &o);

        let c = Intersection::prepare_computations(&i, &r);

        let point_expected = Tuple4D::new_point(0.0, 0., -1.0);
        let eye_vector_expected = Tuple4D::new_vector(0.0, 0., -1.0);
        let normal_vector_expected = Tuple4D::new_vector(0.0, 0., -1.0);

        assert_tuple(&point_expected, c.get_point());
        assert_tuple(&eye_vector_expected, c.get_eye_vector());
        assert_tuple(&normal_vector_expected, c.get_normal_vector());
    }

    // page 94
    #[test]
    fn test_prepare_computations_hit_outside() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();
        let o = Shape::Sphere(s);

        let i = Intersection::new(4.0, &o);

        let c = Intersection::prepare_computations(&i, &r);

        let point_expected = Tuple4D::new_point(0.0, 0., -1.0);
        let eye_vector_expected = Tuple4D::new_vector(0.0, 0., -1.0);
        let normal_vector_expected = Tuple4D::new_vector(0.0, 0., -1.0);

        assert_tuple(&point_expected, c.get_point());
        assert_tuple(&eye_vector_expected, c.get_eye_vector());
        assert_tuple(&normal_vector_expected, c.get_normal_vector());
        assert_eq!(false, c.get_inside());
    }

    // page 95 top
    #[test]
    fn test_precomputations_hit_inside() {
        let o = Tuple4D::new_point(0.0, 0.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();
        let o = Shape::Sphere(s);

        let i = Intersection::new(1.0, &o);

        let c = Intersection::prepare_computations(&i, &r);

        let point_expected = Tuple4D::new_point(0.0, 0.0, 1.0);
        let eye_vector_expected = Tuple4D::new_vector(0.0, 0., -1.0);
        let normal_vector_expected = Tuple4D::new_vector(0.0, 0., -1.0);

        assert_tuple(&point_expected, c.get_point());
        assert_tuple(&eye_vector_expected, c.get_eye_vector());
        assert_tuple(&normal_vector_expected, c.get_normal_vector());
        assert_eq!(true, c.get_inside());
    }
}
