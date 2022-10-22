use crate::graph::draw_graph;
use std::fmt::{format, Display, Formatter};
use std::ops::{Add, Mul};

use crate::micrograd_rs_v1::ValueRefV1;
use crate::micrograd_rs_v1::ValueV1;

mod graph;
mod micrograd_rs_v1;

fn main() {
    let a = ValueRefV1::new_value(2.0 as f64, "a".to_string());
    let b = ValueRefV1::new_value(3.0, "b".to_string());
    let c = ValueRefV1::new(ValueV1::new_value(8.0, "c".to_string()));

    let mut x = &a + &b;
    x.set_label("x".to_string());
    let mut y = &a * &x;
    y.set_label("y".to_string());

    println!("a = {:?}", a.borrow().data());
    println!("b = {:?}", b.borrow().data());
    println!("d = {:?}", c.borrow().data());
    println!(
        "x = a + b = {:?} + {:?} = {:?} ",
        a.borrow().data(),
        b.borrow().data(),
        x.borrow().data()
    );
    println!(
        "y = a * x = {:?} * {:?} = {:?}",
        a.borrow().data(),
        x.borrow().data(),
        y.borrow().data()
    );

    println!(" {} ", a);
    println!(" {} ", b);
    println!(" {} ", c);
    println!(" {} ", x);
    println!(" {} ", y);

    let e = &(&a * &b) + &c;
    println!(
        "e= a * b + c = {:?} * {:?} +  {:?} = {:?}",
        a.borrow().data(),
        b.borrow().data(),
        c.borrow().data(),
        e.borrow().data()
    );

    draw_graph(e, "main".to_string());

    println!("DONE");
}
