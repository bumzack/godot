// TODO: thats stupid - everything is a 4x4 Matrix3

#[cfg(feature = "cuda")]
extern crate rustacuda_core;

use core::ops::{Index, IndexMut, Mul};
use std::ops::Add;

use crate::{EPSILON, intri_abs, intri_cos, intri_sin, Tuple, Tuple3, Tuple3D, Tuple4D};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Matrix3 {
    pub rows: usize,
    pub cols: usize,
    pub m: [f32; 9],
}

pub trait Matrix3Ops {
    fn new() -> Matrix3;
    fn new_matrix3_3x3(a1: f32, b1: f32, c1: f32, a2: f32, b2: f32, c2: f32, a3: f32, b3: f32, c3: f32) -> Matrix3;
    fn new_identity_3x3() -> Matrix3;
}

impl Matrix3Ops for Matrix3 {
    fn new() -> Matrix3 {
        let m = Matrix3 {
            rows: 3,
            cols: 3,
            m: [0.0; 9],
        };
        m
    }

    fn new_matrix3_3x3(a1: f32, b1: f32, c1: f32, a2: f32, b2: f32, c2: f32, a3: f32, b3: f32, c3: f32) -> Matrix3 {
        let mut m = Matrix3 {
            rows: 3,
            cols: 3,
            m: [0.0; 9],
        };

        m[0][0] = a1;
        m[0][1] = b1;
        m[0][2] = c1;
        m[1][0] = a2;
        m[1][1] = b2;
        m[1][2] = c2;
        m[2][0] = a3;
        m[2][1] = b3;
        m[2][2] = c3;

        m
    }

    fn new_identity_3x3() -> Matrix3 {
        let mut m = Matrix3 {
            rows: 3,
            cols: 3,
            m: [0.0; 9],
        };

        m[0][0] = 1.0;
        m[1][1] = 1.0;
        m[2][2] = 1.0;

        m
    }
}

impl Mul for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Matrix3 {
        // TODO: thats not a generic check for matrices which are non-quadratic
        //        assert!(self.rows == rhs.rows);
        let mut m = Matrix3::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let mut sum: f32 = 0.0;

                // TODO: not a generic code for general Matrix3 dimensions
                for i in 0..self.cols {
                    sum += self[row][i] * rhs[i][col];
                }
                m[row][col] = sum;
            }
        }
        m
    }
}

impl<'a, 'b> Mul<&'b Matrix3> for &'a Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: &'b Matrix3) -> Matrix3 {
        // TODO: thats not a generic check for matrices which are non-quadratic
        //        assert!(self.rows == rhs.rows);
        let mut m = Matrix3::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let mut sum: f32 = 0.0;

                // TODO: not a generic code for general Matrix3 dimensions
                for i in 0..self.cols {
                    sum += self[row][i] * rhs[i][col];
                }
                m[row][col] = sum;
            }
        }
        m
    }
}

impl Mul<Tuple3D> for Matrix3 {
    type Output = Tuple3D;

    fn mul(self, rhs: Tuple3D) -> Tuple3D {
        //        assert!(self.rows == 4);
        let mut t = Tuple3D::empty();

        // TODO: not a generic code for general Matrix3 dimensions
        t.x = self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z;
        t.y = self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z;
        t.z = self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z;

        t
    }
}

impl<'a, 'b> Mul<&'b Tuple3D> for &'a Matrix3 {
    type Output = Tuple3D;

    fn mul(self, rhs: &'b Tuple3D) -> Tuple3D {
        //  assert_eq!(self.rows, 4);
        let mut t = Tuple3D::empty();

        // TODO: not a generic code for general Matrix3 dimensions
        t.x = self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z;
        t.y = self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z;
        t.z = self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z;

        t
    }
}

impl Add for Matrix3 {
    type Output = Matrix3;

    fn add(self, rhs: Matrix3) -> Matrix3 {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);
        let mut m = Matrix3::new();

        for c in 0..self.cols {
            for r in 0..self.rows {
                m[r][c] = self[r][c] + rhs[r][c];
            }
        }
        m
    }
}

impl<'a, 'b> Add<&'b Matrix3> for &'a Matrix3 {
    type Output = Matrix3;

    fn add(self, rhs: &'b Matrix3) -> Matrix3 {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);
        let mut m = Matrix3::new();

        for c in 0..self.cols {
            for r in 0..self.rows {
                m[r][c] = self[r][c] + rhs[r][c];
            }
        }
        m
    }
}

impl Mul<f32> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: f32) -> Matrix3 {
        let mut m = Matrix3::new();

        for c in 0..self.cols {
            for r in 0..self.rows {
                m[r][c] = self[r][c] * rhs;
            }
        }
        m
    }
}

impl<'a> Mul<f32> for &'a Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: f32) -> Matrix3 {
        let mut m = Matrix3::new();

        for c in 0..self.cols {
            for r in 0..self.rows {
                m[r][c] = self[r][c] * rhs;
            }
        }
        m
    }
}

impl Index<usize> for Matrix3 {
    type Output = [f32];
    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index * self.cols..(index + 1) * self.cols]
    }
}

impl IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index * self.cols..(index + 1) * self.cols]
    }
}
