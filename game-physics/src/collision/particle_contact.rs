use crate::force::particle_force_registry::{ParticleForceRegistry, ParticleForceRegistryOps};
use crate::force::particle_force_types::ParticleIdx;

use crate::particle::particle::Particle;
use crate::particle::particle::ParticleOps;
use math::prelude::*;

pub struct ParticleContact {
    particle: Vec<Option<ParticleIdx>>,
    restitution: f32,
    penetration: f32,
    contact_normal: Tuple4D,
    particle_movement: Vec<Option<Tuple4D>>,
}

pub trait ParticleContactOps {
    fn resolve(&mut self, duration: f32, registry: &mut ParticleForceRegistry);
    fn calculate_separating_velocity(&self, registry: &mut ParticleForceRegistry) -> f32;

    fn resolve_velocity(&self, duration: f32, registry: &mut ParticleForceRegistry);

    fn calc_total_inverse_mass(&self, registry: &ParticleForceRegistry) -> f32;

    fn resolve_interpenetration(&mut self, duration: f32, registry: &mut ParticleForceRegistry);
}

impl ParticleContactOps for ParticleContact {
    fn resolve(&mut self, duration: f32, registry: &mut ParticleForceRegistry) {
        self.resolve_velocity(duration, registry);
        self.resolve_interpenetration(duration, registry);
    }

    fn calculate_separating_velocity(&self, registry: &mut ParticleForceRegistry) -> f32 {
        let p0 = registry.get_particle(self.particle[0].unwrap());
        let mut relative_velocity = Tuple4D::new_vector_from(p0.get_velocity());

        if self.particle[1].is_some() {
            let p1 = registry.get_particle(self.particle[1].unwrap());
            relative_velocity = &relative_velocity - p1.get_velocity();
        }
        println!(
            "calculate_separating_velocity = {:?}",
            relative_velocity ^ self.contact_normal
        );

        relative_velocity ^ self.contact_normal
    }

    fn resolve_velocity(&self, duration: f32, registry: &mut ParticleForceRegistry) {
        let separating_velocity = self.calculate_separating_velocity(registry);

        if separating_velocity > 0.0 {
            // the contact is either separating or stationary -> simple return
            return;
        }
        let mut new_sep_velocity = -separating_velocity * self.restitution;

        let p0 = registry.get_particle(self.particle[0].unwrap());
        let p1 = registry.get_particle(self.particle[1].unwrap());

        // check velocity buildup due to acc.
        let mut acc_caused_velocity = Tuple4D::new_vector_from(p0.get_acceleration());
        if self.particle[1].is_some() {
            acc_caused_velocity = &acc_caused_velocity - p1.get_acceleration();
        }
        let mut acc_caused_sep_velocity = (acc_caused_velocity ^ self.contact_normal) * duration;

        //if closing velocity due to acc. build up exists -> remove it from sep. velocity
        if acc_caused_sep_velocity < 0.0 {
            new_sep_velocity = new_sep_velocity + &acc_caused_sep_velocity * self.restitution;
            if new_sep_velocity < 0.0 {
                new_sep_velocity = 0.0;
            }
        }

        let delta_velocity = new_sep_velocity - separating_velocity;

        // apply change in veloctiy to all objects
        let total_inverse_mass = self.calc_total_inverse_mass(registry);
        if (total_inverse_mass <= 0.0) {
            return;
        }

        // calc impulse to apply
        let impulse = delta_velocity / total_inverse_mass;
        println!("impulse = {:?}", impulse);

        let impulse_per_mass = &self.contact_normal * impulse;

        // amount of imp per unit of inv. mass
        let impulse_per_inverse_mass = &self.contact_normal * impulse;

        let particle0_new_velocity = p0.get_velocity() + &(&impulse_per_inverse_mass * p0.get_inverse_mass());
        println!("particle0: old_velocity = {:?}", p0.get_velocity());
        println!("particle0_new_velocity = {:?}", particle0_new_velocity);
        registry.set_velocity(self.particle[0].unwrap(), particle0_new_velocity);

        if self.particle[1].is_some() {
            // particle 1 goes into the inverse direction -> -
            let p1 = registry.get_particle(self.particle[1].unwrap());
            let particle1_new_velocity = p1.get_velocity() + &(&impulse_per_inverse_mass * (-p1.get_inverse_mass()));
            println!("particle1: old_velocity = {:?}", p1.get_velocity());
            println!("particle1_new_velocity = {:?}", particle1_new_velocity);
            registry.set_velocity(self.particle[1].unwrap(), particle1_new_velocity);
        }
    }

    fn calc_total_inverse_mass(&self, registry: &ParticleForceRegistry) -> f32 {
        let p0 = registry.get_particle(self.particle[0].unwrap());
        let mut total_inverse_mass = p0.get_inverse_mass();
        if self.particle[1].is_some() {
            let p1 = registry.get_particle(self.particle[1].unwrap());
            total_inverse_mass = total_inverse_mass + p1.get_inverse_mass();
        }
        println!("total_inverse_mass = {:?}", total_inverse_mass);
        total_inverse_mass
    }

    fn resolve_interpenetration(&mut self, duration: f32, registry: &mut ParticleForceRegistry) {
        // if there is no interpenetration -> skip
        if self.penetration <= 0.0 {
            return;
        }

        // total movement is based on both inverse masses
        let total_inverse_mass = self.calc_total_inverse_mass(registry);
        if (total_inverse_mass <= 0.0) {
            return;
        }

        // amount of penetration per resolution per unit of inv ass
        let move_per_inverse_mass = self.contact_normal * (self.penetration / total_inverse_mass);

        // calc movement amounts
        let p0 = registry.get_particle(self.particle[0].unwrap());
        self.particle_movement[0] = Some(move_per_inverse_mass * p0.get_inverse_mass());
        let p0_position_new = p0.get_position() + &self.particle_movement[0].unwrap();
        registry.set_velocity(self.particle[0].unwrap(), p0_position_new);

        let particle_movement1 = Tuple4D::new_vector(0.0, 0.0, 0.0);
        if self.particle[1].is_some() {
            let p1 = registry.get_particle(self.particle[1].unwrap());
            self.particle_movement[1] = Some(move_per_inverse_mass * (-p1.get_inverse_mass()));
            let p1_position_new = p1.get_position() + &self.particle_movement[1].unwrap();
            registry.set_velocity(self.particle[1].unwrap(), p1_position_new);
        }
    }
}

impl ParticleContact {
    pub fn new() -> ParticleContact {
        ParticleContact {
            particle: vec![None; 2],
            restitution: 0.0,
            penetration: 0.0,
            contact_normal: Tuple4D::new_vector(0.0, 0.0, 0.0),
            particle_movement: vec![None; 2],
        }
    }

    pub fn set_particle0(&mut self, p0: ParticleIdx) {
        self.particle[0] = Some(p0);
    }

    pub fn set_particle1(&mut self, p1: ParticleIdx) {
        self.particle[1] = Some(p1);
    }

    pub fn set_restitution(&mut self, restitution: f32) {
        self.restitution = restitution;
    }

    pub fn set_penetration(&mut self, penetration: f32) {
        self.penetration = penetration;
    }

    pub fn get_penetration(&self) -> f32 {
        self.penetration
    }

    pub fn set_contact_normal(&mut self, contact_normal: Tuple4D) {
        self.contact_normal = contact_normal;
    }
}

// TODO
#[test]
fn test_particle_contact() {}
