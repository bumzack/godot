use rand::distributions::Uniform;
use rand::prelude::StdRng;
use rand::SeedableRng;

use micrograd_rs::micrograd_rs_engine_v3::{print_predictions, Network, FC, SGD};

fn main() {
    let mut r = StdRng::seed_from_u64(1337);
    let normal = Uniform::from(-1.0..1.0);

    let epochs = 100;

    let mut mlp = Network::new();
    let l1 = FC::new(3, 4, &normal, &mut r);
    let l2 = FC::new(4, 4, &normal, &mut r);
    let l3 = FC::new(4, 1, &normal, &mut r);

    mlp.add_layer(Box::new(l1));
    mlp.add_layer(Box::new(l2));
    mlp.add_layer(Box::new(l3));

    let optimizer = SGD::new(0.9, epochs as f64);
    mlp.optimizer(Box::new(optimizer));

    // input values
    let xs = vec![
        vec![2.0, 3.0, -1.0],
        vec![2.0, -1.0, 0.5],
        vec![0.5, 1.0, 1.0],
        vec![1.0, 1.0, -1.0],
    ];

    // desired targets
    let ys = vec![1.0, -1.0, -1.0, 1.0];
    let mut y_pred = vec![];

    for i in 0..200 {
        // forward pass
        y_pred = mlp.forward(&xs);

        // calculate loss
        let (mut loss, accuracy) = mlp.calc_loss(&ys, &y_pred, mlp.parameters());

        // print_params(&mlp);
        // backward pass consists of 2 steps
        mlp.reset_grades();
        loss.backward();

        // update parameters
        mlp.update(i);

        // keep track of loss improvement
        println!(
            "iteration {}   loss {}  accuracy {:.2}",
            i + 1,
            loss.get_data(),
            accuracy * 100.0
        );
    }

    print_predictions(y_pred, &ys);
    // print_params(&mlp);
    println!("DONE");
}