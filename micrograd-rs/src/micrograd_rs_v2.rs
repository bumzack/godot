use std::cell::{Ref, RefCell};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum OpEnumV2 {
    NONE,
    ADD,
    MUL,
    TANH,
}

#[derive(Debug, Clone)]
pub struct ValueV2 {
    data: f64,
    grad: f64,
    op: OpEnumV2,
    children: Vec<ValueRefV2>,
    label: String,
}

#[derive(Debug, Clone)]
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

    pub fn tanh(&self) -> ValueRefV2 {
        let x = self.r.borrow().data();
        let y = ((2.0 as f64 * x).exp() - 1.0) / ((2.0 * x).exp() + 1.0);

        let v = ValueV2 {
            data: y,
            op: OpEnumV2::TANH,
            children: vec![self.clone()],
            label: format!("tanh({})", self.borrow().label),
            grad: 0.0,
        };

        ValueRefV2::new(v)
    }

    pub fn backward(&mut self) {
        match self.op() {
            OpEnumV2::ADD => {
                self.grad
            }

            _ => {}
        };
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
        }
    }

    pub fn new_value(data: f64, label: String) -> Self {
        ValueV2 {
            data,
            op: OpEnumV2::NONE,
            children: vec![],
            label,
            grad: 0.0,
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

    fn add(self, rhs: &'b ValueRefV2) -> Self::Output {
        let x1 = self.r.borrow();
        let x2 = rhs.r.borrow();
        let out = ValueV2 {
            data: x1.data + x2.data,
            op: OpEnumV2::ADD,
            children: vec![self.clone(), rhs.clone()],
            label: format!("{} + {}", self.borrow().label, rhs.borrow().label).to_string(),
            grad: 0.0,
        };
        ValueRefV2::new(out)
    }
}

impl<'a, 'b> Mul<&'b ValueRefV2> for &'a ValueRefV2 {
    type Output = ValueRefV2;

    fn mul(self, rhs: &'b ValueRefV2) -> Self::Output {
        let x1 = self.r.borrow();
        let x2 = rhs.r.borrow();
        let out = ValueV2 {
            data: x1.data * x2.data,
            op: OpEnumV2::MUL,
            children: vec![self.clone(), rhs.clone()],
            label: format!("{} * {}", self.borrow().label, rhs.borrow().label).to_string(),
            grad: 0.0,
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
        }
    }
}

impl Default for ValueRefV2 {
    fn default() -> Self {
        ValueRefV2::new(ValueV2::default())
    }
}

// TODO why is this not working for f64
// impl  Display for ValueV2
//     where
//         T: Clone + Add + Mul + Add<Output=T> + Mul<Output=T> + Default + Debug,
//         <T as Add>::Output: Clone,
//         <T as Mul>::Output: Clone,
//         <T as Add>::Output: Add,
//         <T as Mul>::Output: Mul,
//         <T as Add>::Output: Mul,
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}: {:?}", self.label, self.data)
//     }
// }

// TODO why is this not working for f64
// impl  Display for ValueRefV2
//     where
//         T: Clone + Add + Mul + Add<Output=T> + Mul<Output=T> + Default + Debug,
//         <T as Add>::Output: Clone,
//         <T as Mul>::Output: Clone,
//         <T as Add>::Output: Add,
//         <T as Mul>::Output: Mul,
//         <T as Add>::Output: Mul,
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.r.borrow().clone())
//     }
// }

impl Display for ValueV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.label, self.data)
    }
}

impl Display for ValueRefV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.r.borrow().clone())
    }
}

pub const EPS: f64 = 0.0000001;

pub fn assert_two_float(a: f64, b: f64) {
    assert!((a - b).abs() < EPS);
}

#[cfg(test)]
mod tests {
    use crate::micrograd_rs_V2::assert_two_float;
    use crate::micrograd_rs_v2::assert_two_float;
    use crate::ValueRefV2;

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

        println!("a {}", a);
        println!("b {}", b);
        println!("c {}", c);
        println!("d {}", d);
        println!("e {}", e);
        println!("f {}", f);
        println!("l {}", l);
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
}
