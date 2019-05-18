struct Vector4D {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

trait Vector {
    fn add(a: &Vector4D, b: &Vector4D) -> Vector4D;
    fn mul_by_scalar(a: &Vector4D, f: f32) -> Vector4D;

    fn new(x: f32, y: f32, z: f32) -> Vector4D;
}


impl Vector for Vector4D {
    fn add(a: &Vector4D, b: &Vector4D) -> Vector4D {
        Vector4D {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
            w: 0.0,
        }
    }

    fn mul_by_scalar(a: &Vector4D, f: f32) -> Vector4D {
        Vector4D {
            x: a.x * f,
            y: a.y * f,
            z: a.z * f,
            w: 0.0,
        }
    }

    fn new(x: f32, y: f32, z: f32) -> Vector4D {
        Vector4D {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }
}

#[test]
fn it_adds_two() {
    let a = Vector4D::new(1.0, 2.0, 3.0);
    let b = Vector4D::mul_by_scalar(&a, 2.0);

    assert_eq!(b.x, 2.0);
    assert_eq!(b.y, 4.0);
    assert_eq!(b.z, 6.0);
}
