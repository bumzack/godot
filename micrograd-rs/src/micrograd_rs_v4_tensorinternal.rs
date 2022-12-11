use std::fmt::{Display, Formatter};
use std::ops::{Add, BitXor, Mul, Neg, Sub};

use crate::micrograd_rs_v4_backward::Backward;
use crate::micrograd_rs_v4_mathtensor::MathTensor;
use crate::micrograd_rs_v4_tensor::{OpEnumV4, Tensor};

// TODO
// pub backward is ugly, but Option and borrowing is a pita
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
        let shape = t.shape_copy();
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
        let shape = t.shape_copy();
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

    pub fn grad_mut(&mut self) -> &mut MathTensor {
        &mut self.grad
    }

    pub fn set_grad(&mut self, grad: MathTensor) {
        self.grad = grad;
    }

    pub fn shape_vec(&self) -> &Vec<usize> {
        self.t.shape_vec()
    }

    pub fn shape_copy(&self) -> Vec<usize> {
        self.t.shape_copy()
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
        let exp = self.t().exp();
        TensorInternal::from(exp, OpEnumV4::EXP, "exp".to_string())
    }

    pub fn tanh(&self) -> TensorInternal {
        let tanh = self.t().tanh();
        TensorInternal::from(tanh, OpEnumV4::TANH, "tanh".to_string())
    }

    pub fn relu(&self) -> TensorInternal {
        let relu = self.t().relu();
        TensorInternal::from(relu, OpEnumV4::RELU, "relu".to_string())
    }

    pub fn pow(&self, n: f64) -> TensorInternal {
        let pow = self.t().pow(n);
        TensorInternal::from(pow, OpEnumV4::POW, "pow".to_string())
    }
}

impl Add for &TensorInternal {
    type Output = TensorInternal;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.t() + rhs.t();
        TensorInternal::from(sum, OpEnumV4::ADD, "add".to_string())
    }
}

impl Add<f64> for &TensorInternal {
    type Output = TensorInternal;

    fn add(self, rhs: f64) -> Self::Output {
        let sum = self.t() + rhs;
        TensorInternal::from(sum, OpEnumV4::SUB, "add".to_string())
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
        let sum = self.t() - rhs.t();
        TensorInternal::from(sum, OpEnumV4::SUB, "sub".to_string())
    }
}

impl Sub<f64> for &TensorInternal {
    type Output = TensorInternal;

    fn sub(self, rhs: f64) -> Self::Output {
        let sum = self.t() - rhs;
        TensorInternal::from(sum, OpEnumV4::SUB, "sub".to_string())
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
        let neg = -self.t();
        TensorInternal::from(neg, OpEnumV4::NEG, "sum".to_string())
    }
}

impl Mul for &TensorInternal {
    type Output = TensorInternal;

    fn mul(self, rhs: Self) -> Self::Output {
        let prod = self.t() * rhs.t();
        TensorInternal::from(prod, OpEnumV4::MUL, "mul".to_string())
    }
}

impl Mul<f64> for &TensorInternal {
    type Output = TensorInternal;

    fn mul(self, rhs: f64) -> Self::Output {
        let prod = self.t() * rhs;
        TensorInternal::from(prod, OpEnumV4::MUL, "mul".to_string())
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
        let prod = self.t() ^ rhs.t();
        TensorInternal::from(prod, OpEnumV4::DOT, "dot".to_string())
    }
}

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

    #[test]
    pub fn test_tensor_internal_add() {
        let a = MathTensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = MathTensor::new(vec![2, 3], vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);

        let c_expected: Vec<f64> = a.data().iter().zip(b.data().iter()).map(|(aa, bb)| aa + bb).collect();
        let a = TensorInternal::from(a, OpEnumV4::NONE, "a".to_string());
        let b = TensorInternal::from(b, OpEnumV4::NONE, "b".to_string());

        let c = &a + &b;

        assert_vec_f64(&c_expected, c.t().data());

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
