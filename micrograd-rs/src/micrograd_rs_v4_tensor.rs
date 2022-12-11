use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::rc::Rc;

use crate::micrograd_rs_v4_backward::{
    BackwardAdd, BackwardDot, BackwardExp, BackwardMul, BackwardPow, BackwardReLU, BackwardSub, BackwardSum,
    BackwardTanh,
};
use crate::micrograd_rs_v4_mathtensor::MathTensor;
use crate::micrograd_rs_v4_tensorinternal::TensorInternal;

#[derive(PartialEq, Eq)]
pub enum OpEnumV4 {
    NONE,
    ADD,
    SUB,
    NEG,
    MUL,
    TANH,
    EXP,
    DIV,
    POW,
    DOT,
    RELU,
    SUM,
}

impl PartialEq for Tensor {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.r, &other.r)
    }
}

impl Hash for Tensor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.r).hash(state)
    }
}

impl Eq for Tensor {}

#[derive(Clone)]
pub struct Tensor {
    r: Rc<RefCell<TensorInternal>>,
}

impl Tensor {
    pub fn new(t: TensorInternal) -> Tensor {
        Tensor {
            r: Rc::new(RefCell::new(t)),
        }
    }

    pub fn zeroes(shape: Vec<usize>, label: String) -> Self {
        let tensor_internal = TensorInternal::zeroes(shape, OpEnumV4::NONE, label);
        Tensor {
            r: Rc::new(RefCell::new(tensor_internal)),
        }
    }

    pub fn ones(shape: Vec<usize>, label: String) -> Self {
        let tensor_internal = TensorInternal::ones(shape, OpEnumV4::NONE, label);
        Tensor {
            r: Rc::new(RefCell::new(tensor_internal)),
        }
    }

    pub fn from(t: MathTensor, label: String) -> Tensor {
        let tensor_internal = TensorInternal::from(t, OpEnumV4::NONE, label);
        Tensor {
            r: Rc::new(RefCell::new(tensor_internal)),
        }
    }

    pub fn r(&self) -> &Rc<RefCell<TensorInternal>> {
        &self.r
    }

    pub fn elem(&self, pos: Vec<usize>) -> f64 {
        self.r.borrow().t().elem(pos)
    }

    pub fn shape(&self) -> String {
        self.r.borrow().t().shape()
    }

    pub fn shape_copy(&self) -> Vec<usize> {
        self.r.borrow().shape_copy()
    }

    // convenience methods to simplify code, to avoid value.r.borrow()
    pub fn borrow(&self) -> Ref<TensorInternal> {
        self.r.borrow()
    }

    pub fn set_label(&mut self, label: String) {
        self.r.borrow_mut().set_label(label);
    }

    pub fn set_grad(&mut self, grad: MathTensor) {
        self.r.borrow_mut().set_grad(grad);
    }

    pub fn set_t(&mut self, t: MathTensor) {
        self.r.borrow_mut().set_t(t);
    }

    pub fn transpose(&self) -> Tensor {
        let transpose = self.r().borrow().t().transpose();
        Tensor::from(transpose, "transpose".to_string())
    }

    pub fn tanh(&self) -> Tensor {
        let x1 = self.r.borrow();
        let x1 = x1.t();

        let mut children = vec![];
        children.push(self.clone());

        let tanh = BackwardTanh::new(self.clone());
        let t = x1.tanh();
        let out = TensorInternal::new(t, OpEnumV4::TANH, children, "tanh".to_string(), Some(Box::new(tanh)));
        Tensor::new(out)
    }

    pub fn exp(&self) -> Tensor {
        let x1 = self.r.borrow();
        let x1 = x1.t();

        let mut children = vec![];
        children.push(self.clone());

        let exp = BackwardExp::new(self.clone());
        let t = x1.exp();
        let out = TensorInternal::new(t, OpEnumV4::EXP, children, "exp".to_string(), Some(Box::new(exp)));
        Tensor::new(out)
    }

    pub fn pow(&self, f: f64) -> Tensor {
        let x1 = self.r.borrow();
        let x1 = x1.t();

        let mut children = vec![];
        children.push(self.clone());

        let pow = BackwardPow::new(self.clone(), f);
        let t = x1.pow(f);
        let out = TensorInternal::new(t, OpEnumV4::POW, children, "pow".to_string(), Some(Box::new(pow)));
        Tensor::new(out)
    }

    pub fn relu(&self) -> Tensor {
        let x1 = self.r.borrow();
        let x1 = x1.t();

        let mut children = vec![];
        children.push(self.clone());

        let relu = BackwardReLU::new(self.clone());
        let t = x1.relu();
        let out = TensorInternal::new(t, OpEnumV4::RELU, children, "relu".to_string(), Some(Box::new(relu)));
        Tensor::new(out)
    }

    pub fn sum(&self) -> Tensor {
        let x1 = self.r.borrow();
        let x1 = x1.t();

        let mut children = vec![];
        children.push(self.clone());

        let sum = BackwardSum::new(self.clone());
        let t = x1.sum();
        let out = TensorInternal::new(t, OpEnumV4::SUM, children, "sum".to_string(), Some(Box::new(sum)));
        Tensor::new(out)
    }

    pub fn backward(&mut self) {
        let shape = self.r().borrow().shape_copy();
        let grad_1 = MathTensor::ones(shape);
        self.set_grad(grad_1);
        let topo = Self::traverse(self);
        println!("topo size   {} ", topo.len());
        for n in topo.iter().rev() {
            // thats not pretty, but easier because there were some problems with an backward() method which returns an &Option<...>
            match &n.r.borrow().backward {
                Some(backward) => {
                    backward.apply(n.clone());
                }
                None => {}
            }
        }
    }

    pub fn traverse(o: &Tensor) -> Vec<Tensor> {
        let mut topo = vec![];
        let mut visited = HashSet::new();

        Self::build_topo(o, &mut topo, &mut visited);

        topo
    }

    fn build_topo(v: &Tensor, topo: &mut Vec<Tensor>, visited: &mut HashSet<Tensor>) {
        if !visited.contains(v) {
            visited.insert(v.clone());
            for child in v.borrow().children() {
                Self::build_topo(child, topo, visited);
            }
            topo.push(v.clone());
        }
    }
}

// for AddAssign
impl Add for Tensor {
    type Output = Tensor;

    fn add(self, rhs: Tensor) -> Self::Output {
        let x1 = self.r.borrow();
        let x1 = x1.t();
        let x2 = rhs.r.borrow();
        let x2 = x2.t();

        let mut children = vec![];
        children.push(self.clone());
        children.push(rhs.clone());

        let add = BackwardAdd::new(self.clone(), rhs.clone());
        let t = x1 + x2;
        let out = TensorInternal::new(t, OpEnumV4::ADD, children, "add".to_string(), Some(Box::new(add)));
        Tensor::new(out)
    }
}

impl Add for &Tensor {
    type Output = Tensor;

    fn add(self, rhs: &Tensor) -> Self::Output {
        let x1 = self.r.borrow();
        let x1 = x1.t();
        let x2 = rhs.r.borrow();
        let x2 = x2.t();

        let mut children = vec![];
        children.push(self.clone());
        children.push(rhs.clone());

        let add = BackwardAdd::new(self.clone(), rhs.clone());
        let t = x1 + x2;
        let out = TensorInternal::new(t, OpEnumV4::ADD, children, "add".to_string(), Some(Box::new(add)));
        Tensor::new(out)
    }
}

impl Add<f64> for &Tensor {
    type Output = Tensor;

    fn add(self, rhs: f64) -> Self::Output {
        let shape = self.shape_copy();

        let x1 = self.r.borrow();
        let x1 = x1.t();
        let x2 = MathTensor::value(shape, rhs);
        let sum = x1 + &x2;
        let rhs = Tensor::from(x2, "add".to_string());
        let mut children = vec![];
        children.push(self.clone());
        children.push(rhs.clone());

        let add = BackwardAdd::new(self.clone(), rhs);
        let out = TensorInternal::new(sum, OpEnumV4::ADD, children, "add".to_string(), Some(Box::new(add)));
        Tensor::new(out)
    }
}

impl Add<&Tensor> for f64 {
    type Output = Tensor;

    fn add(self, rhs: &Tensor) -> Self::Output {
        rhs + self
    }
}

// create a Tensor and then adds it
// let a += &b +1.0
impl AddAssign for Tensor {
    fn add_assign(&mut self, rhs: Tensor) {
        *self = self.clone() + rhs
    }
}

// add ref to Tensor
// let a += &b
impl AddAssign<&Tensor> for Tensor {
    fn add_assign(&mut self, rhs: &Tensor) {
        *self = self.clone() + rhs.clone()
    }
}

// for MulAssign
impl Mul for Tensor {
    type Output = Tensor;

    fn mul(self, rhs: Tensor) -> Self::Output {
        let x1 = self.r.borrow();
        let x1 = x1.t();
        let x2 = rhs.r.borrow();
        let x2 = x2.t();

        let mut children = vec![];
        children.push(self.clone());
        children.push(rhs.clone());

        let mul = BackwardMul::new(self.clone(), rhs.clone());
        let t = x1 * x2;
        let out = TensorInternal::new(t, OpEnumV4::MUL, children, "mul".to_string(), Some(Box::new(mul)));
        Tensor::new(out)
    }
}

impl Mul for &Tensor {
    type Output = Tensor;

    fn mul(self, rhs: &Tensor) -> Self::Output {
        let x1 = self.r.borrow();
        let x1 = x1.t();
        let x2 = rhs.r.borrow();
        let x2 = x2.t();

        let mut children = vec![];
        children.push(self.clone());
        children.push(rhs.clone());

        let mul = BackwardMul::new(self.clone(), rhs.clone());
        let t = x1 * x2;
        let out = TensorInternal::new(t, OpEnumV4::MUL, children, "mul".to_string(), Some(Box::new(mul)));
        Tensor::new(out)
    }
}

impl Mul<f64> for &Tensor {
    type Output = Tensor;

    fn mul(self, rhs: f64) -> Self::Output {
        let shape = self.shape_copy();
        let x1 = self.r.borrow();
        let x1 = x1.t();
        let x2 = MathTensor::value(shape, rhs);
        let prod = x1 * &x2;
        let rhs = Tensor::from(x2, "mul".to_string());
        let mut children = vec![];
        children.push(self.clone());
        children.push(rhs.clone());

        let mul = BackwardMul::new(self.clone(), rhs);
        let out = TensorInternal::new(prod, OpEnumV4::MUL, children, "mul".to_string(), Some(Box::new(mul)));
        Tensor::new(out)
    }
}

impl Mul<&Tensor> for f64 {
    type Output = Tensor;

    fn mul(self, rhs: &Tensor) -> Self::Output {
        rhs * self
    }
}

impl MulAssign for Tensor {
    fn mul_assign(&mut self, rhs: Tensor) {
        *self = self.clone() * rhs
    }
}

impl MulAssign<&Tensor> for Tensor {
    fn mul_assign(&mut self, rhs: &Tensor) {
        *self = self.clone() * rhs.clone()
    }
}

impl Neg for &Tensor {
    type Output = Tensor;

    fn neg(self) -> Self::Output {
        self * (-1.0_f64)
    }
}

// for DivAssign
impl Div for Tensor {
    type Output = Tensor;

    fn div(self, rhs: Tensor) -> Self::Output {
        self * rhs.pow(-1.0)
    }
}

impl Div for &Tensor {
    type Output = Tensor;

    fn div(self, rhs: &Tensor) -> Self::Output {
        self * &rhs.pow(-1.0)
    }
}

impl Div<f64> for &Tensor {
    type Output = Tensor;

    fn div(self, rhs: f64) -> Self::Output {
        self * rhs.powf(-1.0)
    }
}

impl Div<&Tensor> for f64 {
    type Output = Tensor;

    fn div(self, rhs: &Tensor) -> Self::Output {
        self * &rhs.pow(-1.0)
    }
}

impl DivAssign for Tensor {
    fn div_assign(&mut self, rhs: Tensor) {
        *self = self.clone() * rhs.pow(-1.0)
    }
}

impl DivAssign<&Tensor> for Tensor {
    fn div_assign(&mut self, rhs: &Tensor) {
        *self = self.clone() * rhs.pow(-1.0)
    }
}

// for SubAssign
impl Sub for Tensor {
    type Output = Tensor;

    fn sub(self, rhs: Tensor) -> Self::Output {
        let x1 = self.r.borrow();
        let x1 = x1.t();
        let x2 = rhs.r.borrow();
        let x2 = x2.t();

        let mut children = vec![];
        children.push(self.clone());
        children.push(rhs.clone());

        let sub = BackwardSub::new(self.clone(), rhs.clone());
        let t = x1 - x2;
        let out = TensorInternal::new(t, OpEnumV4::SUB, children, "sub".to_string(), Some(Box::new(sub)));
        Tensor::new(out)
    }
}

impl Sub for &Tensor {
    type Output = Tensor;

    fn sub(self, rhs: &Tensor) -> Self::Output {
        let x1 = self.r.borrow();
        let x1 = x1.t();
        let x2 = rhs.r.borrow();
        let x2 = x2.t();

        let mut children = vec![];
        children.push(self.clone());
        children.push(rhs.clone());

        let sub = BackwardSub::new(self.clone(), rhs.clone());
        let t = x1 - x2;
        let out = TensorInternal::new(t, OpEnumV4::SUB, children, "sub".to_string(), Some(Box::new(sub)));
        Tensor::new(out)
    }
}

impl Sub<f64> for &Tensor {
    type Output = Tensor;

    fn sub(self, rhs: f64) -> Self::Output {
        let shape = self.shape_copy();
        let x1 = self.r.borrow();
        let x1 = x1.t();
        let x2 = MathTensor::value(shape, rhs);
        let sum = x1 - &x2;
        let rhs = Tensor::from(x2, "sub".to_string());
        let mut children = vec![];
        children.push(self.clone());
        children.push(rhs.clone());

        let sub = BackwardSub::new(self.clone(), rhs);
        let out = TensorInternal::new(sum, OpEnumV4::SUB, children, "sub".to_string(), Some(Box::new(sub)));
        Tensor::new(out)
    }
}

impl Sub<&Tensor> for f64 {
    type Output = Tensor;

    fn sub(self, rhs: &Tensor) -> Self::Output {
        rhs - self
    }
}

// create a Tensor and then subs it
// let a += &b +1.0
impl SubAssign for Tensor {
    fn sub_assign(&mut self, rhs: Tensor) {
        *self = self.clone() - rhs
    }
}

// sub ref to Tensor
// let a += &b
impl SubAssign<&Tensor> for Tensor {
    fn sub_assign(&mut self, rhs: &Tensor) {
        *self = self.clone() - rhs.clone()
    }
}

impl BitXor for &Tensor {
    type Output = Tensor;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let x1 = self.r.borrow();
        let x1 = x1.t();
        let x2 = rhs.r.borrow();
        let x2 = x2.t();

        let mut children = vec![];
        children.push(self.clone());
        children.push(rhs.clone());

        let dot = BackwardDot::new(self.clone(), rhs.clone());
        let t = x1 ^ x2;
        let out = TensorInternal::new(t, OpEnumV4::DOT, children, "dot".to_string(), Some(Box::new(dot)));
        Tensor::new(out)
    }
}

impl Display for OpEnumV4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpEnumV4::ADD => write!(f, "+"),
            OpEnumV4::NONE => write!(f, ""),
            OpEnumV4::MUL => write!(f, "*"),
            OpEnumV4::TANH => write!(f, "tanh"),
            OpEnumV4::EXP => write!(f, "exp"),
            OpEnumV4::DIV => write!(f, "/"),
            OpEnumV4::POW => write!(f, "^"),
            OpEnumV4::SUB => write!(f, "-"),
            OpEnumV4::NEG => write!(f, "neg"),
            OpEnumV4::DOT => write!(f, "dot"),
            OpEnumV4::RELU => write!(f, "relu"),
            OpEnumV4::SUM => write!(f, "sum"),
        }
    }
}

// impl Default for Tensor {
//     fn default() -> Self {
//         Tensor::new(TensorInternal::default())
//     }
// }

//
// impl Display for Tensor {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}: {}", self.r.borrow().label(), self.r.borrow().shape())
//     }
// }

// impl Debug for Tensor {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.r.borrow())
//     }
// }

#[cfg(test)]
mod tests {
    use crate::micrograd_rs_v4_mathtensor::MathTensor;
    use crate::micrograd_rs_v4_tensor::Tensor;
    use crate::{assert_float, assert_two_float, assert_vec_f64, EPS};

    #[test]
    pub fn test_tensor_internal_new() {
        let a = MathTensor::new(vec![3, 2], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let a = Tensor::from(a, "a".to_string());

        let b = MathTensor::new(vec![2, 3], vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
        let b = Tensor::from(b, "b".to_string());

        // trivial assertions
        assert_eq!(a.elem(vec![0, 0]), 1.0);
        assert_eq!(a.elem(vec![0, 1]), 2.0);
        assert_eq!(a.elem(vec![1, 0]), 3.0);
        assert_eq!(a.elem(vec![1, 1]), 4.0);
        assert_eq!(a.elem(vec![2, 0]), 5.0);
        assert_eq!(a.elem(vec![2, 1]), 6.0);

        // trivial assertions
        assert_eq!(b.elem(vec![0, 0]), 11.0);
        assert_eq!(b.elem(vec![0, 1]), 12.0);
        assert_eq!(b.elem(vec![0, 2]), 13.0);
        assert_eq!(b.elem(vec![1, 0]), 14.0);
        assert_eq!(b.elem(vec![1, 1]), 15.0);
        assert_eq!(b.elem(vec![1, 2]), 16.0);

        // not so trivial assertions
        let a_shape_expected = "(3, 2)".to_string();
        let b_shape_expected = "(2, 3)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("b.shape expected {},   actual {}", b_shape_expected, b.shape());

        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(b.shape(), b_shape_expected);
    }
    //

    // use std::f64::consts::SQRT_2;
    //
    // use crate::graph_v3::draw_graph;
    // use crate::micrograd_rs_v3::{assert_float, EPS, ValueRefV3};
    //
    // // before starting to add grad
    // // https://youtu.be/VMj-3S1tku0?t=1875
    // #[test]
    // pub fn test_video() {
    //     let a = ValueRefV3::new_value(2.0 as f64, "a".to_string());
    //     let b = ValueRefV3::new_value(-3.0, "b".to_string());
    //     let c = ValueRefV3::new_value(10.0, "c".to_string());
    //     let f = ValueRefV3::new_value(-2.0, "f".to_string());
    //
    //     let mut e = &a * &b;
    //     e.set_label("e".to_string());
    //
    //     let mut d = &e + &c;
    //     d.set_label("d".to_string());
    //
    //     let mut l = &d * &f;
    //     l.set_label("L".to_string());
    //
    //     assert_float(l.borrow().data, -8.0);
    // }
    //
    #[test]
    pub fn test_add() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a + b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let a = Tensor::from(a, "a".to_string());
        let b = Tensor::from(b, "b".to_string());

        let mut x = &a + &b;
        x.set_label("x".to_string());

        assert_float(c_expected, x.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_add_3x2() {
        let b = 3.0_f64;
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let c_expected: Vec<f64> = data.iter().map(|d| d + b).collect();
        let a = MathTensor::new(vec![3, 2], data);
        let a = Tensor::from(a, "a".to_string());

        let x = &a + b;
        let actual = x.r().borrow();
        let actual = actual.t().data();
        assert_vec_f64(&c_expected, actual);
    }

    #[test]
    pub fn test_add_scalar1() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a + b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let mut x = &a + b;
        x.set_label("x".to_string());

        assert_float(c_expected, x.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_add_scalar2() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a + b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let mut x = b + &a;
        x.set_label("x".to_string());

        assert_float(c_expected, x.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_addassign() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a + b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let mut a = Tensor::from(a, "a".to_string());
        let b = Tensor::from(b, "b".to_string());

        a += &b;

        assert_float(c_expected, a.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_addassign1() {
        let a = 2.0_f64;
        let a_expected = a + a;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let mut a = Tensor::from(a, "a".to_string());

        a += a.clone();

        println!("a expected   {}   actual {}", a_expected, a.elem(vec![0, 0]));
        assert_float(a_expected, a.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_addassign2() {
        let a = 2.0_f64;
        let b = 3.0;
        let a_expected = a + a + b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let mut a = Tensor::from(a, "a".to_string());

        a += &a + b;

        println!("a expected   {}   actual {}", a_expected, a.elem(vec![0, 0]));
        assert_float(a_expected, a.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_mul() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a * b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let a = Tensor::from(a, "a".to_string());
        let b = Tensor::from(b, "b".to_string());

        let x = &a * &b;

        assert_float(c_expected, x.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_mul_scalar1() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a * b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let mut x = &a * b;
        x.set_label("x".to_string());

        assert_float(c_expected, x.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_mul_scalar2() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a * b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let mut x = b * &a;
        x.set_label("x".to_string());

        assert_float(c_expected, x.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_mulassign() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a * b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let mut a = Tensor::from(a, "a".to_string());
        let b = Tensor::from(b, "b".to_string());

        a *= &b;

        assert_float(c_expected, a.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_mulassign1() {
        let a = 2.0_f64;
        let a_expected = a * a;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let mut a = Tensor::from(a, "a".to_string());

        a *= a.clone();

        println!("a expected   {}   actual {}", a_expected, a.elem(vec![0, 0]));
        assert_float(a_expected, a.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_mulassign2() {
        let a = 2.0_f64;
        let b = 3.0;
        let a_expected = a * a * b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let mut a = Tensor::from(a, "a".to_string());

        a *= &a * b;

        println!("a expected   {}   actual {}", a_expected, a.elem(vec![0, 0]));
        assert_float(a_expected, a.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_neg() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let a_expected = -a;
        let b_expected = -b;
        let x = MathTensor::new(vec![2, 1], vec![a, b]);
        let x = Tensor::from(x, "x".to_string());

        let y = -&x;

        assert_float(a_expected, y.elem(vec![0, 0]));
        assert_float(b_expected, y.elem(vec![1, 0]));
    }

    // https://youtu.be/VMj-3S1tku0?t=4977
    #[test]
    pub fn test_a_plus_a() {
        let a = 3.0_f64;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());
        let mut b = &a + &a;

        b.set_label("b".to_string());

        b.backward();

        let topo = Tensor::traverse(&b);
        for t in topo.iter() {
            println!(
                "{}  data {},  grad  {}",
                t.r().borrow().label(),
                t.r().borrow().t().elem(vec![0, 0]),
                t.r().borrow().grad().elem(vec![0, 0])
            );
        }

        println!("b {:?}  {}", b.shape(), b.elem(vec![0, 0]));

        let a_grad_expected = 2.0;
        let b_expected = 6.0;
        assert_float(b_expected, b.elem(vec![0, 0]));

        assert_float(a_grad_expected, a.r().borrow().grad().elem(vec![0, 0]));
    }

    #[test]
    pub fn test_pow() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a.powf(b);
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let x = a.pow(b);

        assert_float(c_expected, x.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_div() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a / b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let a = Tensor::from(a, "a".to_string());
        let b = Tensor::from(b, "b".to_string());

        let x = &a / &b;

        assert_float(c_expected, x.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_div_scalar1() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a / b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let mut x = &a / b;
        x.set_label("x".to_string());

        assert_float(c_expected, x.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_div_scalar2() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = b / a;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let mut x = b / &a;
        x.set_label("x".to_string());

        assert_float(c_expected, x.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_divassign() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a / b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let mut a = Tensor::from(a, "a".to_string());
        let b = Tensor::from(b, "b".to_string());

        a /= &b;

        assert_float(c_expected, a.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_divassign1() {
        let a = 2.0_f64;
        let a_expected = a / a;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let mut a = Tensor::from(a, "a".to_string());

        a /= a.clone();

        println!("a expected   {}   actual {}", a_expected, a.elem(vec![0, 0]));
        assert_float(a_expected, a.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_divassign2() {
        let a = 2.0_f64;
        let b = 3.0;
        let mut a_expected = a;
        a_expected /= a / b;

        let a = MathTensor::new(vec![1, 1], vec![a]);
        let mut a = Tensor::from(a, "a".to_string());

        a /= &a / b;

        println!("a expected   {}   actual {}", a_expected, a.elem(vec![0, 0]));
        assert_float(a_expected, a.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_value_exp() {
        let a = 2.0_f64;
        let b_expected = a.exp();
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let b = a.exp();

        println!("b expected   {}   actual {}", b_expected, b.elem(vec![0, 0]));
        assert_float(b_expected, b.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_value_pow() {
        let a = 2.0_f64;
        let n = 3.0_f64;
        let b_expected = a.powf(n);
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let b = a.pow(n);

        println!("b expected   {}   actual {}", b_expected, b.elem(vec![0, 0]));
        assert_float(b_expected, b.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_value_relu() {
        let a = 2.0_f64;
        let b_expected = a.max(0.0);
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let b = a.relu();

        println!("b expected   {}   actual {}", b_expected, b.elem(vec![0, 0]));
        assert_float(b_expected, b.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_sub() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a - b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let a = Tensor::from(a, "a".to_string());
        let b = Tensor::from(b, "b".to_string());

        let x = &a - &b;

        assert_float(c_expected, x.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_sub_3x2() {
        let b = 3.0_f64;
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let c_expected: Vec<f64> = data.iter().map(|d| d - b).collect();
        let a = MathTensor::new(vec![3, 2], data);
        let a = Tensor::from(a, "a".to_string());

        let x = &a - b;
        let actual = x.r().borrow();
        let actual = actual.t().data();
        assert_vec_f64(&c_expected, actual);
    }

    #[test]
    pub fn test_sub_scalar1() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a - b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let x = &a - b;

        assert_float(c_expected, x.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_sub_scalar2() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a - b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let x = b - &a;

        assert_float(c_expected, x.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_subassign() {
        let a = 2.0_f64;
        let b = 3.0_f64;
        let c_expected = a - b;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let mut a = Tensor::from(a, "a".to_string());
        let b = Tensor::from(b, "b".to_string());

        a -= &b;

        assert_float(c_expected, a.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_subassign1() {
        let a = 2.0_f64;
        let a_expected = a - a;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let mut a = Tensor::from(a, "a".to_string());

        a -= a.clone();

        println!("a expected   {}   actual {}", a_expected, a.elem(vec![0, 0]));
        assert_float(a_expected, a.elem(vec![0, 0]));
    }

    #[test]
    pub fn test_subassign2() {
        let a = 2.0_f64;
        let b = 3.0;
        let mut a_expected = a;
        a_expected -= a - b;

        let a = MathTensor::new(vec![1, 1], vec![a]);
        let mut a = Tensor::from(a, "a".to_string());

        a -= &a - b;

        println!("a expected   {}   actual {}", a_expected, a.elem(vec![0, 0]));
        assert_float(a_expected, a.elem(vec![0, 0]));
    }

    // https://github.com/karpathy/micrograd
    #[test]
    pub fn test_grad() {
        let a = -4.0_f64;
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let mut a = Tensor::from(a, "a".to_string());

        let b = 2.0_f64;
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let mut b = Tensor::from(b, "b".to_string());

        let mut c = &a + &b; //         c = a + b
        let mut d = &a * &b + b.pow(3.0); //         d = a * b + b**3
        c += &c + 1.0; //         c += c + 1
        c += (1.0 + &c) + (-&a); //         c += 1 + c + (-a)
        d += &d * 2.0 + (&b + &a).relu(); //         d += d * 2 + (b + a).relu()
        d += 3.0 * &d + (&b - &a).relu(); //         d += 3 * d + (b - a).relu()
        let mut e = &c - &d; //         e = c - d
        let mut f = (&e).pow(2.0); //         f = e**2
        let mut g = &f / 2.0; //         g = f / 2.0
        g += 10.0 / &f; //         g += 10.0 / f

        g.backward();

        a.set_label("a".to_string());
        b.set_label("b".to_string());
        c.set_label("c".to_string());
        d.set_label("d".to_string());
        e.set_label("e".to_string());
        f.set_label("f".to_string());
        g.set_label("g".to_string());

        let topo = Tensor::traverse(&g);

        println!("#################################");
        println!("Topo");
        for t in topo.iter() {
            println!(
                "{}  data {},  grad  {}",
                t.r().borrow().label(),
                t.r().borrow().t().elem(vec![0, 0]),
                t.r().borrow().grad().elem(vec![0, 0])
            );
        }
        println!("#################################");

        let a_grad_expected = 138.83381924;
        let b_grad_expected = 645.57725948;
        let c_grad_expected = -6.94169096;
        let d_grad_expected = 6.94169096;
        let e_grad_expected = -6.94169096;
        let f_grad_expected = 0.49583507;

        let g_value_expected = 24.70408163;

        let gg = g.r().borrow();

        println!(
            "value g expected {}, actual {}    assert {}",
            g_value_expected,
            gg.t().elem(vec![0, 0]),
            (g_value_expected - gg.t().elem(vec![0, 0])).abs() < EPS
        );
        assert_float(g_value_expected, gg.t().elem(vec![0, 0]));

        let aa = a.r().borrow();
        let bb = b.r().borrow();
        let cc = c.r().borrow();
        let dd = d.r().borrow();
        let ee = e.r().borrow();
        let ff = f.r().borrow();

        println!(
            "grad f expected {}, actual {}   ",
            f_grad_expected,
            ff.grad().elem(vec![0, 0])
        );
        println!(
            "grad e expected {}, actual {}   ",
            e_grad_expected,
            ee.grad().elem(vec![0, 0])
        );
        println!(
            "grad d expected {}, actual {}   ",
            d_grad_expected,
            dd.grad().elem(vec![0, 0])
        );
        println!(
            "grad c expected {}, actual {}   ",
            c_grad_expected,
            cc.grad().elem(vec![0, 0])
        );
        println!(
            "grad b expected {}, actual {}   ",
            b_grad_expected,
            bb.grad().elem(vec![0, 0])
        );
        println!(
            "grad a expected {}, actual {}   ",
            a_grad_expected,
            aa.grad().elem(vec![0, 0])
        );

        assert_float(f_grad_expected, ff.grad().elem(vec![0, 0]));
        assert_float(e_grad_expected, ee.grad().elem(vec![0, 0]));
        assert_float(d_grad_expected, dd.grad().elem(vec![0, 0]));
        assert_float(c_grad_expected, cc.grad().elem(vec![0, 0]));
        assert_float(b_grad_expected, bb.grad().elem(vec![0, 0]));
        assert_float(a_grad_expected, aa.grad().elem(vec![0, 0]));

        // draw_graph(g, "test_all_math_ops_graph".to_string());
    }

    fn assert_two_math_tensors(expected: &MathTensor, actual: &MathTensor) {
        assert_eq!(expected.shape(), actual.shape());

        expected.data().iter().zip(actual.data().iter()).for_each(|(a, b)| {
            if !assert_two_float(*a, *b) {
                println!("expected {}, actual {} not equal", a, b);
            }
            assert_float(*a, *b);
        });
    }

    #[test]
    pub fn test_grad_add() {
        let a = 23.0;
        let b = 45.0;
        let c_expected = a + b;

        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let b = Tensor::from(b, "b".to_string());

        let c_expected = MathTensor::new(vec![1, 1], vec![c_expected]);
        let c_expected = Tensor::from(c_expected, "c_expected".to_string());

        let mut c = &a + &b;

        c.backward();

        let a_grad_expected = vec![1.0];
        let b_grad_expected = vec![1.0];

        let a_grad_expected = MathTensor::new(vec![1, 1], a_grad_expected);
        let b_grad_expected = MathTensor::new(vec![1, 1], b_grad_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();

        let bb = b.r().borrow();
        let b_grad = bb.grad();

        assert_two_math_tensors(&a_grad_expected, a_grad);
        assert_two_math_tensors(&b_grad_expected, b_grad);

        let cc = c.r().borrow();
        let cc = cc.t();

        let cc_expected = c_expected.r().borrow();
        let cc_expected = cc_expected.t();

        assert_two_math_tensors(cc_expected, cc);
    }

    #[test]
    pub fn test_grad_sub() {
        let a = 23.0;
        let b = 45.0;
        let c_expected = a - b;

        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let b = Tensor::from(b, "b".to_string());

        let c_expected = MathTensor::new(vec![1, 1], vec![c_expected]);
        let c_expected = Tensor::from(c_expected, "c_expected".to_string());

        let mut c = &a - &b;

        c.backward();

        let a_grad_expected = vec![1.0];
        let b_grad_expected = vec![-1.0];

        let a_grad_expected = MathTensor::new(vec![1, 1], a_grad_expected);
        let b_grad_expected = MathTensor::new(vec![1, 1], b_grad_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();

        let bb = b.r().borrow();
        let b_grad = bb.grad();

        assert_two_math_tensors(&a_grad_expected, a_grad);
        assert_two_math_tensors(&b_grad_expected, b_grad);

        let cc = c.r().borrow();
        let cc = cc.t();

        let cc_expected = c_expected.r().borrow();
        let cc_expected = cc_expected.t();

        assert_two_math_tensors(cc_expected, cc);
    }

    #[test]
    pub fn test_grad_mul() {
        let a = 12.0;
        let b = 23.0;
        let a_grad_expected = b;
        let b_grad_expected = a;
        let c_expected = a * b;

        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let b = Tensor::from(b, "b".to_string());

        let c_expected = MathTensor::new(vec![1, 1], vec![c_expected]);
        let c_expected = Tensor::from(c_expected, "c_expected".to_string());

        let mut c = &a * &b;

        c.backward();

        let a_grad_expected = vec![a_grad_expected];
        let b_grad_expected = vec![b_grad_expected];

        let a_grad_expected = MathTensor::new(vec![1, 1], a_grad_expected);
        let b_grad_expected = MathTensor::new(vec![1, 1], b_grad_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();

        let bb = b.r().borrow();
        let b_grad = bb.grad();

        assert_two_math_tensors(&a_grad_expected, a_grad);
        assert_two_math_tensors(&b_grad_expected, b_grad);

        let cc = c.r().borrow();
        let cc = cc.t();

        let cc_expected = c_expected.r().borrow();
        let cc_expected = cc_expected.t();

        assert_two_math_tensors(cc_expected, cc);
    }

    #[test]
    pub fn test_grad_div() {
        let a = 7.0;
        let b = 2.0;

        let a_grad_expected = 0.5;
        let b_grad_expected = -1.75;
        let c_expected = a / b;

        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let b = Tensor::from(b, "b".to_string());

        let c_expected = MathTensor::new(vec![1, 1], vec![c_expected]);
        let c_expected = Tensor::from(c_expected, "c_expected".to_string());

        let mut c = &a / &b;

        c.backward();

        let a_grad_expected = vec![a_grad_expected];
        let b_grad_expected = vec![b_grad_expected];

        let a_grad_expected = MathTensor::new(vec![1, 1], a_grad_expected);
        let b_grad_expected = MathTensor::new(vec![1, 1], b_grad_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();

        let bb = b.r().borrow();
        let b_grad = bb.grad();

        assert_two_math_tensors(&a_grad_expected, a_grad);
        assert_two_math_tensors(&b_grad_expected, b_grad);

        let cc = c.r().borrow();
        let cc = cc.t();

        let cc_expected = c_expected.r().borrow();
        let cc_expected = cc_expected.t();

        assert_two_math_tensors(cc_expected, cc);
    }

    pub fn test_relu(a: f64, c_expected: f64, a_grad_expected: f64) {
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let c_expected = MathTensor::new(vec![1, 1], vec![c_expected]);
        let c_expected = Tensor::from(c_expected, "c_expected".to_string());

        let mut c = a.relu();

        c.backward();

        let a_grad_expected = vec![a_grad_expected];

        let a_grad_expected = MathTensor::new(vec![1, 1], a_grad_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();

        assert_two_math_tensors(&a_grad_expected, a_grad);

        let cc = c.r().borrow();
        let cc = cc.t();

        let cc_expected = c_expected.r().borrow();
        let cc_expected = cc_expected.t();

        assert_two_math_tensors(cc_expected, cc);
    }

    #[test]
    pub fn test_grad_relu_positive() {
        let a_f64 = 1.1_f64;
        let expected = a_f64.max(0.0);
        test_relu(a_f64, expected, 1.0);
    }

    #[test]
    pub fn test_grad_relu_zero() {
        let a_f64 = 0.0_f64;
        let expected = a_f64.max(0.0);
        test_relu(a_f64, expected, 0.0);
    }

    #[test]
    pub fn test_grad_relu_negative() {
        let a_f64 = -10.2_f64;
        let expected = a_f64.max(0.0);
        test_relu(a_f64, expected, 0.0);
    }

    pub fn test_pow_grad(a: f64, b: f64, c_expected: f64, a_grad_expected: f64) {
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let c_expected = MathTensor::new(vec![1, 1], vec![c_expected]);
        let c_expected = Tensor::from(c_expected, "c_expected".to_string());

        let mut c = a.pow(b);

        c.backward();

        let a_grad_expected = vec![a_grad_expected];

        let a_grad_expected = MathTensor::new(vec![1, 1], a_grad_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();

        assert_two_math_tensors(&a_grad_expected, a_grad);

        let cc = c.r().borrow();
        let cc = cc.t();

        let cc_expected = c_expected.r().borrow();
        let cc_expected = cc_expected.t();

        assert_two_math_tensors(cc_expected, cc);
    }

    #[test]
    pub fn test_grad_pow_1() {
        let a_f64 = -1.0_f64;
        let b = 2.0;
        let expected = a_f64.powf(b);
        test_pow_grad(a_f64, b, expected, -2.0);
    }

    #[test]
    pub fn test_grad_pow_2() {
        let a_f64 = 3.0_f64;
        let b = 3.0;
        let expected = a_f64.powf(b);
        test_pow_grad(a_f64, b, expected, 27.0);
    }

    #[test]
    pub fn test_grad_pow_3() {
        let a_f64 = 3.0_f64;
        let b = 1.5;
        let expected = a_f64.powf(b);
        test_pow_grad(a_f64, b, expected, 2.598076211353316);
    }

    pub fn test_tanh(a: f64, c_expected: f64, a_grad_expected: f64) {
        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let c_expected = MathTensor::new(vec![1, 1], vec![c_expected]);
        let c_expected = Tensor::from(c_expected, "c_expected".to_string());

        let mut c = a.tanh();

        c.backward();

        let a_grad_expected = vec![a_grad_expected];
        let a_grad_expected = MathTensor::new(vec![1, 1], a_grad_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();

        assert_two_math_tensors(&a_grad_expected, a_grad);

        let cc = c.r().borrow();
        let cc = cc.t();

        let cc_expected = c_expected.r().borrow();
        let cc_expected = cc_expected.t();

        assert_two_math_tensors(cc_expected, cc);
    }

    #[test]
    pub fn test_grad_tanh_1() {
        let a_f64 = 2.0_f64;
        let expected = a_f64.tanh();
        test_tanh(a_f64, expected, 0.07065082485316443);
    }

    #[test]
    pub fn test_grad_tanh_2() {
        let a_f64 = -1.0_f64;
        let expected = a_f64.tanh();
        test_tanh(a_f64, expected, 0.41997434161402614);
    }

    #[test]
    pub fn test_grad_tanh_3() {
        let a_f64 = 0.0_f64;
        let expected = a_f64.tanh();
        test_tanh(a_f64, expected, 1.0);
    }

    #[test]
    pub fn test_add_same_variable() {
        let a = -5.0;

        let a_grad_expected = 2.0;
        let c_expected = a + a;

        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let c_expected = MathTensor::new(vec![1, 1], vec![c_expected]);
        let c_expected = Tensor::from(c_expected, "c_expected".to_string());

        let mut c = &a + &a;

        c.backward();

        let a_grad_expected = vec![a_grad_expected];

        let a_grad_expected = MathTensor::new(vec![1, 1], a_grad_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();

        assert_two_math_tensors(&a_grad_expected, a_grad);

        let cc = c.r().borrow();
        let cc = cc.t();

        let cc_expected = c_expected.r().borrow();
        let cc_expected = cc_expected.t();

        assert_two_math_tensors(cc_expected, cc);
    }

    #[test]
    pub fn test_mul_same_variable() {
        let a = -5.0;

        let a_grad_expected = 2.0 * a;
        let c_expected = a * a;

        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let c_expected = MathTensor::new(vec![1, 1], vec![c_expected]);
        let c_expected = Tensor::from(c_expected, "c_expected".to_string());

        let mut c = &a * &a;

        c.backward();

        let a_grad_expected = vec![a_grad_expected];

        let a_grad_expected = MathTensor::new(vec![1, 1], a_grad_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();

        assert_two_math_tensors(&a_grad_expected, a_grad);

        let cc = c.r().borrow();
        let cc = cc.t();

        let cc_expected = c_expected.r().borrow();
        let cc_expected = cc_expected.t();

        assert_two_math_tensors(cc_expected, cc);
    }
    //
    // #[test]
    // pub fn test_mul2() {
    //     let a = -5.0_f64;
    //     let b = 3.0_f64;
    //     let expected = a * b;
    //     let a_grad_expected = b;
    //     let b_grad_expected = a;
    //     let a = ValueRefV3::new_value(a, "a".to_string());
    //     let b = ValueRefV3::new_value(b, "b".to_string());
    //     let mut c = &a * &b;
    //     c.backward();
    //
    //     println!("a.grad   expected {},   actual {}", a_grad_expected, a.get_grad());
    //     println!("b.grad   expected {},   actual {}", b_grad_expected, b.get_grad());
    //     assert_float(c.get_data(), expected);
    //     assert_float(a.get_grad(), a_grad_expected);
    //     assert_float(b.get_grad(), b_grad_expected);
    //
    //     let topo = ValueRefV3::traverse(&c);
    //     topo.iter().for_each(|t|
    // {
    // println!(
    //                 "{}  data {},  grad  {}",
    //                 x.label(),
    //                 x.t().elem(vec![0, 0]),
    //                 x.grad().elem(vec![0, 0])
    //             );
    // });
    //     assert_eq!(topo.len(), 3);
    // }

    // see micrograd_test_topo.py
    // he uses a set for the children and in case of an c = a + a
    // the variable a is added only once in the children set
    #[test]
    pub fn test_mul_same_variable_topo() {
        let a = -5.0;

        let a_grad_expected = 2.0 * a;
        let c_expected = a * a;

        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let c_expected = MathTensor::new(vec![1, 1], vec![c_expected]);
        let c_expected = Tensor::from(c_expected, "c_expected".to_string());

        let mut c = &a * &a;

        c.backward();

        let a_grad_expected = vec![a_grad_expected];
        let a_grad_expected = MathTensor::new(vec![1, 1], a_grad_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();

        assert_two_math_tensors(&a_grad_expected, a_grad);

        let cc = c.r().borrow();
        let cc = cc.t();

        let cc_expected = c_expected.r().borrow();
        let cc_expected = cc_expected.t();

        assert_two_math_tensors(cc_expected, cc);

        let topo = Tensor::traverse(&c);
        topo.iter().for_each(|t| {
            println!(
                "{}  data {},  grad  {}",
                t.r().borrow().label(),
                t.r().borrow().t().elem(vec![0, 0]),
                t.r().borrow().grad().elem(vec![0, 0])
            );
        });

        assert_eq!(topo.len(), 2);
    }

    #[test]
    pub fn test_neg_grad() {
        let a = -5.0_f64;
        let a_grad_expected = -1.0;
        let c_expected = -a;

        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());

        let c_expected = MathTensor::new(vec![1, 1], vec![c_expected]);
        let c_expected = Tensor::from(c_expected, "c_expected".to_string());

        let mut c = -(&a);

        c.backward();

        let a_grad_expected = vec![a_grad_expected];

        let a_grad_expected = MathTensor::new(vec![1, 1], a_grad_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();

        assert_two_math_tensors(&a_grad_expected, a_grad);

        let cc = c.r().borrow();
        let cc = cc.t();

        let cc_expected = c_expected.r().borrow();
        let cc_expected = cc_expected.t();

        assert_two_math_tensors(cc_expected, cc);
    }

    #[test]
    pub fn test_neg_grad1() {
        let a = -4.0;
        let b = 2.0;

        let a_expected = a;
        let b_expected = b;
        let mut c_expected = a + b;
        c_expected += c_expected + 1.0;
        c_expected += (1.0 + c_expected) + (-a);

        let a_grad_expected = 3.0;
        let b_grad_expected = 4.0;

        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let b = Tensor::from(b, "b".to_string());

        let c_expected = MathTensor::new(vec![1, 1], vec![c_expected]);
        let c_expected = Tensor::from(c_expected, "c_expected".to_string());

        let mut c = &a + &b; //         c = a + b
        c += &c + 1.0; //         c += c + 1
        c += (1.0 + &c) + (-&a); //         c += 1 + c + (-a)
        c.backward();

        let a_expected = vec![a_expected];
        let b_expected = vec![b_expected];

        let a_expected = MathTensor::new(vec![1, 1], a_expected);
        let b_expected = MathTensor::new(vec![1, 1], b_expected);

        let a_grad_expected = vec![a_grad_expected];
        let b_grad_expected = vec![b_grad_expected];

        let a_grad_expected = MathTensor::new(vec![1, 1], a_grad_expected);
        let b_grad_expected = MathTensor::new(vec![1, 1], b_grad_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();
        let aa = aa.t();

        let bb = b.r().borrow();
        let b_grad = bb.grad();
        let bb = bb.t();

        assert_two_math_tensors(&a_expected, aa);
        assert_two_math_tensors(&b_expected, bb);

        assert_two_math_tensors(&a_grad_expected, a_grad);
        assert_two_math_tensors(&b_grad_expected, b_grad);

        let cc = c.r().borrow();
        let cc = cc.t();

        let cc_expected = c_expected.r().borrow();
        let cc_expected = cc_expected.t();

        assert_two_math_tensors(cc_expected, cc);
    }

    #[test]
    pub fn test_sub_relu_grad() {
        let a = -4.0_f64;
        let b = 2.0;

        let a_expected = a;
        let b_expected = b;
        let c_expected = (b - a).max(0.0);

        let a_grad_expected = -1.0;
        let b_grad_expected = 1.0;

        let a = MathTensor::new(vec![1, 1], vec![a]);
        let a = Tensor::from(a, "a".to_string());
        let b = MathTensor::new(vec![1, 1], vec![b]);
        let b = Tensor::from(b, "b".to_string());

        let c_expected = MathTensor::new(vec![1, 1], vec![c_expected]);
        let c_expected = Tensor::from(c_expected, "c_expected".to_string());

        let mut c = (&b - &a).relu();

        c.backward();

        let a_expected = vec![a_expected];
        let b_expected = vec![b_expected];

        let a_expected = MathTensor::new(vec![1, 1], a_expected);
        let b_expected = MathTensor::new(vec![1, 1], b_expected);

        let a_grad_expected = vec![a_grad_expected];
        let b_grad_expected = vec![b_grad_expected];

        let a_grad_expected = MathTensor::new(vec![1, 1], a_grad_expected);
        let b_grad_expected = MathTensor::new(vec![1, 1], b_grad_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();
        let aa = aa.t();

        let bb = b.r().borrow();
        let b_grad = bb.grad();
        let bb = bb.t();

        assert_two_math_tensors(&a_expected, aa);
        assert_two_math_tensors(&b_expected, bb);

        assert_two_math_tensors(&a_grad_expected, a_grad);
        assert_two_math_tensors(&b_grad_expected, b_grad);

        let cc = c.r().borrow();
        let cc = cc.t();

        let cc_expected = c_expected.r().borrow();
        let cc_expected = cc_expected.t();

        assert_two_math_tensors(cc_expected, cc);

        let topo = Tensor::traverse(&c);
        println!("#################################");
        println!("Topo");
        for t in topo.iter() {
            println!(
                "{}  data {},  grad  {}",
                t.r().borrow().label(),
                t.r().borrow().t().elem(vec![0, 0]),
                t.r().borrow().grad().elem(vec![0, 0])
            );
        }
        println!("#################################");
    }

    #[test]
    pub fn test_transpose() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let a = MathTensor::new(vec![2, 3], a);
        let a = Tensor::from(a, "a".to_string());

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

    // sum([1,2,3,4])
    #[test]
    pub fn test_math_tensor_sum_1dim_backward() {
        let a = vec![1.0, 2.0, 3.0, 4.0];

        let c_expected = vec![10.0];

        let a = MathTensor::new(vec![1, 4], a);
        let a = Tensor::from(a, "a".to_string());

        let c_expected = MathTensor::new(vec![1, 4], c_expected);

        let aa = a.r().borrow();
        let _aa = aa.t();

        let mut c = a.sum();

        c.backward();

        let cc = c.r().borrow();
        let cc = cc.t();

        // assert_two_math_tensors(&a_expected, aa);
        assert_two_math_tensors(&c_expected, cc);

        // not so trivial assertions
        let a_shape_expected = "(1, 4)".to_string();
        let c_shape_expected = "(1, 1)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());

        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);

        let aa = a.r().borrow();
        let a_grad = aa.grad();

        let a_grad_expected = MathTensor::ones(vec![1, 4]);

        assert_two_math_tensors(&a_grad_expected, a_grad);
    }

    // sum([[1,2],[3,4]])
    #[test]
    pub fn test_math_tensor_sum_2dim() {
        let a = vec![1.0, 2.0, 3.0, 4.0];

        let c_expected = vec![10.0];

        let a = MathTensor::new(vec![2, 2], a);

        let c = a.sum();

        assert_vec_f64(&c_expected, c.data());

        // not so trivial assertions
        let a_shape_expected = "(2, 2)".to_string();
        let c_shape_expected = "(1, 1)".to_string();

        println!("a.shape expected {},   actual {}", a_shape_expected, a.shape());
        println!("c.shape expected {},   actual {}", c_shape_expected, c.shape());

        assert_eq!(a.shape(), a_shape_expected);
        assert_eq!(c.shape(), c_shape_expected);
    }
}
