// ToDO remove this file? inheritance is not Rusts strong suit

use std::f32::MAX;

use crate::collision::particle_contact::{ParticleContact, ParticleContactOps};
use crate::force::particle_force_registry::{ParticleForceRegistry, ParticleForceRegistryOps};
use crate::force::particle_force_types::{ParticleContactsContainer, ParticleIdx};

use crate::particle::particle::Particle;
use crate::particle::particle::ParticleOps;
use math::prelude::*;

pub trait ParticleLinkOps {
    fn current_length(&self, registry: &ParticleForceRegistry) -> f32;
    fn add_contact(&mut self, contact: &mut ParticleContact, limit: usize, registry: &ParticleForceRegistry) -> usize;
}

// TODO
#[test]
fn test_particle_contact() {}
