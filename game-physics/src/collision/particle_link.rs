// ToDO remove this file? inheritance is not Rusts strong suit

use crate::collision::particle_contact::ParticleContact;
use crate::force::particle_force_registry::ParticleForceRegistry;

pub trait ParticleLinkOps {
    fn current_length(&self, registry: &ParticleForceRegistry) -> f32;
    fn add_contact(&mut self, contact: &mut ParticleContact, limit: usize, registry: &ParticleForceRegistry) -> usize;
}

// TODO
#[test]
fn test_particle_contact() {}
