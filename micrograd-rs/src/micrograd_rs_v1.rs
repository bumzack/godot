use std::cell::{Ref, RefCell};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum OpEnumV1 {
    NONE,
    ADD,
    MUL,
}

#[derive(Debug, Clone)]
pub struct ValueV1<T>
where
    T: Clone + Add + Mul,
{
    data: T,
    op: OpEnumV1,
    children: Vec<ValueRefV1<T>>,
    label: String,
}

#[derive(Debug, Clone)]
pub struct ValueRefV1<T>
where
    T: Clone + Add + Mul,
{
    r: Rc<RefCell<ValueV1<T>>>,
}

impl<T> ValueRefV1<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T>,
{
    pub fn new(v: ValueV1<T>) -> Self {
        ValueRefV1 {
            r: Rc::new(RefCell::new(v)),
        }
    }

    pub fn new_value(v: T, label: String) -> ValueRefV1<T> {
        let v = ValueV1::new_value(v, label);
        ValueRefV1 {
            r: Rc::new(RefCell::new(v)),
        }
    }

    // convenience method to simplify code, to avoid value.r.borrow()
    pub fn borrow(&self) -> Ref<ValueV1<T>> {
        self.r.borrow()
    }

    pub fn set_label(&mut self, label: String) {
        self.r.borrow_mut().set_label(label);
    }
}

impl<T> ValueV1<T>
where
    T: Clone + Add + Mul,
{
    pub fn new(data: T, op: OpEnumV1, label: String) -> Self {
        ValueV1 {
            data,
            op,
            children: vec![],
            label,
        }
    }

    pub fn new_value(data: T, label: String) -> Self {
        ValueV1 {
            data,
            op: OpEnumV1::NONE,
            children: vec![],

            label,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn children(&self) -> &Vec<ValueRefV1<T>> {
        &self.children
    }
}

//
impl<'a, 'b, T> Add<&'b ValueRefV1<T>> for &'a ValueRefV1<T>
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T>,
    <T as Add>::Output: Clone,
    <T as Mul>::Output: Clone,
    <T as Add>::Output: Add,
    <T as Mul>::Output: Mul,
    <T as Add>::Output: Mul,
{
    type Output = ValueRefV1<T>;

    fn add(self, rhs: &'b ValueRefV1<T>) -> Self::Output {
        let x1 = self.r.borrow().clone();
        let x2 = rhs.r.borrow().clone();
        let v = ValueV1 {
            data: x1.data + x2.data,
            op: OpEnumV1::ADD,
            children: vec![self.clone(), rhs.clone()],
            label: format!("{} + {}", self.borrow().label, rhs.borrow().label).to_string(),
        };
        ValueRefV1::new(v)
    }
}

impl<'a, 'b, T> Mul<&'b ValueRefV1<T>> for &'a ValueRefV1<T>
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T>,
    <T as Add>::Output: Clone,
    <T as Mul>::Output: Clone,
    <T as Add>::Output: Add,
    <T as Mul>::Output: Mul,
    <T as Add>::Output: Mul,
{
    type Output = ValueRefV1<T>;

    fn mul(self, rhs: &'b ValueRefV1<T>) -> Self::Output {
        let x1 = self.r.borrow().clone();
        let x2 = rhs.r.borrow().clone();
        let v = ValueV1 {
            data: x1.data * x2.data,
            op: OpEnumV1::ADD,
            children: vec![self.clone(), rhs.clone()],
            label: format!("{} * {}", self.borrow().label, rhs.borrow().label).to_string(),
        };
        ValueRefV1::new(v)
    }
}

impl Display for OpEnumV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpEnumV1::ADD => write!(f, "ADD"),
            OpEnumV1::NONE => write!(f, "NONE"),
            OpEnumV1::MUL => write!(f, "MUL"),
        }
    }
}

impl<T> Default for ValueV1<T>
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default,
    <T as Add>::Output: Clone,
    <T as Mul>::Output: Clone,
    <T as Add>::Output: Add,
    <T as Mul>::Output: Mul,
    <T as Add>::Output: Mul,
{
    fn default() -> Self {
        ValueV1 {
            data: T::default(),
            op: OpEnumV1::NONE,
            children: vec![],

            label: "default".to_string(),
        }
    }
}

impl<T> Default for ValueRefV1<T>
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default,
    <T as Add>::Output: Clone,
    <T as Mul>::Output: Clone,
    <T as Add>::Output: Add,
    <T as Mul>::Output: Mul,
    <T as Add>::Output: Mul,
{
    fn default() -> Self {
        ValueRefV1::new(ValueV1::default())
    }
}

// TODO why is this not working for f64
// impl<T> Display for ValueV1<T>
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
// impl<T> Display for ValueRefV1<T>
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

impl Display for ValueV1<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.label, self.data)
    }
}

impl Display for ValueRefV1<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.r.borrow().clone())
    }
}

#[cfg(test)]
mod tests {
    fn assert_two_float(a: f64, b: f64) -> bool {
        false
    }
}
