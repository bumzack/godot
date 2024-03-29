use math::prelude::*;

#[derive(Debug)]
pub struct PrecomputedComponent {
    t: f32,
    shape_idx: usize,
    point: Tuple4D,
    over_point: Tuple4D,
    under_point: Tuple4D,
    eye_vector: Tuple4D,
    normal_vector: Tuple4D,
    reflected_vector: Tuple4D,
    inside: bool,
    n1: f32,
    n2: f32,
}

impl PrecomputedComponent {
    pub fn new(
        t: f32,
        shape_idx: usize,
        point: Tuple4D,
        over_point: Tuple4D,
        under_point: Tuple4D,
        eye_vector: Tuple4D,
        normal_vector: Tuple4D,
        reflected_vector: Tuple4D,
        inside: bool,
    ) -> PrecomputedComponent {
        PrecomputedComponent {
            t,
            shape_idx,
            point,
            over_point,
            under_point,
            eye_vector,
            normal_vector,
            reflected_vector,
            inside,
            n1: 0.0,
            n2: 0.0,
        }
    }

    pub fn get_t(&self) -> f32 {
        self.t
    }

    pub fn get_point(&self) -> &Tuple4D {
        &self.point
    }

    pub fn get_over_point(&self) -> &Tuple4D {
        &self.over_point
    }

    pub fn get_eye_vector(&self) -> &Tuple4D {
        &self.eye_vector
    }

    pub fn get_normal_vector(&self) -> &Tuple4D {
        &self.normal_vector
    }

    pub fn get_inside(&self) -> bool {
        self.inside
    }

    pub fn get_object(&self) -> usize {
        self.shape_idx
    }

    pub fn get_reflected_vector(&self) -> &Tuple4D {
        &self.reflected_vector
    }

    pub fn get_under_point(&self) -> &Tuple4D {
        &self.under_point
    }

    pub fn get_n1(&self) -> f32 {
        self.n1
    }

    pub fn get_n2(&self) -> f32 {
        self.n2
    }

    pub fn set_n1(&mut self, n1: f32) {
        self.n1 = n1;
    }

    pub fn set_n2(&mut self, n2: f32) {
        self.n2 = n2;
    }
}
