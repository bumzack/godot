use game_physics::prelude::*;
use math::prelude::*;
fn main() {
    let mut registry = ParticleForceRegistry::new();

    let pfg1 = ParticleForceGravity::new();
    let pfg2 = ParticleForceGravity::new();

    let mut pfg3 = ParticleForceDrag::new();
    pfg3.set_drag_k1(0.95);
    pfg3.set_drag_k2(0.5);

    let mut pfg4 = ParticleForceSpring::new();

    let v1 = Tuple4D::new_vector(1.0, 2.0, 3.0);
    let mut p1 = Particle::new();
    p1.set_inverse_mass(0.1);
    p1.set_velocity(v1);
    p1.set_id(1);

    let v2 = Tuple4D::new_vector(1.0, 2.0, 3.0);
    let mut p2 = Particle::new();
    p2.set_inverse_mass(0.1);
    p2.set_velocity(v2);
    p2.set_id(2);

    let p1_idx = registry.add_particle(p1);
    let p2_idx = registry.add_particle(p2);

    pfg4.set_other(p1_idx);

    let pfg1_idx = registry.add_particle_force_generator(Box::new(pfg1));
    let pfg2_idx = registry.add_particle_force_generator(Box::new(pfg2));
    let pfg3_idx = registry.add_particle_force_generator(Box::new(pfg3));
    let pfg4_idx = registry.add_particle_force_generator(Box::new(pfg4));

    registry.add_force_for_particle(p1_idx, pfg1_idx);
    registry.add_force_for_particle(p2_idx, pfg2_idx);

    registry.add_force_for_particle(p2_idx, pfg4_idx);

    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p1 velocity = {:?}", registry.get_particle(p1_idx).get_velocity());
    println!("p2 position = {:?}", registry.get_particle(p2_idx).get_position());
    println!("p2 velocity = {:?}", registry.get_particle(p2_idx).get_velocity());

    println!("");
    println!("");
    registry.update_forces(2.0);
    println!("");
    println!("");
    registry.get_particle_mut(p1_idx).integrate(2.0);
    registry.get_particle_mut(p2_idx).integrate(2.0);
    println!("");
    println!("");

    println!("p2 has 1 pfg expected changes ");
    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p1 velocity = {:?}", registry.get_particle(p1_idx).get_velocity());
    println!("p2 position = {:?}", registry.get_particle(p2_idx).get_position());
    println!("p2 velocity = {:?}", registry.get_particle(p2_idx).get_velocity());

    registry.add_force_for_particle(p2_idx, pfg3_idx);

    println!("");
    println!("");
    registry.update_forces(2.0);
    println!("");
    println!("");
    registry.get_particle_mut(p1_idx).integrate(2.0);
    registry.get_particle_mut(p2_idx).integrate(2.0);
    println!("");
    println!("");

    println!("p2 shoudl be different from p1");
    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p1 velocity = {:?}", registry.get_particle(p1_idx).get_velocity());
    println!("p2 position = {:?}", registry.get_particle(p2_idx).get_position());
    println!("p2 velocity = {:?}", registry.get_particle(p2_idx).get_velocity());

    println!("");
    println!("");
    registry.update_forces(2.0);
    println!("");
    println!("");
    registry.get_particle_mut(p1_idx).integrate(2.0);
    registry.get_particle_mut(p2_idx).integrate(2.0);
    println!("");
    println!("");

    println!("p2 shoudl be different from p1");
    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p1 velocity = {:?}", registry.get_particle(p1_idx).get_velocity());
    println!("p2 position = {:?}", registry.get_particle(p2_idx).get_position());
    println!("p2 velocity = {:?}", registry.get_particle(p2_idx).get_velocity());

    // after all particles and generators are added to the registry, set the registry in the spring generator :-(((

    println!("ADDED spring to p2");
    println!("");
    registry.update_forces(1.0);
    println!("");
    println!("");
    // registry.get_particle_mut(p1_idx).integrate(2.0);
    registry.get_particle_mut(p2_idx).integrate(2.0);
    // println!("");
    println!("");

    println!("p2 shoudl be different from above");
    println!("p2 position = {:?}", registry.get_particle(p2_idx).get_position());
    println!("p2 velocity = {:?}", registry.get_particle(p2_idx).get_velocity());
}
