use crate::math::sphere::{Sphere, SphereOps};
use crate::math::tuple4d::Tuple4D;

pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn normal_at(&self, p: &Tuple4D) -> Tuple4D {
        let res = match self {
            Shape::Sphere(ref s) => { s.normal_at(p) }
        };
        res
    }
}

