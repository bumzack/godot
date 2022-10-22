use std::cell::{Ref, RefCell};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum OpEnumV2 {
    NONE,
    ADD,
    MUL,
}

#[derive(Debug, Clone)]
pub struct ValueV2<T>
where
    T: Clone + Add + Mul,
{
    data: T,
    grad: T,
    op: OpEnumV2,
    children: Vec<ValueRefV2<T>>,
    label: String,
}

#[derive(Debug, Clone)]
pub struct ValueRefV2<T>
where
    T: Clone + Add + Mul,
{
    r: Rc<RefCell<ValueV2<T>>>,
}

impl<T> ValueRefV2<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default,
{
    pub fn new(v: ValueV2<T>) -> Self {
        ValueRefV2 {
            r: Rc::new(RefCell::new(v)),
        }
    }

    pub fn new_value(v: T, label: String) -> ValueRefV2<T> {
        let v = ValueV2::new_value(v, label);
        ValueRefV2 {
            r: Rc::new(RefCell::new(v)),
        }
    }

    // convenience methods to simplify code, to avoid value.r.borrow()
    pub fn borrow(&self) -> Ref<ValueV2<T>> {
        self.r.borrow()
    }

    pub fn set_label(&mut self, label: String) {
        self.r.borrow_mut().set_label(label);
    }
}

impl<T> ValueV2<T>
where
    T: Clone + Add + Mul + Default,
{
    pub fn new(data: T, op: OpEnumV2, label: String) -> Self {
        ValueV2 {
            data,
            op,
            children: vec![],
            label,
            grad: T::default(),
        }
    }

    pub fn new_value(data: T, label: String) -> Self {
        ValueV2 {
            data,
            op: OpEnumV2::NONE,
            children: vec![],
            label,
            grad: T::default(),
        }
    }

    pub fn data(&self) -> &T {
        &self.data
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

    pub fn children(&self) -> &Vec<ValueRefV2<T>> {
        &self.children
    }

    pub fn grad(&self) -> &T {
        &self.grad
    }

    pub fn set_grad(&mut self, grad: T) {
        self.grad = grad;
    }
}

//
impl<'a, 'b, T> Add<&'b ValueRefV2<T>> for &'a ValueRefV2<T>
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default,
    <T as Add>::Output: Clone,
    <T as Mul>::Output: Clone,
    <T as Add>::Output: Add,
    <T as Mul>::Output: Mul,
    <T as Add>::Output: Mul,
{
    type Output = ValueRefV2<T>;

    fn add(self, rhs: &'b ValueRefV2<T>) -> Self::Output {
        let x1 = self.r.borrow().clone();
        let x2 = rhs.r.borrow().clone();
        let v = ValueV2 {
            data: x1.data + x2.data,
            op: OpEnumV2::ADD,
            children: vec![self.clone(), rhs.clone()],
            label: format!("{} + {}", self.borrow().label, rhs.borrow().label).to_string(),
            grad: T::default(),
        };
        ValueRefV2::new(v)
    }
}

impl<'a, 'b, T> Mul<&'b ValueRefV2<T>> for &'a ValueRefV2<T>
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default,
    <T as Add>::Output: Clone,
    <T as Mul>::Output: Clone,
    <T as Add>::Output: Add,
    <T as Mul>::Output: Mul,
    <T as Add>::Output: Mul,
{
    type Output = ValueRefV2<T>;

    fn mul(self, rhs: &'b ValueRefV2<T>) -> Self::Output {
        let x1 = self.r.borrow().clone();
        let x2 = rhs.r.borrow().clone();
        let v = ValueV2 {
            data: x1.data * x2.data,
            op: OpEnumV2::MUL,
            children: vec![self.clone(), rhs.clone()],
            label: format!("{} * {}", self.borrow().label, rhs.borrow().label).to_string(),
            grad: T::default(),
        };
        ValueRefV2::new(v)
    }
}

impl Display for OpEnumV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpEnumV2::ADD => write!(f, "+"),
            OpEnumV2::NONE => write!(f, ""),
            OpEnumV2::MUL => write!(f, "*"),
        }
    }
}

impl<T> Default for ValueV2<T>
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default,
    <T as Add>::Output: Clone,
    <T as Mul>::Output: Clone,
    <T as Add>::Output: Add,
    <T as Mul>::Output: Mul,
    <T as Add>::Output: Mul,
{
    fn default() -> Self {
        ValueV2 {
            data: T::default(),
            op: OpEnumV2::NONE,
            children: vec![],
            grad: T::default(),
            label: "default".to_string(),
        }
    }
}

impl<T> Default for ValueRefV2<T>
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default,
    <T as Add>::Output: Clone,
    <T as Mul>::Output: Clone,
    <T as Add>::Output: Add,
    <T as Mul>::Output: Mul,
    <T as Add>::Output: Mul,
{
    fn default() -> Self {
        ValueRefV2::new(ValueV2::default())
    }
}

// TODO why is this not working for f64
// impl<T> Display for ValueV2<T>
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
// impl<T> Display for ValueRefV2<T>
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

impl Display for ValueV2<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.label, self.data)
    }
}

impl Display for ValueRefV2<f64> {
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
