use std::cell::{Ref, RefCell};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};
use std::rc::Rc;

fn main() {
    let a = ValueRef::new_value(2.0 as f64);
    let b = ValueRef::new_value(3.0);
    let d = ValueRef::new(Value::new_value(8.0));

    let x = &a + &b;
    let y = &a * &x;

    println!("a = {:?}", a.borrow().data);
    println!("b = {:?}", b.borrow().data);
    println!("d ={:?}", d.borrow().data);
    println!("x = {:?} = a+b", x.borrow().data);
    println!("y = {:?} = a*x", y.borrow().data);

    let t1 = ValueRef::new_value(Tensor { x: 3.0 });
    let t2 = ValueRef::new_value(Tensor { x: 6.0 });

    let t1_plus_t2 = &t1 + &t2;
    let t1_x_t2 = &t1 * &t2;

    println!("t1 = {:?}  ", t1.borrow().data);
    println!("t2 = {:?}  ", t2.borrow().data);
    println!("t1_plus_t2 = {:?} = t1 + t2 ", t1_plus_t2.borrow().data);
    println!("t1_x_t2 = {:?} = t1 * t2 ", t1_x_t2.borrow().data);

    println!("DONE");
}

#[derive(Debug, Clone)]
struct Tensor {
    x: f64,
}

impl Add<Tensor> for Tensor {
    type Output = Tensor;

    fn add(self, rhs: Tensor) -> Self::Output {
        Tensor { x: self.x + rhs.x }
    }
}

impl<'a, 'b> Add<&'a Tensor> for &'b Tensor {
    type Output = Tensor;

    fn add(self, rhs: &'a Tensor) -> Self::Output {
        Tensor { x: self.x + rhs.x }
    }
}

impl<'a, 'b> Mul<&'a Tensor> for &'b Tensor {
    type Output = Tensor;

    fn mul(self, rhs: &'a Tensor) -> Self::Output {
        Tensor { x: self.x * rhs.x }
    }
}

impl Mul<Tensor> for Tensor {
    type Output = Tensor;

    fn mul(self, rhs: Tensor) -> Self::Output {
        Tensor { x: self.x * rhs.x }
    }
}

#[derive(Debug, Clone)]
enum OpEnum {
    NONE,
    ADD,
    MUL,
}

#[derive(Debug, Clone)]
struct Value<T>
where
    T: Clone + Add + Mul,
{
    data: T,
    op: OpEnum,
    //  grad: T,
    inp1: Option<ValueRef<T>>,
    inp2: Option<ValueRef<T>>,
}

#[derive(Debug, Clone)]
struct ValueRef<T>
where
    T: Clone + Add + Mul,
{
    r: Rc<RefCell<Value<T>>>,
}

impl<T> ValueRef<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T>,
{
    fn new(v: Value<T>) -> Self {
        ValueRef {
            r: Rc::new(RefCell::new(v)),
        }
    }

    fn new_value(v: T) -> ValueRef<T> {
        let v = Value::new_value(v);
        ValueRef {
            r: Rc::new(RefCell::new(v)),
        }
    }

    fn borrow(&self) -> Ref<Value<T>> {
        self.r.borrow()
    }

    // fn add(a: &ValueRef<T>, b: &ValueRef<T>) -> Self
    // where
    //     <T as Add>::Output: Clone,
    //     <T as Add>::Output: Add,
    //     <T as Mul>::Output: Clone,
    //     <T as Mul>::Output: Mul,
    // {
    //     let x1 = a.borrow().clone();
    //     let x2 = b.borrow().clone();
    //     let v = Value {
    //         data: x1.data + x2.data,
    //         op: OpEnum::ADD,
    //         //   grad: 1.0,
    //         inp1: Some(a.clone()),
    //         inp2: Some(b.clone()),
    //     };
    //     ValueRef::new(v)
    // }
    //
    // fn mul(a: &ValueRef<T>, b: &ValueRef<T>) -> Self
    // where
    //     <T as Add>::Output: Clone,
    //     <T as Add>::Output: Add,
    //     <T as Mul>::Output: Clone,
    //     <T as Mul>::Output: Mul,
    // {
    //     let x1 = a.borrow().clone();
    //     let x2 = b.borrow().clone();
    //     let v = Value {
    //         data: x1.data * x2.data,
    //         op: OpEnum::MUL,
    //         //   grad: 1.0,
    //         inp1: Some(a.clone()),
    //         inp2: Some(b.clone()),
    //     };
    //     ValueRef::new(v)
    // }
}

impl<T> Value<T>
where
    T: Clone + Add + Mul,
{
    fn new(data: T, op: OpEnum) -> Self {
        Value {
            data,
            op,
            //      grad: T::default(),
            inp1: None,
            inp2: None,
        }
    }

    fn new_value(data: T) -> Self {
        Value {
            data,
            op: OpEnum::NONE,
            //         grad: T::default(),
            inp1: None,
            inp2: None,
        }
    }
}

//
impl<'a, 'b, T> Add<&'b ValueRef<T>> for &'a ValueRef<T>
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T>,
    <T as Add>::Output: Clone,
    <T as Mul>::Output: Clone,
    <T as Add>::Output: Add,
    <T as Mul>::Output: Mul,
    <T as Add>::Output: Mul,
{
    type Output = ValueRef<T>;

    fn add(self, rhs: &'b ValueRef<T>) -> Self::Output {
        let x1 = self.r.borrow().clone();
        let x2 = rhs.r.borrow().clone();
        let v = Value {
            data: x1.data + x2.data,
            op: OpEnum::ADD,
            //   grad: 1.0,
            inp1: Some(self.clone()),
            inp2: Some(rhs.clone()),
        };
        ValueRef::new(v)
    }

    // fn add(self, rhs: &'b ValueRef<T>) -> ValueRef<T> {
    //     let v = ValueRef::<T>::blupp(self, rhs, OpEnum::ADD);
    //     ValueRef::new(v)
    //}
}

impl<'a, 'b, T> Mul<&'b ValueRef<T>> for &'a ValueRef<T>
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T>,
    <T as Add>::Output: Clone,
    <T as Mul>::Output: Clone,
    <T as Add>::Output: Add,
    <T as Mul>::Output: Mul,
    <T as Add>::Output: Mul,
{
    type Output = ValueRef<T>;

    fn mul(self, rhs: &'b ValueRef<T>) -> Self::Output {
        let x1 = self.r.borrow().clone();
        let x2 = rhs.r.borrow().clone();
        let v = Value {
            data: x1.data * x2.data,
            op: OpEnum::ADD,
            //   grad: 1.0,
            inp1: Some(self.clone()),
            inp2: Some(rhs.clone()),
        };
        ValueRef::new(v)
    }
}

impl Display for OpEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpEnum::ADD => write!(f, "ADD"),
            OpEnum::NONE => write!(f, "NONE"),
            OpEnum::MUL => write!(f, "MUL"),
        }
    }
}

impl<T> Default for Value<T>
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default,
    <T as Add>::Output: Clone,
    <T as Mul>::Output: Clone,
    <T as Add>::Output: Add,
    <T as Mul>::Output: Mul,
    <T as Add>::Output: Mul,
{
    fn default() -> Self {
        Value {
            data: T::default(),
            op: OpEnum::NONE,
            //      grad: T::default(),
            inp1: None,
            inp2: None,
        }
    }
}

impl<T> Default for ValueRef<T>
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default,
    <T as Add>::Output: Clone,
    <T as Mul>::Output: Clone,
    <T as Add>::Output: Add,
    <T as Mul>::Output: Mul,
    <T as Add>::Output: Mul,
{
    fn default() -> Self {
        ValueRef::new(Value::default())
    }
}
