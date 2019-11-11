use core::ops::{Add, BitXor, Div, Mul, Sub};

use crate::prelude::math_ops::math_ops::{intri_sqrt, intri_cos, intri_sin};

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Tuple3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait Tuple3 {
    fn normalize(a: &Tuple3D) -> Tuple3D;
    fn magnitude(a: &Tuple3D) -> f32;

    fn new_vector(x: f32, y: f32, z: f32) -> Tuple3D;
    fn new(x: f32, y: f32, z: f32) -> Tuple3D;
    fn empty() -> Tuple3D;
}

impl Tuple3 for Tuple3D {
    #[inline]
    fn normalize(a: &Tuple3D) -> Tuple3D {
        let m = Tuple3D::magnitude(a);
        Tuple3D {
            x: a.x / m,
            y: a.y / m,
            z: a.z / m,
        }
    }

    #[inline]
    fn magnitude(a: &Tuple3D) -> f32 {
        intri_sqrt(a.x * a.x + a.y * a.y + a.z * a.z)
    }

    #[inline]
    fn new_vector(x: f32, y: f32, z: f32) -> Tuple3D {
        Tuple3D { x, y, z }
    }

    #[inline]
    fn new(x: f32, y: f32, z: f32) -> Tuple3D {
        Tuple3D { x, y, z }
    }

    #[inline]
    fn empty() -> Tuple3D {
        Tuple3D { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Add for Tuple3D {
    type Output = Tuple3D;

    fn add(self, other: Tuple3D) -> Tuple3D {
        Tuple3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> Add<&'b Tuple3D> for &'a Tuple3D {
    type Output = Tuple3D;

    fn add(self, rhs: &'b Tuple3D) -> Tuple3D {
        Tuple3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Tuple3D {
    type Output = Tuple3D;

    fn sub(self, other: Tuple3D) -> Tuple3D {
        Tuple3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, 'b> Sub<&'b Tuple3D> for &'a Tuple3D {
    type Output = Tuple3D;

    fn sub(self, rhs: &'b Tuple3D) -> Tuple3D {
        Tuple3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Tuple3D {
    type Output = Tuple3D;

    fn mul(self, rhs: f32) -> Tuple3D {
        Tuple3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<'a> Mul<f32> for &'a Tuple3D {
    type Output = Tuple3D;

    fn mul(self, rhs: f32) -> Tuple3D {
        Tuple3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul for Tuple3D {
    type Output = Tuple3D;

    fn mul(self, rhs: Tuple3D) -> Tuple3D {
        Tuple3D::new_vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl<'a, 'b> Mul<&'b Tuple3D> for &'a Tuple3D {
    type Output = Tuple3D;

    fn mul(self, rhs: &'b Tuple3D) -> Tuple3D {
        Tuple3D::new_vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

// a ^ b
impl BitXor for Tuple3D {
    type Output = f32;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<'a, 'b> BitXor<&'b Tuple3D> for &'a Tuple3D {
    type Output = f32;

    fn bitxor(self, rhs: &'b Tuple3D) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Div<f32> for Tuple3D {
    type Output = Tuple3D;

    fn div(self, rhs: f32) -> Tuple3D {
        Tuple3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<'a> Div<f32> for &'a Tuple3D {
    type Output = Tuple3D;

    fn div(self, rhs: f32) -> Tuple3D {
        Tuple3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<usize> for Tuple3D {
    type Output = Tuple3D;

    fn div(self, rhs: usize) -> Tuple3D {
        Tuple3D {
            x: self.x / rhs as f32,
            y: self.y / rhs as f32,
            z: self.z / rhs as f32,
        }
    }
}

impl<'a> Div<usize> for &'a Tuple3D {
    type Output = Tuple3D;

    fn div(self, rhs: usize) -> Tuple3D {
        Tuple3D {
            x: self.x / rhs as f32,
            y: self.y / rhs as f32,
            z: self.z / rhs as f32,
        }
    }
}
