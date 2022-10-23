use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul};
use std::rc::Rc;

#[derive(PartialEq)]
pub enum OpEnumV2 {
    NONE,
    ADD,
    MUL,
    TANH,
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

struct BackwardTanh {}

impl Backward for BackwardTanh {
    fn apply(&self, out: ValueRefV2, children: &Vec<ValueRefV2>) {
        println!("TANH a {:?}children [0] {:?} ", out, children[0]);
        let mut self__ = children[0].clone();
        let x = out.get_data();
        let y = 1.0 - x * x;
        self__.set_grad(y * out.r.borrow().grad());
    }
}

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
        self.r.borrow().grad().clone()
    }

    pub fn get_data(&self) -> f64 {
        self.r.borrow().data().clone()
    }

    pub fn tanh(&self) -> ValueRefV2 {
        let x = self.r.borrow().data();
        let y = ((2.0 as f64 * x).exp() - 1.0) / ((2.0 * x).exp() + 1.0);

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
        if !visited.contains(&v) {
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

    pub fn grad(&self) -> &f64 {
        &self.grad
    }

    pub fn set_grad(&mut self, grad: f64) {
        self.grad = grad;
    }
}

//
impl<'a, 'b> Add<&'b ValueRefV2> for &'a ValueRefV2 {
    type Output = ValueRefV2;

    fn add(self, rhs: &ValueRefV2) -> Self::Output {
        let x1 = self.r.borrow();
        let x2 = rhs.r.borrow();
        let out = ValueV2 {
            data: x1.data + x2.data,
            op: OpEnumV2::ADD,
            children: vec![self.clone(), rhs.clone()],
            label: format!("{} + {}", self.borrow().label, rhs.borrow().label).to_string(),
            grad: 0.0,
            backward: Some(Box::new(BackwardAdd {})),
        };
        ValueRefV2::new(out)
    }
}

impl<'a, 'b> Mul<&'b ValueRefV2> for &'a ValueRefV2 {
    type Output = ValueRefV2;

    fn mul(self, rhs: &ValueRefV2) -> Self::Output {
        let x1 = self.r.borrow();
        let x2 = rhs.r.borrow();
        let out = ValueV2 {
            data: x1.data * x2.data,
            op: OpEnumV2::MUL,
            children: vec![self.clone(), rhs.clone()],
            label: format!("{} * {}", self.borrow().label, rhs.borrow().label).to_string(),
            grad: 0.0,
            backward: Some(Box::new(BackwardMul {})),
        };
        ValueRefV2::new(out)
    }
}

impl Display for OpEnumV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpEnumV2::ADD => write!(f, "+"),
            OpEnumV2::NONE => write!(f, ""),
            OpEnumV2::MUL => write!(f, "*"),
            OpEnumV2::TANH => write!(f, "tanh"),
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
        write!(f, "{}", self.r.borrow())
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
    use crate::micrograd_rs_v2::{assert_two_float, ValueV2};
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
}
