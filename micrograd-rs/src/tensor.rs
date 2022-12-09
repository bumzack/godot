use std::cell::{Ref, RefCell};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};
use std::rc::Rc;


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
