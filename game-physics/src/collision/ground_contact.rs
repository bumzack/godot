use math::prelude::*;

use crate::{Particle, ParticleContact};
use crate::particle::particle::ParticleOps;

pub struct GroundContact<'a> {
    particles: &'a Vec<Particle>,
}

pub trait GroundContactOps {
    fn add_contact(&mut self) -> Vec<ParticleContact>;
}

impl<'a> GroundContactOps for GroundContact<'a> {
    fn add_contact(&mut self) -> Vec<ParticleContact> {
        let mut res: Vec<ParticleContact> = Vec::new();

        for (idx, p) in self.particles.iter().enumerate() {
            let y = p.get_position().get_y();
            if y < 0.0 {
                let mut contact = ParticleContact::new();
                contact.set_contact_normal(Tuple4D::up());
                contact.set_particle0(idx);
                contact.set_penetration(-y);
                contact.set_restitution(0.2);
                res.push(contact);
            }
        }
        res
    }
}

impl<'a> GroundContact<'a> {
    fn new(particles: &Vec<Particle>) -> GroundContact {
        GroundContact { particles }
    }
}
