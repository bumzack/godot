use std::ops::Rem;

fn main() {
    let z: f32 = 0.9;
    let y:f32 = -0.9;

    let z_minus1 = z - 1.0;
    let z_minus1_mod_2 = z_minus1 .rem_euclid( 2.0);
    let corr = 2.0+ z_minus1 .rem( 2.0);

    let y_plus_1 = y + 1.0;
    let y_plus_1_mod_2 = y_plus_1 .rem_euclid( 2.0);
    let corr2 =2.0+  y_plus_1 .rem( 2.0);


    println!("z {}, z_minus1 {}, z_minus1_mod_2 {}          corr {}", z, z_minus1, z_minus1_mod_2, corr);
    println!("y {}, y_plus_1 {}, y_plus_1_mod_2 {}              corr {}", y, y_plus_1, y_plus_1_mod_2, corr2    );

}