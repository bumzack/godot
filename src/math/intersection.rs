use std::fs::File;
use std::io::{Error, Write};

use crate::math::color::Color;
use crate::math::color::ColorOps;
use crate::math::commonshape::CommonShape;
use crate::math::sphere::Sphere;
use crate::math::sphere::SphereOps;

pub struct Intersection<'a> {
    t: f32,
    obj: &'a CommonShape,
}

pub struct IntersectionList<'a> {
    l: Vec<Intersection<'a>>,
}

pub trait IntersectionOps<'a> {
    fn new(t: f32, obj: &CommonShape) -> Intersection;
}

impl<'a> IntersectionOps<'a> for Intersection<'a> {
    fn new(t: f32, obj: &CommonShape) -> Intersection {
        Intersection {
            t,
            obj,
        }
    }
}

pub trait IntersectionListOps<'a> {
    fn new() -> IntersectionList<'a>;
    fn add(&mut self, i: Intersection<'a>);

    fn hit(&self) -> Option<&Intersection<'a>>;
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
