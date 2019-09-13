use game_physics::prelude::*;

use math::prelude::*;

fn main() {
    let mut registry = ParticleForceRegistry::new();

    let anchor = Tuple4D::new_point(10.0, 10.0, 10.0);

    let mut pfg1 = ParticleForceFakeSpring::new();
    pfg1.set_anchor(anchor);
    pfg1.set_spring_constant(4.0);
    pfg1.set_damping(0.98);

    let v1 = Tuple4D::new_vector(-1.0, -2.0, -3.0);
    let mut p1 = Particle::new();
    p1.set_inverse_mass(0.1);
    p1.set_velocity(v1);
    p1.set_id(1);

    let v2 = Tuple4D::new_vector(-1.0, -2.0, -3.0);
    let mut p2 = Particle::new();
    p2.set_inverse_mass(0.1);
    p2.set_velocity(v2);
    p2.set_id(1);

    let p1_idx = registry.add_particle(p1);
    let p2_idx = registry.add_particle(p2);

    let pfg1_idx = registry.add_particle_force_generator(Box::new(pfg1));

    registry.add_force_for_particle(p1_idx, pfg1_idx);
    registry.add_force_for_particle(p2_idx, pfg1_idx);

    println!("initial position and velocity");
    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p1 velocity = {:?}", registry.get_particle(p1_idx).get_velocity());

    registry.update_forces(2.0);
    println!("");
    registry.get_particle_mut(p1_idx).integrate(2.0);
    println!("");

    println!("after p1 has been integrated1 ");
    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p1 velocity = {:?}", registry.get_particle(p1_idx).get_velocity());

    registry.update_forces(2.0);
    println!("");
    registry.get_particle_mut(p1_idx).integrate(2.0);
    println!("");

    println!("after p1 has been 2x integrated1 ");
    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p1 velocity = {:?}", registry.get_particle(p1_idx).get_velocity());

    let mut pc1 = ParticleContact::new();
    let n = Tuple4D::new_vector(1.0, 2.0, 3.0);
    pc1.set_contact_normal(n);
    pc1.set_penetration(2.0);
    pc1.set_restitution(3.0);
    pc1.set_particle0(p1_idx);
    pc1.set_particle1(p2_idx);

    pc1.resolve(2.0, &mut registry);

    let mut pc2 = ParticleContact::new();
    let n = Tuple4D::new_vector(-1.0, -2.0, -3.0);
    pc2.set_contact_normal(n);
    pc2.set_penetration(3.0);
    pc2.set_restitution(4.0);
    pc2.set_particle0(p2_idx);
    pc2.set_particle1(p1_idx);

    pc2.resolve(2.0, &mut registry);

    let mut pc_arr = Vec::new();
    pc_arr.push(pc1);
    pc_arr.push(pc2);

    let mut pcr = ParticleContactResolver::new();
    pcr.set_iterations(10);
    let l = pc_arr.len();
    pcr.resolve_contacts(&mut pc_arr, l, 2.0, &mut registry);

    let mut cable_link = ParticleCableLink::new();
    cable_link.set_particle1(p1_idx);
    cable_link.set_particle2(p2_idx);
    cable_link.set_max_lengths(12.0);
    cable_link.set_restitutiond(12.0);

    let mut pc3 = ParticleContact::new();
    cable_link.add_contact(&mut pc3, 2, &registry);

    pc_arr.push(pc3);

    pcr.resolve_contacts(&mut pc_arr, l, 2.0, &mut registry);
}
