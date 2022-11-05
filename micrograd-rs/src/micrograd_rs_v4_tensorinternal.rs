use std::fmt::{Display, Formatter};
use std::ops::{Add, BitXor, Mul, Neg, Sub};

use crate::micrograd_rs_v4_backward::Backward;
use crate::micrograd_rs_v4_mathtensor::MathTensor;
use crate::micrograd_rs_v4_tensor::{OpEnumV4, Tensor};

pub struct TensorInternal {
    t: MathTensor,
    grad: MathTensor,
    op: OpEnumV4,
    children: Vec<Tensor>,
    label: String,
    pub backward: Option<Box<dyn Backward>>,
}

impl TensorInternal {
    pub fn new(
        t: MathTensor,
        op: OpEnumV4,
        children: Vec<Tensor>,
        label: String,
        backward: Option<Box<dyn Backward>>,
    ) -> Self {
        let mut shape = vec![];
        t.shape_vec().iter().for_each(|s| shape.push(*s));
        TensorInternal {
            t,
            op,
            children,
            label,
            grad: MathTensor::zeroes(shape),
            backward,
        }
    }

    pub fn from(t: MathTensor, op: OpEnumV4, label: String) -> Self {
        let mut shape = vec![];
        t.shape_vec().iter().for_each(|s| shape.push(*s));
        TensorInternal {
            t,
            grad: MathTensor::zeroes(shape),
            op,
            children: vec![],
            label,
            backward: None,
        }
    }

    pub fn zeroes(shape: Vec<usize>, op: OpEnumV4, label: String) -> Self {
        let t = MathTensor::zeroes(shape.clone());
        TensorInternal {
            t,
            op,
            children: vec![],
            label,
            grad: MathTensor::zeroes(shape),
            backward: None,
        }
    }

    pub fn ones(shape: Vec<usize>, op: OpEnumV4, label: String) -> Self {
        let t = MathTensor::ones(shape.clone());
        TensorInternal {
            t,
            op,
            children: vec![],
            label,
            grad: MathTensor::zeroes(shape),
            backward: None,
        }
    }

    pub fn t(&self) -> &MathTensor {
        &self.t
    }

    pub fn set_t(&mut self, t: MathTensor) {
        self.t = t;
    }

    pub fn grad(&self) -> &MathTensor {
        &self.grad
    }

    pub fn set_grad(&mut self, grad: MathTensor) {
        self.grad = grad;
    }

    pub fn shape_vec(&self) -> &Vec<usize> {
        &self.t.shape_vec()
    }

    pub fn op(&self) -> &OpEnumV4 {
        &self.op
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn children(&self) -> &Vec<Tensor> {
        &self.children
    }

    pub fn set_elem(&mut self, pos: Vec<usize>, data: f64) {
        self.t.set_elem(pos, data);
    }

    pub fn elem(&self, pos: Vec<usize>) -> f64 {
        self.t.elem(pos)
    }

    pub fn exp(&self) -> TensorInternal {
        let mut shape = vec![];
        self.shape_vec().iter().for_each(|s| shape.push(*s));
        let exp = self.t().exp();
        let t = TensorInternal::from(exp, OpEnumV4::EXP, "exp".to_string());
        t
    }

    pub fn tanh(&self) -> TensorInternal {
        let mut shape = vec![];
        self.shape_vec().iter().for_each(|s| shape.push(*s));
        let tanh = self.t().tanh();
        let t = TensorInternal::from(tanh, OpEnumV4::TANH, "tanh".to_string());
        t
    }

    pub fn relu(&self) -> TensorInternal {
        let mut shape = vec![];
        self.shape_vec().iter().for_each(|s| shape.push(*s));
        let relu = self.t().relu();
        let t = TensorInternal::from(relu, OpEnumV4::RELU, "relu".to_string());
        t
    }

    pub fn pow(&self, n: f64) -> TensorInternal {
        let mut shape = vec![];
        self.shape_vec().iter().for_each(|s| shape.push(*s));
        let pow = self.t().pow(n);
        let t = TensorInternal::from(pow, OpEnumV4::POW, "pow".to_string());
        t
    }
}

impl Add for &TensorInternal {
    type Output = TensorInternal;

    fn add(self, rhs: Self) -> Self::Output {
        let mut shape = vec![];
        self.shape_vec().iter().for_each(|s| shape.push(*s));
        let sum = self.t() + rhs.t();
        let t = TensorInternal::from(sum, OpEnumV4::ADD, "add".to_string());
        t
    }
}

impl Add<f64> for &TensorInternal {
    type Output = TensorInternal;

    fn add(self, rhs: f64) -> Self::Output {
        let mut shape = vec![];
        self.shape_vec().iter().for_each(|s| shape.push(*s));
        let sum = self.t() + rhs;
        let t = TensorInternal::from(sum, OpEnumV4::SUB, "add".to_string());
        t
    }
}

impl Add<&TensorInternal> for f64 {
    type Output = TensorInternal;

    fn add(self, rhs: &TensorInternal) -> Self::Output {
        rhs + self
    }
}

impl Sub for &TensorInternal {
    type Output = TensorInternal;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut shape = vec![];
        self.shape_vec().iter().for_each(|s| shape.push(*s));
        let sum = self.t() - rhs.t();
        let t = TensorInternal::from(sum, OpEnumV4::SUB, "sub".to_string());
        t
    }
}

impl Sub<f64> for &TensorInternal {
    type Output = TensorInternal;

    fn sub(self, rhs: f64) -> Self::Output {
        let mut shape = vec![];
        self.shape_vec().iter().for_each(|s| shape.push(*s));
        let sum = self.t() - rhs;
        let t = TensorInternal::from(sum, OpEnumV4::SUB, "sub".to_string());
        t
    }
}

impl Sub<&TensorInternal> for f64 {
    type Output = TensorInternal;

    fn sub(self, rhs: &TensorInternal) -> Self::Output {
        &(-rhs) + self
    }
}

impl Neg for &TensorInternal {
    type Output = TensorInternal;

    fn neg(self) -> Self::Output {
        let mut shape = vec![];
        self.shape_vec().iter().for_each(|s| shape.push(*s));
        let neg = -self.t();
        let t = TensorInternal::from(neg, OpEnumV4::NEG, "sum".to_string());
        t
    }
}

impl Mul for &TensorInternal {
    type Output = TensorInternal;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut shape = vec![];
        self.shape_vec().iter().for_each(|s| shape.push(*s));
        let prod = self.t() * rhs.t();
        let t = TensorInternal::from(prod, OpEnumV4::MUL, "mul".to_string());
        t
    }
}

impl Mul<f64> for &TensorInternal {
    type Output = TensorInternal;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut shape = vec![];
        self.shape_vec().iter().for_each(|s| shape.push(*s));
        let prod = self.t() * rhs;
        let t = TensorInternal::from(prod, OpEnumV4::MUL, "mul".to_string());
        t
    }
}

impl Mul<&TensorInternal> for f64 {
    type Output = TensorInternal;

    fn mul(self, rhs: &TensorInternal) -> Self::Output {
        rhs * self
    }
}

impl BitXor for &TensorInternal {
    type Output = TensorInternal;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut shape = vec![];
        self.shape_vec().iter().for_each(|s| shape.push(*s));
        let prod = self.t() ^ rhs.t();
        let t = TensorInternal::from(prod, OpEnumV4::DOT, "dot".to_string());
        t
    }
}

//
//
// impl Mul for &TensorInternal {
//     type Output = TensorInternal;
//
//     fn mul(self, rhs: Self) -> Self::Output {
//         assert_eq!(self.shape, rhs.shape);
//         let a: Vec<f64> = self.data().iter().zip(rhs.data().iter()).map(|(a, b)| a * b).collect();
//         let mut shape = vec![];
//         self.shape_vec().iter().for_each(|s| shape.push(*s));
//         let t = TensorInternal::new(shape, a, OpEnumV4::MUL, "mul".to_string());
//         t
//     }
// }
//
// impl Mul<f64> for &TensorInternal {
//     type Output = TensorInternal;
//
//     fn mul(self, rhs: f64) -> Self::Output {
//         let a: Vec<f64> = self.data().iter().map(|a| a * rhs).collect();
//         let mut shape = vec![];
//         self.shape_vec().iter().for_each(|s| shape.push(*s));
//         let t = TensorInternal::new(shape, a, OpEnumV4::MUL, "mul".to_string());
//         t
//     }
// }
//
// impl Mul<&TensorInternal> for f64 {
//     type Output = TensorInternal;
//
//     fn mul(self, rhs: &TensorInternal) -> Self::Output {
//         rhs * self
//     }
// }
//
// impl BitXor for &TensorInternal {
//     type Output = TensorInternal;
//
//     fn bitxor(self, rhs: Self) -> Self::Output {
//         assert_eq!(self.shape_vec()[1], rhs.shape_vec()[0]);
//
//         let res_shape = vec![self.shape_vec()[0], rhs.shape_vec()[1]];
//         let mut res = TensorInternal::zeroes(res_shape, OpEnumV4::DOT, "c".to_string());
//
//         for i in 0..self.shape_vec()[0] {
//             for j in 0..rhs.shape_vec()[1] {
//                 let mut sum = 0_f64;
//                 for k in 0..self.shape_vec()[1] {
//                     sum += self.elem(vec![i, k]) * rhs.elem(vec![k, j]);
//                 }
//                 res.set_elem(vec![i, j], sum);
//             }
//         }
//         res
//     }
// }
//
//
// impl Default for TensorInternal {
//     fn default() -> Self {
//         TensorInternal {
//             data: vec![],
//             op: OpEnumV4::NONE,
//             children: vec![],
//             grad: vec![],
//             label: "default".to_string(),
//             backward: None,
//             shape: vec![],
//         }
//     }
// }

impl Display for TensorInternal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.label, self.t.shape())
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_vec_f64;
    use crate::micrograd_rs_v4_mathtensor::MathTensor;
    use crate::micrograd_rs_v4_tensor::OpEnumV4;
    use crate::micrograd_rs_v4_tensorinternal::TensorInternal;

    // use crate::{assert_float, assert_two_float};
    // use crate::micrograd_rs_v4_tensor::OpEnumV4;
    // use crate::micrograd_rs_v4_tensorinternal::TensorInternal;
    //
    // #[test]
    // pub fn test_tensor_internal_new() {
    //     let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    //     let a = TensorInternal::new(vec![2, 3], a, OpEnumV4::NONE, "a".to_string());
    //
    //     let b = vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
    //     let b = TensorInternal::new(vec![3, 2], b, OpEnumV4::NONE, "b".to_string());
    //
    //     // trivial assertions
    //     assert_eq!(a.data[0], 1.0);
    //     assert_eq!(a.data[1], 2.0);
    //     assert_eq!(a.data[2], 3.0);
    //     assert_eq!(a.data[3], 4.0);
    //     assert_eq!(a.data[4], 5.0);
    //     assert_eq!(a.data[5], 6.0);
    //
    //     // trivial assertions
    //     assert_eq!(b.data[0], 11.0);
    //     assert_eq!(b.data[1], 12.0);
    //     assert_eq!(b.data[2], 13.0);
    //     assert_eq!(b.data[3], 14.0);
    //     assert_eq!(b.data[4], 15.0);
    //     assert_eq!(b.data[5], 16.0);
    //
    //     // not so trivial assertions
    //     let a_shape_expected = "(2, 3)".to_string();
    //     let b_shape_expected = "(3, 2)".to_string();
    //
    //     println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
    //     println!("b.shape expected {},   actual {}", b_shape_expected, b.shape());
    //
    //     assert_eq!(a.shape(), a_shape_expected);
    //     assert_eq!(b.shape(), b_shape_expected);
    // }
    //
    // fn assert_vec_f64(expected: &Vec<f64>, actual: &Vec<f64>) {
    //     assert_eq!(expected.len(), actual.len());
    //     expected.iter().zip(actual.iter()).for_each(|(a, b)| {
    //         if !assert_two_float(*a, *b) {
    //             println!("expected {}  !=  actual {}", a, b);
    //         }
    //         assert_float(*a, *b);
    //     });
    // }
    //
    #[test]
    pub fn test_tensor_internal_add() {
        let a = MathTensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = MathTensor::new(vec![2, 3], vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);

        let c_expected: Vec<f64> = a.data().iter().zip(b.data().iter()).map(|(aa, bb)| aa + bb).collect();
        let a = TensorInternal::from(a, OpEnumV4::NONE, "a".to_string());
        let b = TensorInternal::from(b, OpEnumV4::NONE, "b".to_string());

        let c = &a + &b;

        assert_vec_f64(&c_expected, &c.t().data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let b_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.t().shape());
        println!("b.shape expected {},   actual {}", b_shape_expected, b.t().shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.t().shape());

        assert_eq!(a.t().shape(), a_shape_expected);
        assert_eq!(b.t().shape(), b_shape_expected);
        assert_eq!(c.t().shape(), c_shape_expected);
    }

    #[test]
    pub fn test_tensor_internal_add_scalar_elementwise_1() {
        let a = MathTensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = 3.0_f64;

        let c_expected: Vec<f64> = a.data().iter().map(|aa| aa + b).collect();
        let a = TensorInternal::from(a, OpEnumV4::NONE, "a".to_string());

        let c = &a + b;

        assert_vec_f64(&c_expected, &c.t().data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.t().shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.t().shape());

        assert_eq!(a.t().shape(), a_shape_expected);
        assert_eq!(c.t().shape(), c_shape_expected);
    }

    #[test]
    pub fn test_tensor_internal_add_scalar_elementwise_2() {
        let a = MathTensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = 3.0_f64;

        let c_expected: Vec<f64> = a.data().iter().map(|aa| b + aa).collect();
        let a = TensorInternal::from(a, OpEnumV4::NONE, "a".to_string());

        let c = b + &a;

        assert_vec_f64(&c_expected, &c.t().data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.t().shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.t().shape());

        assert_eq!(a.t().shape(), a_shape_expected);
        assert_eq!(c.t().shape(), c_shape_expected);
    }

    #[test]
    pub fn test_tensor_internal_sub() {
        let a = MathTensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = MathTensor::new(vec![2, 3], vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);

        let c_expected: Vec<f64> = a.data().iter().zip(b.data().iter()).map(|(aa, bb)| aa - bb).collect();
        let a = TensorInternal::from(a, OpEnumV4::NONE, "a".to_string());
        let b = TensorInternal::from(b, OpEnumV4::NONE, "b".to_string());

        let c = &a - &b;

        assert_vec_f64(&c_expected, &c.t().data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let b_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.t().shape());
        println!("b.shape expected {},   actual {}", b_shape_expected, b.t().shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.t().shape());

        assert_eq!(a.t().shape(), a_shape_expected);
        assert_eq!(b.t().shape(), b_shape_expected);
        assert_eq!(c.t().shape(), c_shape_expected);
    }

    #[test]
    pub fn test_tensor_internal_sub_scalar_elementwise_1() {
        let a = MathTensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = 3.0_f64;

        let c_expected: Vec<f64> = a.data().iter().map(|aa| aa - b).collect();
        let a = TensorInternal::from(a, OpEnumV4::NONE, "a".to_string());

        let c = &a - b;

        assert_vec_f64(&c_expected, &c.t().data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.t().shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.t().shape());

        assert_eq!(a.t().shape(), a_shape_expected);
        assert_eq!(c.t().shape(), c_shape_expected);
    }

    #[test]
    pub fn test_tensor_internal_sub_scalar_elementwise_2() {
        let a = MathTensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = 3.0_f64;

        let c_expected: Vec<f64> = a.data().iter().map(|aa| b - aa).collect();
        let a = TensorInternal::from(a, OpEnumV4::NONE, "a".to_string());

        let c = b - &a;

        assert_vec_f64(&c_expected, &c.t().data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.t().shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.t().shape());

        assert_eq!(a.t().shape(), a_shape_expected);
        assert_eq!(c.t().shape(), c_shape_expected);
    }

    #[test]
    pub fn test_tensor_internal_neg() {
        let a = MathTensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let c_expected: Vec<f64> = a.data().iter().map(|aa| -aa).collect();
        let a = TensorInternal::from(a, OpEnumV4::NONE, "a".to_string());

        let c = -(&a);

        assert_vec_f64(&c_expected, &c.t().data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.t().shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.t().shape());

        assert_eq!(a.t().shape(), a_shape_expected);
        assert_eq!(c.t().shape(), c_shape_expected);
    }

    #[test]
    pub fn test_tensor_internal_mul_elementwise() {
        let a = MathTensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = MathTensor::new(vec![2, 3], vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);

        let c_expected: Vec<f64> = a.data().iter().zip(b.data().iter()).map(|(aa, bb)| aa * bb).collect();
        let a = TensorInternal::from(a, OpEnumV4::NONE, "a".to_string());
        let b = TensorInternal::from(b, OpEnumV4::NONE, "b".to_string());

        let c = &a * &b;

        assert_vec_f64(&c_expected, &c.t().data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let b_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.t().shape());
        println!("b.shape expected {},   actual {}", b_shape_expected, b.t().shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.t().shape());

        assert_eq!(a.t().shape(), a_shape_expected);
        assert_eq!(b.t().shape(), b_shape_expected);
        assert_eq!(c.t().shape(), c_shape_expected);
    }

    #[test]
    pub fn test_tensor_internal_mul_elementwise_scalar1() {
        let a = MathTensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = 3.5_f64;

        let c_expected: Vec<f64> = a.data().iter().map(|aa| aa * b).collect();
        let a = TensorInternal::from(a, OpEnumV4::NONE, "a".to_string());

        let c = &a * b;

        assert_vec_f64(&c_expected, &c.t().data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.t().shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.t().shape());

        assert_eq!(a.t().shape(), a_shape_expected);
        assert_eq!(c.t().shape(), c_shape_expected);
    }

    #[test]
    pub fn test_tensor_internal_mul_elementwise_scalar2() {
        let a = MathTensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = 5.5_f64;

        let c_expected: Vec<f64> = a.data().iter().map(|aa| b * aa).collect();
        let a = TensorInternal::from(a, OpEnumV4::NONE, "a".to_string());

        let c = b * &a;

        assert_vec_f64(&c_expected, &c.t().data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let c_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.t().shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.t().shape());

        assert_eq!(a.t().shape(), a_shape_expected);
        assert_eq!(c.t().shape(), c_shape_expected);
    }

    // Matrix Product
    // for simple (mxn) ^ ( nxo) cases
    #[test]
    pub fn test_tensor_internal_matrix_product() {
        let a = MathTensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = MathTensor::new(vec![3, 2], vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);

        let a = TensorInternal::from(a, OpEnumV4::NONE, "a".to_string());
        let b = TensorInternal::from(b, OpEnumV4::NONE, "b".to_string());

        let c_shape = vec![a.shape_vec()[0], b.shape_vec()[1]];
        let mut c_expected = TensorInternal::zeroes(c_shape, OpEnumV4::NONE, "c".to_string());

        for i in 0..a.shape_vec()[0] {
            for j in 0..b.shape_vec()[1] {
                let mut sum = 0_f64;
                for k in 0..a.shape_vec()[1] {
                    sum += a.elem(vec![i, k]) * b.elem(vec![k, j]);
                }
                c_expected.set_elem(vec![i, j], sum);
            }
        }

        let c = &a ^ &b;

        assert_vec_f64(&c_expected.t().data(), &c.t().data());

        // not so trivial assertions
        let a_shape_expected = "(2, 3)".to_string();
        let b_shape_expected = "(3, 2)".to_string();
        let c_shape_expected = "(2, 2)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.t().shape());
        println!("b.shape expected {},   actual {}", b_shape_expected, b.t().shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.t().shape());

        assert_eq!(a.t().shape(), a_shape_expected);
        assert_eq!(b.t().shape(), b_shape_expected);
        assert_eq!(c.t().shape(), c_shape_expected);
    }
}
