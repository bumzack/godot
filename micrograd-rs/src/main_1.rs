use crate::graph::draw_graph;

use crate::micrograd_rs_v1::ValueRefV1;
use crate::micrograd_rs_v1::ValueV1;

mod graph;
mod micrograd_rs_v1;

fn main() {
    let a = ValueRefV1::new_value(2.0 as f64, "a".to_string());
    let b = ValueRefV1::new_value(3.0, "b".to_string());
    let c = ValueRefV1::new(ValueV1::new_value(8.0, "c".to_string()));

    let mut d = &a + &b;
    d.set_label("d".to_string());
    let mut e = &d * &c;
    e.set_label("e".to_string());

    draw_graph(e, "main".to_string());

    println!("DONE");
}
