use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, BitXor, Mul, Neg, Sub};

pub struct MathTensor {
    data: Vec<f64>,
    shape: Vec<usize>,
}

impl MathTensor {
    pub fn new(shape: Vec<usize>, data: Vec<f64>) -> Self {
        MathTensor { data, shape }
    }

    pub fn zeroes(shape: Vec<usize>) -> Self {
        let mut size = 1;
        shape.iter().for_each(|s| size *= s);
        let data = vec![0_f64; size];
        MathTensor { data, shape }
    }

    pub fn ones(shape: Vec<usize>) -> Self {
        let mut size = 1;
        shape.iter().for_each(|s| size *= s);
        let data = vec![1_f64; size];
        MathTensor { data, shape }
    }

    pub fn value(shape: Vec<usize>, value: f64) -> Self {
        let mut size = 1;
        shape.iter().for_each(|s| size *= s);
        let data = vec![value; size];
        MathTensor { data, shape }
    }

    pub fn data(&self) -> &Vec<f64> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Vec<f64> {
        &mut self.data
    }

    pub fn shape(&self) -> String {
        let shapes: Vec<String> = self.shape.iter().map(|i| i.to_string()).collect();
        let s = format!("({})", shapes.join(", "));
        s
    }

    pub fn shape_vec(&self) -> &Vec<usize> {
        &self.shape
    }

    pub fn shape_copy(&self) -> Vec<usize> {
        let copy: Vec<usize> = self.shape_vec().iter().map(|s| *s).collect();
        copy
    }

    pub fn set_elem(&mut self, pos: Vec<usize>, data: f64) {
        // TODO multidimensional
        let idx = self.idx(pos);
        self.data[idx] = data;
    }

    pub fn elem(&self, pos: Vec<usize>) -> f64 {
        // TODO multidimensional
        //  println!("pos {:?}  ", &pos);
        let idx = self.idx(pos);
        // println!("                 -->   idx {:?}  ", idx);
        self.data[idx]
    }

    fn idx(&self, pos: Vec<usize>) -> usize {
        self.shape[1] * pos[0] + pos[1]
    }

    pub fn pow(&self, n: f64) -> MathTensor {
        let a: Vec<f64> = self.data().iter().map(|a| a.powf(n)).collect();
        let shape = self.shape_copy();
        MathTensor::new(shape, a)
    }

    pub fn exp(&self) -> MathTensor {
        let a: Vec<f64> = self.data().iter().map(|a| a.exp()).collect();
        let shape = self.shape_copy();
        MathTensor::new(shape, a)
    }

    pub fn tanh(&self) -> MathTensor {
        let a: Vec<f64> = self.data().iter().map(|a| a.tanh()).collect();
        let shape = self.shape_copy();
        MathTensor::new(shape, a)
    }

    pub fn relu(&self) -> MathTensor {
        let a: Vec<f64> = self.data().iter().map(|a| a.max(0.0)).collect();
        let shape = self.shape_copy();
        MathTensor::new(shape, a)
    }

    pub fn sum(&self) -> MathTensor {
        let sum: f64 = self.data().iter().map(|x| *x).sum();
        let shape = vec![1, 1];
        MathTensor::new(shape, vec![sum])
    }

    pub(crate) fn set_zero(&mut self) {
        self.data_mut().iter_mut().for_each(|a| *a = 1.0);
    }

    pub fn transpose(&self) -> MathTensor {
        // TODO
        if self.shape_copy().len() != 2 {
            panic!("can't handle transpose for tensor != dim 2");
        }
        let shape = self.shape_vec().iter().rev().map(|s| *s).collect();
        let data: Vec<f64> = self.data().iter().map(|d| *d).collect();
        MathTensor::new(shape, data)
    }
}

#[derive(Debug)]
struct BroadcastError {
    details: String,
}

impl BroadcastError {
    fn new(msg: &str) -> BroadcastError {
        BroadcastError {
            details: msg.to_string(),
        }
    }
}

impl Display for BroadcastError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for BroadcastError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
enum BroadcastType {
    None,
    First,
}

fn fmt_shape(a: &Vec<usize>) -> String {
    let shapes: Vec<String> = a.iter().map(|i| i.to_string()).collect();
    let s = format!("({})", shapes.join(", "));
    s
}

// simplified rules, only supports non matching dimensions if it is the first one
// https://pytorch.org/docs/stable/notes/broadcasting.html
fn broadcast(a: &Vec<usize>, b: &Vec<usize>) -> std::result::Result<BroadcastType, BroadcastError> {
    if a.len() != b.len() && (b.len() - a.len()) != 1 {
        return Err(BroadcastError::new(&format!(
            "can't broadcast a: {:?} b: {:?}",
            fmt_shape(a),
            fmt_shape(b)
        )));
    }
    if a.len() == b.len() {
        let equal = a
            .iter()
            .zip(b.iter())
            .map(|(aa, bb)| *aa == *bb)
            .fold(true, |acc, x| acc && x);
        if equal {
            return Ok(BroadcastType::None);
        } else {
            let equal = a
                .iter()
                .skip(1)
                .zip(b.iter().skip(1))
                .map(|(aa, bb)| *aa == *bb)
                .fold(true, |acc, x| acc && x);
            if equal {
                return Ok(BroadcastType::First);
            }
        }
        return Err(BroadcastError::new(&format!(
            "can't broadcast a: {:?} b: {:?}",
            fmt_shape(a),
            fmt_shape(b)
        )));
    }
    if (b.len() - a.len()) == 1 {
        let equal = a
            .iter()
            .skip(1)
            .zip(b.iter())
            .map(|(aa, bb)| *aa == *bb)
            .fold(true, |acc, x| acc && x);
        if equal {
            return Ok(BroadcastType::First);
        }
    }
    Err(BroadcastError::new(&format!(
        "can't broadcast a: {:?} b: {:?}",
        fmt_shape(a),
        fmt_shape(b)
    )))
}

impl Add for &MathTensor {
    type Output = MathTensor;

    fn add(self, rhs: Self) -> Self::Output {
        let e = broadcast(self.shape_vec(), rhs.shape_vec());
        println!(
            "a + b --> broadcast:  {:?} + {:?} ->  {:?}",
            self.shape(),
            rhs.shape(),
            e
        );
        let a: Vec<f64> = match e {
            Ok(b) => {
                let res: Vec<f64> = match b {
                    BroadcastType::None => self.data().iter().zip(rhs.data().iter()).map(|(a, b)| a + b).collect(),
                    BroadcastType::First => self.data().iter().map(|a| *a + 1.0).collect(),
                };
                res
            }
            Err(e) => panic!("can't add shapes  {:?} + {:?} ", &self.shape_vec(), &rhs.shape_vec()),
        };
        assert_eq!(self.shape, rhs.shape);
        let shape = self.shape_copy();
        let t = MathTensor::new(shape, a);
        t
    }
}

impl Add<f64> for &MathTensor {
    type Output = MathTensor;

    fn add(self, rhs: f64) -> Self::Output {
        let a: Vec<f64> = self.data().iter().map(|a| a + rhs).collect();
        let shape = self.shape_copy();
        let t = MathTensor::new(shape, a);
        t
    }
}

impl Add<&MathTensor> for f64 {
    type Output = MathTensor;

    fn add(self, rhs: &MathTensor) -> Self::Output {
        rhs + self
    }
}

impl Sub for &MathTensor {
    type Output = MathTensor;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape, rhs.shape);
        let a: Vec<f64> = self.data().iter().zip(rhs.data().iter()).map(|(a, b)| a - b).collect();
        let shape = self.shape_copy();
        let t = MathTensor::new(shape, a);
        t
    }
}

impl Sub<f64> for &MathTensor {
    type Output = MathTensor;

    fn sub(self, rhs: f64) -> Self::Output {
        let a: Vec<f64> = self.data().iter().map(|a| a - rhs).collect();
        let shape = self.shape_copy();
        let t = MathTensor::new(shape, a);
        t
    }
}

impl Sub<&MathTensor> for f64 {
    type Output = MathTensor;

    fn sub(self, rhs: &MathTensor) -> Self::Output {
        &(-rhs) + self
    }
}

impl Neg for MathTensor {
    type Output = MathTensor;

    fn neg(self) -> Self::Output {
        let a: Vec<f64> = self.data().iter().map(|a| -a).collect();
        let shape = self.shape_copy();
        let t = MathTensor::new(shape, a);
        t
    }
}

impl Neg for &MathTensor {
    type Output = MathTensor;

    fn neg(self) -> Self::Output {
        let a: Vec<f64> = self.data().iter().map(|a| -a).collect();
        let shape = self.shape_copy();
        let t = MathTensor::new(shape, a);
        t
    }
}

impl Mul for &MathTensor {
    type Output = MathTensor;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape, rhs.shape);
        let a: Vec<f64> = self.data().iter().zip(rhs.data().iter()).map(|(a, b)| a * b).collect();
        let shape = self.shape_copy();
        let t = MathTensor::new(shape, a);
        t
    }
}

impl Mul<f64> for &MathTensor {
    type Output = MathTensor;

    fn mul(self, rhs: f64) -> Self::Output {
        let a: Vec<f64> = self.data().iter().map(|a| a * rhs).collect();
        let shape = self.shape_copy();
        let t = MathTensor::new(shape, a);
        t
    }
}

impl Mul<&MathTensor> for f64 {
    type Output = MathTensor;

    fn mul(self, rhs: &MathTensor) -> Self::Output {
        rhs * self
    }
}

impl BitXor for &MathTensor {
    type Output = MathTensor;

    fn bitxor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape_vec()[1], rhs.shape_vec()[0]);

        let res_shape = vec![self.shape_vec()[0], rhs.shape_vec()[1]];
        let mut res = MathTensor::zeroes(res_shape);

        for i in 0..self.shape_vec()[0] {
            for j in 0..rhs.shape_vec()[1] {
                let mut sum = 0_f64;
                for k in 0..self.shape_vec()[1] {
                    sum += self.elem(vec![i, k]) * rhs.elem(vec![k, j]);
                }
                res.set_elem(vec![i, j], sum);
            }
        }
        res
    }
}

impl Default for MathTensor {
    fn default() -> Self {
        MathTensor {
            data: vec![],
            shape: vec![],
        }
    }
}

impl Display for MathTensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.shape())
    }
}

#[cfg(test)]
mod tests {
    use crate::micrograd_rs_v4_mathtensor::MathTensor;
    use crate::{assert_float, assert_vec_f64};

    #[test]
    pub fn test_math_tensor_new() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let a = MathTensor::new(vec![2, 3], a);

        let b = vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        let b = MathTensor::new(vec![3, 2], b);

        // trivial assertions
        assert_eq!(a.data[0], 1.0);
        assert_eq!(a.data[1], 2.0);
        assert_eq!(a.data[2], 3.0);
        assert_eq!(a.data[3], 4.0);
        assert_eq!(a.data[4], 5.0);
        assert_eq!(a.data[5], 6.0);

        // trivial assertions
        assert_eq!(b.data[0], 11.0);
        assert_eq!(b.data[1], 12.0);
        assert_eq!(b.data[2], 13.0);
        assert_eq!(b.data[3], 14.0);
        assert_eq!(b.data[4], 15.0);
        assert_eq!(b.data[5], 16.0);

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let b_shape_expected = "(3, 2)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("b.shape expected {},   actual {}", b_shape_expected, b.shape());

        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(b.shape(), b_shape_expected);
    }

    #[test]
    pub fn test_math_tensor() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0];

        let c_expected: Vec<f64> = a.iter().zip(b.iter()).map(|(aa, bb)| aa + bb).collect();
        let a = MathTensor::new(vec![2, 3], a);
        let b = MathTensor::new(vec![2, 3], b);

        let c = &a + &b;

        assert_vec_f64(&c_expected, &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let b_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("b.shape expected {},   actual {}", b_shape_expected, b.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());

        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(b.shape(), b_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_math_tensor_add_scalar_elementwise_1() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = 2.0;

        let c_expected: Vec<f64> = a.iter().map(|aa| aa + b).collect();
        let a = MathTensor::new(vec![2, 3], a);

        let c = &a + b;

        assert_vec_f64(&c_expected, &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());
        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_math_tensor_add_scalar_elementwise_2() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = 2.0;

        let c_expected: Vec<f64> = a.iter().map(|aa| aa + b).collect();
        let a = MathTensor::new(vec![2, 3], a);

        let c = b + &a;

        assert_vec_f64(&c_expected, &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());
        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_math_tensor_sub() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0];

        let c_expected: Vec<f64> = a.iter().zip(b.iter()).map(|(aa, bb)| aa - bb).collect();
        let a = MathTensor::new(vec![2, 3], a);
        let b = MathTensor::new(vec![2, 3], b);

        let c = &a - &b;

        assert_vec_f64(&c_expected, &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let b_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("b.shape expected {},   actual {}", b_shape_expected, b.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());

        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(b.shape(), b_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_math_tensor_sub_scalar_elementwise_1() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = 2.0;

        let c_expected: Vec<f64> = a.iter().map(|aa| aa - b).collect();
        let a = MathTensor::new(vec![2, 3], a);

        let c = &a - b;

        assert_vec_f64(&c_expected, &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());
        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_math_tensor_sub_scalar_elementwise_2() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = 2.0;

        let c_expected: Vec<f64> = a.iter().map(|aa| b - aa).collect();
        let a = MathTensor::new(vec![2, 3], a);

        let c = b - &a;

        assert_vec_f64(&c_expected, &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());
        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_math_tensor_neg() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let c_expected: Vec<f64> = a.iter().map(|aa| -aa).collect();
        let a = MathTensor::new(vec![2, 3], a);

        let c = -&a;

        assert_vec_f64(&c_expected, &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());
        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_math_tensor_mul_elementwise() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0];

        let c_expected: Vec<f64> = a.iter().zip(b.iter()).map(|(aa, bb)| aa * bb).collect();
        let a = MathTensor::new(vec![2, 3], a);
        let b = MathTensor::new(vec![2, 3], b);

        let c = &a * &b;

        assert_vec_f64(&c_expected, &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let b_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("b.shape expected {},   actual {}", b_shape_expected, b.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());

        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(b.shape(), b_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_math_tensor_mul_scalar_elementwise_1() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = 2.0;

        let c_expected: Vec<f64> = a.iter().map(|aa| aa * b).collect();
        let a = MathTensor::new(vec![2, 3], a);

        let c = &a * b;

        assert_vec_f64(&c_expected, &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());
        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_math_tensor_mul_scalar_elementwise_2() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = 2.0;

        let c_expected: Vec<f64> = a.iter().map(|aa| b * aa).collect();
        let a = MathTensor::new(vec![2, 3], a);

        let c = b * &a;

        assert_vec_f64(&c_expected, &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());
        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    // Matrix Product
    // for simple (mxn) ^ ( nxo) cases
    #[test]
    pub fn test_math_tensor_matrix_product() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0];

        let a = MathTensor::new(vec![2, 3], a);
        let b = MathTensor::new(vec![3, 2], b);

        let c_shape = vec![a.shape_vec()[0], b.shape_vec()[1]];
        let c_expected = vec![82., 88., 199., 214.];
        let c_expected = MathTensor::new(c_shape, c_expected);

        let c = &a ^ &b;

        assert_vec_f64(&c_expected.data(), &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let b_shape_expected = "(3, 2)".to_string();
        let c_shape_expected = "(2, 2)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("b.shape expected {},   actual {}", b_shape_expected, b.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());

        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(b.shape(), b_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_math_tensor_relu() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let a = MathTensor::new(vec![2, 3], a);

        let c_shape = vec![a.shape_vec()[0], a.shape_vec()[1]];
        let mut c_expected = MathTensor::zeroes(c_shape);

        for i in 0..a.shape_vec()[0] {
            for j in 0..a.shape_vec()[1] {
                let relu = a.elem(vec![i, j]).max(0.0);
                c_expected.set_elem(vec![i, j], relu);
            }
        }

        let c = a.relu();

        assert_vec_f64(&c_expected.data(), &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());
        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_math_tensor_tanh() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let a = MathTensor::new(vec![2, 3], a);

        let c_shape = vec![a.shape_vec()[0], a.shape_vec()[1]];
        let mut c_expected = MathTensor::zeroes(c_shape);

        for i in 0..a.shape_vec()[0] {
            for j in 0..a.shape_vec()[1] {
                let relu = a.elem(vec![i, j]).tanh();
                c_expected.set_elem(vec![i, j], relu);
            }
        }

        let c = a.tanh();

        assert_vec_f64(&c_expected.data(), &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());
        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_math_tensor_exp() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let a = MathTensor::new(vec![2, 3], a);

        let c_shape = vec![a.shape_vec()[0], a.shape_vec()[1]];
        let mut c_expected = MathTensor::zeroes(c_shape);

        for i in 0..a.shape_vec()[0] {
            for j in 0..a.shape_vec()[1] {
                let relu = a.elem(vec![i, j]).exp();
                c_expected.set_elem(vec![i, j], relu);
            }
        }

        let c = a.exp();

        assert_vec_f64(&c_expected.data(), &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());
        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_math_tensor_pow() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let a = MathTensor::new(vec![2, 3], a);
        let n = 2.3_f64;
        let c_shape = vec![a.shape_vec()[0], a.shape_vec()[1]];
        let mut c_expected = MathTensor::zeroes(c_shape);

        for i in 0..a.shape_vec()[0] {
            for j in 0..a.shape_vec()[1] {
                let relu = a.elem(vec![i, j]).powf(n);
                c_expected.set_elem(vec![i, j], relu);
            }
        }

        let c = a.pow(n);

        assert_vec_f64(&c_expected.data(), &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());
        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    #[test]
    pub fn test_transpose() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let a = MathTensor::new(vec![2, 3], a);

        let b = a.transpose();

        assert_float(a.elem(vec![0, 0]), 1.0);
        assert_float(a.elem(vec![0, 1]), 2.0);
        assert_float(a.elem(vec![0, 2]), 3.0);
        assert_float(a.elem(vec![1, 0]), 4.0);
        assert_float(a.elem(vec![1, 1]), 5.0);
        assert_float(a.elem(vec![1, 2]), 6.0);

        assert_float(b.elem(vec![0, 0]), 1.0);
        assert_float(b.elem(vec![0, 1]), 2.0);
        assert_float(b.elem(vec![1, 0]), 3.0);
        assert_float(b.elem(vec![1, 1]), 4.0);
        assert_float(b.elem(vec![2, 0]), 5.0);
        assert_float(b.elem(vec![2, 1]), 6.0);

        let shape_expected = "(3, 2)";

        assert_eq!(shape_expected, b.shape());
    }

    // add (2,2) + (1,2)
    // https://stackoverflow.com/questions/66089318/back-propagation-of-y-x-sumx-dim-0-where-size-of-tensor-x-is-h-w
    #[test]
    pub fn test_math_tensor_add_broadcast() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![11.0, 12.0];

        let c_expected = vec![12.0, 14.0, 14.0, 16.0];

        let a = MathTensor::new(vec![2, 2], a);
        let b = MathTensor::new(vec![1, 2], b);

        let c = &a + &b;

        assert_vec_f64(&c_expected, &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 2)".to_string();
        let b_shape_expected = "(1, 2)".to_string();
        let c_shape_expected = "(2, 2)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("b.shape expected {},   actual {}", b_shape_expected, b.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());

        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(b.shape(), b_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    // sum([1,2,3,4])
    #[test]
    pub fn test_math_tensor_sum_1dim() {
        let a = vec![1.0, 2.0, 3.0, 4.0];

        let c_expected = vec![10.0];

        let a = MathTensor::new(vec![1, 4], a);

        let c = a.sum();

        assert_vec_f64(&c_expected, &c.data());

        // not so trivial assertions
        let a_shape_expected = "(1, 4)".to_string();
        let c_shape_expected = "(1, 1)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());

        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }

    // sum([[1,2],[3,4]])
    #[test]
    pub fn test_math_tensor_sum_2dim() {
        let a = vec![1.0, 2.0, 3.0, 4.0];

        let c_expected = vec![10.0];

        let a = MathTensor::new(vec![2, 2], a);

        let c = a.sum();

        assert_vec_f64(&c_expected, &c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 2)".to_string();
        let c_shape_expected = "(1, 1)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());

        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }
}
