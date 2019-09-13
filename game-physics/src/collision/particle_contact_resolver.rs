use std::f32::MAX;

use crate::collision::particle_contact::ParticleContactOps;
use crate::force::particle_force_registry::{ParticleForceRegistry, ParticleForceRegistryOps};
use crate::force::particle_force_types::{ParticleContactsContainer, ParticleIdx};

use crate::particle::particle::Particle;
use crate::particle::particle::ParticleOps;
use math::prelude::*;

pub struct ParticleContactResolver {
    iterations: usize,
    iterations_used: usize,
}
pub trait ParticleContactResolverOps {
    fn resolve_contacts(
        &mut self,
        contact_array: &mut ParticleContactsContainer,
        num_contacts: usize,
        duration: f32,
        registry: &mut ParticleForceRegistry,
    );
}

impl ParticleContactResolverOps for ParticleContactResolver {
    fn resolve_contacts(
        &mut self,
        contact_array: &mut ParticleContactsContainer,
        num_contacts: usize,
        duration: f32,
        registry: &mut ParticleForceRegistry,
    ) {
        self.iterations_used = 0;

        while self.iterations_used < self.iterations {
            // find contact with largest closing vel
            let mut max = MAX;
            let mut max_index = num_contacts;

            // TODO: can this be a .iter() thingi ? - is the length of contact_array the same as num_contacts? probably -we will see when
            // things come together
            for i in 0..num_contacts {
                let sep_vel = contact_array[i].calculate_separating_velocity(registry);
                if sep_vel < max && (sep_vel < 0.0 || contact_array[i].get_penetration() > 0.0) {
                    max = sep_vel;
                    max_index = i;
                }
            }
            if max_index == num_contacts {
                break;
            }
            contact_array[max_index].resolve(duration, registry);

            self.iterations_used += 1;
        }
    }
}

impl ParticleContactResolver {
    pub fn new() -> ParticleContactResolver {
        ParticleContactResolver {
            iterations: 10,
            iterations_used: 0,
        }
    }
    pub fn set_iterations(&mut self, iterations: usize) {
        self.iterations = iterations;
    }

    pub fn set_iterations_used(&mut self, iterations_used: usize) {
        self.iterations_used = iterations_used;
    }
}

// TODO
#[test]
fn test_particle_contact() {}
