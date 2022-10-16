use std::cell::{Ref, RefCell};
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::rc::Rc;

fn main() {
    let a = ValueRef::new_value(2.0);
    let b = ValueRef::new_value(3.0);

    let c = ValueRef::add(&a, &b);

    //  println!("c = {}", c.r.borrow().data);
    let d = ValueRef::new(Value::new_value(8.0));

    // let e = c * d;

    // println!("a = {:?}", &*a.r.borrow());
    // println!("b = {:?}", &*b.r.borrow());
    //  println!("c = {:?}", &*c.r.borrow());
    // println!("d = {}", &*d.0.borrow());
    // println!("e = {}", &*e.0.borrow());

    println!("DONE");
}

#[derive(Debug, Clone)]
enum OpEnum {
    NONE,
    ADD,
    //   MUL,
}

#[derive(Debug, Clone)]
struct Value {
    data: f64,
    op: OpEnum,
    //  grad: T,
    inp1: Option<ValueRef>,
    inp2: Option<ValueRef>,
}

#[derive(Debug, Clone)]
struct ValueRef {
    r: Rc<RefCell<Value>>,
}

impl ValueRef {
    fn new(v: Value) -> Self {
        ValueRef {
            r: Rc::new(RefCell::new(v)),
        }
    }

    fn new_value(v: f64) -> ValueRef {
        let v = Value::new_value(v);
        ValueRef {
            r: Rc::new(RefCell::new(v)),
        }
    }

    fn borrow(&self) -> Ref<Value> {
        self.r.borrow()
    }

    fn add(a: &ValueRef, b: &ValueRef) -> Self {
        let x1 = a.borrow();
        let x2 = b.borrow();
        let v = Value {
            data: &x1.data + &x2.data,
            op: OpEnum::ADD,
            //   grad: 1.0,
            inp1: Some(a.clone()),
            inp2: Some(b.clone()),
        };
        ValueRef::new(v)
    }
}

impl Value
{
    fn new(data: f64, op: OpEnum) -> Self {
        Value {
            data,
            op,
            //      grad: T::default(),
            inp1: None,
            inp2: None,
        }
    }

    fn new_value(data: f64) -> Self {
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
// impl<'a, T> Add<ValueRef<T>> for ValueRef<T>
//     where
//         T: Add + Default + Display+'a,
//         &'a T: Add<&'a T>,
// {
//     type Output = ValueRef<T>;
//
//     fn add(self, rhs: ValueRef<T>) -> Self::Output {
//         let x1 = &self.r.clone().borrow().data;
//         let x2 = &rhs.r.clone().borrow().data;
//         let v = Value {
//             data: x1 + x2,
//             op: OpEnum::ADD,
//             grad: T::default(),
//             inp1: None,
//             inp2: None,
//         };
//         ValueRef::new(v)
//     }
// }
//
// impl<T: Add + Default + Display> Add<T> for Value<T> {
//     type Output = Value<T>;
//
//     fn add(self, rhs: T) -> Self::Output {
//         Value {
//             data: &self.0.borrow().data + &rhs.0.borrow().data,
//             op: OpEnum::ADD,
//             grad: T::default(),
//             inp1: None,
//             inp2: None,
//         }
//     }
// }
// //
// // impl<'a, T: Mul<Output=T> + Default + Display + Mul<Output=T>> Mul<ValueRef<T>> for ValueRef<T>
// //     where T: Add<T, Output=T> + Mul<T, Output=T>+'a, &'a T: Add<&'a T>,&'a T: Mul<&'a T>
// // {
// //     type Output = ValueRef<T>;
// //
// //     fn mul(self, rhs: ValueRef<T>) -> Self::Output {
// //         let x1 = &self.r.clone().borrow().data;
// //         let x2 = &rhs.r.clone().borrow().data;
// //         let v = Value {
// //             data: x1 * x2,
// //             op: OpEnum::ADD,
// //             grad: T::default(),
// //             inp1: None,
// //             inp2: None,
// //         };
// //         ValueRef::new(v)
// //     }
// // }
// //
// // impl<T: Add<Output=T> + Default + Display + Mul<Output=T>> Mul<T> for Value<T>
// //     where T: Add<T, Output=T> + Mul<T, Output=T>,
// // {
// //     type Output = Value<T>;
// //
// //     fn mul(self, rhs: T) -> Self::Output {
// //         Value {
// //             data: &self.0.borrow().data * &rhs.0.borrow().data,
// //             op: OpEnum::ADD,
// //             grad: T::default(),
// //             inp1: None,
// //             inp2: None,
// //         }
// //     }
// // }
//
// impl<T> Default for Value<T>
//     where
//         T: Default + Display + Add,
// {
//     fn default() -> Self {
//         Value {
//             data: T::default(),
//             op: OpEnum::NONE,
//             grad: T::default(),
//             inp1: None,
//             inp2: None,
//         }
//     }
// }

//
// impl<T> Display for Value<T>
//     where
//         T: Display + Default + Add,
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Value data {}, op {}, grad {} ", self.data, self.op, self.grad)
//     }
// }

impl Display for OpEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpEnum::ADD => write!(f, "ADD"),
            OpEnum::NONE => write!(f, "NONE"),
            //       OpEnum::MUL => write!(f, "MUL"),
        }
    }
}

// impl<'a, T> Default for ValueRef<T>
//     where
//         T: Display + Default + Add + Add<&'a T> + 'a,
//         &'a T: Add<&'a T>,
// {
//     fn default() -> Self {
//         todo!()
//     }
// }
