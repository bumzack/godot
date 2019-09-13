use math::prelude::*;

fn main() {
    let v1 = Tuple4D::new_vector(1.0, 2.0, 3.0);
    let v2 = Tuple4D::new_vector(11.0, 12.0, 13.0);

    let c = &v1 + &v2;

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
    println!("v1 + v2= {:?}", c);

    println!("DONE");
}
