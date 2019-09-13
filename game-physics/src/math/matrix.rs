// TODO
// THATS FOR A LEFT HANDED COORDINATE SYSTEM ...
// where are modifications needed

use crate::math::common::{assert_float, assert_matrix, assert_two_float, assert_Tuple4D};
use crate::math::Tuple4D::Tuple4D;
use crate::math::Tuple4D::Tuple4DOps;
use std::f32::consts::{PI, SQRT_2};
use std::ops::Mul;

#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub m: Vec<Vec<f32>>,
}

pub trait MatrixOps {
    fn new(row: usize, col: usize) -> Matrix;

    fn new_matrix_2x2(a1: f32, b1: f32, a2: f32, b2: f32) -> Matrix;

    fn new_matrix_3x3(
        a1: f32,
        b1: f32,
        c1: f32,
        a2: f32,
        b2: f32,
        c2: f32,
        a3: f32,
        b3: f32,
        c3: f32,
    ) -> Matrix;

    fn new_matrix_4x4(
        a1: f32,
        b1: f32,
        c1: f32,
        d1: f32,
        a2: f32,
        b2: f32,
        c2: f32,
        d2: f32,
        a3: f32,
        b3: f32,
        c3: f32,
        d3: f32,
        a4: f32,
        b4: f32,
        c4: f32,
        d4: f32,
    ) -> Matrix;

    fn new_identity_4x4() -> Matrix;
    fn transpose(m: &Matrix) -> Matrix;

    fn determinant(m: &Matrix) -> f32;

    fn sub_matrix(m: &Matrix, row: usize, col: usize) -> Matrix;
    fn minor(m: &Matrix, row: usize, col: usize) -> f32;
    fn cofactor(m: &Matrix, row: usize, col: usize) -> f32;
    fn invert(m: &Matrix) -> Option<Matrix>;

    fn translation(tx: f32, ty: f32, tz: f32) -> Matrix;
    fn scale(sx: f32, sy: f32, sz: f32) -> Matrix;
    fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix;
    fn rotate_x(rad: f32) -> Matrix;
    fn rotate_y(rad: f32) -> Matrix;
    fn rotate_z(rad: f32) -> Matrix;

    fn view_transform(from: &Tuple4D, to: &Tuple4D, up: &Tuple4D) -> Matrix;
}

impl MatrixOps for Matrix {
    fn new(row: usize, col: usize) -> Matrix {
        let m = Matrix {
            rows: row,
            cols: col,
            m: vec![vec![0.0; row]; col],
        };
        m
    }

    fn new_matrix_2x2(a1: f32, b1: f32, a2: f32, b2: f32) -> Matrix {
        let mut m = Matrix {
            rows: 2,
            cols: 2,
            m: vec![vec![0.0; 2]; 2],
        };

        m.m[0][0] = a1;
        m.m[0][1] = b1;
        m.m[1][0] = a2;
        m.m[1][1] = b2;

        m
    }

    fn new_matrix_3x3(
        a1: f32,
        b1: f32,
        c1: f32,
        a2: f32,
        b2: f32,
        c2: f32,
        a3: f32,
        b3: f32,
        c3: f32,
    ) -> Matrix {
        let mut m = Matrix {
            rows: 3,
            cols: 3,
            m: vec![vec![0.0; 3]; 3],
        };

        m.m[0][0] = a1;
        m.m[0][1] = b1;
        m.m[0][2] = c1;
        m.m[1][0] = a2;
        m.m[1][1] = b2;
        m.m[1][2] = c2;
        m.m[2][0] = a3;
        m.m[2][1] = b3;
        m.m[2][2] = c3;

        m
    }

    fn new_matrix_4x4(
        a1: f32,
        b1: f32,
        c1: f32,
        d1: f32,
        a2: f32,
        b2: f32,
        c2: f32,
        d2: f32,
        a3: f32,
        b3: f32,
        c3: f32,
        d3: f32,
        a4: f32,
        b4: f32,
        c4: f32,
        d4: f32,
    ) -> Matrix {
        let mut m = Matrix {
            rows: 4,
            cols: 4,
            m: vec![vec![0.0; 4]; 4],
        };

        m.m[0][0] = a1;
        m.m[0][1] = b1;
        m.m[0][2] = c1;
        m.m[0][3] = d1;
        m.m[1][0] = a2;
        m.m[1][1] = b2;
        m.m[1][2] = c2;
        m.m[1][3] = d2;
        m.m[2][0] = a3;
        m.m[2][1] = b3;
        m.m[2][2] = c3;
        m.m[2][3] = d3;
        m.m[3][0] = a4;
        m.m[3][1] = b4;
        m.m[3][2] = c4;
        m.m[3][3] = d4;

        m
    }

    fn new_identity_4x4() -> Matrix {
        let mut m = Matrix {
            rows: 4,
            cols: 4,
            m: vec![vec![0.0; 4]; 4],
        };

        m.m[0][0] = 1.0;
        m.m[1][1] = 1.0;
        m.m[2][2] = 1.0;
        m.m[3][3] = 1.0;

        m
    }

    fn transpose(m: &Matrix) -> Matrix {
        let mut transpose = Matrix {
            rows: m.rows,
            cols: m.cols,
            m: vec![vec![0.0; m.rows]; m.cols],
        };

        for row in 0..m.rows {
            for col in 0..m.cols {
                transpose.m[col][row] = m.m[row][col];
            }
        }
        transpose
    }

    fn determinant(m: &Matrix) -> f32 {
        if m.rows == 2 {
            return m.m[0][0] * m.m[1][1] - m.m[0][1] * m.m[1][0];
        } else if m.rows == 3 {
            return m.m[0][0] * m.m[1][1] * m.m[2][2]
                + m.m[0][1] * m.m[1][2] * m.m[2][0]
                + m.m[0][2] * m.m[1][0] * m.m[2][1]
                - m.m[0][2] * m.m[1][1] * m.m[2][0]
                - m.m[0][0] * m.m[1][2] * m.m[2][1]
                - m.m[0][1] * m.m[1][0] * m.m[2][2];
        }
        let mut det = 0.0;
        for col in 0..m.cols {
            det += m.m[0][col] * Self::cofactor(&m, 0, col);
        }
        det
    }

    fn sub_matrix(m: &Matrix, row: usize, col: usize) -> Matrix {
        let mut sub_matrix = Matrix {
            rows: m.rows - 1,
            cols: m.cols - 1,
            m: vec![vec![0.0; m.rows - 1]; m.cols - 1],
        };

        let mut dest_row = 0;
        let mut src_row = 0;

        while src_row < m.rows {
            if src_row == row {
                src_row += 1;
                continue;
            }

            let mut dest_col = 0;
            let mut src_col = 0;

            while src_col < m.cols {
                if src_col == col {
                    src_col += 1;
                    continue;
                }
                sub_matrix.m[dest_row][dest_col] = m.m[src_row][src_col];
                src_col += 1;
                dest_col += 1;
            }
            dest_row += 1;
            src_row += 1;
        }

        sub_matrix
    }

    fn minor(m: &Matrix, row: usize, col: usize) -> f32 {
        let sub = Self::sub_matrix(m, row, col);
        Self::determinant(&sub)
    }

    fn cofactor(m: &Matrix, row: usize, col: usize) -> f32 {
        let minor = Self::minor(m, row, col);
        if (row + col) % 2 != 0 && minor != 0.0 {
            return -minor;
        }
        minor
    }

    fn invert(m: &Matrix) -> Option<Matrix> {
        let mut inv = Matrix {
            rows: m.rows,
            cols: m.cols,
            m: vec![vec![0.0; m.rows]; m.cols],
        };
        if assert_two_float(Self::determinant(&m), 0.0) {
            return None;
        }

        let det = Self::determinant(&m);
        for row in 0..m.rows {
            for col in 0..m.cols {
                let c = Self::cofactor(&m, row, col);
                inv.m[col][row] = c / det;
            }
        }
        Some(inv)
    }

    fn translation(tx: f32, ty: f32, tz: f32) -> Matrix {
        let mut m = Self::new_identity_4x4();
        m.m[0][3] = tx;
        m.m[1][3] = ty;
        m.m[2][3] = tz;

        m
    }

    fn scale(sx: f32, sy: f32, sz: f32) -> Matrix {
        let mut m = Self::new_identity_4x4();
        m.m[0][0] = sx;
        m.m[1][1] = sy;
        m.m[2][2] = sz;

        m
    }

    fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix {
        let mut m = Self::new_identity_4x4();
        m.m[0][1] = xy;
        m.m[0][2] = xz;
        m.m[1][0] = yx;
        m.m[1][2] = yz;
        m.m[2][0] = zx;
        m.m[2][1] = zy;
        m
    }

    fn rotate_x(rad: f32) -> Matrix {
        let mut m = Self::new_identity_4x4();
        m.m[1][1] = rad.cos();
        m.m[1][2] = -rad.sin();
        m.m[2][1] = rad.sin();
        m.m[2][2] = rad.cos();

        m
    }

    fn rotate_y(rad: f32) -> Matrix {
        let mut m = Self::new_identity_4x4();
        m.m[0][0] = rad.cos();
        m.m[0][2] = rad.sin();
        m.m[2][0] = -rad.sin();
        m.m[2][2] = rad.cos();

        m
    }

    fn rotate_z(rad: f32) -> Matrix {
        let mut m = Self::new_identity_4x4();
        m.m[0][0] = rad.cos();
        m.m[0][1] = -rad.sin();
        m.m[1][0] = rad.sin();
        m.m[1][1] = rad.cos();

        m
    }

    fn view_transform(from: &Tuple4D, to: &Tuple4D, up: &Tuple4D) -> Matrix {
        let forward = Tuple4D::norm(&(to - from));
        let left = &forward * &Tuple4D::norm(up);
        let true_up = &left * &forward;

        let orientation = Matrix::new_matrix_4x4(
            left.x, left.y, left.z, 0.0, true_up.x, true_up.y, true_up.z, 0.0, -forward.x,
            -forward.y, -forward.z, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        orientation * Matrix::translation(-from.x, -from.y, -from.z)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        assert!(self.rows == other.rows);
        assert!(self.cols == other.cols);

        // TODO: row col and widht height correct?
        for row in 0..self.cols {
            for col in 0..self.rows {
                if !assert_two_float(self.m[col][row], other.m[col][row]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        // TODO: thats not a generic check for matrices which are non-quadratic
        assert!(self.rows == rhs.rows);
        let mut m = Matrix::new(self.rows, self.cols);

        for row in 0..self.rows {
            for col in 0..self.cols {
                let mut sum: f32 = 0.0;

                // TODO: not a generic code for general matrix dimensions
                for i in 0..self.cols {
                    sum += self.m[row][i] * rhs.m[i][col];
                }
                m.m[row][col] = sum;
            }
        }
        m
    }
}

impl<'a, 'b> Mul<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'b Matrix) -> Matrix {
        // TODO: thats not a generic check for matrices which are non-quadratic
        assert!(self.rows == rhs.rows);
        let mut m = Matrix::new(self.rows, self.cols);

        for row in 0..self.rows {
            for col in 0..self.cols {
                let mut sum: f32 = 0.0;

                // TODO: not a generic code for general matrix dimensions
                for i in 0..self.cols {
                    sum += self.m[row][i] * rhs.m[i][col];
                }
                m.m[row][col] = sum;
            }
        }
        m
    }
}

impl Mul<Tuple4D> for Matrix {
    type Output = Tuple4D;

    fn mul(self, rhs: Tuple4D) -> Tuple4D {
        assert!(self.rows == 4);
        let mut t = Tuple4D::empty();

        // TODO: not a generic code for general matrix dimensions
        t.x = self.m[0][0] * rhs.x
            + self.m[0][1] * rhs.y
            + self.m[0][2] * rhs.z
            + self.m[0][3] * rhs.w;
        t.y = self.m[1][0] * rhs.x
            + self.m[1][1] * rhs.y
            + self.m[1][2] * rhs.z
            + self.m[1][3] * rhs.w;
        t.z = self.m[2][0] * rhs.x
            + self.m[2][1] * rhs.y
            + self.m[2][2] * rhs.z
            + self.m[2][3] * rhs.w;
        t.w = self.m[3][0] * rhs.x
            + self.m[3][1] * rhs.y
            + self.m[3][2] * rhs.z
            + self.m[3][3] * rhs.w;

        t
    }
}

impl<'a, 'b> Mul<&'b Tuple4D> for &'a Matrix {
    type Output = Tuple4D;

    fn mul(self, rhs: &'b Tuple4D) -> Tuple4D {
        assert!(self.rows == 4);
        let mut t = Tuple4D::empty();

        // TODO: not a generic code for general matrix dimensions
        t.x = self.m[0][0] * rhs.x
            + self.m[0][1] * rhs.y
            + self.m[0][2] * rhs.z
            + self.m[0][3] * rhs.w;
        t.y = self.m[1][0] * rhs.x
            + self.m[1][1] * rhs.y
            + self.m[1][2] * rhs.z
            + self.m[1][3] * rhs.w;
        t.z = self.m[2][0] * rhs.x
            + self.m[2][1] * rhs.y
            + self.m[2][2] * rhs.z
            + self.m[2][3] * rhs.w;
        t.w = self.m[3][0] * rhs.x
            + self.m[3][1] * rhs.y
            + self.m[3][2] * rhs.z
            + self.m[3][3] * rhs.w;

        t
    }
}

#[test]
fn test_matrix_components() {
    let a4x4 = Matrix::new_matrix_4x4(
        1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
    );

    assert_eq!(a4x4.m[0][0], 1.0);
    assert_eq!(a4x4.m[0][3], 4.0);
    assert_eq!(a4x4.m[1][0], 5.5);
    assert_eq!(a4x4.m[1][2], 7.5);
    assert_eq!(a4x4.m[2][2], 11.0);
    assert_eq!(a4x4.m[3][0], 13.5);
    assert_eq!(a4x4.m[3][2], 15.5);

    let a3x3 = Matrix::new_matrix_3x3(-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0);

    assert_eq!(a3x3.m[0][0], -3.0);
    assert_eq!(a3x3.m[1][1], -2.0);
    assert_eq!(a3x3.m[2][2], 1.0);

    let a2x2 = Matrix::new_matrix_2x2(-3.0, 5.0, 1.0, -2.0);

    assert_eq!(a2x2.m[0][0], -3.0);
    assert_eq!(a2x2.m[0][1], 5.0);
    assert_eq!(a2x2.m[1][0], 1.0);
    assert_eq!(a2x2.m[1][1], -2.0);
}

#[test]
fn test_matrix_equal() {
    let a1 = Matrix::new_matrix_4x4(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    );

    let a2 = Matrix::new_matrix_4x4(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    );

    assert!(a1 == a2);

    let a1 = Matrix::new_matrix_4x4(
        1.1, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    );

    let a2 = Matrix::new_matrix_4x4(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    );

    assert!(a1 != a2);
}

#[test]
fn test_matrix_mul() {
    let a = Matrix::new_matrix_4x4(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
    );

    let b = Matrix::new_matrix_4x4(
        -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
    );

    let c = a * b;

    assert_float(c.m[0][0], 20.0);
    assert_float(c.m[0][1], 22.0);
    assert_float(c.m[0][2], 50.0);
    assert_float(c.m[0][3], 48.0);

    assert_float(c.m[1][0], 44.0);
    assert_float(c.m[1][1], 54.0);
    assert_float(c.m[1][2], 114.0);
    assert_float(c.m[1][3], 108.0);

    assert_float(c.m[2][0], 40.0);
    assert_float(c.m[2][1], 58.0);
    assert_float(c.m[2][2], 110.0);
    assert_float(c.m[2][3], 102.0);

    assert_float(c.m[3][0], 16.0);
    assert_float(c.m[3][1], 26.0);
    assert_float(c.m[3][2], 46.0);
    assert_float(c.m[3][3], 42.0);
}

#[test]
fn test_matrix_tuple_mul() {
    let a = Matrix::new_matrix_4x4(
        1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
    );

    let b = Tuple4D::new(1.0, 2.0, 3.0, 1.0);

    let c = &a * &b;

    assert_float(c.x, 18.0);
    assert_float(c.y, 24.0);
    assert_float(c.z, 33.0);
    assert_float(c.w, 1.0);
}

#[test]
fn test_matrix_mul_identity() {
    let a = Matrix::new_matrix_4x4(
        1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
    );

    let e = Matrix::new_identity_4x4();

    let c = a * e;

    assert_float(c.m[0][0], 1.0);
    assert_float(c.m[0][1], 2.0);
    assert_float(c.m[0][2], 3.0);
    assert_float(c.m[0][3], 4.0);

    assert_float(c.m[1][0], 2.0);
    assert_float(c.m[1][1], 4.0);
    assert_float(c.m[1][2], 4.0);
    assert_float(c.m[1][3], 2.0);

    assert_float(c.m[2][0], 8.0);
    assert_float(c.m[2][1], 6.0);
    assert_float(c.m[2][2], 4.0);
    assert_float(c.m[2][3], 1.0);

    assert_float(c.m[3][0], 0.0);
    assert_float(c.m[3][1], 0.0);
    assert_float(c.m[3][2], 0.0);
    assert_float(c.m[3][3], 1.0);
}

#[test]
fn test_matrix_transpose() {
    let a = Matrix::new_matrix_4x4(
        1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
    );

    let b = Matrix::transpose(&a);

    assert_float(b.m[0][0], 1.0);
    assert_float(b.m[0][1], 2.0);
    assert_float(b.m[0][2], 8.0);
    assert_float(b.m[0][3], 0.0);

    assert_float(b.m[1][0], 2.0);
    assert_float(b.m[1][1], 4.0);
    assert_float(b.m[1][2], 6.0);
    assert_float(b.m[1][3], 0.0);

    assert_float(b.m[2][0], 3.0);
    assert_float(b.m[2][1], 4.0);
    assert_float(b.m[2][2], 4.0);
    assert_float(b.m[2][3], 0.0);

    assert_float(b.m[3][0], 4.0);
    assert_float(b.m[3][1], 2.0);
    assert_float(b.m[3][2], 1.0);
    assert_float(b.m[3][3], 1.0);
}

#[test]
fn test_matrix_identity_is_transpose() {
    let a = Matrix::new_identity_4x4();
    let b = Matrix::transpose(&a);

    assert!(a == b);
}

#[test]
fn test_matrix_determinant() {
    let a = Matrix::new_matrix_2x2(1.0, 5.0, -3.0, 2.0);
    let b = Matrix::determinant(&a);

    assert_float(b, 17.0);
}

#[test]
fn test_matrix_submatrix() {
    let a = Matrix::new_matrix_3x3(1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0);
    let b = Matrix::sub_matrix(&a, 0, 2);

    assert_float(b.m[0][0], -3.0);
    assert_float(b.m[0][1], 2.0);
    assert_float(b.m[1][0], 0.0);
    assert_float(b.m[1][1], 6.0);

    let a = Matrix::new_matrix_4x4(
        -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
    );
    let b = Matrix::sub_matrix(&a, 2, 1);

    assert_float(b.m[0][0], -6.0);
    assert_float(b.m[0][1], 1.0);
    assert_float(b.m[0][2], 6.0);

    assert_float(b.m[1][0], -8.0);
    assert_float(b.m[1][1], 8.0);
    assert_float(b.m[1][2], 6.0);

    assert_float(b.m[2][0], -7.0);
    assert_float(b.m[2][1], -1.0);
    assert_float(b.m[2][2], 1.0);
}

#[test]
fn test_matrix_minor() {
    let a = Matrix::new_matrix_3x3(3.0, 5.0, 0.0, 3.0, -1.0, -7.0, 6.0, -1.0, 5.0);
    let b = Matrix::sub_matrix(&a, 1, 0);
    let det_b = Matrix::determinant(&b);
    let minor_a = Matrix::minor(&a, 1, 0);

    assert_float(det_b, 25.0);
    assert_float(minor_a, 25.0);
}

#[test]
fn test_matrix_cofactor() {
    let a = Matrix::new_matrix_3x3(3.0, 5.0, 0.0, 3.0, -1.0, -7.0, 6.0, -1.0, 5.0);
    let minor_a = Matrix::minor(&a, 0, 0);
    let cofactor_a = Matrix::cofactor(&a, 0, 0);

    assert_float(minor_a, -12.0);
    assert_float(cofactor_a, -12.0);

    let minor_a = Matrix::minor(&a, 1, 0);
    let cofactor_a = Matrix::cofactor(&a, 1, 0);

    assert_float(minor_a, 25.0);
    assert_float(cofactor_a, -25.0);
}

#[test]
fn test_matrix_determinant_3x3() {
    let a = Matrix::new_matrix_3x3(1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0);

    let cofactor_a1 = Matrix::cofactor(&a, 0, 0);
    let cofactor_a2 = Matrix::cofactor(&a, 0, 1);
    let cofactor_a3 = Matrix::cofactor(&a, 0, 2);

    let det_a = Matrix::determinant(&a);

    assert_float(cofactor_a1, 56.0);
    assert_float(cofactor_a2, 12.0);
    assert_float(cofactor_a3, -46.0);

    assert_float(det_a, -196.0);
}

#[test]
fn test_matrix_determinant_4x4() {
    let a = Matrix::new_matrix_4x4(
        -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
    );

    let cofactor_a1 = Matrix::cofactor(&a, 0, 0);
    let cofactor_a2 = Matrix::cofactor(&a, 0, 1);
    let cofactor_a3 = Matrix::cofactor(&a, 0, 2);
    let cofactor_a4 = Matrix::cofactor(&a, 0, 3);

    let det_a = Matrix::determinant(&a);

    assert_float(cofactor_a1, 690.0);
    assert_float(cofactor_a2, 447.0);
    assert_float(cofactor_a3, 210.0);
    assert_float(cofactor_a4, 51.0);

    assert_float(det_a, -4071.0);

    let m_inv = Matrix::invert(&a).unwrap();
    let m_expected = Matrix::new_matrix_4x4(
        -0.169492, 0.0621469, -0.135593, -0.163842, -0.109801, 0.0967821, 0.0338983, -0.006141,
        -0.0515844, 0.105871, -0.0847458, -0.0498649, -0.0125276, 0.116188, 0.0508475, -0.0454434,
    );

    println!("m_inv = {:#?}", m_inv);
    println!("m_expected = {:#?}", m_expected);

    assert_matrix(&m_inv, &m_expected);
}

#[test]
fn test_matrix_inversion1() {
    let a = Matrix::new_matrix_4x4(
        6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
    );

    let det_a = Matrix::determinant(&a);
    let a_inv = Matrix::new_matrix_4x4(
        0.315094, -0.181132, 0.0377358, -0.0150943, -0.0528302, -0.0160377, -0.132075, 0.10283,
        -0.292453, 0.259434, 0.0188679, 0.0424528, 0.122642, 0.0283019, 0.0566038, -0.122642,
    );

    assert_float(det_a, -2120.0);
    assert_matrix(&Matrix::invert(&a).unwrap(), &a_inv);

    let a = Matrix::new_matrix_4x4(
        -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
    );

    let det_a = Matrix::determinant(&a);

    assert_float(det_a, 0.0);
}

#[test]
fn test_matrix_inversion2() {
    let a = Matrix::new_matrix_4x4(
        -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
    );

    // wolfram alpha  can use this {{-5.0, 2.0, 6.0, -8.},{1.0, -5.0, 1.0, 8.0},{7.0, 7.0, -6.0, -7.0},{1.0, -3.0, 7.0, 4.}}

    let b = Matrix::invert(&a).unwrap();

    let det_a = Matrix::determinant(&a);
    let cofactor_a1 = Matrix::cofactor(&a, 2, 3);
    let cofactor_a2 = Matrix::cofactor(&a, 3, 2);

    assert_float(det_a, 532.0);

    assert_float(cofactor_a1, -160.0);
    assert_float(cofactor_a2, 105.0);

    assert_float(b.m[3][2], -160.0 / 532.0);
    assert_float(b.m[2][3], 105.0 / 532.0);

    assert_float(b.m[0][0], 0.21805);
    assert_float(b.m[0][1], 0.45113);
    assert_float(b.m[0][2], 0.24060);
    assert_float(b.m[0][3], -0.04511);

    assert_float(b.m[1][0], -0.80827);
    assert_float(b.m[1][1], -1.45677);
    assert_float(b.m[1][2], -0.44361);
    assert_float(b.m[1][3], 0.52068);

    assert_float(b.m[2][0], -0.07895);
    assert_float(b.m[2][1], -0.22368);
    assert_float(b.m[2][2], -0.05263);
    assert_float(b.m[2][3], 0.19737);

    assert_float(b.m[3][0], -0.52256);
    assert_float(b.m[3][1], -0.81391);
    assert_float(b.m[3][2], -0.30075);
    assert_float(b.m[3][3], 0.30639);
}

#[test]
fn test_matrix_inversion3() {
    let a = Matrix::new_matrix_4x4(
        8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
    );

    let b = Matrix::invert(&a).unwrap();

    assert_float(b.m[0][0], -0.15385);
    assert_float(b.m[0][1], -0.15385);
    assert_float(b.m[0][2], -0.28205);
    assert_float(b.m[0][3], -0.53846);

    assert_float(b.m[1][0], -0.07692);
    assert_float(b.m[1][1], 0.12308);
    assert_float(b.m[1][2], 0.02564);
    assert_float(b.m[1][3], 0.03077);

    assert_float(b.m[2][0], 0.35897);
    assert_float(b.m[2][1], 0.35897);
    assert_float(b.m[2][2], 0.43590);
    assert_float(b.m[2][3], 0.92308);

    assert_float(b.m[3][0], -0.69231);
    assert_float(b.m[3][1], -0.69231);
    assert_float(b.m[3][2], -0.76923);
    assert_float(b.m[3][3], -1.92308);
}

#[test]
fn test_matrix_inversion4() {
    let a = Matrix::new_matrix_4x4(
        9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0, 2.0,
    );

    let b = Matrix::invert(&a).unwrap();

    assert_float(b.m[0][0], -0.04074);
    assert_float(b.m[0][1], -0.07778);
    assert_float(b.m[0][2], 0.14444);
    assert_float(b.m[0][3], -0.22222);

    assert_float(b.m[1][0], -0.07778);
    assert_float(b.m[1][1], 0.03333);
    assert_float(b.m[1][2], 0.36667);
    assert_float(b.m[1][3], -0.33333);

    assert_float(b.m[2][0], -0.02901);
    assert_float(b.m[2][1], -0.14630);
    assert_float(b.m[2][2], -0.10926);
    assert_float(b.m[2][3], 0.12963);

    assert_float(b.m[3][0], 0.17778);
    assert_float(b.m[3][1], 0.06667);
    assert_float(b.m[3][2], -0.26667);
    assert_float(b.m[3][3], 0.33333);
}

#[test]
fn test_matrix_inversion5() {
    let a = Matrix::new_matrix_4x4(
        3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0,
    );

    let b = Matrix::new_matrix_4x4(
        8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
    );

    let c = &a * &b;

    let a2 = c * Matrix::invert(&b).unwrap();

    assert_float(a.m[0][0], a2.m[0][0]);
    assert_float(a.m[0][1], a2.m[0][1]);
    assert_float(a.m[0][2], a2.m[0][2]);
    assert_float(a.m[0][3], a2.m[0][3]);

    assert_float(a.m[1][0], a2.m[1][0]);
    assert_float(a.m[1][1], a2.m[1][1]);
    assert_float(a.m[1][2], a2.m[1][2]);
    assert_float(a.m[1][3], a2.m[1][3]);

    assert_float(a.m[2][0], a2.m[2][0]);
    assert_float(a.m[2][1], a2.m[2][1]);
    assert_float(a.m[2][2], a2.m[2][2]);
    assert_float(a.m[2][3], a2.m[2][3]);

    assert_float(a.m[3][0], a2.m[3][0]);
    assert_float(a.m[3][1], a2.m[3][1]);
    assert_float(a.m[3][2], a2.m[3][2]);
    assert_float(a.m[3][3], a2.m[3][3]);
}

#[test]
fn test_matrix_transformation() {
    let transform = Matrix::translation(5.0, -3.0, 2.0);
    let p = Tuple4D::new_point(-3.0, 4.0, 5.0);

    let p_transformed = &transform * &p;

    assert_float(p_transformed.x, 2.0);
    assert_float(p_transformed.y, 1.0);
    assert_float(p_transformed.z, 7.0);
    assert!(Tuple4D::is_point(&p_transformed));
}

#[test]
fn test_matrix_transformation_invert() {
    let transform = Matrix::translation(5.0, -3.0, 2.0);
    let inv = Matrix::invert(&transform).unwrap();

    let p = Tuple4D::new_point(-3.0, 4.0, 5.0);

    let p_transformed = &inv * &p;

    assert_float(p_transformed.x, -8.0);
    assert_float(p_transformed.y, 7.0);
    assert_float(p_transformed.z, 3.0);
    assert!(Tuple4D::is_point(&p_transformed));
}

#[test]
fn test_matrix_transformation_Tuple4D() {
    let transform = Matrix::translation(5.0, -3.0, 2.0);
    let v = Tuple4D::new_vector(-3.0, 4.0, 5.0);

    let v_transformed = &transform * &v;

    assert_float(v_transformed.x, v.x);
    assert_float(v_transformed.y, v.y);
    assert_float(v_transformed.z, v.z);
    assert!(Tuple4D::is_Tuple4D(&v_transformed));
}

#[test]
fn test_matrix_scale_point() {
    let transform = Matrix::scale(2.0, 3.0, 4.0);
    let p = Tuple4D::new_point(-4.0, 6.0, 8.0);

    let p_transformed = &transform * &p;

    assert_float(p_transformed.x, -8.0);
    assert_float(p_transformed.y, 18.0);
    assert_float(p_transformed.z, 32.0);
    assert!(Tuple4D::is_point(&p_transformed));
}

#[test]
fn test_matrix_scale_Tuple4D() {
    let transform = Matrix::scale(2.0, 3.0, 4.0);
    let v = Tuple4D::new_point(-4.0, 6.0, 8.0);

    let v_transformed = &transform * &v;

    assert_float(v_transformed.x, -8.0);
    assert_float(v_transformed.y, 18.0);
    assert_float(v_transformed.z, 32.0);
    assert!(Tuple4D::is_point(&v_transformed));
}

#[test]
fn test_matrix_scale_Tuple4D_invert() {
    let transform = Matrix::scale(2.0, 3.0, 4.0);
    let inv = Matrix::invert(&transform).unwrap();

    let v = Tuple4D::new_vector(-4.0, 6.0, 8.0);

    let v_transformed = &inv * &v;

    assert_float(v_transformed.x, -2.0);
    assert_float(v_transformed.y, 2.0);
    assert_float(v_transformed.z, 2.0);
    assert!(Tuple4D::is_Tuple4D(&v_transformed));
}

#[test]
fn test_matrix_scale_Tuple4D_reflection() {
    let transform = Matrix::scale(-1.0, 1.0, 1.0);
    let p = Tuple4D::new_point(2.0, 3.0, 4.0);

    let p_transformed = &transform * &p;

    assert_float(p_transformed.x, -2.0);
    assert_float(p_transformed.y, 3.0);
    assert_float(p_transformed.z, 4.0);
    assert!(Tuple4D::is_point(&p_transformed));
}

#[test]
fn test_matrix_rotation_x() {
    let p = Tuple4D::new_point(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotate_x(PI / 4.0);
    let full_quarter = Matrix::rotate_x(PI / 2.0);

    let half = &half_quarter * &p;
    let full = &full_quarter * &p;

    let sqrt2_half = 2.0_f32.sqrt() / 2.0;

    assert_float(half.x, 0.0);
    assert_float(half.y, sqrt2_half);
    assert_float(half.z, sqrt2_half);
    assert!(Tuple4D::is_point(&half));

    assert_float(full.x, 0.0);
    assert_float(full.y, 0.0);
    assert_float(full.z, 1.0);
    assert!(Tuple4D::is_point(&full));
}

#[test]
fn test_matrix_rotation_x_invert() {
    let p = Tuple4D::new_point(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotate_x(PI / 4.0);
    let inv = Matrix::invert(&half_quarter).unwrap();

    let half = &inv * &p;

    let sqrt2_half = SQRT_2 / 2.0;
    let expected = Tuple4D::new_point(0.0, sqrt2_half, -sqrt2_half);
    assert_Tuple4D(&half, &expected);

    assert_float(half.x, 0.0);
    assert_float(half.y, sqrt2_half);
    assert_float(half.z, -sqrt2_half);
    assert!(Tuple4D::is_point(&half));
}

#[test]
fn test_matrix_rotation_y() {
    let p = Tuple4D::new_point(0.0, 0.0, 1.0);
    let half_quarter = Matrix::rotate_y(PI / 4.0);
    let full_quarter = Matrix::rotate_y(PI / 2.0);

    let half = &half_quarter * &p;
    let full = &full_quarter * &p;

    let sqrt2_half = SQRT_2 / 2.0;

    let half_expected = Tuple4D::new_point(sqrt2_half, 0.0, sqrt2_half);
    assert_Tuple4D(&half, &half_expected);

    assert_float(half.x, sqrt2_half);
    assert_float(half.y, 0.0);
    assert_float(half.z, sqrt2_half);
    assert!(Tuple4D::is_point(&half));

    let full_expected = Tuple4D::new_point(1.0, 0.0, 0.0);
    assert_Tuple4D(&full, &full_expected);

    assert_float(full.x, 1.0);
    assert_float(full.y, 0.0);
    assert_float(full.z, 0.0);
    assert!(Tuple4D::is_point(&full));
}

#[test]
fn test_matrix_rotation_z() {
    let p = Tuple4D::new_point(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotate_z(PI / 4.0);
    let full_quarter = Matrix::rotate_z(PI / 2.0);

    let half = &half_quarter * &p;
    let full = &full_quarter * &p;

    let sqrt2_half = SQRT_2 / 2.0;

    let half_expected = Tuple4D::new_point(-sqrt2_half, sqrt2_half, 0.0);
    assert_Tuple4D(&half, &half_expected);

    assert_float(half.x, -sqrt2_half);
    assert_float(half.y, sqrt2_half);
    assert_float(half.z, 0.0);
    assert!(Tuple4D::is_point(&half));

    let full_expected = Tuple4D::new_point(-1.0, 0.0, 0.0);
    assert_Tuple4D(&full, &full_expected);

    assert_float(full.x, -1.0);
    assert_float(full.y, 0.0);
    assert_float(full.z, 0.0);
    assert!(Tuple4D::is_point(&full));
}

#[test]
fn test_matrix_shear1() {
    let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let p = Tuple4D::new_point(2.0, 3.0, 4.0);
    let p_transformed = &transform * &p;

    assert_float(p_transformed.x, 5.0);
    assert_float(p_transformed.y, 3.0);
    assert_float(p_transformed.z, 4.0);
    assert!(Tuple4D::is_point(&p_transformed));

    let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let p = Tuple4D::new_point(2.0, 3.0, 4.0);
    let p_transformed = &transform * &p;

    assert_float(p_transformed.x, 6.0);
    assert_float(p_transformed.y, 3.0);
    assert_float(p_transformed.z, 4.0);
    assert!(Tuple4D::is_point(&p_transformed));

    let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let p = Tuple4D::new_point(2.0, 3.0, 4.0);
    let p_transformed = &transform * &p;

    assert_float(p_transformed.x, 2.0);
    assert_float(p_transformed.y, 5.0);
    assert_float(p_transformed.z, 4.0);
    assert!(Tuple4D::is_point(&p_transformed));

    let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let p = Tuple4D::new_point(2.0, 3.0, 4.0);
    let p_transformed = &transform * &p;

    assert_float(p_transformed.x, 2.0);
    assert_float(p_transformed.y, 7.0);
    assert_float(p_transformed.z, 4.0);
    assert!(Tuple4D::is_point(&p_transformed));

    let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let p = Tuple4D::new_point(2.0, 3.0, 4.0);
    let p_transformed = &transform * &p;

    assert_float(p_transformed.x, 2.0);
    assert_float(p_transformed.y, 3.0);
    assert_float(p_transformed.z, 6.0);
    assert!(Tuple4D::is_point(&p_transformed));

    let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let p = Tuple4D::new_point(2.0, 3.0, 4.0);
    let p_transformed = &transform * &p;

    assert_float(p_transformed.x, 2.0);
    assert_float(p_transformed.y, 3.0);
    assert_float(p_transformed.z, 7.0);
    assert!(Tuple4D::is_point(&p_transformed));
}

#[test]
fn test_matrix_transformation_sequential() {
    let p = Tuple4D::new_point(1.0, 0.0, 1.0);
    let a = Matrix::rotate_x(PI / 2.0);
    let b = Matrix::scale(5.0, 5.0, 5.0);
    let c = Matrix::translation(10.0, 5.0, 7.0);

    let p2 = &a * &p;
    let p3 = &b * &p2;
    let p4 = &c * &p3;

    assert_float(p2.x, 1.0);
    assert_float(p2.y, -1.0);
    assert_float(p2.z, 0.0);
    assert!(Tuple4D::is_point(&p2));

    assert_float(p3.x, 5.0);
    assert_float(p3.y, -5.0);
    assert_float(p3.z, 0.0);
    assert!(Tuple4D::is_point(&p3));

    assert_float(p4.x, 15.0);
    assert_float(p4.y, 0.0);
    assert_float(p4.z, 7.0);
    assert!(Tuple4D::is_point(&p4));
}

#[test]
fn test_matrix_transformation_chained() {
    let p = Tuple4D::new_point(1.0, 0.0, 1.0);
    let a = Matrix::rotate_x(PI / 2.0);
    let b = Matrix::scale(5.0, 5.0, 5.0);
    let c = Matrix::translation(10.0, 5.0, 7.0);

    let t = &(&c * &b) * &a;

    let p2 = &t * &p;

    assert_float(p2.x, 15.0);
    assert_float(p2.y, 0.0);
    assert_float(p2.z, 7.0);
    assert!(Tuple4D::is_point(&p2));
}

#[test]
fn test_matrix_view_transform_default_direction() {
    let from = Tuple4D::new_point(0.0, 0.0, 0.0);
    let to = Tuple4D::new_point(0.0, 0.0, -1.0);
    let up = Tuple4D::new_vector(0.0, 1.0, 0.0);

    let v = Matrix::view_transform(&from, &to, &up);
    let v_expected = Matrix::new_identity_4x4();

    assert_matrix(&v, &v_expected);
}

#[test]
fn test_matrix_view_transform_positive_z_direction() {
    let from = Tuple4D::new_point(0.0, 0.0, 0.0);
    let to = Tuple4D::new_point(0.0, 0.0, 1.0);
    let up = Tuple4D::new_vector(0.0, 1.0, 0.0);

    let v = Matrix::view_transform(&from, &to, &up);
    let v_expected = Matrix::scale(-1.0, 1.0, -1.0);

    assert_matrix(&v, &v_expected);
}

#[test]
fn test_matrix_view_transform_translation() {
    let from = Tuple4D::new_point(0.0, 0.0, 8.0);
    let to = Tuple4D::new_point(0.0, 0.0, 0.0);
    let up = Tuple4D::new_vector(0.0, 1.0, 0.0);

    let v = Matrix::view_transform(&from, &to, &up);
    let v_expected = Matrix::translation(0.0, 0.0, -8.0);

    assert_matrix(&v, &v_expected);
}

#[test]
fn test_matrix_view_transform_arbitrary() {
    let from = Tuple4D::new_point(1.0, 3.0, 2.0);
    let to = Tuple4D::new_point(4.0, -2.0, 8.0);
    let up = Tuple4D::new_vector(1.0, 1.0, 0.0);

    let v = Matrix::view_transform(&from, &to, &up);

    let v_expected = Matrix::new_matrix_4x4(
        -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843, -0.35857,
        0.59761, -0.71714, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    assert_matrix(&v, &v_expected);
}
