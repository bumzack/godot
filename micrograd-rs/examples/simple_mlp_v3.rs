use micrograd_rs::prelude::micrograd_rs_engine_v3::{calc_loss_mse, Net};
use micrograd_rs::prelude::{draw_graph, print_predictions, Net, ValueRefV2, FC, MLP};

fn main() {
    let mut mlp = Net::new();
    let l1 = FC::new(3, 4);
    let l2 = FC::new(4, 4);
    let l3 = FC::new(4, 1);

    mlp.add_layer(Box::new(l1));
    mlp.add_layer(Box::new(l2));
    mlp.add_layer(Box::new(l3));
    mlp.add_layer(Box::new(l4));

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

    for i in 0..2000 {
        // forward pass
        y_pred = mlp.forward(&xs);

        // calculate loss
        let mut loss = mlp.calc_loss(&ys, &y_pred);

        // print_params(&mlp);
        // backward pass consists of 2 steps
        mlp.reset_grades();
        loss.backward();

        // update parameters
        mlp.update();

        // keep track of loss improvement
        println!("iteration {}   loss {}", i + 1, loss.get_data());
    }

    print_predictions(y_pred, &ys);
    // print_params(&mlp);
    println!("DONE");
}
