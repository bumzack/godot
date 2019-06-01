use crate::math::shape::Shape;
use crate::math::tuple4d::Tuple4D;

pub struct PrecomputedComponent<'a> {
    t: f32,
    obj: &'a Shape,
    point: Tuple4D,
    eye_vector:  Tuple4D,
    normal_vector: Tuple4D,
}

impl<'a> PrecomputedComponent<'a> {

    pub fn new(t: f32, obj: &'a Shape, point: Tuple4D, eye_vector: Tuple4D, normal_vector: Tuple4D) -> PrecomputedComponent<'a>{
        PrecomputedComponent {
            t,
            obj,
            point,
            eye_vector,
            normal_vector
        }
    }

    pub fn get_t(&self) -> f32 {
        self.t
    }

    pub fn get_point(&self) -> &Tuple4D {
        &self.point
    }

    pub fn get_eye_vector(&self) -> &Tuple4D {
        &self.eye_vector
    }

    pub fn get_normal_vector(&self) -> &Tuple4D {
        &self.normal_vector
    }
}
