use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use std::rc::Rc;

#[derive(PartialEq)]
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
    fn apply(&self, out: ValueRefV2, children: &Vec<ValueRefV2>);
}

struct BackwardAdd {}

impl Backward for BackwardAdd {
    fn apply(&self, out: ValueRefV2, children: &Vec<ValueRefV2>) {
        println!(
            "ADD a {:?},  children [0] {:?}, children[1] {:?}",
            out, children[0], children[1]
        );
        println!("backward add ");
        let mut self__ = children[0].clone();
        let mut other = children[1].clone();

        self__.set_grad(self__.get_grad() + 1.0 * out.r.borrow().grad());
        other.set_grad(other.get_grad() + 1.0 * out.r.borrow().grad());
    }
}

struct BackwardSub {}

impl Backward for BackwardSub {
    fn apply(&self, out: ValueRefV2, children: &Vec<ValueRefV2>) {
        println!(
            "SUB a {:?},  children [0] {:?}, children[1] {:?}",
            out, children[0], children[1]
        );
        println!("backward sub ");
        let mut self__ = children[0].clone();
        let mut other = children[1].clone();

        self__.set_grad(self__.get_grad() + 1.0 * out.r.borrow().grad());
        other.set_grad(other.get_grad() - 1.0 * out.r.borrow().grad());
    }
}

struct BackwardMul {}

impl Backward for BackwardMul {
    fn apply(&self, out: ValueRefV2, children: &Vec<ValueRefV2>) {
        println!(
            "MUL a {:?} children [0] {:?}, children[1] {:?}",
            out, children[0], children[1]
        );

        let mut self__ = children[0].clone();
        let mut other = children[1].clone();

        self__.set_grad(self__.get_grad() + other.borrow().data() * out.r.borrow().grad());
        other.set_grad(other.get_grad() + self__.borrow().data() * out.r.borrow().grad());
        println!("backward mul ");
    }
}

struct BackwardDiv {}

impl Backward for BackwardDiv {
    fn apply(&self, out: ValueRefV2, children: &Vec<ValueRefV2>) {
        println!(
            "MUL a {:?} children [0] {:?}, children[1] {:?}",
            out, children[0], children[1]
        );

        let mut self__ = children[0].clone();
        let mut other = children[1].clone();

        self__.set_grad(self__.get_grad() + other.borrow().data() * out.r.borrow().grad());
        other.set_grad(other.get_grad() + self__.borrow().data() * out.r.borrow().grad());
        println!("backward div ");
    }
}

struct BackwardTanh {}

impl Backward for BackwardTanh {
    fn apply(&self, out: ValueRefV2, children: &Vec<ValueRefV2>) {
        println!("TANH a {:?}children [0] {:?} ", out, children[0]);
        let mut self__ = children[0].clone();
        let x = out.get_data();
        let y = 1.0 - x * x;
        self__.set_grad(self__.get_grad() + y * out.r.borrow().grad());
    }
}

struct BackwardExp {}

impl Backward for BackwardExp {
    fn apply(&self, out: ValueRefV2, children: &Vec<ValueRefV2>) {
        println!("EXP a {:?}, children [0] {:?} ", out, children[0]);
        let mut self__ = children[0].clone();
        self__.set_grad(self__.get_grad() + out.r.borrow().data() * out.r.borrow().grad());
    }
}

struct BackwardPow {}

impl Backward for BackwardPow {
    fn apply(&self, out: ValueRefV2, children: &Vec<ValueRefV2>) {
        println!("POW a {:?}, children [0] {:?} ", out, children[0]);
        let mut self__ = children[0].clone();
        let other = children[1].clone().borrow().data;
        let x = other * (self__.borrow().data().powf(other - 1.0)) * out.r.borrow().grad();
        self__.set_grad(self__.get_grad() + x);
    }
}

//
// struct BackwardDiv {}
//
// impl Backward for BackwardDiv {
//     fn apply(&self, out: ValueRefV2, children: &Vec<ValueRefV2>) {
//         // println!("EXP a {:?}children [0] {:?} ", out, children[0]);
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

    pub fn backward(&mut self) {
        self.set_grad(1.0);
        let topo = Self::traverse(self);

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
            backward: Some(Box::new(BackwardAdd {})),
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
            backward: Some(Box::new(BackwardAdd {})),
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
        let string = "f64 div".to_string();
        let r = ValueRefV2::new_value(rhs, string.clone());
        let x1 = self.r.borrow();
        let out = ValueV2 {
            data: x1.data / rhs,
            op: OpEnumV2::DIV,
            children: vec![self.clone(), r],
            label: format!("{} + {}", self.borrow().label, string),
            grad: 0.0,
            backward: Some(Box::new(BackwardDiv {})),
        };
        ValueRefV2::new(out)
    }
}

impl Div<f64> for &ValueRefV2 {
    type Output = ValueRefV2;

    fn div(self, rhs: f64) -> Self::Output {
        let string = "f64 div".to_string();
        let r = ValueRefV2::new_value(rhs, string.clone());
        let x1 = self.r.borrow();
        let out = ValueV2 {
            data: x1.data / rhs,
            op: OpEnumV2::DIV,
            children: vec![self.clone(), r],
            label: format!("{} + {}", self.borrow().label, string),
            grad: 0.0,
            backward: Some(Box::new(BackwardDiv {})),
        };
        ValueRefV2::new(out)
    }
}

impl Div<ValueRefV2> for f64 {
    type Output = ValueRefV2;

    fn div(self, rhs: ValueRefV2) -> Self::Output {
        let string = "f64 div".to_string();
        let r = ValueRefV2::new_value(self, string.clone());
        let x1 = rhs.r.borrow();
        let out = ValueV2 {
            data: self / x1.data,
            op: OpEnumV2::DIV,
            children: vec![r, rhs.clone()],
            label: format!("{} + {}", string, rhs.borrow().label),
            grad: 0.0,
            backward: Some(Box::new(BackwardDiv {})),
        };
        ValueRefV2::new(out)
    }
}

impl Div<&ValueRefV2> for f64 {
    type Output = ValueRefV2;

    fn div(self, rhs: &ValueRefV2) -> Self::Output {
        let string = "f64 div".to_string();
        let r = ValueRefV2::new_value(self, string.clone());
        let x1 = rhs.r.borrow();
        let out = ValueV2 {
            data: self / x1.data,
            op: OpEnumV2::DIV,
            children: vec![r, rhs.clone()],
            label: format!("{} + {}", string, rhs.borrow().label),
            grad: 0.0,
            backward: Some(Box::new(BackwardDiv {})),
        };
        ValueRefV2::new(out)
    }
}

impl Neg for ValueRefV2 {
    type Output = ValueRefV2;

    fn neg(self) -> Self::Output {
        self * 1.0
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

pub const EPS: f64 = 0.0000001;

pub fn assert_two_float(a: f64, b: f64) {
    assert!((a - b).abs() < EPS);
}

#[cfg(test)]
mod tests {
    use crate::micrograd_rs_v2::assert_two_float;
    use crate::{draw_graph, ValueRefV2};

    // before starting to add grad
    // https://youtu.be/VMj-3S1tku0?t=1875
    #[test]
    pub fn test_video() {
        let a = ValueRefV2::new_value(2.0 as f64, "a".to_string());
        let b = ValueRefV2::new_value(-3.0, "b".to_string());
        let c = ValueRefV2::new_value(10.0, "c".to_string());
        let f = ValueRefV2::new_value(-2.0, "f".to_string());

        let mut e = &a * &b;
        e.set_label("e".to_string());

        let mut d = &e + &c;
        d.set_label("d".to_string());

        let mut l = &d * &f;
        l.set_label("L".to_string());

        assert_two_float(l.borrow().data, -8.0);
    }

    #[test]
    pub fn test_add() {
        let a = ValueRefV2::new_value(2.0 as f64, "a".to_string());
        let b = ValueRefV2::new_value(3.0, "b".to_string());

        let mut x = &a + &b;
        x.set_label("x".to_string());

        assert_two_float(x.borrow().data, 5.0);
    }

    #[test]
    pub fn test_mul() {
        let a = ValueRefV2::new_value(2.0 as f64, "a".to_string());
        let b = ValueRefV2::new_value(3.0, "b".to_string());

        let mut x = &a * &b;
        x.set_label("x".to_string());

        assert_two_float(x.borrow().data, 6.0);
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

        assert_two_float(b.borrow().data, 6.0);

        draw_graph(b, "test_a_plus_a".to_string());

        assert_two_float(a.get_grad(), 2.0);
    }

    #[test]
    pub fn test_value_plus_f64_rhs() {
        let a = ValueRefV2::new_value(3.0, "a".to_string());
        let mut b = &a + 1.0 as f64;
        assert_two_float(b.borrow().data, 4.0);
        b.backward();
        draw_graph(b, "test_a_plus_f64_rhs".to_string());
    }

    #[test]
    pub fn test_value_plus_f64_lhs() {
        let a = ValueRefV2::new_value(4.0, "a".to_string());
        let mut b = 23.0 as f64 + &a;
        b.backward();
        assert_two_float(b.borrow().data, 27.0);

        draw_graph(b, "test_a_plus_f64_lhs".to_string());
    }

    #[test]
    pub fn test_value_mul_f64_rhs() {
        let a = ValueRefV2::new_value(3.0, "a".to_string());
        let mut b = &a * 3.0 as f64;
        assert_two_float(b.borrow().data, 9.0);
        b.backward();
        draw_graph(b, "test_a_mul_f64_rhs".to_string());
    }

    #[test]
    pub fn test_value_mul_f64_lhs() {
        let a = ValueRefV2::new_value(4.0, "a".to_string());
        let mut b = 23.0 as f64 * &a;
        b.backward();
        assert_two_float(b.borrow().data, 92.0);

        draw_graph(b, "test_a_mul_f64_lhs".to_string());
    }

    #[test]
    pub fn test_value_div() {
        let a = ValueRefV2::new_value(4.0, "a".to_string());
        let b = ValueRefV2::new_value(2.0, "b".to_string());
        let mut c = &a / &b;
        c.backward();
        assert_two_float(c.borrow().data, 2.0);

        draw_graph(c, "test_a_div_b".to_string());
    }

    #[test]
    pub fn test_value_exp() {
        let expected = (4.0 as f64).exp();
        let a = ValueRefV2::new_value(4.0, "a".to_string());
        let mut b = a.exp();

        b.backward();
        assert_two_float(b.borrow().data, expected);

        draw_graph(b, "test_exp_4".to_string());
    }

    #[test]
    pub fn test_value_pow() {
        let expected = (4.0 as f64).powf(3.0);
        let a = ValueRefV2::new_value(4.0, "a".to_string());
        let b = 3.0;
        let mut b = a.pow(b);

        b.backward();
        assert_two_float(b.borrow().data, expected);

        draw_graph(b, "test_pow".to_string());
    }

    #[test]
    pub fn test_value_sub() {
        let expected = 4.0 - 23.0;
        let a = ValueRefV2::new_value(4.0, "a".to_string());
        let b = ValueRefV2::new_value(23.0, "b".to_string());
        let mut c = &a - &b;
        c.set_label("c".to_string());

        c.backward();
        assert_two_float(c.borrow().data, expected);

        draw_graph(c, "test_sub".to_string());
    }

    // https://github.com/karpathy/micrograd
    #[test]
    pub fn test_grad() {
        let a = ValueRefV2::new_value(-4.0, "a".to_string()); //         a = Value(-4.0)
        let b = ValueRefV2::new_value(2.0, "b".to_string()); //         b = Value(2.0)
        let mut c = &a + &b; //         c = a + b
        let mut d = &a * &b + b.pow(3.0); //         d = a * b + b**3
        c += &c + 1.0; //         c += c + 1
        c += 1.0 + &c + (-a); //         c += 1 + c + (-a)
                              //  d+= &d*2.0  + (&b + &a).relu();                                                 //         d += d * 2 + (b + a).relu()
                              //       d+=3.0* &d  + (&b - &a).relu();                                            //         d += 3 * d + (b - a).relu()
        let e = &c - &d; //         e = c - d
        let f = &e.pow(2.0); //         f = e**2
        let mut g = f / 2.0 as f64; //         g = f / 2.0
        g += 10.0 as f64 / f; //         g += 10.0 / f
                              //         print(f'{g.data:.4f}') # prints 24.7041, the outcome of this forward pass
                              //     g.backward()
                              //     print(f'{a.grad:.4f}') # prints 138.8338, i.e. the numerical value of dg/da
                              // print(f'{b.grad:.4f}') # prints 645.5773, i.e. the numerical value of dg/db

        g.backward();

        let topo = ValueRefV2::traverse(&g);

        println!("Topo");
        for t in topo.iter() {
            println!("{}", t);
        }

        draw_graph(g, "test_all_math_ops_graph".to_string());
    }
}
