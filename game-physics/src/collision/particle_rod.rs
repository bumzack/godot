use std::f32::MAX;

use crate::collision::particle_contact::{ParticleContact, ParticleContactOps};
use crate::collision::particle_link::ParticleLinkOps;
use crate::force::particle_force_registry::{ParticleForceRegistry, ParticleForceRegistryOps};
use crate::force::particle_force_types::{ParticleContactsContainer, ParticleIdx};
use math::prelude::*;

use crate::particle::particle::Particle;
use crate::particle::particle::ParticleOps;

pub struct ParticleRod {
    particle: Vec<Option<ParticleIdx>>,
    max_length: f32,
    restitution: f32,
    length: f32,
}

impl ParticleLinkOps for ParticleRod {
    fn current_length(&self, registry: &ParticleForceRegistry) -> f32 {
        let p0 = registry.get_particle(self.particle[0].unwrap());
        let p1 = registry.get_particle(self.particle[1].unwrap());
        let relative_pos = p0.get_position() - p1.get_position();
        Tuple4D::magnitude(&relative_pos)
    }

    fn add_contact(&mut self, contact: &mut ParticleContact, limit: usize, registry: &ParticleForceRegistry) -> usize {
        let p0 = registry.get_particle(self.particle[0].unwrap());
        let p1 = registry.get_particle(self.particle[1].unwrap());

        // find length of cable
        let current_length = self.current_length(registry);

        //check overextended
        if current_length == self.length {
            return 0;
        }

        contact.set_particle0(self.particle[0].unwrap());
        contact.set_particle1(self.particle[1].unwrap());

        let normal = p1.get_position() - p0.get_position();
        let normal = Tuple4D::normalize(&normal);

        // the contact normal depends on whether extend or compress
        if current_length > self.length {
            contact.set_contact_normal(normal);
            contact.set_penetration(current_length - self.length)
        } else {
            contact.set_contact_normal(&normal * (-1.0));
            contact.set_penetration(self.length - current_length);
        }
        contact.set_restitution(0.0);
        1
    }
}

impl ParticleRod {
    pub fn new() -> ParticleRod {
        ParticleRod {
            particle: vec![None; 2],
            max_length: 0.0,
            restitution: 0.0,
            length: 0.0,
        }
    }

    pub fn set_max_lengths(&mut self, max_length: f32) {
        self.max_length = max_length;
    }

    pub fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    pub fn set_restitutiond(&mut self, restitution: f32) {
        self.restitution = restitution;
    }

    pub fn get_max_lengths(&mut self) -> f32 {
        self.max_length
    }

    pub fn get_restitutiond(&mut self) -> f32 {
        self.restitution
    }

    pub fn get_length(&mut self) -> f32 {
        self.length
    }

    pub fn set_particle1(&mut self, p1: ParticleIdx) {
        self.particle[0] = Some(p1);
    }

    pub fn set_particle2(&mut self, p2: ParticleIdx) {
        self.particle[1] = Some(p2);
    }
}

// TODO
#[test]
fn test_particle_contact() {}
