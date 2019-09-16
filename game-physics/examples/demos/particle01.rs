use game_physics::prelude::*;
use math::prelude::*;

fn main() {
    let initial_velocity = Tuple4D::new_vector(1.0, 2.0, 3.0);

    let mut p = Particle::new();
    p.set_inverse_mass(0.1);
    p.set_velocity(initial_velocity);

    println!("position: {:?}", p.get_position());
    println!("velocity: {:?}", p.get_velocity());
    println!("acceleration: {:?}", p.get_acceleration());
    println!("DONE");

    let force = Tuple4D::new_vector(1.0, 2.0, 3.0);
    p.add_force(&force);
    p.integrate(2.0);
    p.integrate(2.0);
    p.integrate(2.0);

    println!("position: {:?}", p.get_position());
    println!("velocity: {:?}", p.get_velocity());
    println!("acceleration: {:?}", p.get_acceleration());
    println!("DONE");
}
