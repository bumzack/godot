use raytracer_lib_no_std::prelude::*;

// source: https://math.stackexchange.com/questions/180418/calculate-rotation-matrix-to-align-vector-a-to-vector-b-in-3d
pub fn rotation_matrix_v1_to_v2_3d(v1: &Tuple3D, v2: &Tuple3D) -> Matrix3 {
    let v = v1 * v2;
    let s = Tuple3D::magnitude(&(v1 * v2));
    let c = v1 ^ v2;

    let v_x = Matrix3::new_matrix3_3x3(0.0, -v.z, v.y, v.z, 0.0, -v.x, -v.y, v.x, 0.0);
    let v_x_squared = &v_x * &v_x;

    let mut r = &Matrix3::new_identity_3x3() + &v_x;
    r = &r + &(v_x_squared * ((1.0 - c) / s.powi(2)));

    r
}

pub fn rotation_matrix_v1_to_v2(v1: &Tuple4D, v2: &Tuple4D) -> Matrix {
    let v = v1 * v2;
    let s = Tuple4D::magnitude(&(v1 * v2));
    let c = v1 ^ v2;

    let v_x = Matrix::new_matrix_4x4(
        0.0, -v.z, v.y, 0.0, v.z, 0.0, -v.x, 0.0, -v.y, v.x, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    let v_x_squared = &v_x * &v_x;

    let mut r = &Matrix::new_identity_4x4() + &v_x;
    r = &r + &(v_x_squared * ((1.0 - c) / s.powi(2)));

    r
}

pub fn cylinder_between_two_points(p1: &Tuple4D, p2: &Tuple4D, radius: f32) -> Cylinder {
    let delta = p2 - p1;
    let delta_norm = Tuple4D::normalize(&delta);
    let length = Tuple4D::magnitude(&delta);
    // let length = 0.5;

    let y_axis = Tuple4D::up();

    let center = p1 + &(delta / 2.0);
    // let center = p1  +&delta;

    // position centered aroudn origin
    let m_scale = Matrix::scale(radius, length, radius);
    let m_trans = Matrix::translation(center.x, center.y, center.z);
    let m_rot = rotation_matrix_v1_to_v2(&y_axis, &delta_norm);

    //  &m_trans * &(m_rot * m_scale);
    let m = &m_trans * &(&m_rot * &m_scale);

    println!("p1  :         {:?}", p1);
    println!("p2  :         {:?}", p2);
    println!("center  :     {:?}", center);

    //    let delta_calc = &m_rot * &y_axis;
    //    println!("delta_norm  expected:     {:?}", delta_norm);
    //    println!("delta_calc:               {:?}", delta_calc);

    let mut c = Cylinder::new();
    c.set_transformation(m);
    c.set_minimum(-0.5);
    c.set_maximum(0.5);

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
