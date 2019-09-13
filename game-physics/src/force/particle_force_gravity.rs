use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::force::particle_force_registry::ParticleForceRegistry;
use crate::force::particle_force_types::ParticleContainer;

use crate::particle::particle::{Particle, ParticleOps};
use math::prelude::*;

#[derive(Clone)]
pub struct ParticleForceGravity {
    gravity: Tuple4D,
}

impl<'a> ParticleForceGeneratorOps for ParticleForceGravity {
    fn update_force(&self, particle: &mut Particle, duration: f32, all_particles: &ParticleContainer) {
        if !particle.has_finite_mass() {
            return;
        }
        let f = &self.gravity * particle.get_mass();
        println!(
            "GRAVITY           add force from gravity: {:?}         particle.id = {}",
            f,
            particle.get_id()
        );
        particle.add_force(&f);
    }
}

impl<'a> ParticleForceGravity {
    pub fn new() -> ParticleForceGravity {
        ParticleForceGravity {
            gravity: Tuple4D::new_vector(0.0, 10.0, 0.0),
        }
    }

    pub fn set_gravity(&mut self, g: Tuple4D) {
        self.gravity = g;
    }

    pub fn get_gravity(&self) -> &Tuple4D {
        &self.gravity
    }

    pub fn get_gravity_mut(&mut self) -> &mut Tuple4D {
        &mut self.gravity
    }
}

// TODO
#[test]
fn test_particle_force_enerator() {}
