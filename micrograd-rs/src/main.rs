use std::fmt::Debug;
use std::ops::{Add, Mul};

use crate::graph_v2::draw_graph;
use crate::micrograd_rs_v2::{ValueRefV2, ValueV2};

mod graph_v2;
mod micrograd_rs_v2;

fn main() {
    let h = 0.001;
    let l1: f64 = *l(0.0).borrow().data();
    let l2: f64 = *l(h).borrow().data();

    let dL_da: f64 = (l2 - l1) / h;

    println!(" l1 {},   l2 {}   h = {}  DL_da = (l2-l1)/h = {}", l1, l2, h, dL_da);
    println!("DONE");
}

pub fn l(h: f64) -> ValueRefV2<f64> {
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
