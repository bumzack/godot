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

    fn helper(out: &Tensor, t: &Tensor) -> MathTensor {
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

        let t = Self::helper(&out, &self__);
        self__.set_grad(t);

        let t = Self::helper(&out, &other);
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

    fn helper_add(out: &Tensor, t: &Tensor) -> MathTensor {
        let x = t.r().borrow();
        let grad = x.grad();
        let tensor = grad + &(1.0 * out.r().borrow().grad());
        tensor
    }

    fn helper_sub(out: &Tensor, t: &Tensor) -> MathTensor {
        let x = t.r().borrow();
        let grad = x.grad();
        let tensor = grad - &(1.0 * out.r().borrow().grad());
        tensor
    }
}

impl Backward for BackwardSub {
    fn apply(&self, out: Tensor) {
        let mut self__ = self.left.clone();
        let mut other = self.right.clone();

        let t = Self::helper_add(&out, &self__);
        self__.set_grad(t);

        let t = Self::helper_sub(&out, &other);
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

    fn bla(out: &Tensor, self__: &Tensor, other: &Tensor) -> MathTensor {
        let x = other.borrow();
        let x = x.t();
        let tensor = self__.r().borrow().grad() + &(x * out.r().borrow().grad());
        tensor
    }

    fn bla2(out: &Tensor, self__: &Tensor, other: &Tensor) -> MathTensor {
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
        let tensor = Self::bla(&out, &self__, &other);
        self__.set_grad(tensor);
        let tensor1 = Self::bla2(&out, &self__, &other);
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

    fn helper(out: Tensor, self__: &Tensor) -> MathTensor {
        let x = out.r().borrow();
        let x = x.t();
        let y = 1.0 - &(x * x);
        let x1 = self__.r().borrow();
        let x1 = x1.grad();
        let x2 = out.r().borrow();
        let x2 = x2.grad();
        let res = x1 + &(&y * x2);
        res
    }
}

impl Backward for BackwardTanh {
    fn apply(&self, out: Tensor) {
        let mut self__ = self.left.clone();
        let res = Self::helper(out, &self__);
        self__.set_grad(res);

        // let x = out.get_data();
        // let y = 1.0 - x * x;
        // self__.set_grad(self__.get_grad() + y * out.r.borrow().grad());
    }
}

pub struct BackwardExp {
    left: Tensor,
}

impl BackwardExp {
    pub fn new(left: Tensor) -> BackwardExp {
        BackwardExp { left }
    }

    fn helper(out: Tensor, self__: &Tensor) -> MathTensor {
        let x = self__.r().borrow();
        let x = x.grad();
        let out = out.r().borrow();
        let y = out.t();
        let z = out.grad();
        let x1 = x + &(y * z);
        x1
    }
}

impl Backward for BackwardExp {
    fn apply(&self, out: Tensor) {
        let mut self__ = self.left.clone();
        let x1 = Self::helper(out, &self__);
        self__.set_grad(x1);

        // let mut self__ = self.left.clone();
        // self__.set_grad(self__.get_grad() + out.r().borrow().data() * out.r.borrow().grad());
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
        let mut self__ = self.left.clone();
        let other = self.right;
        let tensor = self__.r().borrow().t().pow(other - 1.0);
        let x = &(other * &tensor) * out.r().borrow().grad();
        let res = self__.r().borrow().grad() + &x;
        self__.set_grad(res);
    }
}

pub struct BackwardReLU {
    left: Tensor,
}

impl BackwardReLU {
    pub fn new(left: Tensor) -> BackwardReLU {
        BackwardReLU { left }
    }

    fn helper(out: Tensor, self__: &Tensor) -> MathTensor {
        let x1 = out.r().borrow();
        let t = x1.t();
        let grad = x1.grad();

        let shape: Vec<usize> = t.shape_vec().iter().map(|s| *s).collect();
        let data: Vec<f64> = t
            .data()
            .iter()
            .enumerate()
            .map(|(idx, d)| if *d > 0.0 { grad.data()[idx] } else { 0.0 })
            .collect();

        let x = MathTensor::new(shape, data);
        let y = self__.r().borrow();
        let y = y.grad();
        let res = y + &x;
        res
    }
}

impl Backward for BackwardReLU {
    fn apply(&self, out: Tensor) {
        let mut self__ = self.left.clone();

        let res = Self::helper(out, &self__);
        self__.set_grad(res);

        // println!("ReLU out {:?},  'self' {:?} ", out, self.left);
        // let mut self__ = self.left.clone();
        // let x = if out.get_data() > 0.0 { out.get_grad() } else { 0.0 };
        // self__.set_grad(self__.get_grad() + x);
    }
}
