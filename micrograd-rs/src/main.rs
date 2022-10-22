use std::fmt::Debug;
use std::ops::{Add, Mul};

use crate::graph_v2::draw_graph;
use crate::micrograd_rs_v2::{ValueRefV2, ValueV2};

mod graph_v2;
mod micrograd_rs_v2;

fn main() {
    let h = 0.001;
    let l1: f64 = l(0.0).borrow().data();
    let l2: f64 = l(h).borrow().data();

    let dL_da: f64 = (l2 - l1) / h;

    println!(" l1 {},   l2 {}   h = {}  DL_da = (l2-l1)/h = {}", l1, l2, h, dL_da);

    let x = l1.tan();
    println!("x = {}", x);

    nn();

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
    let x1 = ValueRefV2::new_value(2.0, "x1".to_string());
    let x2 = ValueRefV2::new_value(0.0, "x2".to_string());

    let w1 = ValueRefV2::new_value(-3.0, "w1".to_string());
    let w2 = ValueRefV2::new_value(1.0, "w2".to_string());

    let b = ValueRefV2::new_value(6.881373587019, "b".to_string());

    let w1x1 = &w1 * &x1;
    let w2x2 = &w2 * &x2;
    let w1x1_plus_w2x2 = &w1x1 + &w2x2;

    let mut n = &w1x1_plus_w2x2 + &b;
    n.set_label("n".to_string());
    let mut o = n.tanh();
    n.set_label("o".to_string());

    draw_graph(o, "single_neuron".to_string());
}
