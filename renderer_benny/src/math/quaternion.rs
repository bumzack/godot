use std::ops::{Add, BitXor, Mul, Sub};

#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

use crate::math::matrix::{Matrix, MatrixOps};
use crate::math::tuple4d::{Tuple, Tuple4D};
use std::f32;

const QUATERNION_EPISLON: f32 = 0.0001;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion { x, y, z, w }
    }

    #[inline]
    pub fn new_from_tuple_and_angle(axis: Tuple4D, angle: f32) -> Quaternion {
        let sin_half_angle = (angle / 2.0).sin();
        let cos_half_angle = (angle / 2.0).cos();

        Quaternion {
            x: axis.get_x() * sin_half_angle,
            y: axis.get_y() * sin_half_angle,
            z: axis.get_z() * sin_half_angle,
            w: cos_half_angle,
        }
    }

    #[inline]
    pub fn new_from_rot_matrix(rot: Matrix) -> Quaternion {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut w = 0.0;
        let trace = rot[0][0] + rot[1][1] + rot[2][2];

        if trace > 0.0 {
            let s = 0.5 / (trace + 1.0).sqrt();
            w = 0.25 / s;
            x = (rot[1][2] - rot[2][1]) * s;
            y = (rot[2][0] - rot[0][2]) * s;
            z = (rot[0][1] - rot[1][0]) * s;
        } else {
            if rot[0][0] > rot[1][1] && rot[0][0] > rot[2][2] {
                let s = 2.0 * (1.0 + rot[0][0] - rot[1][1] - rot[2][2]).sqrt();

                w = (rot[1][2] - rot[2][1]) / s;
                x = 0.25 * s;
                y = (rot[1][0] - rot[0][1]) / s;
                z = (rot[2][0] - rot[0][2]) / s;
            } else if rot[1][1] > rot[2][2] {
                let s = 2.0 * (1.0 + rot[1][1] - rot[0][0] - rot[2][2]).sqrt();

                w = (rot[2][0] - rot[0][2]) / s;
                x = (rot[1][0] - rot[0][1]) / s;
                y = 0.25 * s;
                z = (rot[2][1] - rot[1][2]) / s;
            } else {
                let s = 2.0 * (1.0 + rot[2][2] - rot[0][0] - rot[1][1]).sqrt();
                w = (rot[0][1] - rot[1][0]) / s;
                x = (rot[2][0] - rot[0][2]) / s;
                y = (rot[1][2] - rot[2][1]) / s;
                z = 0.25 * s;
            }
        }

        let length = (x * x + y * y + z * z + w * w).sqrt();
        x /= length;
        y /= length;
        z /= length;
        w /= length;
        Quaternion { x, y, z, w }
    }

    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Quaternion {
        let l = self.len();
        Quaternion {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
            w: self.w / l,
        }
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }

    pub fn to_rotation_matrix(&self) -> Matrix {
        let forward = Tuple4D::new_point(
            2.0 * (self.x * self.z - self.w * self.y),
            2.0 * (self.y * self.z + self.w * self.x),
            1.0 - 2.0 * (self.x * self.x + self.y * self.y),
        );
        let up = Tuple4D::new_point(
            2.0 * (self.x * self.y + self.w * self.z),
            1.0 - 2.0 * (self.x * self.x + self.z * self.z),
            2.0 * (self.y * self.z - self.w * self.x),
        );
        let right = Tuple4D::new_point(
            1.0 - 2.0 * (self.y * self.y + self.z * self.z),
            2.0 * (self.x * self.y - self.w * self.z),
            2.0 * (self.x * self.z + self.w * self.y),
        );

        println!("");
        println!("forward = {}", forward);
        println!("up = {}", up);
        println!("right = {}", right);

        Matrix::new_matrix_4x4(
            right.get_x(),
            right.get_y(),
            right.get_z(),
            0.0,
            up.get_x(),
            up.get_y(),
            up.get_z(),
            0.0,
            forward.get_x(),
            forward.get_y(),
            forward.get_z(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    pub fn n_lerp(&self, dest: &Quaternion, lerp_factor: f32, shortest: bool) -> Quaternion {
        let mut corrected_dest = dest.clone();

        if shortest && self ^ dest < 0.0 {
            corrected_dest = Quaternion::new(-dest.x(), -dest.y(), -dest.z(), -dest.w());
        }
        let q = &((&corrected_dest - self) * lerp_factor) + self;
        q.normalized()
    }

    pub fn s_lerp(&self, dest: &Quaternion, lerp_factor: f32, shortest: bool) -> Quaternion {
        let mut cos = self ^ dest;
        let mut corrected_dest = dest.clone();

        if shortest && cos < 0.0 {
            cos = -cos;
            corrected_dest = Quaternion::new(-dest.x(), -dest.y(), -dest.z(), -dest.w());
        }
        if (cos).abs() >= 1.0 - QUATERNION_EPISLON {
            return self.n_lerp(&corrected_dest, lerp_factor, false);
        }

        let sin = (1.0 - cos * cos).sqrt();
        let angle = sin.atan2(cos);
        let inv_sin = 1.0 / sin;

        let src_factor = ((1.0 - lerp_factor) * angle).sin() * inv_sin;
        let dest_factor = (lerp_factor * angle).sin() * inv_sin;

        self * src_factor + corrected_dest * dest_factor
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }
    pub fn w(&self) -> f32 {
        self.w
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f32) -> Quaternion {
        Quaternion {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl core::fmt::Display for Quaternion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Quaternion: x = {}, y = {}, z = {}: w = {}",
            self.x, self.y, self.z, self.w
        )
    }
}

impl<'a> Mul<f32> for &'a Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f32) -> Quaternion {
        Quaternion {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion::new(
            self.x * rhs.w + self.w * rhs.x + self.y * rhs.z - self.z * rhs.y,
            self.y * rhs.w + self.w * rhs.y + self.z * rhs.x - self.x * rhs.z,
            self.z * rhs.w + self.w * rhs.z + self.x * rhs.y - self.y * rhs.x,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

impl<'a, 'b> Mul<&'b Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &'b Quaternion) -> Quaternion {
        Quaternion::new(
            self.x * rhs.w + self.w * rhs.x + self.y * rhs.z - self.z * rhs.y,
            self.y * rhs.w + self.w * rhs.y + self.z * rhs.x - self.x * rhs.z,
            self.z * rhs.w + self.w * rhs.z + self.x * rhs.y - self.y * rhs.x,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

impl Mul<Tuple4D> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Tuple4D) -> Quaternion {
        Quaternion::new(
            self.w * rhs.x + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y + self.z * rhs.x - self.x * rhs.z,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x,
            -self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

impl<'a, 'b> Mul<&'b Tuple4D> for &'a Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &'b Tuple4D) -> Quaternion {
        Quaternion::new(
            self.w * rhs.x + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y + self.z * rhs.x - self.x * rhs.z,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x,
            -self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;

    fn sub(self, other: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<'a, 'b> Sub<&'b Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn sub(self, rhs: &'b Quaternion) -> Quaternion {
        Quaternion {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Add for Quaternion {
    type Output = Quaternion;

    fn add(self, other: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<'a, 'b> Add<&'b Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn add(self, rhs: &'b Quaternion) -> Quaternion {
        Quaternion {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

// a ^ b
impl BitXor for Quaternion {
    type Output = f32;

    fn bitxor(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl<'a, 'b> BitXor<&'b Quaternion> for &'a Quaternion {
    type Output = f32;

    fn bitxor(self, rhs: &'b Quaternion) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}
