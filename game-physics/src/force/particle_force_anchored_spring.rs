use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::force::particle_force_registry::{ParticleForceRegistry, ParticleForceRegistryOps};
use crate::force::particle_force_types::ParticleContainer;

use crate::particle::particle::{Particle, ParticleOps};
use math::prelude::*;

#[derive(Clone)]
pub struct ParticleForceAnchoredSpring {
    anchor: Tuple4D,
    spring_constant: f32,
    rest_length: f32,
}

impl ParticleForceGeneratorOps for ParticleForceAnchoredSpring {
    fn update_force(&self, particle: &mut Particle, _duration: f32, all_particles: &ParticleContainer) {
        let mut f = Tuple4D::new_point_from(particle.get_position());
        f = &f - &self.anchor;

        let mut magnitude = Tuple4D::magnitude(&f);
        magnitude = (self.rest_length - magnitude) * self.spring_constant;

        // calc. final force and apply
        let mut f = Tuple4D::normalize(&f);
        f = f * (-magnitude);
        println!("add force from spring: {:?},    particle.id = {}", f, particle.get_id());
        particle.add_force(&f);
    }
}

impl ParticleForceAnchoredSpring {
    pub fn new() -> ParticleForceAnchoredSpring {
        ParticleForceAnchoredSpring {
            anchor: Tuple4D::new_point(0.0, 0.0, 0.0),
            spring_constant: 0.0,
            rest_length: 0.0,
        }
    }

    pub fn set_anchor(&mut self, anchor: Tuple4D) {
        self.anchor = anchor;
    }

    pub fn get_anchor(&self) -> &Tuple4D {
        &self.anchor
    }

    pub fn set_spring_constant(&mut self, spring_constant: f32) {
        self.spring_constant = spring_constant;
    }
    pub fn set_rest_length(&mut self, rest_length: f32) {
        self.rest_length = rest_length;
    }
}

// TODO
#[test]
fn test_particle_force_enerator() {}
