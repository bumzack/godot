use std::f32::consts::PI;

use raytracer_lib_no_std::prelude::*;

pub fn rotation_matrix_v1_to_v2_3d(v1: &Tuple3D, v2: &Tuple3D) -> Matrix3 {
    let v = v1 * v2;
    let s = Tuple3D::magnitude( &(v1 * v2));
    let c = v1 ^ v2;

    let v_x = Matrix3::new_matrix3_3x3(0.0, -v.z, v.y, v.z, 0.0, -v.x, -v.y, v.x, 0.0);
    let v_x_squared = &v_x * &v_x;

    let mut r = &Matrix3::new_identity_3x3() + &v_x;
    r = &r + &(v_x_squared * ((1.0-c) / s.powi(2)));

    r
}


pub fn rotation_matrix_v1_to_v2(v1: &Tuple4D, v2: &Tuple4D) -> Matrix {
    let v = v1 * v2;
    let s = Tuple4D::magnitude( &(v1 * v2));
    let c = v1 ^ v2;

    let v_x = Matrix::new_matrix_4x4(0.0, -v.z, v.y, 0.0, v.z, 0.0, -v.x, 0.0, -v.y, v.x, 0.0, 0.0,0.0, 0.0, 0.0, 1.0);
    let v_x_squared = &v_x * &v_x;

    let mut r = &Matrix::new_identity_4x4() + &v_x;
    r = &r + &(v_x_squared * ((1.0-c) / s.powi(2)));

    r
}

pub fn cylinder_between_two_points(p1: &Tuple4D, p2: &Tuple4D, radius: f32) -> Cylinder {
    let delta = p2 - p1;
    let length = Tuple4D::magnitude(&delta);
    // let length = 0.5;

    let delta_normalized = Tuple4D::normalize(&delta);
    let x_axis = Tuple4D::new_vector(1.0, 0.0, 0.0);
    let y_axis = Tuple4D::new_vector(0.0, 1.0, 0.0);
    let z_axis = Tuple4D::new_vector(0.0, 0.0, 1.0);

    let delta_dot_x_axis = -(delta_normalized ^ x_axis);
    let rot_y = delta_dot_x_axis.acos();
    println!(
        " length = {},    delta = {:?},  delta_normalized = {:?} ",
        length, delta, delta_normalized
    );
    println!("rot_y    {:?}     delta_dot_x_axis {}", rot_y, delta_dot_x_axis);

    let delta_dot_z_axis = delta_normalized ^ y_axis;
    let rot_x = delta_dot_z_axis.acos();
    println!("rot_x    {:?}     delta_dot_z_axis {}", rot_x, delta_dot_z_axis);


    // position centered aroudn origin
    let m_scale = Matrix::scale(radius, length, radius);
    let m_trans = Matrix::translation(0.0, -length / 2.0, 0.0);
    let m_rot = Matrix::rotate_z(PI / 2.0);

    // rotate according to required direction
    let m_rot_y = Matrix::rotate_y(rot_y);
    let m_rot_x = Matrix::rotate_x(rot_x);

    let m = m_trans * m_scale;
    let m = m_rot * m;


    let rot = &m_rot_y * &m_rot_x;
    let x_axis = Tuple4D::new_vector(1.0, 0.0, 0.0);
    let check_x = &rot * &x_axis;
    println!("expected:     {:?}", delta);
    println!("check_x:     {:?}", check_x);

    let y_axis = Tuple4D::new_vector(0.0, 1.0, 0.0);
    let check_y = &rot * &y_axis;
    println!("expected:     {:?}", delta);
    println!("check_y:     {:?}", check_y);


    let mut c = Cylinder::new();
    c.set_transformation(m);
    c.set_minimum(0.0);
    c.set_maximum(1.0);

    c
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_v1_in_v2_3d() {
        let v1 = Tuple3D::new_vector(1.0, 2.0, 3.0);
        let v2 = Tuple3D::new_vector(2.0, 5.0, 4.0);
        let v1 = Tuple3D::normalize(&v1);
        let v2 = Tuple3D::normalize(&v2);


        let rot = rotation_matrix_v1_to_v2_3d(&v1, &v2);

        let v2_calc = &rot * &v1;

        println!("v1:                   {:?}", v1);
        println!("v2:                   {:?}", v2);
        println!("calculated v2:        {:?}", v2_calc);
        println!("\n\n rotation matrix :        {:?}", rot);

        assert_float(v2.x, v2_calc.x);
        assert_float(v2.y, v2_calc.y);
        assert_float(v2.z, v2_calc.z);
    }

    #[test]
    fn test_rotate_v1_in_v_stackexchange2_3d() {
        let a = Tuple3D::new_vector(1.0, 0.0, 0.0);
        let b = Tuple3D::new_vector(0.0, 1.0, 0.0);


        let r_a_b = rotation_matrix_v1_to_v2_3d(&a, &b);
        let b_calc = &r_a_b * &a;

        println!("a:                 {:?}", a);
        println!("b:                 {:?}", b);
        println!("b_calc:            {:?}", b_calc);
        println!("r_a_b         {:?}", r_a_b);

        let a = Tuple3D::new_vector(0.043477, 0.036412, 0.998391);
        let b = Tuple3D::new_vector(0.60958, 0.7354, 0.29597);
        let r_a_b = rotation_matrix_v1_to_v2_3d(&a, &b);
        let b_rot = &r_a_b * &a;

        println!("a:                        {:?}", a);
        println!("b:                        {:?}", b);
        println!("b calculated:             {:?}", b_rot);
        println!("r_a_b                 {:?}", r_a_b);

        assert_float(b.x, b_rot.x);
        assert_float(b.y, b_rot.y);
        assert_float(b.z, b_rot.z);
    }

    #[test]
    fn test_rotate_v1_in_v2() {
        let v1 = Tuple4D::new_vector(1.0, 2.0, 3.0);
        let v2 = Tuple4D::new_vector(2.0, 5.0, 4.0);
        let v1 = Tuple4D::normalize(&v1);
        let v2 = Tuple4D::normalize(&v2);


        let rot = rotation_matrix_v1_to_v2(&v1, &v2);

        let v2_calc = &rot * &v1;

        println!("v1:                   {:?}", v1);
        println!("v2:                   {:?}", v2);
        println!("calculated v2:        {:?}", v2_calc);
        println!("\n\n rotation matrix :        {:?}", rot);

        assert_float(v2.x, v2_calc.x);
        assert_float(v2.y, v2_calc.y);
        assert_float(v2.z, v2_calc.z);
    }

    #[test]
    fn test_rotate_v1_in_v_stackexchange2() {
        let a = Tuple4D::new_vector(1.0, 0.0, 0.0);
        let b = Tuple4D::new_vector(0.0, 1.0, 0.0);


        let r_a_b = rotation_matrix_v1_to_v2(&a, &b);
        let b_calc = &r_a_b * &a;

        println!("a:                 {:?}", a);
        println!("b:                 {:?}", b);
        println!("b_calc:            {:?}", b_calc);
        println!("r_a_b         {:?}", r_a_b);

        let a = Tuple4D::new_vector(0.043477, 0.036412, 0.998391);
        let b = Tuple4D::new_vector(0.60958, 0.7354, 0.29597);
        let r_a_b = rotation_matrix_v1_to_v2(&a, &b);
        let b_rot = &r_a_b * &a;

        println!("a:                        {:?}", a);
        println!("b:                        {:?}", b);
        println!("b calculated:             {:?}", b_rot);
        println!("r_a_b                 {:?}", r_a_b);

        assert_float(b.x, b_rot.x);
        assert_float(b.y, b_rot.y);
        assert_float(b.z, b_rot.z);
    }
}
