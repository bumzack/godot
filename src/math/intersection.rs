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
    fn add(&mut self, i: Intersection);
}

//impl<'a> IntersectionListOps for IntersectionList<'a> {
//    fn new<'a>( ) -> IntersectionList<'a> {
//        IntersectionList  {
//            l: Vec::new(),
//        }
//    }
//
//    fn add(&mut self, i: Intersection) {
//        self.l.push(i);
//    }
//}


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

//    let mut il = IntersectionList::new();
//    il.add(i1);
//    il.add(i2);

    // TODO: test ???
}




