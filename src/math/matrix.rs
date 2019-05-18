use std::ops::Mul;

use crate::math::common::float_equal;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub  m: Vec<Vec<f32>>,
}


pub trait MatrixOps {
    fn new(row: usize, col: usize) -> Matrix;

    fn new_matrix_2x2(a1: f32, b1: f32,
                      a2: f32, b2: f32) -> Matrix;

    fn new_matrix_3x3(a1: f32, b1: f32, c1: f32,
                      a2: f32, b2: f32, c2: f32,
                      a3: f32, b3: f32, c3: f32) -> Matrix;

    fn new_matrix_4x4(a1: f32, b1: f32, c1: f32, d1: f32,
                      a2: f32, b2: f32, c2: f32, d2: f32,
                      a3: f32, b3: f32, c3: f32, d3: f32,
                      a4: f32, b4: f32, c4: f32, d4: f32) -> Matrix;

    fn new_identity_4x4() -> Matrix;
    fn transpose(m: &Matrix) -> Matrix;

    fn determinant(m: &Matrix) -> f32;

    fn sub_matrix(m: &Matrix, row: usize, col: usize) -> Matrix;
    fn minor(m: &Matrix, row: usize, col: usize) -> f32;
    fn cofactor(m: &Matrix, row: usize, col: usize) -> f32;
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

    fn new_matrix_3x3(a1: f32, b1: f32, c1: f32, a2: f32, b2: f32, c2: f32, a3: f32, b3: f32, c3: f32) -> Matrix {
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

    fn new_matrix_4x4(a1: f32, b1: f32, c1: f32, d1: f32,
                      a2: f32, b2: f32, c2: f32, d2: f32,
                      a3: f32, b3: f32, c3: f32, d3: f32,
                      a4: f32, b4: f32, c4: f32, d4: f32) -> Matrix {
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
            return m.m[0][0] * m.m[1][1] * m.m[2][2] +
                m.m[0][1] * m.m[1][2] * m.m[2][0] +
                m.m[0][2] * m.m[1][0] * m.m[2][1] -
                m.m[0][2] * m.m[1][1] * m.m[2][0] -
                m.m[0][0] * m.m[1][2] * m.m[2][1] -
                m.m[0][1] * m.m[1][0] * m.m[2][2];
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
        if (row + col) % 2 != 0 {
            return -minor;
        }
        minor
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        assert!(self.rows == other.rows);
        assert!(self.cols == other.cols);

        // TODO: row col and widht height correct?
        for row in 0..self.cols {
            for col in 0..self.rows {
                if !float_equal(self.m[col][row], other.m[col][row]) {
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

impl Mul<Tuple4D> for Matrix {
    type Output = Tuple4D;

    fn mul(self, rhs: Tuple4D) -> Tuple4D {
        assert!(self.rows == 4);
        let mut t = Tuple4D::empty();

        // TODO: not a generic code for general matrix dimensions
        t.x = self.m[0][0] * rhs.x + self.m[0][1] * rhs.y + self.m[0][2] * rhs.z + self.m[0][3] * rhs.w;
        t.y = self.m[1][0] * rhs.x + self.m[1][1] * rhs.y + self.m[1][2] * rhs.z + self.m[1][3] * rhs.w;
        t.z = self.m[2][0] * rhs.x + self.m[2][1] * rhs.y + self.m[2][2] * rhs.z + self.m[2][3] * rhs.w;
        t.w = self.m[3][0] * rhs.x + self.m[3][1] * rhs.y + self.m[3][2] * rhs.z + self.m[3][3] * rhs.w;

        t
    }
}

#[test]
fn test_matrix_components() {
    let a4x4 = Matrix::new_matrix_4x4(1.0, 2.0, 3.0, 4.0,
                                      5.5, 6.5, 7.5, 8.5,
                                      9.0, 10.0, 11.0, 12.0,
                                      13.5, 14.5, 15.5, 16.5);

    assert_eq!(a4x4.m[0][0], 1.0);
    assert_eq!(a4x4.m[0][3], 4.0);
    assert_eq!(a4x4.m[1][0], 5.5);
    assert_eq!(a4x4.m[1][2], 7.5);
    assert_eq!(a4x4.m[2][2], 11.0);
    assert_eq!(a4x4.m[3][0], 13.5);
    assert_eq!(a4x4.m[3][2], 15.5);

    let a3x3 = Matrix::new_matrix_3x3(-3.0, 5.0, 0.0,
                                      1.0, -2.0, -7.0,
                                      0.0, 1.0, 1.0);

    assert_eq!(a3x3.m[0][0], -3.0);
    assert_eq!(a3x3.m[1][1], -2.0);
    assert_eq!(a3x3.m[2][2], 1.0);

    let a2x2 = Matrix::new_matrix_2x2(-3.0, 5.0,
                                      1.0, -2.0);

    assert_eq!(a2x2.m[0][0], -3.0);
    assert_eq!(a2x2.m[0][1], 5.0);
    assert_eq!(a2x2.m[1][0], 1.0);
    assert_eq!(a2x2.m[1][1], -2.0);
}


#[test]
fn test_matrix_equal() {
    let a1 = Matrix::new_matrix_4x4(1.0, 2.0, 3.0, 4.0,
                                    5.0, 6.0, 7.0, 8.0,
                                    9.0, 10.0, 11.0, 12.0,
                                    13.0, 14.0, 15.0, 16.0);

    let a2 = Matrix::new_matrix_4x4(1.0, 2.0, 3.0, 4.0,
                                    5.0, 6.0, 7.0, 8.0,
                                    9.0, 10.0, 11.0, 12.0,
                                    13.0, 14.0, 15.0, 16.0);

    assert_eq!(a1 == a2, true);

    let a1 = Matrix::new_matrix_4x4(1.1, 2.0, 3.0, 4.0,
                                    5.0, 6.0, 7.0, 8.0,
                                    9.0, 10.0, 11.0, 12.0,
                                    13.0, 14.0, 15.0, 16.0);

    let a2 = Matrix::new_matrix_4x4(1.0, 2.0, 3.0, 4.0,
                                    5.0, 6.0, 7.0, 8.0,
                                    9.0, 10.0, 11.0, 12.0,
                                    13.0, 14.0, 15.0, 16.0);

    assert_eq!(a1 != a2, true);
}


#[test]
fn test_matrix_mul() {
    let a = Matrix::new_matrix_4x4(1.0, 2.0, 3.0, 4.0,
                                   5.0, 6.0, 7.0, 8.0,
                                   9.0, 8.0, 7.0, 6.0,
                                   5.0, 4.0, 3.0, 2.0);

    let b = Matrix::new_matrix_4x4(-2.0, 1.0, 2.0, 3.0,
                                   3.0, 2.0, 1.0, -1.0,
                                   4.0, 3.0, 6.0, 5.0,
                                   1.0, 2.0, 7.0, 8.0);

    let c = a * b;


    assert_eq!(float_equal(c.m[0][0], 20.0), true);
    assert_eq!(float_equal(c.m[0][1], 22.0), true);
    assert_eq!(float_equal(c.m[0][2], 50.0), true);
    assert_eq!(float_equal(c.m[0][3], 48.0), true);

    assert_eq!(float_equal(c.m[1][0], 44.0), true);
    assert_eq!(float_equal(c.m[1][1], 54.0), true);
    assert_eq!(float_equal(c.m[1][2], 114.0), true);
    assert_eq!(float_equal(c.m[1][3], 108.0), true);

    assert_eq!(float_equal(c.m[2][0], 40.0), true);
    assert_eq!(float_equal(c.m[2][1], 58.0), true);
    assert_eq!(float_equal(c.m[2][2], 110.0), true);
    assert_eq!(float_equal(c.m[2][3], 102.0), true);

    assert_eq!(float_equal(c.m[3][0], 16.0), true);
    assert_eq!(float_equal(c.m[3][1], 26.0), true);
    assert_eq!(float_equal(c.m[3][2], 46.0), true);
    assert_eq!(float_equal(c.m[3][3], 42.0), true);
}


#[test]
fn test_matrix_tuple_mul() {
    let a = Matrix::new_matrix_4x4(1.0, 2.0, 3.0, 4.0,
                                   2.0, 4.0, 4.0, 2.0,
                                   8.0, 6.0, 4.0, 1.0,
                                   0.0, 0.0, 0.0, 1.0);

    let b = Tuple4D::new(1.0, 2.0, 3.0, 1.0);

    let c = a * b;

    assert_eq!(float_equal(c.x, 18.0), true);
    assert_eq!(float_equal(c.y, 24.0), true);
    assert_eq!(float_equal(c.z, 33.0), true);
    assert_eq!(float_equal(c.w, 1.0), true);
}


#[test]
fn test_matrix_mul_identity() {
    let a = Matrix::new_matrix_4x4(1.0, 2.0, 3.0, 4.0,
                                   2.0, 4.0, 4.0, 2.0,
                                   8.0, 6.0, 4.0, 1.0,
                                   0.0, 0.0, 0.0, 1.0);

    let e = Matrix::new_identity_4x4();

    let c = a * e;

    assert_eq!(float_equal(c.m[0][0], 1.0), true);
    assert_eq!(float_equal(c.m[0][1], 2.0), true);
    assert_eq!(float_equal(c.m[0][2], 3.0), true);
    assert_eq!(float_equal(c.m[0][3], 4.0), true);

    assert_eq!(float_equal(c.m[1][0], 2.0), true);
    assert_eq!(float_equal(c.m[1][1], 4.0), true);
    assert_eq!(float_equal(c.m[1][2], 4.0), true);
    assert_eq!(float_equal(c.m[1][3], 2.0), true);

    assert_eq!(float_equal(c.m[2][0], 8.0), true);
    assert_eq!(float_equal(c.m[2][1], 6.0), true);
    assert_eq!(float_equal(c.m[2][2], 4.0), true);
    assert_eq!(float_equal(c.m[2][3], 1.0), true);

    assert_eq!(float_equal(c.m[3][0], 0.0), true);
    assert_eq!(float_equal(c.m[3][1], 0.0), true);
    assert_eq!(float_equal(c.m[3][2], 0.0), true);
    assert_eq!(float_equal(c.m[3][3], 1.0), true);
}


#[test]
fn test_matrix_transpose() {
    let a = Matrix::new_matrix_4x4(1.0, 2.0, 3.0, 4.0,
                                   2.0, 4.0, 4.0, 2.0,
                                   8.0, 6.0, 4.0, 1.0,
                                   0.0, 0.0, 0.0, 1.0);

    let b = Matrix::transpose(&a);


    assert_eq!(float_equal(b.m[0][0], 1.0), true);
    assert_eq!(float_equal(b.m[0][1], 2.0), true);
    assert_eq!(float_equal(b.m[0][2], 8.0), true);
    assert_eq!(float_equal(b.m[0][3], 0.0), true);

    assert_eq!(float_equal(b.m[1][0], 2.0), true);
    assert_eq!(float_equal(b.m[1][1], 4.0), true);
    assert_eq!(float_equal(b.m[1][2], 6.0), true);
    assert_eq!(float_equal(b.m[1][3], 0.0), true);

    assert_eq!(float_equal(b.m[2][0], 3.0), true);
    assert_eq!(float_equal(b.m[2][1], 4.0), true);
    assert_eq!(float_equal(b.m[2][2], 4.0), true);
    assert_eq!(float_equal(b.m[2][3], 0.0), true);

    assert_eq!(float_equal(b.m[3][0], 4.0), true);
    assert_eq!(float_equal(b.m[3][1], 2.0), true);
    assert_eq!(float_equal(b.m[3][2], 1.0), true);
    assert_eq!(float_equal(b.m[3][3], 1.0), true);
}


#[test]
fn test_matrix_identity_is_transpose() {
    let a = Matrix::new_identity_4x4();
    let b = Matrix::transpose(&a);

    assert_eq!(a == b, true);
}

#[test]
fn test_matrix_determinant() {
    let a = Matrix::new_matrix_2x2(1.0, 5.0, -3.0, 2.0);
    let b = Matrix::determinant(&a);

    assert_eq!(float_equal(b, 17.0), true);
}

#[test]
fn test_matrix_submatrix() {
    let a = Matrix::new_matrix_3x3(1.0, 5.0, 0.0,
                                   -3.0, 2.0, 7.0,
                                   0.0, 6.0, -3.0);
    let b = Matrix::sub_matrix(&a, 0, 2);

    assert_eq!(float_equal(b.m[0][0], -3.0), true);
    assert_eq!(float_equal(b.m[0][1], 2.0), true);
    assert_eq!(float_equal(b.m[1][0], 0.0), true);
    assert_eq!(float_equal(b.m[1][1], 6.0), true);

    let a = Matrix::new_matrix_4x4(-6.0, 1.0, 1.0, 6.0,
                                   -8.0, 5.0, 8.0, 6.0,
                                   -1.0, 0.0, 8.0, 2.0,
                                   -7.0, 1.0, -1.0, 1.0);
    let b = Matrix::sub_matrix(&a, 2, 1);

    assert_eq!(float_equal(b.m[0][0], -6.0), true);
    assert_eq!(float_equal(b.m[0][1], 1.0), true);
    assert_eq!(float_equal(b.m[0][2], 6.0), true);

    assert_eq!(float_equal(b.m[1][0], -8.0), true);
    assert_eq!(float_equal(b.m[1][1], 8.0), true);
    assert_eq!(float_equal(b.m[1][2], 6.0), true);

    assert_eq!(float_equal(b.m[2][0], -7.0), true);
    assert_eq!(float_equal(b.m[2][1], -1.0), true);
    assert_eq!(float_equal(b.m[2][2], 1.0), true);
}


#[test]
fn test_matrix_minor() {
    let a = Matrix::new_matrix_3x3(3.0, 5.0, 0.0,
                                   3.0, -1.0, -7.0,
                                   6.0, -1.0, 5.0);
    let b = Matrix::sub_matrix(&a, 1, 0);
    let det_b = Matrix::determinant(&b);
    let minor_a = Matrix::minor(&a, 1, 0);

    assert_eq!(float_equal(det_b, 25.0), true);
    assert_eq!(float_equal(minor_a, 25.0), true);
}

#[test]
fn test_matrix_cofactor() {
    let a = Matrix::new_matrix_3x3(3.0, 5.0, 0.0,
                                   3.0, -1.0, -7.0,
                                   6.0, -1.0, 5.0);
    let minor_a = Matrix::minor(&a, 0, 0);
    let cofactor_a = Matrix::cofactor(&a, 0, 0);

    assert_eq!(float_equal(minor_a, -12.0), true);
    assert_eq!(float_equal(cofactor_a, -12.0), true);

    let minor_a = Matrix::minor(&a, 1, 0);
    let cofactor_a = Matrix::cofactor(&a, 1, 0);

    assert_eq!(float_equal(minor_a, 25.0), true);
    assert_eq!(float_equal(cofactor_a, -25.0), true);
}

#[test]
fn test_matrix_determinant_3x3() {
    let a = Matrix::new_matrix_3x3(1.0, 2.0, 6.0,
                                   -5.0, 8.0, -4.0,
                                   2.0, 6.0, 4.0);

    let cofactor_a1 = Matrix::cofactor(&a, 0, 0);
    let cofactor_a2 = Matrix::cofactor(&a, 0, 1);
    let cofactor_a3 = Matrix::cofactor(&a, 0, 2);

    let det_a = Matrix::determinant(&a);

    assert_eq!(float_equal(cofactor_a1, 56.0), true);
    assert_eq!(float_equal(cofactor_a2, 12.0), true);
    assert_eq!(float_equal(cofactor_a3, -46.0), true);

    assert_eq!(float_equal(det_a, -196.0), true);
}

#[test]
fn test_matrix_determinant_4x4() {
    let a = Matrix::new_matrix_4x4(-2.0, -8.0, 3.0, 5.0,
                                   -3.0, 1.0, 7.0, 3.0,
                                   1.0, 2.0, -9.0, 6.0,
                                   -6.0, 7.0, 7.0, -9.0);

    let cofactor_a1 = Matrix::cofactor(&a, 0, 0);
    let cofactor_a2 = Matrix::cofactor(&a, 0, 1);
    let cofactor_a3 = Matrix::cofactor(&a, 0, 2);
    let cofactor_a4 = Matrix::cofactor(&a, 0, 3);

    let det_a = Matrix::determinant(&a);

    assert_eq!(float_equal(cofactor_a1, 690.0), true);
    assert_eq!(float_equal(cofactor_a2, 447.0), true);
    assert_eq!(float_equal(cofactor_a3, 210.0), true);
    assert_eq!(float_equal(cofactor_a4, 51.0), true);

    assert_eq!(float_equal(det_a, -4071.0), true);
}


