use math::prelude::*;

use crate::{ParticleConstraintOps, ParticleContact, ParticleForceRegistry, ParticleForceRegistryOps, ParticleOps};
use crate::force::particle_force_types::ParticleIdx;

pub struct ParticleCableConstraint {
    max_length: f32,
    restitution: f32,
    particle: Option<ParticleIdx>,
    anchor: Tuple4D,
}

impl ParticleConstraintOps for ParticleCableConstraint {
    fn add_contact(&mut self, registry: &ParticleForceRegistry, limit: usize) -> Option<Vec<ParticleContact>> {
        // find length of cable
        let length = self.current_length(registry);

        //check overextended
        if length < self.max_length {
            return None;
        }

        let mut contact: ParticleContact = ParticleContact::new();

        contact.set_particle0(self.particle);
        contact.set_particle1(None);

        let p0 = registry.get_particle(self.particle.unwrap());

        let mut normal = &self.anchor - p0.get_position();
        normal = Tuple4D::normalize(&normal);
        contact.set_contact_normal(normal);
        contact.set_penetration(length - self.max_length);
        contact.set_restitution(self.restitution);
        Some(vec![contact])
    }

    fn current_length(&self, registry: &ParticleForceRegistry) -> f32 {
        let p0 = &registry.get_particle(self.particle.unwrap());
        let relative_distance = &self.anchor - p0.get_position();
        Tuple4D::magnitude(&relative_distance)
    }
}

impl ParticleCableConstraint {
    pub fn new() -> ParticleCableConstraint {
        ParticleCableConstraint {
            max_length: 0.0,
            restitution: 0.0,
            particle: None,
            anchor: Tuple4D::empty(),
        }
    }

    pub fn set_max_lengths(&mut self, max_length: f32) {
        self.max_length = max_length;
    }

    pub fn set_restitution(&mut self, restitution: f32) {
        self.restitution = restitution;
    }

    pub fn get_max_lengths(&mut self) -> f32 {
        self.max_length
    }

    pub fn get_restitutiond(&mut self) -> f32 {
        self.restitution
    }

    pub fn set_particle0(&mut self, p0: Option<ParticleIdx>) {
        self.particle = p0;
    }

    pub fn set_anchor(&mut self, anchor: Tuple4D) {
        self.anchor = anchor;
    }
}

// TODO
#[test]
fn test_particle_contact() {}
