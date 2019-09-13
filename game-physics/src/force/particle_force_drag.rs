use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::force::particle_force_registry::ParticleForceRegistry;

use crate::particle::particle::{Particle, ParticleOps};

use crate::force::particle_force_types::ParticleContainer;
use math::prelude::*;

#[derive(Clone)]
pub struct ParticleForceDrag {
    k1: f32,
    k2: f32,
}

impl<'a> ParticleForceGeneratorOps for ParticleForceDrag {
    fn update_force(&self, particle: &mut Particle, duration: f32, all_particles: &ParticleContainer) {
        let f = Tuple4D::new_vector_from(particle.get_velocity());
        let mut drag_coeff = Tuple4D::magnitude(&f);
        drag_coeff = self.k1 * drag_coeff + self.k2 * drag_coeff * drag_coeff;
        let mut f = Tuple4D::normalize(&f);
        f = f * (-drag_coeff);
        println!(
            "DRAG               add force from drag: {:?},    particle.id = {}",
            f,
            particle.get_id()
        );
        particle.add_force(&f);
    }
}

impl<'a> ParticleForceDrag {
    pub fn new() -> ParticleForceDrag {
        // defualt: no drag ?!
        ParticleForceDrag { k1: 1.0, k2: 1.0 }
    }

    pub fn set_drag_k1(&mut self, k1: f32) {
        self.k1 = k1;
    }
    pub fn set_drag_k2(&mut self, k2: f32) {
        self.k2 = k2;
    }

    pub fn get_drag_k1(&self) -> f32 {
        self.k1
    }

    pub fn get_drag_k2(&self) -> f32 {
        self.k2
    }
}

// TODO
#[test]
fn test_particle_force_enerator() {}
