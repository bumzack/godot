use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use petgraph::visit::Topo;

use crate::micrograd_rs_v4_mathtensor::MathTensor;
use crate::micrograd_rs_v4_tensorinternal::TensorInternal;

#[derive(PartialEq)]
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

    pub fn elem(&self, pos: Vec<usize>) -> f64 {
        self.r.borrow().t().elem(pos)
    }

    pub fn shape(&self) -> String {
        self.r.borrow().t().shape()
    }

    //
    // // convenience methods to simplify code, to avoid value.r.borrow()
    // pub fn borrow(&self) -> Ref<TensorInternal> {
    //     self.r.borrow()
    // }
    //
    // pub fn set_label(&mut self, label: String) {
    //     self.r.borrow_mut().set_label(label);
    // }
    //
    // pub fn set_grad(&mut self, grad: Vec<f64>) {
    //     self.r.borrow_mut().set_grad(grad);
    // }
    //
    // pub fn set_data(&mut self, data: Vec<f64>) {
    //     self.r.borrow_mut().set_data(data);
    // }
    //
    // pub fn get_grad(&self) ->  &Vec<f64> {
    //    & self.r.borrow().grad()
    // }
    //
    // pub fn get_data(&self) ->  &Vec<f64> {
    //     &self.r.borrow().data()
    // }

    // pub fn tanh(&self) -> Tensor {
    //     let x = self.r.borrow().data();
    //     let y = ((2.0_f64 * x).exp() - 1.0) / ((2.0 * x).exp() + 1.0);
    //
    //     let mut children = vec![];
    //     children.push(self.clone());
    //     let tanh = BackwardTanh { left: self.clone() };
    //     let v = TensorInternal {
    //         data: y,
    //         op: OpEnumV4::TANH,
    //         children,
    //         label: format!("tanh({})", self.borrow().label),
    //         grad: 0.0,
    //         backward: Some(Box::new(tanh)),
    //     };
    //
    //     Tensor::new(v)
    // }

    // pub fn exp(&self) -> Tensor {
    //     let x = self.r.borrow().data();
    //     let y = x.exp();
    //
    //     let mut children = vec![];
    //     children.push(self.clone());
    //
    //     let exp = BackwardExp { left: self.clone() };
    //
    //     let v = TensorInternal {
    //         data: y,
    //         op: OpEnumV4::EXP,
    //         children,
    //         label: format!("exp({})", self.borrow().label),
    //         grad: 0.0,
    //         backward: Some(Box::new(exp)),
    //     };
    //
    //     Tensor::new(v)
    // }

    // pub fn pow(&self, f: f64) -> Tensor {
    //     let string = "pow".to_string();
    //     let r = Tensor::new(TensorInternal::new(f, OpEnumV4::NONE, string));
    //     let x = self.r.borrow().data();
    //     let y = x.powf(f);
    //
    //     let mut children = vec![];
    //     children.push(self.clone());
    //     children.push(r.clone());
    //
    //     let pow = BackwardPow {
    //         left: self.clone(),
    //         right: r,
    //     };
    //     let out = TensorInternal {
    //         data: y,
    //         op: OpEnumV4::POW,
    //         children,
    //         label: format!("{}.pow({})", self.borrow().label, f),
    //         grad: 0.0,
    //         backward: Some(Box::new(pow)),
    //     };
    //
    //     Tensor::new(out)
    // }

    // pub fn relu(&self) -> Tensor {
    //     let x = self.r.borrow().data();
    //     let y = if x < 0.0 { 0.0 } else { x };
    //
    //     let mut children = vec![];
    //     children.push(self.clone());
    //
    //     let relu = BackwardReLU { left: self.clone() };
    //
    //     let v = TensorInternal {
    //         data: y,
    //         op: OpEnumV4::POW,
    //         children,
    //         label: format!("relu({})", self.borrow().label),
    //         grad: 0.0,
    //         backward: Some(Box::new(relu)),
    //     };
    //
    //     Tensor::new(v)
    // }

    // pub fn backward(&mut self) {
    //     // TODO
    //     self.set_grad(vec![1.0]);
    //     let topo = Self::traverse(self);
    //     println!("topo size   {} ", topo.len());
    //     for n in topo.iter().rev() {
    //         match &n.r.borrow().backward {
    //             Some(backward) => {
    //                 backward.apply(n.clone());
    //             }
    //             None => {}
    //         }
    //     }
    // }
    //
    // pub fn traverse(o: &Tensor) -> Vec<Tensor> {
    //     let mut topo = vec![];
    //     let mut visited = HashSet::new();
    //
    //     Self::build_topo(o, &mut topo, &mut visited);
    //
    //     topo
    // }
    //
    // fn build_topo(v: &Tensor, topo: &mut Vec<Tensor>, visited: &mut HashSet<Tensor>) {
    //     if !visited.contains(v) {
    //         visited.insert(v.clone());
    //         for child in v.borrow().children() {
    //             Self::build_topo(child, topo, visited);
    //         }
    //         topo.push(v.clone());
    //     }
    // }
}

//
//

//
// struct BackwardAdd {
//     left: Tensor,
//     right: Tensor,
// }
//
// impl Backward for BackwardAdd {
//     fn apply(&self, out: Tensor) {
//         let mut self__ = self.left.clone();
//         let mut other = self.right.clone();
//
//         self__.set_grad(self__.get_grad() + 1.0 * out.r.borrow().grad());
//         other.set_grad(other.get_grad() + 1.0 * out.r.borrow().grad());
//     }
// }

// struct BackwardSub {
//     left: Tensor,
//     right: Tensor,
// }
//
// impl Backward for BackwardSub {
//     fn apply(&self, out: Tensor) {
//         let mut self__ = self.left.clone();
//         let mut other = self.right.clone();
//
//         self__.set_grad(self__.get_grad() + 1.0 * out.r.borrow().grad());
//         other.set_grad(other.get_grad() - 1.0 * out.r.borrow().grad());
//     }
// }
//
// struct BackwardMul {
//     left: Tensor,
//     right: Tensor,
// }
//
// impl Backward for BackwardMul {
//     fn apply(&self, out: Tensor) {
//         let mut self__ = self.left.clone();
//         let mut other = self.right.clone();
//         let x = other.borrow().data();
//         self__.set_grad(self__.get_grad() + x * out.r.borrow().grad());
//         let x1 = self__.borrow().data();
//         other.set_grad(other.get_grad() + x1 * out.r.borrow().grad());
//     }
// }
//
// struct BackwardTanh {
//     left: Tensor,
// }
//
// impl Backward for BackwardTanh {
//     fn apply(&self, out: Tensor) {
//         let mut self__ = self.left.clone();
//         let x = out.get_data();
//         let y = 1.0 - x * x;
//         self__.set_grad(self__.get_grad() + y * out.r.borrow().grad());
//     }
// }
//
// struct BackwardExp {
//     left: Tensor,
// }
//
// impl Backward for BackwardExp {
//     fn apply(&self, out: Tensor) {
//         let mut self__ = self.left.clone();
//         self__.set_grad(self__.get_grad() + out.r.borrow().data() * out.r.borrow().grad());
//     }
// }
//
// struct BackwardPow {
//     left: Tensor,
//     right: Tensor,
// }
//
// impl Backward for BackwardPow {
//     fn apply(&self, out: Tensor) {
//         let mut self__ = self.left.clone();
//         let other = self.right.clone().borrow().data;
//         let x = other * (self__.borrow().data().powf(other - 1.0)) * out.r.borrow().grad();
//         self__.set_grad(self__.get_grad() + x);
//     }
// }
//
// struct BackwardReLU {
//     left: Tensor,
// }
//
// impl Backward for BackwardReLU {
//     fn apply(&self, out: Tensor) {
//         // println!("ReLU out {:?},  'self' {:?} ", out, self.left);
//         let mut self__ = self.left.clone();
//         let x = if out.get_data() > 0.0 { out.get_grad() } else { 0.0 };
//         self__.set_grad(self__.get_grad() + x);
//     }
// }

// impl Add<&Tensor> for &Tensor {
//     type Output = Tensor;
//
//     fn add(self, rhs: &Tensor) -> Self::Output {
//         let x1 = self.r.borrow();
//         let x2 = rhs.r.borrow();
//
//         let mut children = vec![];
//         children.push(self.clone());
//         children.push(rhs.clone());
//
//         let add = BackwardAdd {
//             left: self.clone(),
//             right: rhs.clone(),
//         };
//         let out = TensorInternal {
//             data: x1.data + x2.data,
//             op: OpEnumV4::ADD,
//             children,
//             label: format!("{} + {}", self.borrow().label, rhs.borrow().label),
//             grad: 0.0,
//             backward: Some(Box::new(add)),
//             shape: vec![]
//         };
//         Tensor::new(out)
//     }
// }
//
// impl Add for Tensor {
//     type Output = Tensor;
//
//     fn add(self, rhs: Tensor) -> Self::Output {
//         let x1 = self.r.borrow();
//         let x2 = rhs.r.borrow();
//         let mut children = vec![];
//         children.push(self.clone());
//         children.push(rhs.clone());
//         let add = BackwardAdd {
//             left: self.clone(),
//             right: rhs.clone(),
//         };
//         let out = TensorInternal {
//             data: x1.data + x2.data,
//             op: OpEnumV4::ADD,
//             children,
//             label: format!("{} + {}", self.borrow().label, rhs.borrow().label),
//             grad: 0.0,
//             backward: Some(Box::new(add)),
//         };
//         Tensor::new(out)
//     }
// }
//
// impl AddAssign for Tensor {
//     fn add_assign(&mut self, rhs: Tensor) {
//         *self = self.clone() + rhs
//     }
// }
//
// impl SubAssign for Tensor {
//     fn sub_assign(&mut self, rhs: Tensor) {
//         *self = self.clone() - rhs
//     }
// }
//
// impl Sub for &Tensor {
//     type Output = Tensor;
//
//     fn sub(self, rhs: &Tensor) -> Self::Output {
//         let x1 = self.r.borrow();
//         let x2 = rhs.r.borrow();
//         let mut children = vec![];
//         children.push(self.clone());
//         children.push(rhs.clone());
//         let sub = BackwardSub {
//             left: self.clone(),
//             right: rhs.clone(),
//         };
//         let out = TensorInternal {
//             data: x1.data - x2.data,
//             op: OpEnumV4::SUB,
//             children,
//             label: format!("{} - {}", self.borrow().label, rhs.borrow().label),
//             grad: 0.0,
//             backward: Some(Box::new(sub)),
//         };
//         Tensor::new(out)
//     }
// }
//
// impl Sub for Tensor {
//     type Output = Tensor;
//
//     fn sub(self, rhs: Tensor) -> Self::Output {
//         let x1 = self.r.borrow();
//         let x2 = rhs.r.borrow();
//
//         let mut children = vec![];
//         children.push(self.clone());
//         children.push(rhs.clone());
//         let sub = BackwardSub {
//             left: self.clone(),
//             right: rhs.clone(),
//         };
//         let out = TensorInternal {
//             data: x1.data - x2.data,
//             op: OpEnumV4::SUB,
//             children,
//             label: format!("{} - {}", self.borrow().label, rhs.borrow().label),
//             grad: 0.0,
//             backward: Some(Box::new(sub)),
//         };
//         Tensor::new(out)
//     }
// }
//
// impl Mul<&Tensor> for &Tensor {
//     type Output = Tensor;
//
//     fn mul(self, rhs: &Tensor) -> Self::Output {
//         let x1 = self.r.borrow();
//         let x2 = rhs.r.borrow();
//
//         let mut children = vec![];
//         children.push(self.clone());
//         children.push(rhs.clone());
//         let mul = BackwardMul {
//             left: self.clone(),
//             right: rhs.clone(),
//         };
//
//         let out = TensorInternal {
//             data: x1.data * x2.data,
//             op: OpEnumV4::MUL,
//             children,
//             label: format!("{} * {}", self.borrow().label, rhs.borrow().label),
//             grad: 0.0,
//             backward: Some(Box::new(mul)),
//         };
//         Tensor::new(out)
//     }
// }
//
// impl Div<&Tensor> for &Tensor {
//     type Output = Tensor;
//
//     fn div(self, rhs: &Tensor) -> Self::Output {
//         self * &rhs.pow(-1.0)
//     }
// }
//
// impl Add<f64> for &Tensor {
//     type Output = Tensor;
//
//     fn add(self, rhs: f64) -> Self::Output {
//         let string = "f64 add".to_string();
//         let r = Tensor::new_value(rhs, string.clone());
//         let x1 = self.r.borrow();
//
//         let mut children = vec![];
//         children.push(self.clone());
//         children.push(r.clone());
//         let add = BackwardAdd {
//             left: self.clone(),
//             right: r,
//         };
//
//         let out = TensorInternal {
//             data: x1.data + rhs,
//             op: OpEnumV4::ADD,
//             children,
//             label: format!("{} + {}", self.borrow().label, string),
//             grad: 0.0,
//             backward: Some(Box::new(add)),
//         };
//         Tensor::new(out)
//     }
// }
//
// impl Add<&Tensor> for f64 {
//     type Output = Tensor;
//
//     fn add(self, rhs: &Tensor) -> Self::Output {
//         let string = "f64 add".to_string();
//         let r = Tensor::new_value(self, string.clone());
//         let x1 = rhs.r.borrow();
//         let add = BackwardAdd {
//             left: r.clone(),
//             right: rhs.clone(),
//         };
//
//         let mut children = vec![];
//         children.push(r);
//         children.push(rhs.clone());
//
//         let out = TensorInternal {
//             data: x1.data + self,
//             op: OpEnumV4::ADD,
//             children,
//             label: format!("{} + {}", string, rhs.borrow().label),
//             grad: 0.0,
//             backward: Some(Box::new(add)),
//         };
//         Tensor::new(out)
//     }
// }
//
// impl Sub<f64> for &Tensor {
//     type Output = Tensor;
//
//     fn sub(self, rhs: f64) -> Self::Output {
//         let string = "f64 sub".to_string();
//         let r = Tensor::new_value(rhs, string.clone());
//         let x1 = self.r.borrow();
//
//         let mut children = vec![];
//         children.push(self.clone());
//         children.push(r.clone());
//         let sub = BackwardSub {
//             left: self.clone(),
//             right: r,
//         };
//
//         let out = TensorInternal {
//             data: x1.data - rhs,
//             op: OpEnumV4::SUB,
//             children,
//             label: format!("{} + {}", self.borrow().label, string),
//             grad: 0.0,
//             backward: Some(Box::new(sub)),
//         };
//         Tensor::new(out)
//     }
// }
//
// impl Sub<&Tensor> for f64 {
//     type Output = Tensor;
//
//     fn sub(self, rhs: &Tensor) -> Self::Output {
//         let string = "f64 sub".to_string();
//         let r = Tensor::new_value(self, string.clone());
//         let x1 = rhs.r.borrow();
//
//         let mut children = vec![];
//         children.push(r.clone());
//         children.push(rhs.clone());
//         let sub = BackwardSub {
//             left: r,
//             right: rhs.clone(),
//         };
//
//         let out = TensorInternal {
//             data: self - x1.data,
//             op: OpEnumV4::SUB,
//             children,
//             label: format!("{} - {}", string, rhs.borrow().label),
//             grad: 0.0,
//             backward: Some(Box::new(sub)),
//         };
//         Tensor::new(out)
//     }
// }
//
// impl Mul<f64> for &Tensor {
//     type Output = Tensor;
//
//     fn mul(self, rhs: f64) -> Self::Output {
//         let string = "f64 mul".to_string();
//         let r = Tensor::new_value(rhs, string.clone());
//         let x1 = self.r.borrow();
//         let mut children = vec![];
//         children.push(self.clone());
//         children.push(r.clone());
//         let mul = BackwardMul {
//             left: self.clone(),
//             right: r,
//         };
//         let out = TensorInternal {
//             data: x1.data * rhs,
//             op: OpEnumV4::MUL,
//             children,
//             label: format!("{} + {}", self.borrow().label, string),
//             grad: 0.0,
//             backward: Some(Box::new(mul)),
//         };
//         Tensor::new(out)
//     }
// }
//
// impl Mul<f64> for Tensor {
//     type Output = Tensor;
//
//     fn mul(self, rhs: f64) -> Self::Output {
//         let string = "f64 mul".to_string();
//         let r = Tensor::new_value(rhs, string.clone());
//         let x1 = self.r.borrow();
//         let mut children = vec![];
//         children.push(self.clone());
//         children.push(r.clone());
//         let mul = BackwardMul {
//             left: self.clone(),
//             right: r,
//         };
//
//         let out = TensorInternal {
//             data: x1.data * rhs,
//             op: OpEnumV4::MUL,
//             children,
//             label: format!("{} + {}", self.borrow().label, string),
//             grad: 0.0,
//             backward: Some(Box::new(mul)),
//         };
//         Tensor::new(out)
//     }
// }
//
// impl Mul<&Tensor> for f64 {
//     type Output = Tensor;
//
//     fn mul(self, rhs: &Tensor) -> Self::Output {
//         let string = "f64 mul".to_string();
//         let r = Tensor::new_value(self, string.clone());
//         let x1 = rhs.r.borrow();
//
//         let mut children = vec![];
//         children.push(r.clone());
//         children.push(rhs.clone());
//         let mul = BackwardMul {
//             left: r,
//             right: rhs.clone(),
//         };
//
//         let out = TensorInternal {
//             data: x1.data * self,
//             op: OpEnumV4::MUL,
//             children,
//             label: format!("{} + {}", string, rhs.borrow().label),
//             grad: 0.0,
//             backward: Some(Box::new(mul)),
//         };
//         Tensor::new(out)
//     }
// }
//
// impl Div<f64> for Tensor {
//     type Output = Tensor;
//
//     fn div(self, rhs: f64) -> Self::Output {
//         self * rhs.powf(-1.0)
//     }
// }
//
// impl Div<f64> for &Tensor {
//     type Output = Tensor;
//
//     fn div(self, rhs: f64) -> Self::Output {
//         self * rhs.powf(-1.0)
//     }
// }
//
// impl Div<Tensor> for f64 {
//     type Output = Tensor;
//
//     fn div(self, rhs: Tensor) -> Self::Output {
//         self * &rhs.pow(-1.0)
//     }
// }
//
// impl Div<&Tensor> for f64 {
//     type Output = Tensor;
//
//     fn div(self, rhs: &Tensor) -> Self::Output {
//         self * &rhs.pow(-1.0)
//     }
// }
//
// impl Div<&mut Tensor> for f64 {
//     type Output = Tensor;
//
//     fn div(self, rhs: &mut Tensor) -> Self::Output {
//         self * &rhs.pow(-1.0)
//     }
// }
//
// impl Neg for Tensor {
//     type Output = Tensor;
//
//     fn neg(self) -> Self::Output {
//         self * (-1.0)
//     }
// }
//
// impl Neg for &Tensor {
//     type Output = Tensor;
//
//     fn neg(self) -> Self::Output {
//         self * (-1.0)
//     }
// }

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
            OpEnumV4::NEG => write!(f, "NEG"),
            OpEnumV4::DOT => write!(f, "^"),
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
    use crate::micrograd_rs_v4_tensor::{OpEnumV4, Tensor, TensorInternal};

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
    // use crate::micrograd_rs_v3::{assert_two_float, EPS, ValueRefV3};
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
    //     assert_two_float(l.borrow().data, -8.0);
    // }
    //
    // #[test]
    // pub fn test_add() {
    //     let a = ValueRefV3::new_value(2.0 as f64, "a".to_string());
    //     let b = ValueRefV3::new_value(3.0, "b".to_string());
    //
    //     let mut x = &a + &b;
    //     x.set_label("x".to_string());
    //
    //     assert_two_float(x.borrow().data, 5.0);
    // }
    //
    // #[test]
    // pub fn test_mul() {
    //     let a = ValueRefV3::new_value(2.0 as f64, "a".to_string());
    //     let b = ValueRefV3::new_value(3.0, "b".to_string());
    //
    //     let mut x = &a * &b;
    //     x.set_label("x".to_string());
    //
    //     assert_two_float(x.borrow().data, 6.0);
    // }
    //
    // // https://youtu.be/VMj-3S1tku0?t=4977
    // #[test]
    // pub fn test_a_plus_a() {
    //     let a = ValueRefV3::new_value(3.0, "a".to_string());
    //     let mut b = &a + &a;
    //
    //     b.set_label("b".to_string());
    //
    //     b.backward();
    //
    //     let topo = ValueRefV3::traverse(&b);
    //     for t in topo.iter() {
    //         println!("topo  {:?}", t);
    //     }
    //
    //     println!("b {}", b);
    //
    //     assert_two_float(b.borrow().data, 6.0);
    //
    //     draw_graph(b, "test_a_plus_a".to_string());
    //
    //     assert_two_float(a.get_grad(), 2.0);
    // }
    //
    // #[test]
    // pub fn test_value_plus_f64_rhs() {
    //     let a = ValueRefV3::new_value(3.0, "a".to_string());
    //     let mut b = &a + 1.0 as f64;
    //     assert_two_float(b.borrow().data, 4.0);
    //     b.backward();
    //     draw_graph(b, "test_a_plus_f64_rhs".to_string());
    // }
    //
    // #[test]
    // pub fn test_value_plus_f64_lhs() {
    //     let a = ValueRefV3::new_value(4.0, "a".to_string());
    //     let mut b = 23.0 as f64 + &a;
    //     b.backward();
    //     assert_two_float(b.borrow().data, 27.0);
    //
    //     draw_graph(b, "test_a_plus_f64_lhs".to_string());
    // }
    //
    // #[test]
    // pub fn test_value_mul_f64_rhs() {
    //     let a = ValueRefV3::new_value(3.0, "a".to_string());
    //     let mut b = &a * 3.0 as f64;
    //     assert_two_float(b.borrow().data, 9.0);
    //     b.backward();
    //     draw_graph(b, "test_a_mul_f64_rhs".to_string());
    // }
    //
    // #[test]
    // pub fn test_value_mul_f64_lhs() {
    //     let a = ValueRefV3::new_value(4.0, "a".to_string());
    //     let mut b = 23.0 as f64 * &a;
    //     b.backward();
    //     assert_two_float(b.borrow().data, 92.0);
    //
    //     draw_graph(b, "test_a_mul_f64_lhs".to_string());
    // }
    //
    // #[test]
    // pub fn test_value_div() {
    //     let a = ValueRefV3::new_value(4.0, "a".to_string());
    //     let b = ValueRefV3::new_value(2.0, "b".to_string());
    //     let mut c = &a / &b;
    //     c.backward();
    //     assert_two_float(c.borrow().data, 2.0);
    //
    //     draw_graph(c, "test_a_div_b".to_string());
    // }
    //
    // #[test]
    // pub fn test_value_exp() {
    //     let expected = (4.0 as f64).exp();
    //     let a = ValueRefV3::new_value(4.0, "a".to_string());
    //     let mut b = a.exp();
    //
    //     b.backward();
    //     assert_two_float(b.borrow().data, expected);
    //
    //     draw_graph(b, "test_exp_4".to_string());
    // }
    //
    // #[test]
    // pub fn test_value_pow() {
    //     let expected = (4.0 as f64).powf(3.0);
    //     let a = ValueRefV3::new_value(4.0, "a".to_string());
    //     let b = 3.0;
    //     let mut b = a.pow(b);
    //
    //     b.backward();
    //     assert_two_float(b.borrow().data, expected);
    //
    //     draw_graph(b, "test_pow".to_string());
    // }
    //
    // #[test]
    // pub fn test_value_relu() {
    //     let expected = 4.0 - 23.0;
    //     let a = ValueRefV3::new_value(4.0, "a".to_string());
    //     let b = ValueRefV3::new_value(23.0, "b".to_string());
    //     let mut c = &a - &b;
    //     c.set_label("c".to_string());
    //
    //     c.backward();
    //     assert_two_float(c.borrow().data, expected);
    //
    //     draw_graph(c, "test_sub".to_string());
    // }
    //
    // #[test]
    // pub fn test_value_sub() {
    //     let expected = 4.0 - 23.0;
    //     let a = ValueRefV3::new_value(4.0, "a".to_string());
    //     let b = ValueRefV3::new_value(23.0, "b".to_string());
    //     let mut c = &a - &b;
    //     c.set_label("c".to_string());
    //
    //     c.backward();
    //     assert_two_float(c.borrow().data, expected);
    //
    //     draw_graph(c, "test_sub".to_string());
    // }
    //
    // // https://github.com/karpathy/micrograd
    // #[test]
    // pub fn test_grad() {
    //     let mut a = ValueRefV3::new_value(-4.0, "a".to_string()); //         a = Value(-4.0)
    //     let mut b = ValueRefV3::new_value(2.0, "b".to_string()); //         b = Value(2.0)
    //     let mut c = &a + &b; //         c = a + b
    //     let mut d = &a * &b + b.pow(3.0); //         d = a * b + b**3
    //     c += &c + 1.0; //         c += c + 1
    //     c += (1.0 + &c) + (-&a); //         c += 1 + c + (-a)
    //     d += &d * 2.0 + (&b + &a).relu(); //         d += d * 2 + (b + a).relu()
    //     d += 3.0 * &d + (&b - &a).relu(); //         d += 3 * d + (b - a).relu()
    //     let mut e = &c - &d; //         e = c - d
    //     let mut f = (&e).pow(2.0); //         f = e**2
    //     let mut g = &f / 2.0 as f64; //         g = f / 2.0
    //     g += 10.0 as f64 / &f; //         g += 10.0 / f
    //
    //     g.backward();
    //
    //     a.set_label("a".to_string());
    //     b.set_label("b".to_string());
    //     c.set_label("c".to_string());
    //     d.set_label("d".to_string());
    //     e.set_label("e".to_string());
    //     f.set_label("f".to_string());
    //     g.set_label("g".to_string());
    //
    //     let topo = ValueRefV3::traverse(&g);
    //
    //     println!("#################################");
    //     println!("Topo");
    //     for t in topo.iter() {
    //         println!(
    //             "{}  data {},  grad  {}",
    //             t.r.borrow().label(),
    //             t.get_data(),
    //             t.get_grad()
    //         );
    //     }
    //     println!("#################################");
    //
    //     let a_grad_expected = 138.83381924;
    //     let b_grad_expected = 645.57725948;
    //     let c_grad_expected = -6.94169096;
    //     let d_grad_expected = 6.94169096;
    //     let e_grad_expected = -6.94169096;
    //     let f_grad_expected = 0.49583507;
    //
    //     let g_value_expected = 24.70408163;
    //
    //     println!(
    //         "value g expected {}, actual {}    assert {}",
    //         g_value_expected,
    //         g.get_data(),
    //         (g_value_expected - g.get_data()).abs() < EPS
    //     );
    //     assert_two_float(g_value_expected, g.get_data());
    //
    //     println!("grad f expected {}, actual {}   ", f_grad_expected, f.get_grad());
    //     println!("grad e expected {}, actual {}   ", e_grad_expected, e.get_grad());
    //     println!("grad d expected {}, actual {}   ", d_grad_expected, d.get_grad());
    //     println!("grad c expected {}, actual {}   ", c_grad_expected, c.get_grad());
    //     println!("grad b expected {}, actual {}   ", b_grad_expected, b.get_grad());
    //     println!("grad a expected {}, actual {}   ", a_grad_expected, a.get_grad());
    //
    //     assert_two_float(f_grad_expected, f.get_grad());
    //     assert_two_float(e_grad_expected, e.get_grad());
    //     assert_two_float(d_grad_expected, d.get_grad());
    //     assert_two_float(c_grad_expected, c.get_grad());
    //     assert_two_float(b_grad_expected, b.get_grad());
    //     assert_two_float(a_grad_expected, a.get_grad());
    //
    //     // draw_graph(g, "test_all_math_ops_graph".to_string());
    // }
    //
    // #[test]
    // pub fn test_grad_add() {
    //     let a = 23.0;
    //     let b = 45.0;
    //     let expected = a + b;
    //
    //     let a = ValueRefV3::new_value(a, "a".to_string());
    //     let b = ValueRefV3::new_value(b, "b".to_string());
    //     let mut c = &a + &b;
    //     c.set_label("c".to_string());
    //
    //     c.backward();
    //     assert_two_float(c.borrow().data, expected);
    //
    //     assert_two_float(a.get_grad(), 1.0);
    //     assert_two_float(b.get_grad(), 1.0);
    // }
    //
    // #[test]
    // pub fn test_grad_sub() {
    //     let a = 23.0;
    //     let b = 45.0;
    //     let expected = a - b;
    //
    //     let a = ValueRefV3::new_value(a, "a".to_string());
    //     let b = ValueRefV3::new_value(b, "b".to_string());
    //     let mut c = &a - &b;
    //     c.set_label("c".to_string());
    //
    //     c.backward();
    //
    //     println!("c actual  {}  expected {}", c.borrow().data, expected);
    //     println!("a.grad  {} ", a.get_grad());
    //     println!("b.grad  {} ", b.get_grad());
    //     assert_two_float(c.borrow().data, expected);
    //
    //     assert_two_float(a.get_grad(), 1.0);
    //     assert_two_float(b.get_grad(), -1.0);
    // }
    //
    // #[test]
    // pub fn test_grad_mul() {
    //     let a_f64 = 12.0;
    //     let b_f64 = 23.0;
    //     let expected = a_f64 * b_f64;
    //
    //     let a = ValueRefV3::new_value(a_f64, "a".to_string());
    //     let b = ValueRefV3::new_value(b_f64, "b".to_string());
    //     let mut c = &a * &b;
    //     c.set_label("c".to_string());
    //
    //     c.backward();
    //
    //     println!("c actual  {}  expected {}", c.borrow().data, expected);
    //     println!("a.grad  {} ", a.get_grad());
    //     println!("b.grad  {} ", b.get_grad());
    //     assert_two_float(c.borrow().data, expected);
    //
    //     assert_two_float(a.get_grad(), b_f64);
    //     assert_two_float(b.get_grad(), a_f64);
    // }
    //
    // #[test]
    // pub fn test_grad_div() {
    //     let a_f64 = 7.0;
    //     let b_f64 = 2.0;
    //     let expected = a_f64 / b_f64;
    //
    //     let a = ValueRefV3::new_value(a_f64, "a".to_string());
    //     let b = ValueRefV3::new_value(b_f64, "b".to_string());
    //     let mut c = &a / &b;
    //     c.set_label("c".to_string());
    //
    //     c.backward();
    //
    //     println!("c actual  {}  expected {}", c.borrow().data, expected);
    //     println!("a.grad  {} ", a.get_grad());
    //     println!("b.grad  {} ", b.get_grad());
    //     assert_two_float(c.borrow().data, expected);
    //
    //     assert_two_float(a.get_grad(), 0.5);
    //     assert_two_float(b.get_grad(), -1.75);
    // }
    //
    // pub fn test_relu(a_f64: f64, c_expected: f64, a_grad_expected: f64) {
    //     let a = ValueRefV3::new_value(a_f64, "a".to_string());
    //     let mut c = a.relu();
    //     c.set_label("c".to_string());
    //
    //     c.backward();
    //
    //     println!("c actual  {}  expected {}", c.borrow().data, c_expected);
    //     println!("a.grad  {} ", a.get_grad());
    //     assert_two_float(c.borrow().data, c_expected);
    //
    //     assert_two_float(a.get_grad(), a_grad_expected);
    // }
    //
    // #[test]
    // pub fn test_grad_relu_positive() {
    //     let a_f64 = 1.1_f64;
    //     let expected = a_f64.max(0.0);
    //     test_relu(a_f64, expected, 1.0);
    // }
    //
    // #[test]
    // pub fn test_grad_relu_zero() {
    //     let a_f64 = 0.0_f64;
    //     let expected = a_f64.max(0.0);
    //     test_relu(a_f64, expected, 0.0);
    // }
    //
    // #[test]
    // pub fn test_grad_relu_negative() {
    //     let a_f64 = -10.2_f64;
    //     let expected = a_f64.max(0.0);
    //     test_relu(a_f64, expected, 0.0);
    // }
    //
    // pub fn test_pow(a_f64: f64, b: f64, c_expected: f64, a_grad_expected: f64) {
    //     let a = ValueRefV3::new_value(a_f64, "a".to_string());
    //     let mut c = a.pow(b);
    //     c.set_label("c".to_string());
    //
    //     c.backward();
    //
    //     println!("c actual  {}  expected {}", c.borrow().data, c_expected);
    //     println!("a.grad  {} ", a.get_grad());
    //     assert_two_float(c.get_data(), c_expected);
    //     assert_two_float(a.get_grad(), a_grad_expected);
    // }
    //
    // #[test]
    // pub fn test_grad_pow_1() {
    //     let a_f64 = -1.0_f64;
    //     let b = 2.0;
    //     let expected = a_f64.powf(b);
    //     test_pow(a_f64, b, expected, -2.0);
    // }
    //
    // #[test]
    // pub fn test_grad_pow_2() {
    //     let a_f64 = 3.0_f64;
    //     let b = 3.0;
    //     let expected = a_f64.powf(b);
    //     test_pow(a_f64, b, expected, 27.0);
    // }
    //
    // #[test]
    // pub fn test_grad_pow_3() {
    //     let a_f64 = 3.0_f64;
    //     let b = 1.5;
    //     let expected = a_f64.powf(b);
    //     test_pow(a_f64, b, expected, 2.598076211353316);
    // }
    //
    // pub fn test_tanh(a_f64: f64, c_expected: f64, a_grad_expected: f64) {
    //     let a = ValueRefV3::new_value(a_f64, "a".to_string());
    //     let mut c = a.tanh();
    //     c.set_label("c".to_string());
    //
    //     c.backward();
    //
    //     println!("c actual  {}  expected {}", c.borrow().data, c_expected);
    //     println!("a.grad  {} ", a.get_grad());
    //     assert_two_float(c.borrow().data, c_expected);
    //     assert_two_float(a.get_grad(), a_grad_expected);
    // }
    //
    // #[test]
    // pub fn test_grad_tanh_1() {
    //     let a_f64 = 2.0_f64;
    //     let expected = a_f64.tanh();
    //     test_tanh(a_f64, expected, 0.07065082485316443);
    // }
    //
    // #[test]
    // pub fn test_grad_tanh_2() {
    //     let a_f64 = -1.0_f64;
    //     let expected = a_f64.tanh();
    //     test_tanh(a_f64, expected, 0.41997434161402614);
    // }
    //
    // #[test]
    // pub fn test_grad_tanh_3() {
    //     let a_f64 = 0.0_f64;
    //     let expected = a_f64.tanh();
    //     test_tanh(a_f64, expected, 1.0);
    // }
    //
    // #[test]
    // pub fn test_simple_neurons_explizt_tanh() {
    //     let x1: ValueRefV3 = ValueRefV3::new_value(2.0, "x1".to_string());
    //     let x2 = ValueRefV3::new_value(0.0, "x2".to_string());
    //
    //     let w1 = ValueRefV3::new_value(-3.0, "w1".to_string());
    //     let w2 = ValueRefV3::new_value(1.0, "w2".to_string());
    //
    //     let b = ValueRefV3::new_value(6.881373587019, "b".to_string());
    //
    //     let w1x1 = &x1 * &w1;
    //     let w2x2 = &x2 * &w2;
    //     let w1x1_plus_w2x2 = &w1x1 + &w2x2;
    //
    //     let mut n = &w1x1_plus_w2x2 + &b;
    //     n.set_label("n".to_string());
    //
    //     let e = (2.0 * &n).exp();
    //     let mut o = &(&e - 1.0) / &(&e + 1.0);
    //
    //     // let mut o = n.tanh();
    //     o.set_label("o".to_string());
    //
    //     o.backward();
    //
    //     println!("x1 grad  expected {},  actual {}", -1.5, x1.get_grad());
    //     println!("w1 grad  expected {},  actual {}", 1.0, w1.get_grad());
    //     assert_two_float(x1.get_grad(), -1.5);
    //     assert_two_float(w1.get_grad(), 1.0);
    //
    //     println!("x2 grad  expected {},  actual {}", 0.5, x2.get_grad());
    //     println!("w2 grad  expected {},  actual {}", 0.0, w2.get_grad());
    //     assert_two_float(x2.get_grad(), 0.5);
    //     assert_two_float(w2.get_grad(), 0.0);
    //
    //     println!("w1x1 data  expected {},  actual {}", -6.0, w1x1.get_data());
    //     println!("w2x2 data  expected {},  actual {}", 0.0, w2x2.get_data());
    //
    //     assert_two_float(w1x1.get_data(), -6.0);
    //     assert_two_float(w2x2.get_data(), 0.0);
    //
    //     println!("w1x1 grad  expected {},  actual {}", 0.5, w1x1.get_grad());
    //     println!("w2x2 grad  expected {},  actual {}", 0.5, w2x2.get_grad());
    //
    //     assert_two_float(w1x1.get_grad(), 0.5);
    //     assert_two_float(w2x2.get_grad(), 0.5);
    //
    //     println!("b  data  expected {},  actual {}", 6.881373587019, b.get_data());
    //     println!("b  grad  expected {},  actual {}", 0.5, b.get_grad());
    //
    //     assert_two_float(b.get_data(), 6.881373587019);
    //     assert_two_float(b.get_grad(), 0.5);
    //
    //     println!(
    //         "w1x1_plus_w2x2 data  expected {},  actual {}",
    //         -6.,
    //         w1x1_plus_w2x2.get_data()
    //     );
    //     println!(
    //         "w1x1_plus_w2x2 grad  expected {},  actual {}",
    //         0.5,
    //         w1x1_plus_w2x2.get_grad()
    //     );
    //
    //     assert_two_float(w1x1_plus_w2x2.get_data(), -6.0);
    //     assert_two_float(w1x1_plus_w2x2.get_grad(), 0.5);
    //
    //     println!("n data  expected {},  actual {}", 0.8814, n.get_data());
    //     println!("n grad  expected {},  actual {}", 0.5, n.get_grad());
    //
    //     assert_two_float(n.get_data(), 0.8814);
    //     assert_two_float(n.get_grad(), 0.5);
    //
    //     println!("e data  expected {},  actual {}", 5.8284, e.get_data());
    //     println!("e grad  expected {},  actual {}", 0.0429, e.get_grad());
    //
    //     assert_two_float(e.get_data(), 5.8284);
    //     assert_two_float(e.get_grad(), 0.0429);
    //
    //     println!("o data  expected {},  actual {}", SQRT_2 / 2.0, o.get_data());
    //     println!("o grad  expected {},  actual {}", 1.0, o.get_grad());
    //
    //     assert_two_float(o.get_data(), SQRT_2 / 2.0);
    //     assert_two_float(o.get_grad(), 1.0);
    // }
    //
    // #[test]
    // pub fn test_simple_neurons_simple_tanh() {
    //     let x1: ValueRefV3 = ValueRefV3::new_value(2.0, "x1".to_string());
    //     let x2 = ValueRefV3::new_value(0.0, "x2".to_string());
    //
    //     let w1 = ValueRefV3::new_value(-3.0, "w1".to_string());
    //     let w2 = ValueRefV3::new_value(1.0, "w2".to_string());
    //
    //     let b = ValueRefV3::new_value(6.881373587019, "b".to_string());
    //
    //     let w1x1 = &x1 * &w1;
    //     let w2x2 = &x2 * &w2;
    //     let w1x1_plus_w2x2 = &w1x1 + &w2x2;
    //
    //     let mut n = &w1x1_plus_w2x2 + &b;
    //     n.set_label("n".to_string());
    //
    //     let mut o = n.tanh();
    //     o.set_label("o".to_string());
    //
    //     o.backward();
    //
    //     println!("x1 grad  expected {},  actual {}", -1.5, x1.get_grad());
    //     println!("w1 grad  expected {},  actual {}", 1.0, w1.get_grad());
    //     assert_two_float(x1.get_grad(), -1.5);
    //     assert_two_float(w1.get_grad(), 1.0);
    //
    //     println!("x2 grad  expected {},  actual {}", 0.5, x2.get_grad());
    //     println!("w2 grad  expected {},  actual {}", 0.0, w2.get_grad());
    //     assert_two_float(x2.get_grad(), 0.5);
    //     assert_two_float(w2.get_grad(), 0.0);
    //
    //     println!("w1x1 data  expected {},  actual {}", -6.0, w1x1.get_data());
    //     println!("w2x2 data  expected {},  actual {}", 0.0, w2x2.get_data());
    //
    //     assert_two_float(w1x1.get_data(), -6.0);
    //     assert_two_float(w2x2.get_data(), 0.0);
    //
    //     println!("w1x1 grad  expected {},  actual {}", 0.5, w1x1.get_grad());
    //     println!("w2x2 grad  expected {},  actual {}", 0.5, w2x2.get_grad());
    //
    //     assert_two_float(w1x1.get_grad(), 0.5);
    //     assert_two_float(w2x2.get_grad(), 0.5);
    //
    //     println!("b  data  expected {},  actual {}", 6.881373587019, b.get_data());
    //     println!("b  grad  expected {},  actual {}", 0.5, b.get_grad());
    //
    //     assert_two_float(b.get_data(), 6.881373587019);
    //     assert_two_float(b.get_grad(), 0.5);
    //
    //     println!(
    //         "w1x1_plus_w2x2 data  expected {},  actual {}",
    //         -6.,
    //         w1x1_plus_w2x2.get_data()
    //     );
    //     println!(
    //         "w1x1_plus_w2x2 grad  expected {},  actual {}",
    //         0.5,
    //         w1x1_plus_w2x2.get_grad()
    //     );
    //
    //     assert_two_float(w1x1_plus_w2x2.get_data(), -6.0);
    //     assert_two_float(w1x1_plus_w2x2.get_grad(), 0.5);
    //
    //     println!("n data  expected {},  actual {}", 0.8814, n.get_data());
    //     println!("n grad  expected {},  actual {}", 0.5, n.get_grad());
    //
    //     assert_two_float(n.get_data(), 0.8814);
    //     assert_two_float(n.get_grad(), 0.5);
    //
    //     println!("o data  expected {},  actual {}", SQRT_2 / 2.0, o.get_data());
    //     println!("o grad  expected {},  actual {}", 1.0, o.get_grad());
    //
    //     assert_two_float(o.get_data(), SQRT_2 / 2.0);
    //     assert_two_float(o.get_grad(), 1.0);
    // }
    //
    // #[test]
    // pub fn test_add_same_variable() {
    //     let a = -5.0_f64;
    //     let expected = a + a;
    //     let a_grad_expected = 2.0;
    //     let a = ValueRefV3::new_value(-5.0, "a".to_string());
    //     let mut c = &a + &a;
    //     c.backward();
    //
    //     println!("a.grad   expected {},   actual {}", a_grad_expected, a.get_grad());
    //     assert_two_float(c.get_data(), expected);
    //     assert_two_float(a.get_grad(), a_grad_expected);
    // }
    //
    // #[test]
    // pub fn test_mul_same_variable() {
    //     let a = -5.0_f64;
    //     let expected = a * a;
    //     let a_grad_expected = -10.0;
    //     let a = ValueRefV3::new_value(a, "a".to_string());
    //     let mut c = &a * &a;
    //     c.backward();
    //
    //     println!("a.grad   expected {},   actual {}", a_grad_expected, a.get_grad());
    //     assert_two_float(c.get_data(), expected);
    //     assert_two_float(a.get_grad(), a_grad_expected);
    // }
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
    //     assert_two_float(c.get_data(), expected);
    //     assert_two_float(a.get_grad(), a_grad_expected);
    //     assert_two_float(b.get_grad(), b_grad_expected);
    //
    //     let topo = ValueRefV3::traverse(&c);
    //     topo.iter().for_each(|t| println!("topo   {}", t));
    //     assert_eq!(topo.len(), 3);
    // }
    //
    // // see micrograd_test_topo.py
    // // he uses a set for the children and in case of an c = a + a
    // // the variable a is only once in the children set
    //
    // #[test]
    // pub fn test_mul_same_variable_topo() {
    //     let a = -5.0_f64;
    //     let expected = a * a;
    //     let a_grad_expected = -10.0;
    //     let a = ValueRefV3::new_value(a, "a".to_string());
    //     let mut c = &a * &a;
    //     let topo = ValueRefV3::traverse(&c);
    //     topo.iter().for_each(|t| println!("topo   {}", t));
    //
    //     assert_eq!(topo.len(), 2);
    //
    //     c.backward();
    //
    //     println!("a.grad   expected {},   actual {}", a_grad_expected, a.get_grad());
    //     assert_two_float(c.get_data(), expected);
    //     assert_two_float(a.get_grad(), a_grad_expected);
    // }
    //
    // #[test]
    // pub fn test_neg_grad() {
    //     let a = -5.0_f64;
    //     let expected = -a;
    //     let a_grad_expected = -1.0;
    //     let a = ValueRefV3::new_value(a, "a".to_string());
    //     let mut c = -&a;
    //     c.backward();
    //
    //     println!("a.grad   expected {},   actual {}", a_grad_expected, a.get_grad());
    //     assert_two_float(c.get_data(), expected);
    //     assert_two_float(a.get_grad(), a_grad_expected);
    // }
    //
    // #[test]
    // pub fn test_neg_grad1() {
    //     let a = ValueRefV3::new_value(-4.0, "a".to_string()); //         a = Value(-4.0)
    //     let b = ValueRefV3::new_value(2.0, "b".to_string()); //         b = Value(2.0)
    //     let mut c = &a + &b; //         c = a + b
    //     c += &c + 1.0; //         c += c + 1
    //     c += (1.0 + &c) + (-&a); //         c += 1 + c + (-a)
    //
    //     c.backward();
    //
    //     let a_expected = -4.0;
    //     let b_expected = 2.0;
    //     let c_expected = -1.0;
    //
    //     let a_grad_expected = 3.0;
    //     let b_grad_expected = 4.0;
    //     let c_grad_expected = 1.0;
    //
    //     println!("a data {}   expected {}", a.get_data(), a_expected);
    //     println!("b data {}   expected {}", b.get_data(), b_expected);
    //     println!("c data {}   expected {}", c.get_data(), c_expected);
    //
    //     println!("a grad {}   expected {}", a.get_grad(), a_grad_expected);
    //     println!("a grad {}   expected {}", b.get_grad(), b_grad_expected);
    //     println!("a grad {}   expected {}", c.get_grad(), c_grad_expected);
    //
    //     assert_two_float(a.get_data(), a_expected);
    //     assert_two_float(b.get_data(), b_expected);
    //     assert_two_float(c.get_data(), c_expected);
    //
    //     assert_two_float(a.get_grad(), a_grad_expected);
    //     assert_two_float(b.get_grad(), b_grad_expected);
    //     assert_two_float(c.get_grad(), c_grad_expected);
    // }
    //
    // #[test]
    // pub fn test_sub_relu_grad() {
    //     let a = ValueRefV3::new_value(-4.0, "a".to_string()); //         a = Value(-4.0)
    //     let b = ValueRefV3::new_value(2.0, "b".to_string()); //         b = Value(2.0)
    //     let mut c = (&b - &a).relu();
    //
    //     c.backward();
    //
    //     let topo = ValueRefV3::traverse(&c);
    //     println!("#################################");
    //     println!("Topo");
    //     for t in topo.iter() {
    //         println!(
    //             "{}  data {},  grad  {}",
    //             t.r.borrow().label(),
    //             t.get_data(),
    //             t.get_grad()
    //         );
    //     }
    //     println!("#################################");
    //
    //     let a_expected = -4.0;
    //     let b_expected = 2.0;
    //     let c_expected = 6.0;
    //
    //     let a_grad_expected = -1.0;
    //     let b_grad_expected = 1.0;
    //     let c_grad_expected = 1.0;
    //
    //     println!("a data {}   expected {}", a.get_data(), a_expected);
    //     println!("b data {}   expected {}", b.get_data(), b_expected);
    //     println!("c data {}   expected {}", c.get_data(), c_expected);
    //
    //     println!("a grad {}   expected {}", a.get_grad(), a_grad_expected);
    //     println!("b grad {}   expected {}", b.get_grad(), b_grad_expected);
    //     println!("c grad {}   expected {}", c.get_grad(), c_grad_expected);
    //
    //     assert_two_float(a.get_data(), a_expected);
    //     assert_two_float(b.get_data(), b_expected);
    //     assert_two_float(c.get_data(), c_expected);
    //
    //     assert_two_float(a.get_grad(), a_grad_expected);
    //     assert_two_float(b.get_grad(), b_grad_expected);
    //     assert_two_float(c.get_grad(), c_grad_expected);
    // }
}
