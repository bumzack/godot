use std::fs::File;
use std::io::{Error, Write};

use crate::math::color::Color;
use crate::math::color::ColorOps;
use crate::math::common::assert_float;
use crate::math::commonshape::CommonShape;
use crate::math::ray::Ray;
use crate::math::ray::RayOps;
use crate::math::sphere::Sphere;
use crate::math::sphere::SphereOps;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;

pub struct Intersection<'a> {
    t: f32,
    obj: &'a CommonShape,
}

pub trait IntersectionOps<'a> {
    fn new(t: f32, obj: &CommonShape) -> Intersection;
    fn intersect(obj: &'a CommonShape, r: &Ray) -> IntersectionList<'a>;

    fn get_t(&self) -> f32;
    fn get_obj(&self) -> &'a CommonShape;
}

impl<'a> IntersectionOps<'a> for Intersection<'a> {
    fn new(t: f32, obj: &CommonShape) -> Intersection {
        Intersection {
            t,
            obj,
        }
    }

    fn intersect(obj: &'a CommonShape, r: &Ray) -> IntersectionList<'a> {
        let mut intersection_list = IntersectionList::new();
        let res = match obj {
            CommonShape::Sphere(ref s) => {

                //TODO: this soooo important here to inverse the ray ...

                // TODO: refactor the shit out of this
                let r2 = Ray::transform(r, s.get_inverse_transformation());
                let res = Sphere::intersect(s, &r2);
                // println!("intersect: ray = {:#?}, sphere = {:#?}", r, s);

                match res {
                    Some(r) => {
                        let i1 = Intersection::new(r[0], obj);
                        let i2 = Intersection::new(r[1], obj);
                        intersection_list.add(i1);
                        intersection_list.add(i2);
                    }
                    None => {}
                }
                intersection_list
            }
        };
        res
    }

    fn get_t(&self) -> f32 {
        self.t
    }

    fn get_obj(&self) -> &'a CommonShape {
        self.obj
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
}

impl<'a> IntersectionListOps<'a> for IntersectionList<'a> {
    fn new() -> IntersectionList<'a> {
        IntersectionList {
            l: Vec::new(),
        }
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
}


#[test]
fn test_new_intersection() {
    let s = Sphere::new();
    let t: f32 = 3.5;
    let o = CommonShape::Sphere(s);
    let i = Intersection::new(t, &o);
    assert_eq!(i.t, 3.5);
}

#[test]
fn test_new_intersectionlist() {
    let s1 = Sphere::new();
    let t1: f32 = 3.5;
    let o1 = CommonShape::Sphere(s1);
    let i1 = Intersection::new(t1, &o1);

    let s2 = Sphere::new();
    let t2: f32 = 3.5;
    let o2 = CommonShape::Sphere(s2);
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
    let o = CommonShape::Sphere(s);

    let t1: f32 = 1.0;
    let i1 = Intersection::new(t1, &o);

    let t2: f32 = 2.0;
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
    let o = CommonShape::Sphere(s);

    let t1: f32 = -1.0;
    let i1 = Intersection::new(t1, &o);

    let t2: f32 = 1.0;
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
    let o = CommonShape::Sphere(s);

    let t1: f32 = -1.0;
    let i1 = Intersection::new(t1, &o);

    let t2: f32 = -2.0;
    let i2 = Intersection::new(t2, &o);

    let mut il = IntersectionList::new();
    il.add(i2);
    il.add(i1);

    let i = il.hit();

    // TODO - how to assert????
    //  assert_eq!(i, None);
}


#[test]
fn test_intersection_hit_from_list() {
    let s = Sphere::new();
    let o = CommonShape::Sphere(s);

    let t1: f32 = 5.0;
    let i1 = Intersection::new(t1, &o);

    let t2: f32 = 7.0;
    let i2 = Intersection::new(t2, &o);

    let t3: f32 = -3.0;
    let i3 = Intersection::new(t3, &o);

    let t4: f32 = 2.0;
    let i4 = Intersection::new(t4, &o);


    let mut il = IntersectionList::new();
    il.add(i2);
    il.add(i1);
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
    let o = CommonShape::Sphere(s);

    let i = Intersection::intersect(&o, &r);
    let intersections = i.get_intersections();
    assert_eq!(intersections.len(), 2);
}
