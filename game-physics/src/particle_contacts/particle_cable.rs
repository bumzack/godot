use math::prelude::*;

use crate::{ParticleContact, ParticleForceRegistry, ParticleForceRegistryOps, ParticleLinkOps, ParticleOps};
use crate::force::particle_force_types::ParticleIdx;

#[derive(Debug)]
pub struct ParticleCable {
    max_length: f32,
    restitution: f32,
    particle: Vec<Option<ParticleIdx>>,
}

impl ParticleLinkOps for ParticleCable {
    fn add_contact(&mut self, registry: &ParticleForceRegistry, limit: usize) -> Option<Vec<ParticleContact>> {
        // find length of cable
        let length = self.current_length(registry);

        //check overextended
        if length < self.max_length {
            return None;
        }

        let mut contact = ParticleContact::new();

        contact.set_particle0(self.particle[0]);
        contact.set_particle1(self.particle[1]);

        let p0 = registry.get_particle(self.particle[0].unwrap());
        let p1 = registry.get_particle(self.particle[1].unwrap());

        let mut normal = p1.get_position() - p0.get_position();
        normal = Tuple4D::normalize(&normal);
        contact.set_contact_normal(normal);
        contact.set_penetration(length - self.max_length);
        contact.set_restitution(self.restitution);
        Some(vec![contact])
    }

    fn current_length(&self, registry: &ParticleForceRegistry) -> f32 {
        let p0 = &registry.get_particle(self.particle[0].unwrap());
        let p1 = &registry.get_particle(self.particle[0].unwrap());
        let relative_pos = p1.get_position() - p0.get_position();
        Tuple4D::magnitude(&relative_pos)
    }
}

impl ParticleCable {
    pub fn new() -> ParticleCable {
        ParticleCable {
            particle: vec![None; 2],
            max_length: 0.0,
            restitution: 0.0,
        }
    }

    pub fn set_max_length(&mut self, max_length: f32) {
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
        self.particle[0] = p0;
    }

    pub fn set_particle1(&mut self, p1: Option<ParticleIdx>) {
        self.particle[1] = p1;
    }
}

// TODO
#[test]
fn test_particle_contact() {}
