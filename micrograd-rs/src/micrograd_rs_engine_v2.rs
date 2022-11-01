use rand::distributions::Uniform;
use rand::prelude::*;

use crate::micrograd_rs_v2::{ValueRefV2,    EPS2};

pub struct Neuron {
    weights: Vec<ValueRefV2>,
    bias: ValueRefV2,
}

impl Neuron {
    pub fn new(nin: usize) -> Neuron {
        let between = Uniform::from(-1.0..1.0);
        let mut rng = rand::thread_rng();
        let mut weights = vec![];
        for i in 0..nin {
            let y: f64 = between.sample(&mut rng);
            weights.push(ValueRefV2::new_value(y, format!("weight {}", i)));
        }
        let bias: f64 = between.sample(&mut rng);
        let bias = ValueRefV2::new_value(bias, format!("bias"));
        Neuron { weights, bias }
    }

    pub fn new_weights_bias(weights: Vec<f64>, bias: f64) -> Neuron {
        let weights = weights
            .iter()
            .map(|w| ValueRefV2::new_value(*w, "w".to_string()))
            .collect();
        let bias = ValueRefV2::new_value(bias, "b".to_string());
        Neuron { weights, bias }
    }

    pub fn forward(&self, xínp: &Vec<ValueRefV2>) -> ValueRefV2 {
        assert!(xínp.len() == self.weights.len(), "input size does not match layer size");

        let x_w: Vec<ValueRefV2> = xínp
            .iter()
            .zip(self.weights.iter())
            .into_iter()
            .map(|(x, w)| x * w)
            .collect();

        let mut sum = ValueRefV2::new_value(0.0, "sum".to_string());
        for v in x_w {
            sum += v;
        }
        (&sum + &self.bias).tanh()
    }

    pub fn parameters(&self) -> Vec<ValueRefV2> {
        let mut params = vec![];
        self.weights.iter().for_each(|w| params.push(w.clone()));
        params.push(self.bias.clone());
        params
    }
}

pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(nin: usize, nout: usize) -> Layer {
        let mut neurons = vec![];
        for _i in 0..nout {
            neurons.push(Neuron::new(nin));
        }
        Layer { neurons }
    }

    pub fn forward(&self, xínp: &Vec<ValueRefV2>) -> Vec<ValueRefV2> {
        let mut out = vec![];
        for n in &self.neurons {
            out.push(n.forward(xínp))
        }
        out
    }

    pub fn parameters(&self) -> Vec<ValueRefV2> {
        let mut params = vec![];
        self.neurons.iter().for_each(|n| params.append(&mut n.parameters()));
        params
    }
}

pub struct MLP {
    layers: Vec<Layer>,
}

impl MLP {
    pub fn new(nin: usize, mut nouts: Vec<usize>) -> MLP {
        let mut layers = vec![];

        let mut sizes = vec![];
        sizes.push(nin);
        sizes.append(&mut nouts);

        for i in 0..sizes.len() - 1 {
            println!("new layer nin {} -> nout {}", sizes[i], sizes[i + 1]);
            layers.push(Layer::new(sizes[i], sizes[i + 1]));
        }
        MLP { layers }
    }

    fn forward_internal<'a>(&'a self, xinp: &Vec<f64>) -> Vec<ValueRefV2> {
        let mut x = xinp
            .iter()
            .map(|x| ValueRefV2::new_value(*x, "x".to_string()))
            .collect();
        for (_idx, l) in self.layers.iter().enumerate() {
            // // println!("forward layer idx {}", idx);
            x = l.forward(&x);
        }
        x
    }

    pub fn parameters(&self) -> Vec<ValueRefV2> {
        let mut params = vec![];
        self.layers.iter().for_each(|l| params.append(&mut l.parameters()));
        params
    }

    pub fn forward(&self, xs: &Vec<Vec<f64>>) -> Vec<ValueRefV2> {
        let mut y_pred: Vec<ValueRefV2> = vec![];
        for x in xs.iter() {
            let y = self.forward_internal(x);
            y_pred.push(y.get(0).unwrap().clone());
        }

        // print_predictions(&mut y_pred);
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

    pub fn update(&self) {
        for p in self.parameters().iter() {
            let x = p.get_data();
            let grad = p.get_grad();
            let mut p = p.clone();
            p.set_data(x + (-0.1 * grad));
        }
    }

    pub fn update2(&self, k: usize, totol_epochs: usize) {
        let learning_rate = 1.0 - 0.9 * k as f64 / totol_epochs as f64;
        println!("learning rate {:.2}", learning_rate);
        for p in self.parameters().iter() {
            let x = p.get_data();
            let grad = p.get_grad();
            let mut p = p.clone();
            p.set_data(x + (-learning_rate * grad));
        }
    }
}

pub fn calc_loss_mse(ys: &Vec<f64>, y_pred: &Vec<ValueRefV2>) -> ValueRefV2 {
    let loss_vec: Vec<ValueRefV2> = y_pred
        .iter()
        .zip(ys.iter())
        .into_iter()
        .map(|(ypred, ygr)| (ypred - *ygr).pow(2.0))
        .collect();
    //loss_vec.iter().for_each(|y| println!("loss_vec = {}", y.get_data()));
    let mut loss = ValueRefV2::new_value(0.0, "loss".to_string());
    for l in loss_vec.iter() {
        // println!("loss {} += l {} ", loss, l.get_data());
        loss = &loss + l;
    }
    loss
}

pub fn print_predictions(y_pred: Vec<ValueRefV2>, y_expected: &Vec<f64>) {
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
    use crate::micrograd_rs_engine_v2::{Layer, Neuron, MLP};
    use crate::micrograd_rs_v2::{assert_two_float, ValueRefV2};

    // TODO
    // add a method to initialize the weights by hand and not randomly
    #[test]
    pub fn test_neuron() {
        let neuron = Neuron::new_weights_bias(vec![2.0, 3.0], 2.0);
        let xinp = vec![
            ValueRefV2::new_value(11.0_f64, "x1".to_string()),
            ValueRefV2::new_value(12.0_f64, "x2".to_string()),
        ];
        let output = neuron.forward(&xinp);

        println!("output = {}", output.get_data());

        // TODO
        // check if this is really correct
        assert_two_float(output.get_data(), 60.0_f64.tanh());
    }

    #[test]
    pub fn test_layer() {
        let layer = Layer::new(2, 3);
        let xinp = vec![
            ValueRefV2::new_value(1.0_f64, "x1".to_string()),
            ValueRefV2::new_value(21.0_f64, "x2".to_string()),
        ];
        let output = layer.forward(&xinp);

        output
            .iter()
            .enumerate()
            .for_each(|(idx, o)| println!("output neuron {}= {}", idx, o.get_data()));

        // TODO
        // how to initialize the layers with specific weights?
        // let expected_values = [1.0, 2.0, 3.0];
        // output.iter().enumerate().for_each(|(idx, o)| assert_two_float(o.get_data(), expected_values[idx]));
    }

    #[test]
    pub fn test_mlp() {
        let mlp = MLP::new(3, vec![4, 4, 1]);
        let xinp = vec![vec![1.0_f64, 2.0_f64, 3.0_f64]];
        let output = mlp.forward(&xinp);

        output
            .iter()
            .enumerate()
            .for_each(|(idx, o)| println!("output mlp {}= {}", idx, o.get_data()));

        // TODO
        // how to initialize the layers with specific weights?
        //let expected_values = [1.0, 2.0, 3.0];
        // output.iter().enumerate().for_each(|(idx, o)| assert_two_float(o.get_data(), expected_values[idx]));
    }
}
