use crate::micrograd_rs_v4_mathtensor::MathTensor;
use crate::micrograd_rs_v4_tensor::Tensor;

pub trait Backward {
    fn apply(&self, out: Tensor);
}

pub struct BackwardAdd {
    left: Tensor,
    right: Tensor,
}

impl BackwardAdd {
    pub fn new(left: Tensor, right: Tensor) -> BackwardAdd {
        BackwardAdd { left, right }
    }

    fn helper(out: &Tensor, t: &mut Tensor) -> MathTensor {
        let x = t.r().borrow();
        let grad = x.grad();
        let tensor = grad + &(1.0 * out.r().borrow().grad());
        tensor
    }
}

impl Backward for BackwardAdd {
    fn apply(&self, out: Tensor) {
        let mut self__ = self.left.clone();
        let mut other = self.right.clone();

        let t = Self::helper(&out, &mut self__);
        self__.set_grad(t);

        let t = Self::helper(&out, &mut other);
        other.set_grad(t);
    }
}

pub struct BackwardSub {
    left: Tensor,
    right: Tensor,
}

impl BackwardSub {
    pub fn new(left: Tensor, right: Tensor) -> BackwardSub {
        BackwardSub { left, right }
    }

    fn helper_add(out: &Tensor, t: &mut Tensor) -> MathTensor {
        let x = t.r().borrow();
        let grad = x.grad();
        let tensor = grad + &(1.0 * out.r().borrow().grad());
        tensor
    }

    fn helper_sub(out: &Tensor, t: &mut Tensor) -> MathTensor {
        let x = t.r().borrow();
        let grad = x.grad();
        let tensor = grad + &(1.0 * out.r().borrow().grad());
        tensor
    }
}

impl Backward for BackwardSub {
    fn apply(&self, out: Tensor) {
        let mut self__ = self.left.clone();
        let mut other = self.right.clone();

        let t = Self::helper_add(&out, &mut self__);
        self__.set_grad(t);

        let t = Self::helper_sub(&out, &mut other);
        other.set_grad(t);
    }
}

pub struct BackwardMul {
    left: Tensor,
    right: Tensor,
}

impl BackwardMul {
    pub fn new(left: Tensor, right: Tensor) -> BackwardMul {
        BackwardMul { left, right }
    }

    fn bla(out: &Tensor, self__: &mut Tensor, other: &mut Tensor) -> MathTensor {
        let x = other.borrow();
        let x = x.t();
        let tensor = self__.r().borrow().grad() + &(x * out.r().borrow().grad());
        tensor
    }

    fn bla2(out: &Tensor, self__: &mut Tensor, other: &mut Tensor) -> MathTensor {
        let x = self__.borrow();
        let x = x.t();
        let tensor1 = other.r().borrow().grad() + &(x * out.r().borrow().grad());
        tensor1
    }
}

impl Backward for BackwardMul {
    fn apply(&self, out: Tensor) {
        let mut self__ = self.left.clone();
        let mut other = self.right.clone();
        let tensor = Self::bla(&out, &mut self__, &mut other);
        self__.set_grad(tensor);
        let tensor1 = Self::bla2(&out, &mut self__, &mut other);
        other.set_grad(tensor1);
    }
}

pub struct BackwardTanh {
    left: Tensor,
}

impl BackwardTanh {
    pub fn new(left: Tensor) -> BackwardTanh {
        BackwardTanh { left }
    }
}

impl Backward for BackwardTanh {
    fn apply(&self, out: Tensor) {
        // let mut self__ = self.left.clone();
        // let x = out.r().borrow();
        // let x = x.t();
        // let y = 1.0 - &(x * x);
        // let x1 = self__.get_grad() + y * out.r().borrow().grad();
        // self__.set_grad(x1);
    }
}

pub struct BackwardExp {
    left: Tensor,
}

impl BackwardExp {
    pub fn new(left: Tensor) -> BackwardExp {
        BackwardExp { left }
    }
}

impl Backward for BackwardExp {
    fn apply(&self, out: Tensor) {
        // let mut self__ = self.left.clone();
        // let x = self__.r().borrow();
        // let x=x.grad();
        // get_grad() + out.r().borrow().data() * out.r().borrow().grad();
        // self__.set_grad(x);
    }
}

pub struct BackwardPow {
    left: Tensor,
    right: f64,
}

impl BackwardPow {
    pub fn new(left: Tensor, right: f64) -> BackwardPow {
        BackwardPow { left, right }
    }
}

impl Backward for BackwardPow {
    fn apply(&self, out: Tensor) {
        // let mut self__ = self.left.clone();
        // let other = self.right.clone().borrow().data;
        // let x = other * (self__.borrow().data().powf(other - 1.0)) * out.r.borrow().grad();
        // self__.set_grad(self__.get_grad() + x);
    }
}

pub struct BackwardReLU {
    left: Tensor,
}

impl BackwardReLU {
    pub fn new(left: Tensor) -> BackwardReLU {
        BackwardReLU { left }
    }
}

impl Backward for BackwardReLU {
    fn apply(&self, out: Tensor) {
        // println!("ReLU out {:?},  'self' {:?} ", out, self.left);
        // let mut self__ = self.left.clone();
        // let x = if out.get_data() > 0.0 { out.get_grad() } else { 0.0 };
        // self__.set_grad(self__.get_grad() + x);
    }
}
