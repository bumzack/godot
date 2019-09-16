use math::prelude::*;

use crate::{
    ParticleContact, ParticleContactGeneratorOps, ParticleForceRegistry, ParticleForceRegistryOps, ParticleOps,
};

#[derive(Debug)]
pub struct GroundContact {}

impl ParticleContactGeneratorOps for GroundContact {
    fn add_contact(&mut self, registry: &ParticleForceRegistry, limit: usize) -> Option<Vec<ParticleContact>> {
        let mut res: Vec<ParticleContact> = Vec::new();

        for (idx, p) in registry.get_particles().iter().enumerate() {
            let y = p.get_position().get_y();
            if y < 0.0 {
                let mut contact = ParticleContact::new();
                contact.set_contact_normal(Tuple4D::up());
                contact.set_particle0(Some(idx));
                contact.set_penetration(-y);
                contact.set_restitution(0.2);
                res.push(contact);
            }
        }
        Some(res)
    }
}

impl GroundContact {
    pub fn new() -> GroundContact {
        GroundContact {}
    }

    pub fn current_length(&self, registry: &ParticleForceRegistry) -> f32 {
        unimplemented!()
    }
}
