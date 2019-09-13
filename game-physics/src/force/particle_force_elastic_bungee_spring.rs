use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::force::particle_force_registry::{ParticleForceRegistry, ParticleForceRegistryOps};
use crate::force::particle_force_types::{ParticleContainer, ParticleIdx};

use crate::particle::particle::{Particle, ParticleOps};
use math::prelude::*;

#[derive(Clone)]
pub struct ParticleForceElasticBungeeSpring {
    other: Option<ParticleIdx>,
    spring_constant: f32,
    rest_length: f32,
}

impl ParticleForceGeneratorOps for ParticleForceElasticBungeeSpring {
    fn update_force(&self, particle: &mut Particle, _duration: f32, all_particles: &ParticleContainer) {
        let other_particle = all_particles[self.other.unwrap()];
        let mut f = Tuple4D::new_point_from(particle.get_position());
        f = &f - other_particle.get_position();

        let mut magnitude = Tuple4D::magnitude(&f);
        if magnitude <= self.rest_length {
            return;
        }

        magnitude = (self.rest_length - magnitude) * self.spring_constant;

        // calc. final force and apply
        let mut f = Tuple4D::normalize(&f);
        f = f * (-magnitude);
        println!("add force from spring: {:?},    particle.id = {}", f, particle.get_id());
        particle.add_force(&f);
    }
}

impl ParticleForceElasticBungeeSpring {
    pub fn new() -> ParticleForceElasticBungeeSpring {
        ParticleForceElasticBungeeSpring {
            other: None,
            spring_constant: 0.0,
            rest_length: 0.0,
        }
    }

    pub fn set_other(&mut self, other: ParticleIdx) {
        self.other = Some(other);
    }

    pub fn get_other(&self) -> ParticleIdx {
        self.other.unwrap()
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
