use math::prelude::*;

use crate::force::particle_force_types::ParticleIdx;
use crate::{
    ParticleContact, ParticleContactGeneratorOps, ParticleForceRegistry, ParticleForceRegistryOps, ParticleOps,
};

#[derive(Debug)]
pub struct ParticleRod {
    particle: Vec<Option<ParticleIdx>>,
    length: f32,
    restitution: f32,
}

impl ParticleContactGeneratorOps for ParticleRod {
    fn add_contact(&mut self, registry: &ParticleForceRegistry, limit: usize) -> Option<Vec<ParticleContact>> {
        // find length of cable
        let current_len = self.current_length(registry);

        //check overextended
        if current_len == self.length {
            return None;
        }
        let mut contact = ParticleContact::new();

        contact.set_particle0(self.particle[0]);
        contact.set_particle1(self.particle[1]);

        let p0 = registry.get_particle(self.particle[0].unwrap());
        let p1 = registry.get_particle(self.particle[1].unwrap());

        let normal = p1.get_position() - p0.get_position();
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
}

impl ParticleRod {
    pub fn new() -> ParticleRod {
        ParticleRod {
            particle: vec![None; 2],
            length: 0.0,
            restitution: 0.0,
        }
    }

    pub fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    pub fn get_length(&mut self) -> f32 {
        self.length
    }

    pub fn set_particle0(&mut self, p0: Option<ParticleIdx>) {
        self.particle[0] = p0;
    }

    pub fn set_particle1(&mut self, p1: Option<ParticleIdx>) {
        self.particle[1] = p1;
    }

    pub fn current_length(&self, registry: &ParticleForceRegistry) -> f32 {
        let p0 = &registry.get_particle(self.particle[0].unwrap());
        let p1 = &registry.get_particle(self.particle[0].unwrap());
        let relative_pos = p1.get_position() - p0.get_position();
        Tuple4D::magnitude(&relative_pos)
    }
}

// TODO
#[test]
fn test_particle_contact() {}
