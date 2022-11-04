use micrograd_rs::micrograd_rs_engine_v2::{calc_loss_mse, print_predictions, MLP};

fn main() {
    let mlp = MLP::new(3, vec![4, 4, 1]);

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
        let mut loss = calc_loss_mse(&ys, &y_pred);

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
