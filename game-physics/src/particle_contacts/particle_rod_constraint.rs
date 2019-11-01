use math::prelude::*;

use crate::force::particle_force_types::ParticleIdx;
use crate::{ParticleConstraintOps, ParticleContact, ParticleForceRegistry, ParticleForceRegistryOps, ParticleOps};

pub struct ParticleRodConstraint {
    length: f32,
    particle: Option<ParticleIdx>,
    anchor: Tuple4D,
}

impl ParticleConstraintOps for ParticleRodConstraint {
    fn add_contact(&mut self, registry: &ParticleForceRegistry, limit: usize) -> Option<Vec<ParticleContact>> {
        // find length of cable
        let current_len = self.current_length(registry);

        //check overextended
        if current_len == self.length {
            return None;
        }

        let mut contact = ParticleContact::new();

        contact.set_particle0(self.particle);
        contact.set_particle1(None);

        let p0 = registry.get_particle(self.particle.unwrap());

        let normal = &self.anchor - p0.get_position();
        let normal = Tuple4D::normalize(&normal);

        // the contact normal depends on whether extend or compress
        if current_len > self.length {
            contact.set_contact_normal(normal);
            contact.set_penetration(current_len - self.length)
        } else {
            contact.set_contact_normal(&normal * (-1.0));
            contact.set_penetration(self.length - current_len);
        }
        contact.set_restitution(0.0);
        Some(vec![contact])
    }

    fn current_length(&self, registry: &ParticleForceRegistry) -> f32 {
        let p0 = &registry.get_particle(self.particle.unwrap());
        let relative_distance = &self.anchor - p0.get_position();
        Tuple4D::magnitude(&relative_distance)
    }
}

impl ParticleRodConstraint {
    pub fn new() -> ParticleRodConstraint {
        ParticleRodConstraint {
            length: 0.0,
            particle: None,
            anchor: Tuple4D::empty(),
        }
    }

    pub fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    pub fn get_length(&mut self) -> f32 {
        self.length
    }

    pub fn set_particle(&mut self, p0: Option<ParticleIdx>) {
        self.particle = p0;
    }
}

// TODO
#[test]
fn test_particle_contact() {}
