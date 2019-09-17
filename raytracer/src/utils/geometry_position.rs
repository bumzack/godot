use std::f32::consts::PI;

use raytracer_lib_no_std::prelude::*;

pub fn cylinder_between_two_points(p1: &Tuple4D, p2: &Tuple4D, radius: f32) -> Cylinder {
    let delta = p2 - p1;
    let length = Tuple4D::magnitude(&delta);
    // let length = 0.5;

    println!("delta = {:?}, length = {}", delta, length);

    let alpha;
    if delta.x < EPSILON {
        println!("delta.x approx 0         {:?}", delta.x);
        alpha = 0.0;
    } else {
        alpha = intri_tan(delta.y / delta.x);
        println!("delta.x !=  0         {:?}       alpha = {:?}", delta.x, alpha);
    }

    let m_scale = Matrix::scale(radius, length, radius);
    let m_trans = Matrix::translation(0.0, -length / 2.0, 0.0);
    let m_rot = Matrix::rotate_z(PI / 2.0);

    let m = m_trans * m_scale;
    let m = m_rot * m;

    let mut c = Cylinder::new();
    // c.set_minimum(-length / 2.0);
    // c.set_maximum(length / 2.0);

    c.set_transformation(m);

    c.set_minimum(0.0);
    c.set_maximum(1.0);
    c
}
