use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use std::rc::Rc;

#[derive(PartialEq, Eq)]
pub enum OpEnumV2 {
    NONE,
    ADD,
    SUB,
    NEG,
    MUL,
    TANH,
    EXP,
    DIV,
    POW,
}

trait Backward {
    fn apply(&self, out: ValueRefV2, children: &[ValueRefV2]);
}

struct BackwardAdd {}

impl Backward for BackwardAdd {
    fn apply(&self, out: ValueRefV2, children: &[ValueRefV2]) {
        // println!(
        //     "ADD out {:?},  'self' {:?}, 'other' {:?}",
        //     out, children[0], children[1]
        // );
        // println!("backward add ");
        let mut self__ = children[0].clone();
        let mut other = children[1].clone();

        self__.set_grad(self__.get_grad() + 1.0 * out.r.borrow().grad());
        other.set_grad(other.get_grad() + 1.0 * out.r.borrow().grad());
    }
}

struct BackwardSub {}

impl Backward for BackwardSub {
    fn apply(&self, out: ValueRefV2, children: &[ValueRefV2]) {
        // println!(
        //     "SUB out {:?},  'self' {:?}, 'other' {:?}",
        //     out, children[0], children[1]
        // );
        // println!("backward sub ");
        let mut self__ = children[0].clone();
        let mut other = children[1].clone();

        self__.set_grad(self__.get_grad() + 1.0 * out.r.borrow().grad());
        other.set_grad(other.get_grad() - 1.0 * out.r.borrow().grad());
    }
}

struct BackwardMul {}

impl Backward for BackwardMul {
    fn apply(&self, out: ValueRefV2, children: &[ValueRefV2]) {
        // println!(
        //     "MUL out {:?},  'self' {:?}, 'other' {:?}",
        //     out, children[0], children[1]
        // );

        let mut self__ = children[0].clone();
        let mut other = children[1].clone();

        let x = other.borrow().data();
        self__.set_grad(self__.get_grad() + x * out.r.borrow().grad());
        let x1 = self__.borrow().data();
        other.set_grad(other.get_grad() + x1 * out.r.borrow().grad());
        // println!("backward mul ");
    }
}

struct BackwardTanh {}

impl Backward for BackwardTanh {
    fn apply(&self, out: ValueRefV2, children: &[ValueRefV2]) {
        // println!("TANH out {:?},  'self' {:?} ", out, children[0]);
        let mut self__ = children[0].clone();
        let x = out.get_data();
        let y = 1.0 - x * x;
        self__.set_grad(self__.get_grad() + y * out.r.borrow().grad());
    }
}

struct BackwardExp {}

impl Backward for BackwardExp {
    fn apply(&self, out: ValueRefV2, children: &[ValueRefV2]) {
        // println!("EXP out {:?},  'self' {:?} ", out, children[0]);
        let mut self__ = children[0].clone();
        self__.set_grad(self__.get_grad() + out.r.borrow().data() * out.r.borrow().grad());
    }
}

struct BackwardPow {}

impl Backward for BackwardPow {
    fn apply(&self, out: ValueRefV2, children: &[ValueRefV2]) {
        // println!("POW out {:?},  'self' {:?} ", out, children[0]);
        let mut self__ = children[0].clone();
        let other = children[1].clone().borrow().data;
        let x = other * (self__.borrow().data().powf(other - 1.0)) * out.r.borrow().grad();
        self__.set_grad(self__.get_grad() + x);
    }
}

struct BackwardReLU {}

impl Backward for BackwardReLU {
    fn apply(&self, out: ValueRefV2, children: &[ValueRefV2]) {
        // println!("ReLU out {:?},  'self' {:?} ", out, children[0]);
        let mut self__ = children[0].clone();
        let x = if out.get_data() > 0.0 { out.get_grad() } else { 0.0 };
        self__.set_grad(self__.get_grad() + x);
    }
}

//
// struct BackwardDiv {}
//
// impl Backward for BackwardDiv {
//     fn apply(&self, out: ValueRefV2, children: &Vec<ValueRefV2>) {
//         // // println!("EXP a {:?}children [0] {:?} ", out, children[0]);
//         // let mut self__ = children[0].clone();
//         // let x = out.get_data();
//         // let y = 1.0 - x * x;
//         // self__.set_grad(y * out.r.borrow().grad());
//     }
// }

pub struct ValueV2 {
    data: f64,
    grad: f64,
    op: OpEnumV2,
    children: Vec<ValueRefV2>,
    label: String,
    backward: Option<Box<dyn Backward>>,
}

impl PartialEq for ValueRefV2 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.r, &other.r)
    }
}

impl Hash for ValueRefV2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.r).hash(state)
    }
}

impl Eq for ValueRefV2 {}

#[derive(Clone)]
pub struct ValueRefV2 {
    r: Rc<RefCell<ValueV2>>,
}

impl ValueRefV2 {
    pub fn new(v: ValueV2) -> Self {
        ValueRefV2 {
            r: Rc::new(RefCell::new(v)),
        }
    }

    pub fn new_value(v: f64, label: String) -> ValueRefV2 {
        let v = ValueV2::new_value(v, label);
        ValueRefV2 {
            r: Rc::new(RefCell::new(v)),
        }
    }

    // convenience methods to simplify code, to avoid value.r.borrow()
    pub fn borrow(&self) -> Ref<ValueV2> {
        self.r.borrow()
    }

    pub fn set_label(&mut self, label: String) {
        self.r.borrow_mut().set_label(label);
    }

    pub fn set_grad(&mut self, grad: f64) {
        self.r.borrow_mut().set_grad(grad);
    }

    pub fn set_data(&mut self, data: f64) {
        self.r.borrow_mut().set_data(data);
    }

    pub fn get_grad(&self) -> f64 {
        self.r.borrow().grad()
    }

    pub fn get_data(&self) -> f64 {
        self.r.borrow().data()
    }

    pub fn tanh(&self) -> ValueRefV2 {
        let x = self.r.borrow().data();
        let y = ((2.0_f64 * x).exp() - 1.0) / ((2.0 * x).exp() + 1.0);

        let v = ValueV2 {
            data: y,
            op: OpEnumV2::TANH,
            children: vec![self.clone()],
            label: format!("tanh({})", self.borrow().label),
            grad: 0.0,
            backward: Some(Box::new(BackwardTanh {})),
        };

        ValueRefV2::new(v)
    }

    pub fn exp(&self) -> ValueRefV2 {
        let x = self.r.borrow().data();
        let y = x.exp();

        let v = ValueV2 {
            data: y,
            op: OpEnumV2::EXP,
            children: vec![self.clone()],
            label: format!("exp({})", self.borrow().label),
            grad: 0.0,
            backward: Some(Box::new(BackwardExp {})),
        };

        ValueRefV2::new(v)
    }

    pub fn pow(&self, f: f64) -> ValueRefV2 {
        let string = "pow".to_string();
        let r = ValueRefV2::new(ValueV2::new(f, OpEnumV2::NONE, string));
        let x = self.r.borrow().data();
        let y = x.powf(f);

        let v = ValueV2 {
            data: y,
            op: OpEnumV2::POW,
            children: vec![self.clone(), r],
            label: format!("{}.pow({})", self.borrow().label, f),
            grad: 0.0,
            backward: Some(Box::new(BackwardPow {})),
        };

        ValueRefV2::new(v)
    }

    pub fn relu(&self) -> ValueRefV2 {
        let x = self.r.borrow().data();
        let y = if x < 0.0 { 0.0 } else { x };

        let v = ValueV2 {
            data: y,
            op: OpEnumV2::POW,
            children: vec![self.clone()],
            label: format!("relu({})", self.borrow().label),
            grad: 0.0,
            backward: Some(Box::new(BackwardReLU {})),
        };

        ValueRefV2::new(v)
    }

    pub fn backward(&mut self) {
        self.set_grad(1.0);
        let topo = Self::traverse(self);
        // println!("topo size   {} ", topo.len());
        for n in topo.iter().rev() {
            match &n.r.borrow().backward {
                Some(backward) => {
                    backward.apply(n.clone(), &n.borrow().children);
                }
                None => {}
            }
        }
    }

    pub fn traverse(o: &ValueRefV2) -> Vec<ValueRefV2> {
        let mut topo = vec![];
        let mut visited = HashSet::new();

        Self::build_topo(o, &mut topo, &mut visited);

        topo
    }

    fn build_topo(v: &ValueRefV2, topo: &mut Vec<ValueRefV2>, visited: &mut HashSet<ValueRefV2>) {
        if !visited.contains(v) {
            visited.insert(v.clone());
            for child in v.borrow().children() {
                Self::build_topo(child, topo, visited);
            }
            topo.push(v.clone());
        }
    }
}

impl ValueV2 {
    pub fn new(data: f64, op: OpEnumV2, label: String) -> Self {
        ValueV2 {
            data,
            op,
            children: vec![],
            label,
            grad: 0.0,
            backward: None,
        }
    }

    pub fn new_value(data: f64, label: String) -> Self {
        ValueV2 {
            data,
            op: OpEnumV2::NONE,
            children: vec![],
            label,
            grad: 0.0,
            backward: None,
        }
    }

    pub fn data(&self) -> f64 {
        self.data
    }

    pub fn op(&self) -> &OpEnumV2 {
        &self.op
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn children(&self) -> &Vec<ValueRefV2> {
        &self.children
    }

    pub fn grad(&self) -> f64 {
        self.grad
    }

    pub fn set_grad(&mut self, grad: f64) {
        self.grad = grad;
    }

    pub fn set_data(&mut self, data: f64) {
        self.data = data;
    }
}

impl Add<&ValueRefV2> for &ValueRefV2 {
    type Output = ValueRefV2;

    fn add(self, rhs: &ValueRefV2) -> Self::Output {
        let x1 = self.r.borrow();
        let x2 = rhs.r.borrow();
        let out = ValueV2 {
            data: x1.data + x2.data,
            op: OpEnumV2::ADD,
            children: vec![self.clone(), rhs.clone()],
            label: format!("{} + {}", self.borrow().label, rhs.borrow().label),
            grad: 0.0,
            backward: Some(Box::new(BackwardAdd {})),
        };
        ValueRefV2::new(out)
    }
}

impl Add for ValueRefV2 {
    type Output = ValueRefV2;

    fn add(self, rhs: ValueRefV2) -> Self::Output {
        let x1 = self.r.borrow();
        let x2 = rhs.r.borrow();
        let out = ValueV2 {
            data: x1.data + x2.data,
            op: OpEnumV2::ADD,
            children: vec![self.clone(), rhs.clone()],
            label: format!("{} + {}", self.borrow().label, rhs.borrow().label),
            grad: 0.0,
            backward: Some(Box::new(BackwardAdd {})),
        };
        ValueRefV2::new(out)
    }
}

impl AddAssign for ValueRefV2 {
    fn add_assign(&mut self, rhs: ValueRefV2) {
        *self = self.clone() + rhs
    }
}

impl SubAssign for ValueRefV2 {
    fn sub_assign(&mut self, rhs: ValueRefV2) {
        *self = self.clone() - rhs
    }
}

impl Sub for &ValueRefV2 {
    type Output = ValueRefV2;

    fn sub(self, rhs: &ValueRefV2) -> Self::Output {
        let x1 = self.r.borrow();
        let x2 = rhs.r.borrow();
        let out = ValueV2 {
            data: x1.data - x2.data,
            op: OpEnumV2::SUB,
            children: vec![self.clone(), rhs.clone()],
            label: format!("{} - {}", self.borrow().label, rhs.borrow().label),
            grad: 0.0,
            backward: Some(Box::new(BackwardSub {})),
        };
        ValueRefV2::new(out)
    }
}

impl Sub for ValueRefV2 {
    type Output = ValueRefV2;

    fn sub(self, rhs: ValueRefV2) -> Self::Output {
        let x1 = self.r.borrow();
        let x2 = rhs.r.borrow();
        let out = ValueV2 {
            data: x1.data - x2.data,
            op: OpEnumV2::SUB,
            children: vec![self.clone(), rhs.clone()],
            label: format!("{} - {}", self.borrow().label, rhs.borrow().label),
            grad: 0.0,
            backward: Some(Box::new(BackwardSub {})),
        };
        ValueRefV2::new(out)
    }
}

impl Mul<&ValueRefV2> for &ValueRefV2 {
    type Output = ValueRefV2;

    fn mul(self, rhs: &ValueRefV2) -> Self::Output {
        let x1 = self.r.borrow();
        let x2 = rhs.r.borrow();
        let out = ValueV2 {
            data: x1.data * x2.data,
            op: OpEnumV2::MUL,
            children: vec![self.clone(), rhs.clone()],
            label: format!("{} * {}", self.borrow().label, rhs.borrow().label),
            grad: 0.0,
            backward: Some(Box::new(BackwardMul {})),
        };
        ValueRefV2::new(out)
    }
}

impl Div<&ValueRefV2> for &ValueRefV2 {
    type Output = ValueRefV2;

    fn div(self, rhs: &ValueRefV2) -> Self::Output {
        self * &rhs.pow(-1.0)
    }
}

impl Add<f64> for &ValueRefV2 {
    type Output = ValueRefV2;

    fn add(self, rhs: f64) -> Self::Output {
        let string = "f64 add".to_string();
        let r = ValueRefV2::new_value(rhs, string.clone());
        let x1 = self.r.borrow();
        let out = ValueV2 {
            data: x1.data + rhs,
            op: OpEnumV2::ADD,
            children: vec![self.clone(), r],
            label: format!("{} + {}", self.borrow().label, string),
            grad: 0.0,
            backward: Some(Box::new(BackwardAdd {})),
        };
        ValueRefV2::new(out)
    }
}

impl Add<&ValueRefV2> for f64 {
    type Output = ValueRefV2;

    fn add(self, rhs: &ValueRefV2) -> Self::Output {
        let string = "f64 add".to_string();
        let r = ValueRefV2::new_value(self, string.clone());
        let x1 = rhs.r.borrow();
        let out = ValueV2 {
            data: x1.data + self,
            op: OpEnumV2::ADD,
            children: vec![r, rhs.clone()],
            label: format!("{} + {}", string, rhs.borrow().label),
            grad: 0.0,
            backward: Some(Box::new(BackwardAdd {})),
        };
        ValueRefV2::new(out)
    }
}

impl Sub<f64> for &ValueRefV2 {
    type Output = ValueRefV2;

    fn sub(self, rhs: f64) -> Self::Output {
        let string = "f64 sub".to_string();
        let r = ValueRefV2::new_value(rhs, string.clone());
        let x1 = self.r.borrow();
        let out = ValueV2 {
            data: x1.data - rhs,
            op: OpEnumV2::SUB,
            children: vec![self.clone(), r],
            label: format!("{} + {}", self.borrow().label, string),
            grad: 0.0,
            backward: Some(Box::new(BackwardSub {})),
        };
        ValueRefV2::new(out)
    }
}

impl Sub<&ValueRefV2> for f64 {
    type Output = ValueRefV2;

    fn sub(self, rhs: &ValueRefV2) -> Self::Output {
        let string = "f64 sub".to_string();
        let r = ValueRefV2::new_value(self, string.clone());
        let x1 = rhs.r.borrow();
        let out = ValueV2 {
            data: x1.data - self,
            op: OpEnumV2::SUB,
            children: vec![r, rhs.clone()],
            label: format!("{} - {}", string, rhs.borrow().label),
            grad: 0.0,
            backward: Some(Box::new(BackwardSub {})),
        };
        ValueRefV2::new(out)
    }
}

impl Mul<f64> for &ValueRefV2 {
    type Output = ValueRefV2;

    fn mul(self, rhs: f64) -> Self::Output {
        let string = "f64 mul".to_string();
        let r = ValueRefV2::new_value(rhs, string.clone());
        let x1 = self.r.borrow();
        let out = ValueV2 {
            data: x1.data * rhs,
            op: OpEnumV2::MUL,
            children: vec![self.clone(), r],
            label: format!("{} + {}", self.borrow().label, string),
            grad: 0.0,
            backward: Some(Box::new(BackwardMul {})),
        };
        ValueRefV2::new(out)
    }
}

impl Mul<f64> for ValueRefV2 {
    type Output = ValueRefV2;

    fn mul(self, rhs: f64) -> Self::Output {
        let string = "f64 mul".to_string();
        let r = ValueRefV2::new_value(rhs, string.clone());
        let x1 = self.r.borrow();
        let out = ValueV2 {
            data: x1.data * rhs,
            op: OpEnumV2::MUL,
            children: vec![self.clone(), r],
            label: format!("{} + {}", self.borrow().label, string),
            grad: 0.0,
            backward: Some(Box::new(BackwardMul {})),
        };
        ValueRefV2::new(out)
    }
}

impl Mul<&ValueRefV2> for f64 {
    type Output = ValueRefV2;

    fn mul(self, rhs: &ValueRefV2) -> Self::Output {
        let string = "f64 mul".to_string();
        let r = ValueRefV2::new_value(self, string.clone());
        let x1 = rhs.r.borrow();
        let out = ValueV2 {
            data: x1.data * self,
            op: OpEnumV2::MUL,
            children: vec![r, rhs.clone()],
            label: format!("{} + {}", string, rhs.borrow().label),
            grad: 0.0,
            backward: Some(Box::new(BackwardMul {})),
        };
        ValueRefV2::new(out)
    }
}

impl Div<f64> for ValueRefV2 {
    type Output = ValueRefV2;

    fn div(self, rhs: f64) -> Self::Output {
        self * rhs.powf(-1.0)
        // let string = "f64 div".to_string();
        // let r = ValueRefV2::new_value(rhs, string.clone());
        // let x1 = self.r.borrow();
        // let out = ValueV2 {
        //     data: x1.data * rhs.powf(-1.0),
        //     op: OpEnumV2::DIV,
        //     children: vec![self.clone(), r],
        //     label: format!("{} / {}", self.borrow().label, string),
        //     grad: 0.0,
        //     backward: Some(Box::new(BackwardDiv {})),
        // };
        // ValueRefV2::new(out)
    }
}

impl Div<f64> for &ValueRefV2 {
    type Output = ValueRefV2;

    fn div(self, rhs: f64) -> Self::Output {
        self * rhs.powf(-1.0)
        // let string = "f64 div".to_string();
        // let r = ValueRefV2::new_value(rhs, string.clone());
        // let x1 = self.r.borrow();
        // let out = ValueV2 {
        //     data: x1.data  * rhs.powf(-1.0),
        //     op: OpEnumV2::DIV,
        //     children: vec![self.clone(), r],
        //     label: format!("{} / {}", self.borrow().label, string),
        //     grad: 0.0,
        //     backward: Some(Box::new(BackwardDiv {})),
        // };
        // ValueRefV2::new(out)
    }
}

impl Div<ValueRefV2> for f64 {
    type Output = ValueRefV2;

    fn div(self, rhs: ValueRefV2) -> Self::Output {
        self * &rhs.pow(-1.0)
        // let string = "f64 div".to_string();
        // let r = ValueRefV2::new_value(self, string.clone());
        // let x1 = rhs.r.borrow();
        // let out = ValueV2 {
        //     data: self  * x1.data.powf(-1.0),
        //     op: OpEnumV2::DIV,
        //     children: vec![r, rhs.clone()],
        //     label: format!("{} + {}", string, rhs.borrow().label),
        //     grad: 0.0,
        //     backward: Some(Box::new(BackwardDiv {})),
        // };
        // ValueRefV2::new(out)
    }
}

impl Div<&ValueRefV2> for f64 {
    type Output = ValueRefV2;

    fn div(self, rhs: &ValueRefV2) -> Self::Output {
        self * &rhs.pow(-1.0)
        // let string = "f64 div".to_string();
        // let r = ValueRefV2::new_value(self, string.clone());
        // let x1 = rhs.r.borrow();
        // let out = ValueV2 {
        //     data: self * x1.data.powf(-1.0),
        //     op: OpEnumV2::DIV,
        //     children: vec![r, rhs.clone()],
        //     label: format!("{} + {}", string, rhs.borrow().label),
        //     grad: 0.0,
        //     backward: Some(Box::new(BackwardDiv {})),
        // };
        // ValueRefV2::new(out)
    }
}

impl Neg for ValueRefV2 {
    type Output = ValueRefV2;

    fn neg(self) -> Self::Output {
        self * (-1.0)
    }
}

impl Neg for &ValueRefV2 {
    type Output = ValueRefV2;

    fn neg(self) -> Self::Output {
        self * (-1.0)
    }
}

impl Display for OpEnumV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpEnumV2::ADD => write!(f, "+"),
            OpEnumV2::NONE => write!(f, ""),
            OpEnumV2::MUL => write!(f, "*"),
            OpEnumV2::TANH => write!(f, "tanh"),
            OpEnumV2::EXP => write!(f, "exp"),
            OpEnumV2::DIV => write!(f, "/"),
            OpEnumV2::POW => write!(f, "^"),
            OpEnumV2::SUB => write!(f, "-"),
            OpEnumV2::NEG => write!(f, "NEG"),
        }
    }
}

impl Default for ValueV2 {
    fn default() -> Self {
        ValueV2 {
            data: 0.0,
            op: OpEnumV2::NONE,
            children: vec![],
            grad: 0.0,
            label: "default".to_string(),
            backward: None,
        }
    }
}

impl Default for ValueRefV2 {
    fn default() -> Self {
        ValueRefV2::new(ValueV2::default())
    }
}

impl Display for ValueV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.label, self.data)
    }
}

impl Display for ValueRefV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.r.borrow().label(), self.r.borrow().data())
    }
}

impl Debug for ValueRefV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.r.borrow())
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_float, EPS};
    use std::f64::consts::SQRT_2;

    use crate::graph_v2::draw_graph;
    use crate::micrograd_rs_v2::ValueRefV2;

    // before starting to add grad
    // https://youtu.be/VMj-3S1tku0?t=1875
    #[test]
    pub fn test_video() {
        let a = ValueRefV2::new_value(2.0, "a".to_string());
        let b = ValueRefV2::new_value(-3.0, "b".to_string());
        let c = ValueRefV2::new_value(10.0, "c".to_string());
        let f = ValueRefV2::new_value(-2.0, "f".to_string());

        let mut e = &a * &b;
        e.set_label("e".to_string());

        let mut d = &e + &c;
        d.set_label("d".to_string());

        let mut l = &d * &f;
        l.set_label("L".to_string());

        assert_float(l.borrow().data, -8.0);
    }

    #[test]
    pub fn test_add() {
        let a = ValueRefV2::new_value(2.0, "a".to_string());
        let b = ValueRefV2::new_value(3.0, "b".to_string());

        let mut x = &a + &b;
        x.set_label("x".to_string());

        assert_float(x.borrow().data, 5.0);
    }

    #[test]
    pub fn test_mul() {
        let a = ValueRefV2::new_value(2.0, "a".to_string());
        let b = ValueRefV2::new_value(3.0, "b".to_string());

        let mut x = &a * &b;
        x.set_label("x".to_string());

        assert_float(x.borrow().data, 6.0);
    }

    // https://youtu.be/VMj-3S1tku0?t=4977
    #[test]
    pub fn test_a_plus_a() {
        let a = ValueRefV2::new_value(3.0, "a".to_string());
        let mut b = &a + &a;

        b.set_label("b".to_string());

        b.backward();

        let topo = ValueRefV2::traverse(&b);
        for t in topo.iter() {
            println!("topo  {:?}", t);
        }

        println!("b {}", b);

        assert_float(b.borrow().data, 6.0);

        draw_graph(b, "test_a_plus_a".to_string());

        assert_float(a.get_grad(), 2.0);
    }

    #[test]
    pub fn test_value_plus_f64_rhs() {
        let a = ValueRefV2::new_value(3.0, "a".to_string());
        let mut b = &a + 1.0;
        assert_float(b.borrow().data, 4.0);
        b.backward();
        draw_graph(b, "test_a_plus_f64_rhs".to_string());
    }

    #[test]
    pub fn test_value_plus_f64_lhs() {
        let a = ValueRefV2::new_value(4.0, "a".to_string());
        let mut b = 23.0 + &a;
        b.backward();
        assert_float(b.borrow().data, 27.0);

        draw_graph(b, "test_a_plus_f64_lhs".to_string());
    }

    #[test]
    pub fn test_value_mul_f64_rhs() {
        let a = ValueRefV2::new_value(3.0, "a".to_string());
        let mut b = &a * 3.0;
        assert_float(b.borrow().data, 9.0);
        b.backward();
        draw_graph(b, "test_a_mul_f64_rhs".to_string());
    }

    #[test]
    pub fn test_value_mul_f64_lhs() {
        let a = ValueRefV2::new_value(4.0, "a".to_string());
        let mut b = 23.0 * &a;
        b.backward();
        assert_float(b.borrow().data, 92.0);

        draw_graph(b, "test_a_mul_f64_lhs".to_string());
    }

    #[test]
    pub fn test_value_div() {
        let a = ValueRefV2::new_value(4.0, "a".to_string());
        let b = ValueRefV2::new_value(2.0, "b".to_string());
        let mut c = &a / &b;
        c.backward();
        assert_float(c.borrow().data, 2.0);

        draw_graph(c, "test_a_div_b".to_string());
    }

    #[test]
    pub fn test_value_exp() {
        let expected = (4.0 as f64).exp();
        let a = ValueRefV2::new_value(4.0, "a".to_string());
        let mut b = a.exp();

        b.backward();
        assert_float(b.borrow().data, expected);

        draw_graph(b, "test_exp_4".to_string());
    }

    #[test]
    pub fn test_value_pow() {
        let expected = (4.0 as f64).powf(3.0);
        let a = ValueRefV2::new_value(4.0, "a".to_string());
        let b = 3.0;
        let mut b = a.pow(b);

        b.backward();
        assert_float(b.borrow().data, expected);

        draw_graph(b, "test_pow".to_string());
    }

    #[test]
    pub fn test_value_relu() {
        let expected = 4.0 - 23.0;
        let a = ValueRefV2::new_value(4.0, "a".to_string());
        let b = ValueRefV2::new_value(23.0, "b".to_string());
        let mut c = &a - &b;
        c.set_label("c".to_string());

        c.backward();
        assert_float(c.borrow().data, expected);

        draw_graph(c, "test_sub".to_string());
    }

    #[test]
    pub fn test_value_sub() {
        let expected = 4.0 - 23.0;
        let a = ValueRefV2::new_value(4.0, "a".to_string());
        let b = ValueRefV2::new_value(23.0, "b".to_string());
        let mut c = &a - &b;
        c.set_label("c".to_string());

        c.backward();
        assert_float(c.borrow().data, expected);

        draw_graph(c, "test_sub".to_string());
    }

    // https://github.com/karpathy/micrograd
    #[test]
    pub fn test_grad() {
        let mut a = ValueRefV2::new_value(-4.0, "a".to_string()); //         a = Value(-4.0)
        let mut b = ValueRefV2::new_value(2.0, "b".to_string()); //         b = Value(2.0)
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

        let topo = ValueRefV2::traverse(&g);

        println!("#################################");
        println!("Topo");
        for t in topo.iter() {
            println!(
                "{}  data {},  grad  {}",
                t.r.borrow().label(),
                t.get_data(),
                t.get_grad()
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

        println!(
            "value g expected {}, actual {}    assert {}",
            g_value_expected,
            g.get_data(),
            (g_value_expected - g.get_data()).abs() < EPS
        );
        assert_float(g_value_expected, g.get_data());

        println!("grad f expected {}, actual {}   ", f_grad_expected, f.get_grad());
        println!("grad e expected {}, actual {}   ", e_grad_expected, e.get_grad());
        println!("grad d expected {}, actual {}   ", d_grad_expected, d.get_grad());
        println!("grad c expected {}, actual {}   ", c_grad_expected, c.get_grad());
        println!("grad b expected {}, actual {}   ", b_grad_expected, b.get_grad());
        println!("grad a expected {}, actual {}   ", a_grad_expected, a.get_grad());

        assert_float(f_grad_expected, f.get_grad());
        assert_float(e_grad_expected, e.get_grad());
        assert_float(d_grad_expected, d.get_grad());
        assert_float(c_grad_expected, c.get_grad());
        assert_float(b_grad_expected, b.get_grad());
        assert_float(a_grad_expected, a.get_grad());

        // let topo = ValueRefV2::traverse(&g);
        //
        // println!("Topo");
        // for t in topo.iter() {
        //     println!("{}", t);
        // }
        // draw_graph(g, "test_all_math_ops_graph".to_string());
    }

    #[test]
    pub fn test_grad_add() {
        let a = 23.0;
        let b = 45.0;
        let expected = a + b;

        let a = ValueRefV2::new_value(a, "a".to_string());
        let b = ValueRefV2::new_value(b, "b".to_string());
        let mut c = &a + &b;
        c.set_label("c".to_string());

        c.backward();
        assert_float(c.borrow().data, expected);

        assert_float(a.get_grad(), 1.0);
        assert_float(b.get_grad(), 1.0);
    }

    #[test]
    pub fn test_grad_sub() {
        let a = 23.0;
        let b = 45.0;
        let expected = a - b;

        let a = ValueRefV2::new_value(a, "a".to_string());
        let b = ValueRefV2::new_value(b, "b".to_string());
        let mut c = &a - &b;
        c.set_label("c".to_string());

        c.backward();

        println!("c actual  {}  expected {}", c.borrow().data, expected);
        println!("a.grad  {} ", a.get_grad());
        println!("b.grad  {} ", b.get_grad());
        assert_float(c.borrow().data, expected);

        assert_float(a.get_grad(), 1.0);
        assert_float(b.get_grad(), -1.0);
    }

    #[test]
    pub fn test_grad_mul() {
        let a_f64 = 12.0;
        let b_f64 = 23.0;
        let expected = a_f64 * b_f64;

        let a = ValueRefV2::new_value(a_f64, "a".to_string());
        let b = ValueRefV2::new_value(b_f64, "b".to_string());
        let mut c = &a * &b;
        c.set_label("c".to_string());

        c.backward();

        println!("c actual  {}  expected {}", c.borrow().data, expected);
        println!("a.grad  {} ", a.get_grad());
        println!("b.grad  {} ", b.get_grad());
        assert_float(c.borrow().data, expected);

        assert_float(a.get_grad(), b_f64);
        assert_float(b.get_grad(), a_f64);
    }

    #[test]
    pub fn test_grad_div() {
        let a_f64 = 7.0;
        let b_f64 = 2.0;
        let expected = a_f64 / b_f64;

        let a = ValueRefV2::new_value(a_f64, "a".to_string());
        let b = ValueRefV2::new_value(b_f64, "b".to_string());
        let mut c = &a / &b;
        c.set_label("c".to_string());

        c.backward();

        println!("c actual  {}  expected {}", c.borrow().data, expected);
        println!("a.grad  {} ", a.get_grad());
        println!("b.grad  {} ", b.get_grad());
        assert_float(c.borrow().data, expected);

        assert_float(a.get_grad(), 0.5);
        assert_float(b.get_grad(), -1.75);
    }

    pub fn test_relu(a_f64: f64, c_expected: f64, a_grad_expected: f64) {
        let a = ValueRefV2::new_value(a_f64, "a".to_string());
        let mut c = a.relu();
        c.set_label("c".to_string());

        c.backward();

        println!("c actual  {}  expected {}", c.borrow().data, c_expected);
        println!("a.grad  {} ", a.get_grad());
        assert_float(c.borrow().data, c_expected);

        assert_float(a.get_grad(), a_grad_expected);
    }

    #[test]
    pub fn test_grad_relu_positive() {
        let a_f64 = 1.0_f64;
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
        let a_f64 = -10.0_f64;
        let expected = a_f64.max(0.0);
        test_relu(a_f64, expected, 0.0);
    }

    pub fn test_pow(a_f64: f64, b: f64, c_expected: f64, a_grad_expected: f64) {
        let a = ValueRefV2::new_value(a_f64, "a".to_string());
        let mut c = a.pow(b);
        c.set_label("c".to_string());

        c.backward();

        println!("c actual  {}  expected {}", c.borrow().data, c_expected);
        println!("a.grad  {} ", a.get_grad());
        assert_float(c.borrow().data, c_expected);
        assert_float(a.get_grad(), a_grad_expected);
    }

    #[test]
    pub fn test_grad_pow_1() {
        let a_f64 = -1.0_f64;
        let b = 2.0;
        let expected = a_f64.powf(b);
        test_pow(a_f64, b, expected, -2.0);
    }

    #[test]
    pub fn test_grad_pow_2() {
        let a_f64 = 3.0_f64;
        let b = 3.0;
        let expected = a_f64.powf(b);
        test_pow(a_f64, b, expected, 27.0);
    }

    #[test]
    pub fn test_grad_pow_3() {
        let a_f64 = 3.0_f64;
        let b = 1.5;
        let expected = a_f64.powf(b);
        test_pow(a_f64, b, expected, 2.598076211353316);
    }

    pub fn test_tanh(a_f64: f64, c_expected: f64, a_grad_expected: f64) {
        let a = ValueRefV2::new_value(a_f64, "a".to_string());
        let mut c = a.tanh();
        c.set_label("c".to_string());

        c.backward();

        println!("c actual  {}  expected {}", c.borrow().data, c_expected);
        println!("a.grad  {} ", a.get_grad());
        assert_float(c.borrow().data, c_expected);
        assert_float(a.get_grad(), a_grad_expected);
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
    pub fn test_simple_neurons_explizt_tanh() {
        let x1: ValueRefV2 = ValueRefV2::new_value(2.0, "x1".to_string());
        let x2 = ValueRefV2::new_value(0.0, "x2".to_string());

        let w1 = ValueRefV2::new_value(-3.0, "w1".to_string());
        let w2 = ValueRefV2::new_value(1.0, "w2".to_string());

        let b = ValueRefV2::new_value(6.881373587019, "b".to_string());

        let w1x1 = &x1 * &w1;
        let w2x2 = &x2 * &w2;
        let w1x1_plus_w2x2 = &w1x1 + &w2x2;

        let mut n = &w1x1_plus_w2x2 + &b;
        n.set_label("n".to_string());

        let e = (2.0 * &n).exp();
        let mut o = &(&e - 1.0) / &(&e + 1.0);

        // let mut o = n.tanh();
        o.set_label("o".to_string());

        o.backward();

        println!("x1 grad  expected {},  actual {}", -1.5, x1.get_grad());
        println!("w1 grad  expected {},  actual {}", 1.0, w1.get_grad());
        assert_float(x1.get_grad(), -1.5);
        assert_float(w1.get_grad(), 1.0);

        println!("x2 grad  expected {},  actual {}", 0.5, x2.get_grad());
        println!("w2 grad  expected {},  actual {}", 0.0, w2.get_grad());
        assert_float(x2.get_grad(), 0.5);
        assert_float(w2.get_grad(), 0.0);

        println!("w1x1 data  expected {},  actual {}", -6.0, w1x1.get_data());
        println!("w2x2 data  expected {},  actual {}", 0.0, w2x2.get_data());

        assert_float(w1x1.get_data(), -6.0);
        assert_float(w2x2.get_data(), 0.0);

        println!("w1x1 grad  expected {},  actual {}", 0.5, w1x1.get_grad());
        println!("w2x2 grad  expected {},  actual {}", 0.5, w2x2.get_grad());

        assert_float(w1x1.get_grad(), 0.5);
        assert_float(w2x2.get_grad(), 0.5);

        println!("b  data  expected {},  actual {}", 6.881373587019, b.get_data());
        println!("b  grad  expected {},  actual {}", 0.5, b.get_grad());

        assert_float(b.get_data(), 6.881373587019);
        assert_float(b.get_grad(), 0.5);

        println!(
            "w1x1_plus_w2x2 data  expected {},  actual {}",
            -6.,
            w1x1_plus_w2x2.get_data()
        );
        println!(
            "w1x1_plus_w2x2 grad  expected {},  actual {}",
            0.5,
            w1x1_plus_w2x2.get_grad()
        );

        assert_float(w1x1_plus_w2x2.get_data(), -6.0);
        assert_float(w1x1_plus_w2x2.get_grad(), 0.5);

        println!("n data  expected {},  actual {}", 0.8814, n.get_data());
        println!("n grad  expected {},  actual {}", 0.5, n.get_grad());

        assert_float(n.get_data(), 0.8814);
        assert_float(n.get_grad(), 0.5);

        println!("e data  expected {},  actual {}", 5.8284, e.get_data());
        println!("e grad  expected {},  actual {}", 0.0429, e.get_grad());

        assert_float(e.get_data(), 5.8284);
        assert_float(e.get_grad(), 0.0429);

        println!("o data  expected {},  actual {}", SQRT_2 / 2.0, o.get_data());
        println!("o grad  expected {},  actual {}", 1.0, o.get_grad());

        assert_float(o.get_data(), SQRT_2 / 2.0);
        assert_float(o.get_grad(), 1.0);
    }

    #[test]
    pub fn test_simple_neurons_simple_tanh() {
        let x1: ValueRefV2 = ValueRefV2::new_value(2.0, "x1".to_string());
        let x2 = ValueRefV2::new_value(0.0, "x2".to_string());

        let w1 = ValueRefV2::new_value(-3.0, "w1".to_string());
        let w2 = ValueRefV2::new_value(1.0, "w2".to_string());

        let b = ValueRefV2::new_value(6.881373587019, "b".to_string());

        let w1x1 = &x1 * &w1;
        let w2x2 = &x2 * &w2;
        let w1x1_plus_w2x2 = &w1x1 + &w2x2;

        let mut n = &w1x1_plus_w2x2 + &b;
        n.set_label("n".to_string());

        let mut o = n.tanh();
        o.set_label("o".to_string());

        o.backward();

        println!("x1 grad  expected {},  actual {}", -1.5, x1.get_grad());
        println!("w1 grad  expected {},  actual {}", 1.0, w1.get_grad());
        assert_float(x1.get_grad(), -1.5);
        assert_float(w1.get_grad(), 1.0);

        println!("x2 grad  expected {},  actual {}", 0.5, x2.get_grad());
        println!("w2 grad  expected {},  actual {}", 0.0, w2.get_grad());
        assert_float(x2.get_grad(), 0.5);
        assert_float(w2.get_grad(), 0.0);

        println!("w1x1 data  expected {},  actual {}", -6.0, w1x1.get_data());
        println!("w2x2 data  expected {},  actual {}", 0.0, w2x2.get_data());

        assert_float(w1x1.get_data(), -6.0);
        assert_float(w2x2.get_data(), 0.0);

        println!("w1x1 grad  expected {},  actual {}", 0.5, w1x1.get_grad());
        println!("w2x2 grad  expected {},  actual {}", 0.5, w2x2.get_grad());

        assert_float(w1x1.get_grad(), 0.5);
        assert_float(w2x2.get_grad(), 0.5);

        println!("b  data  expected {},  actual {}", 6.881373587019, b.get_data());
        println!("b  grad  expected {},  actual {}", 0.5, b.get_grad());

        assert_float(b.get_data(), 6.881373587019);
        assert_float(b.get_grad(), 0.5);

        println!(
            "w1x1_plus_w2x2 data  expected {},  actual {}",
            -6.,
            w1x1_plus_w2x2.get_data()
        );
        println!(
            "w1x1_plus_w2x2 grad  expected {},  actual {}",
            0.5,
            w1x1_plus_w2x2.get_grad()
        );

        assert_float(w1x1_plus_w2x2.get_data(), -6.0);
        assert_float(w1x1_plus_w2x2.get_grad(), 0.5);

        println!("n data  expected {},  actual {}", 0.8814, n.get_data());
        println!("n grad  expected {},  actual {}", 0.5, n.get_grad());

        assert_float(n.get_data(), 0.8814);
        assert_float(n.get_grad(), 0.5);

        println!("o data  expected {},  actual {}", SQRT_2 / 2.0, o.get_data());
        println!("o grad  expected {},  actual {}", 1.0, o.get_grad());

        assert_float(o.get_data(), SQRT_2 / 2.0);
        assert_float(o.get_grad(), 1.0);
    }

    #[test]
    pub fn test_add_same_variable() {
        let a = -5.0_f64;
        let expected = a + a;
        let a_grad_expected = 2.0;
        let a = ValueRefV2::new_value(-5.0, "a".to_string());
        let mut c = &a + &a;
        c.backward();

        println!("a.grad   expected {},   actual {}", a_grad_expected, a.get_grad());
        assert_float(c.get_data(), expected);
        assert_float(a.get_grad(), a_grad_expected);
    }

    #[test]
    pub fn test_mul_same_variable() {
        let a = -5.0_f64;
        let expected = a * a;
        let a_grad_expected = -10.0;
        let a = ValueRefV2::new_value(a, "a".to_string());
        let mut c = &a * &a;
        c.backward();

        println!("a.grad   expected {},   actual {}", a_grad_expected, a.get_grad());
        assert_float(c.get_data(), expected);
        assert_float(a.get_grad(), a_grad_expected);
    }

    #[test]
    pub fn test_neg_grad() {
        let a = -5.0_f64;
        let expected = -a;
        let a_grad_expected = -1.0;
        let a = ValueRefV2::new_value(a, "a".to_string());
        let mut c = -&a;
        c.backward();

        println!("a.grad   expected {},   actual {}", a_grad_expected, a.get_grad());
        assert_float(c.get_data(), expected);
        assert_float(a.get_grad(), a_grad_expected);
    }

    #[test]
    pub fn test_neg_grad1() {
        let a = ValueRefV2::new_value(-4.0, "a".to_string()); //         a = Value(-4.0)
        let b = ValueRefV2::new_value(2.0, "b".to_string()); //         b = Value(2.0)
        let mut c = &a + &b; //         c = a + b
        c += &c + 1.0; //         c += c + 1
        c += (1.0 + &c) + (-&a); //         c += 1 + c + (-a)

        c.backward();

        let a_expected = -4.0;
        let b_expected = 2.0;
        let c_expected = -1.0;

        let a_grad_expected = 3.0;
        let b_grad_expected = 4.0;
        let c_grad_expected = 1.0;

        println!("a data {}   expected {}", a.get_data(), a_expected);
        println!("b data {}   expected {}", b.get_data(), b_expected);
        println!("c data {}   expected {}", c.get_data(), c_expected);

        println!("a grad {}   expected {}", a.get_grad(), a_grad_expected);
        println!("a grad {}   expected {}", b.get_grad(), b_grad_expected);
        println!("a grad {}   expected {}", c.get_grad(), c_grad_expected);

        assert_float(a.get_data(), a_expected);
        assert_float(b.get_data(), b_expected);
        assert_float(c.get_data(), c_expected);

        assert_float(a.get_grad(), a_grad_expected);
        assert_float(b.get_grad(), b_grad_expected);
        assert_float(c.get_grad(), c_grad_expected);
    }

    #[test]
    pub fn test_sub_relu_grad() {
        let a = ValueRefV2::new_value(-4.0, "a".to_string()); //         a = Value(-4.0)
        let b = ValueRefV2::new_value(2.0, "b".to_string()); //         b = Value(2.0)
        let mut c = (&b - &a).relu();

        c.backward();

        let topo = ValueRefV2::traverse(&c);
        println!("#################################");
        println!("Topo");
        for t in topo.iter() {
            println!(
                "{}  data {},  grad  {}",
                t.r.borrow().label(),
                t.get_data(),
                t.get_grad()
            );
        }
        println!("#################################");

        let a_expected = -4.0;
        let b_expected = 2.0;
        let c_expected = 6.0;

        let a_grad_expected = -1.0;
        let b_grad_expected = 1.0;
        let c_grad_expected = 1.0;

        println!("a data {}   expected {}", a.get_data(), a_expected);
        println!("b data {}   expected {}", b.get_data(), b_expected);
        println!("c data {}   expected {}", c.get_data(), c_expected);

        println!("a grad {}   expected {}", a.get_grad(), a_grad_expected);
        println!("b grad {}   expected {}", b.get_grad(), b_grad_expected);
        println!("c grad {}   expected {}", c.get_grad(), c_grad_expected);

        assert_float(a.get_data(), a_expected);
        assert_float(b.get_data(), b_expected);
        assert_float(c.get_data(), c_expected);

        assert_float(a.get_grad(), a_grad_expected);
        assert_float(b.get_grad(), b_grad_expected);
        assert_float(c.get_grad(), c_grad_expected);
    }
}
