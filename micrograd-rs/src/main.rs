use std::fmt::{Display, Formatter};
use std::ops::Add;

fn main() {
    let a = Value::new_value(2.0);
    let b = Value::new_value(3.0);

    let c = &a + &b;

    println!("a = {}", &a);
    println!("b = {}", &b);
    println!("c = {}", &c);
}

enum OpEnum {
    NONE,
    ADD,
}

struct Value<T: Add + Default> {
    data: T,
    op: OpEnum,
    grad: T,
    prev: Vec<Value<T>>,
}

impl<T: Add + Default> Value<T> {
    fn new(data: T, op: OpEnum) -> Self {
        Value {
            data,
            op,
            grad: T::default(),
            prev: vec![],
        }
    }

    fn new_value(data: T) -> Self {
        Value {
            data,
            op: OpEnum::NONE,
            grad: T::default(),
            prev: vec![],
        }
    }
}

// impl<'a, 'b> Sub<&'b Tuple4D> for &'a Tuple4D {
//
// }
impl<'b, 'a: 'b, T: Add<Output=T> + Default> Add<&'b Value<T>> for &'a Value<T>
    where &'b T: Add<&'b T, Output=T> {
    type Output = Value<T>;

    fn add(self, rhs: &'b Value<T>) -> Self::Output {
        Value {
            data: &self.data + &rhs.data,
            op: OpEnum::ADD,
            grad: T::default(),
            prev: vec![],
        }
    }
}

impl<T> Default for Value<T>
    where T: Default + Add + Display {
    fn default() -> Self {
        Value {
            data: T::default(),
            op: OpEnum::NONE,
            grad: T::default(),
            prev: vec![],
        }
    }
}

impl<T> Display for Value<T>
    where T: Display + Add + Default {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value data {}, op {}, grad {} ", self.data, self.op, self.grad)
    }
}

impl Display for OpEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpEnum::ADD => write!(f, "ADD"),
            OpEnum::NONE => write!(f, "NONE"),
        }
    }
}
