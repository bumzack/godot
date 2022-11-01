use rand::distributions::Uniform;
use rand::prelude::*;

use crate::micrograd_rs_v3::{EPS2, ValueRefV3};

pub struct Neuron {
    weights: Vec<ValueRefV3>,
    bias: ValueRefV3,
}

impl Neuron {
    pub fn new(nin: usize, normal: &Uniform<f64>, rng: &mut StdRng) -> Neuron {
        // let between = Uniform::from(-1.0..1.0);
        let mut weights = vec![];
        for i in 0..nin {
            let y: f64 = normal.sample(rng);
            weights.push(ValueRefV3::new_value(y, format!("weight {}", i)));
        }
        let bias = ValueRefV3::new_value(0.0, format!("bias"));
        Neuron { weights, bias }
    }

    pub fn new_weights_bias(weights: Vec<f64>, bias: f64) -> Neuron {
        let weights = weights
            .iter()
            .map(|w| ValueRefV3::new_value(*w, "w".to_string()))
            .collect();
        let bias = ValueRefV3::new_value(bias, "b".to_string());
        Neuron { weights, bias }
    }

    pub fn forward(&self, xinp: &Vec<ValueRefV3>) -> ValueRefV3 {
        assert!(xinp.len() == self.weights.len(), "input size does not match layer size");

        let x_w: Vec<ValueRefV3> = xinp
            .iter()
            .zip(self.weights.iter())
            .into_iter()
            .map(|(x, w)| x * w)
            .collect();

        let mut sum = ValueRefV3::new_value(0.0, "sum".to_string());
        for v in x_w {
            sum += v;
        }
        (&sum + &self.bias).tanh()
    }

    pub fn parameters(&self) -> Vec<ValueRefV3> {
        let mut params = vec![];
        self.weights.iter().for_each(|w| params.push(w.clone()));
        params.push(self.bias.clone());
        params
    }
}

pub trait Layer {
    fn forward(&self, xinp: &Vec<ValueRefV3>) -> Vec<ValueRefV3>;
    fn parameters(&self) -> Vec<ValueRefV3>;
}

pub struct FC {
    neurons: Vec<Neuron>,
}

impl FC {
    pub fn new(nin: usize, nout: usize, normal: &Uniform<f64>, rng: &mut StdRng) -> FC {
        let mut neurons = vec![];
        for _i in 0..nout {
            neurons.push(Neuron::new(nin, normal, rng));
        }
        FC { neurons }
    }
}

impl Layer for FC {
    fn forward(&self, xinp: &Vec<ValueRefV3>) -> Vec<ValueRefV3> {
        let mut out = vec![];
        for n in &self.neurons {
            out.push(n.forward(xinp))
        }
        out
    }

    fn parameters(&self) -> Vec<ValueRefV3> {
        let mut params = vec![];
        self.neurons.iter().for_each(|n| params.append(&mut n.parameters()));
        params
    }
}

pub struct Network {
    layers: Vec<Box<dyn Layer>>,
    loss: Box<dyn Loss>,
    optimizer: Box<dyn Optimizer>,
}

impl Network {
    pub fn new() -> Box<Network> {
        //TODO fix total_epochs = 0 mess
        let optimizer = Box::new(SGD::new(0.9, 0.0));
        let loss = Box::new(MaxMarginLoss::new());
        Box::new(Network {
            layers: vec![],
            loss,
            optimizer,
        })
    }

    pub fn optimizer(&mut self, optimizer: Box<dyn Optimizer>) {
        self.optimizer = optimizer;
    }

    pub fn loss(&mut self, loss: Box<dyn Loss>) {
        self.loss = loss;
    }

    // pub fn new_fully_connected(nin: usize, mut nouts: Vec<usize>) -> Box<Network> {
    //     let mut sizes = vec![];
    //     sizes.push(nin);
    //     sizes.append(&mut nouts);
    //
    //     let mut network = Network::new();
    //     for i in 0..sizes.len() - 1 {
    //         // println!("new layer nin {} -> nout {}", sizes[i], sizes[i + 1]);
    //         network.add_layer(Box::new(FC::new(sizes[i], sizes[i + 1])));
    //     }
    //     network
    // }

    pub fn add_layer(&mut self, l: Box<dyn Layer>) {
        self.layers.push(l);
    }

    fn forward_internal<'a>(&'a self, xinp: &Vec<f64>) -> Vec<ValueRefV3> {
        let mut x = xinp
            .iter()
            .map(|x| ValueRefV3::new_value(*x, "x".to_string()))
            .collect();
        for (_idx, l) in self.layers.iter().enumerate() {
            // // println!("forward layer idx {}", idx);
            x = l.forward(&x);
        }
        x
    }

    pub fn parameters(&self) -> Vec<ValueRefV3> {
        let mut params = vec![];
        self.layers.iter().for_each(|l| params.append(&mut l.parameters()));
        params
    }

    pub fn forward(&self, xs: &Vec<Vec<f64>>) -> Vec<ValueRefV3> {
        let mut y_pred: Vec<ValueRefV3> = vec![];
        for x in xs.iter() {
            let y = self.forward_internal(x);
            y_pred.push(y.get(0).unwrap().clone());
        }
        y_pred
    }

    pub fn print_params(&self) {
        println!("before param update");
        for p in self.parameters() {
            println!(
                "parameter '{}': data {}, grad {}",
                p.borrow().label(),
                p.get_data(),
                p.get_grad()
            );
        }
    }

    pub fn reset_grades(&self) {
        self.parameters().iter().for_each(|p| p.clone().set_grad(0.0));
    }

    pub fn update(&self, epoch: usize) {
        self.optimizer.update(self.parameters(), epoch);
    }

    pub fn calc_loss(
        &self,
        y_ground_truth: &Vec<f64>,
        y_pred: &Vec<ValueRefV3>,
        parameters: Vec<ValueRefV3>,
    ) -> ValueRefV3 {
        self.loss.calc_loss(y_ground_truth, y_pred, parameters)
    }
}

pub trait Loss {
    fn calc_loss(&self, y_ground_truth: &Vec<f64>, y_pred: &Vec<ValueRefV3>, parameters: Vec<ValueRefV3>)
                 -> ValueRefV3;
}

pub struct MSELoss {}

impl MSELoss {
    pub fn new() -> MSELoss {
        MSELoss {}
    }
}

impl Loss for MSELoss {
    fn calc_loss(
        &self,
        y_ground_truth: &Vec<f64>,
        y_pred: &Vec<ValueRefV3>,
        _parameters: Vec<ValueRefV3>,
    ) -> ValueRefV3 {
        let loss_vec: Vec<ValueRefV3> = y_pred
            .iter()
            .zip(y_ground_truth.iter())
            .into_iter()
            .map(|(ypred, ygr)| (ypred - *ygr).pow(2.0))
            .collect();
        //loss_vec.iter().for_each(|y| println!("loss_vec = {}", y.get_data()));
        let mut loss = ValueRefV3::new_value(0.0, "loss".to_string());
        for l in loss_vec.iter() {
            // println!("loss {} += l {} ", loss, l.get_data());
            loss = &loss + l;
        }
        loss
    }
}

pub struct MaxMarginLoss {}

impl MaxMarginLoss {
    pub fn new() -> MaxMarginLoss {
        MaxMarginLoss {}
    }
}

impl Loss for MaxMarginLoss {
    fn calc_loss(
        &self,
        y_ground_truth: &Vec<f64>,
        y_pred: &Vec<ValueRefV3>,
        parameters: Vec<ValueRefV3>,
    ) -> ValueRefV3 {
        let loss_vec: Vec<ValueRefV3> = y_pred
            .iter()
            .zip(y_ground_truth.iter())
            .into_iter()
            .map(|(ypred, ygr)| (1.0_f64 + &((-*ygr) * ypred)).relu())
            .collect();
        //loss_vec.iter().for_each(|y| println!("loss_vec = {}", y.get_data()));
        let mut loss = ValueRefV3::new_value(0.0, "loss".to_string());
        for l in loss_vec.iter() {
            // println!("loss {} += l {} ", loss, l.get_data());
            loss = &loss + l;
        }
        let data_loss = loss / loss_vec.len() as f64;
        let alpha = 0.0001_f64;
        // let sum_parameters = parameters.iter().map(|p| p * p).collect();
        let mut reg_loss = ValueRefV3::new_value(0.0, "reg_loss".to_string());
        parameters.iter().for_each(|p| reg_loss = &reg_loss + &(p * p));
        reg_loss = &reg_loss * alpha;
        let total_loss = &reg_loss + &data_loss;
        println!(
            "reg_loss {},   data_loss {},   total_loss {}",
            reg_loss.get_data(),
            data_loss.get_data(),
            total_loss.get_data()
        );
        // accuracy

        total_loss
    }
}

pub trait Optimizer {
    fn update(&self, parameters: Vec<ValueRefV3>, epoch: usize);
}

pub struct SGD {
    learning_rate: f64,
    totol_epochs: f64,
}

impl Optimizer for SGD {
    fn update(&self, mut parameters: Vec<ValueRefV3>, epoch: usize) {
        let lr = 1.0 - self.learning_rate * epoch as f64 / self.totol_epochs as f64;
        for    p in parameters.iter_mut() {
            let x = p.get_data();
            let grad = p.get_grad();
            p.set_data(x - (lr * grad));
        }
        println!(
            "epoch: {}/{}, learning_rate {}, actual learning_rate {}",
            epoch, self.totol_epochs, self.learning_rate, lr
        );
    }
}

impl SGD {
    pub fn new(learning_rate: f64, totol_epochs: f64) -> SGD {
        SGD {
            learning_rate,
            totol_epochs,
        }
    }
}

pub fn print_predictions(y_pred: Vec<ValueRefV3>, y_expected: &Vec<f64>) {
    y_pred.iter().enumerate().for_each(|(idx, y)| {
        let res = (y.get_data() - y_expected[idx]).abs() < EPS2;
        println!(
            "y_pred[{}] = {}    expected {}.   pred ok? {}",
            idx,
            y.get_data(),
            y_expected[idx],
            res
        );
    });
}

#[cfg(test)]
mod tests {

}
