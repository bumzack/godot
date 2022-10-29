use micrograd_rs::prelude::{draw_graph, MLP, print_predictions, ValueRefV2};
use micrograd_rs::prelude::calc_loss;

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
        let mut loss = calc_loss(&ys, &y_pred);

        // print_params(&mlp);
        // backward pass consists of 2 steps
        mlp.reset_grades();
        loss.backward();

        // update parameters
        mlp.update();

        // keep track of loss improvement
        println!("iteration {}   loss {}", i + 1, loss.get_data());
    }

    print_predictions(&mut y_pred);
    // print_params(&mlp);
    println!("DONE");
}


pub fn l(h: f64) -> ValueRefV2 {
    let a = ValueRefV2::new_value(2.0 + h, "a".to_string());
    let b = ValueRefV2::new_value(-3.0, "b".to_string());
    let c = ValueRefV2::new_value(10.0, "c".to_string());
    let f = ValueRefV2::new_value(-2.0, "f".to_string());

    let mut e = &a * &b;
    e.set_label("e".to_string());

    let mut d = &e + &c;
    d.set_label("d".to_string());

    let mut l = &d * &f;
    l.set_label("L".to_string());

    l
}

pub fn nn() {
    let x1: ValueRefV2 = ValueRefV2::new_value(2.0, "x1".to_string());
    let x2 = ValueRefV2::new_value(0.0, "x2".to_string());

    let w1 = ValueRefV2::new_value(-3.0, "w1".to_string());
    let w2 = ValueRefV2::new_value(1.0, "w2".to_string());

    let b = ValueRefV2::new_value(6.881373587019, "b".to_string());

    let w1x1 = &x1 * &w1;
    let w2x2 = &x2 * &w2;
    let w1x1_plus_w2x2 = &w1x1 + &w2x2;

    let mut n = &w1x1_plus_w2x2 + &b;
    n.set_label("n".to_string());

    let e = (2.0 * &n).exp();
    let mut o = &(&e - 1.0) / &(&e + 1.0);

    // let mut o = n.tanh();
    o.set_label("o".to_string());

    draw_graph(o.clone(), "single_neuron_no_backwars".to_string());

    o.backward();

    let topo = ValueRefV2::traverse(&o);

    println!("\n\n");
    for t in topo.iter() {
        println!("{}", t);
    }

    draw_graph(o, "single_neuron_with_backward".to_string());
}
