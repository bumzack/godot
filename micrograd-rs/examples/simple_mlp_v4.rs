use micrograd_rs::micrograd_rs_engine_v4_tensor::{print_predictions, Network, RandomUniformInitializer, FC, SGD};
use micrograd_rs::micrograd_rs_v4_mathtensor::MathTensor;
use micrograd_rs::micrograd_rs_v4_tensor::Tensor;

fn main() {
    let epochs = 100;

    let mut initializer = RandomUniformInitializer::new();
    let mut mlp = Network::new();
    let l1 = FC::new(3, 4, "input_layer".to_string(), &mut initializer);
    let l2 = FC::new(4, 4, "hidden_layer".to_string(), &mut initializer);
    let l3 = FC::new(4, 1, "output_layer".to_string(), &mut initializer);

    mlp.add_layer(Box::new(l1));
    mlp.add_layer(Box::new(l2));
    mlp.add_layer(Box::new(l3));

    let optimizer = SGD::new(0.9, epochs as f64);
    mlp.optimizer(Box::new(optimizer));

    // input values
    let mut xs = vec![
        vec![2.0, 3.0, -1.0],
        vec![2.0, -1.0, 0.5],
        vec![0.5, 1.0, 1.0],
        vec![1.0, 1.0, -1.0],
    ];
    let mut y_ground_truth = vec![1.0, -1.0, -1.0, 1.0];

    let shape = vec![3, 4];
    let mut data = vec![];
    xs.iter_mut().for_each(|x| {
        data.append(x);
    });

    let xs = MathTensor::new(shape, data);
    let xs = Tensor::from(xs, "xs".to_string());

    // desired targets
    let shape = vec![1, 4];
    let mut data = vec![];
    y_ground_truth.iter().for_each(|y| {
        data.push(*y);
    });
    let y_ground_truth = MathTensor::new(shape, data);
    let y_ground_truth = Tensor::from(y_ground_truth, "xs".to_string());

    let mut y_pred = Tensor::ones(vec![1, 1], "y_pred".to_string());

    for i in 0..100 {
        // forward pass
        let y_pred = mlp.forward(&xs);

        // calculate loss
        let mut loss = mlp.calc_loss(&y_ground_truth, &y_pred, mlp.parameters());

        // print_params(&mlp);
        // backward pass consists of 2 steps
        mlp.reset_grades();
        loss.backward();

        // update parameters
        mlp.update(i);

        // keep track of loss improvement
        let l = loss.r().borrow();
        let l = l.t().elem(vec![0, 0]);
        println!("iteration {}   loss {} ", i + 1, l);
    }

    print_predictions(&y_pred, &y_ground_truth);
    // print_params(&mlp);
    println!("DONE");
}
