use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::force::particle_force_registry::{ParticleForceRegistry, ParticleForceRegistryOps};
use crate::force::particle_force_types::{ParticleContainer, ParticleIdx};

use crate::particle::particle::{Particle, ParticleOps};
use math::prelude::*;

#[derive(Clone)]
pub struct ParticleForceBuoyancySpring {
    max_depth: f32,
    volume: f32,
    water_height: f32,
    liquid_density: f32,
}

impl ParticleForceGeneratorOps for ParticleForceBuoyancySpring {
    fn update_force(&self, particle: &mut Particle, _duration: f32, all_particles: &ParticleContainer) {
        let depth = particle.get_position().get_y();
        if depth >= self.water_height + self.max_depth {
            return;
        }

        let mut f = Tuple4D::new_vector(0.0, 0.0, 0.0);
        if depth <= self.water_height - self.max_depth {
            f.set_y(self.liquid_density * self.volume);
            particle.add_force(&f);
            return;
        }
        f.set_y(
            self.liquid_density * self.volume * (depth - self.max_depth - self.water_height) / 2.0 * self.max_depth,
        );
        particle.add_force(&f);
    }
}

impl ParticleForceBuoyancySpring {
    pub fn new() -> ParticleForceBuoyancySpring {
        ParticleForceBuoyancySpring {
            max_depth: 0.0,
            volume: 0.0,
            water_height: 0.0,
            liquid_density: 0.0,
        }
    }

    pub fn set_max_depth(&mut self, max_depth: f32) {
        self.max_depth = max_depth;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    pub fn set_water_height(&mut self, water_height: f32) {
        self.water_height = water_height;
    }
    pub fn set_liquid_density(&mut self, liquid_density: f32) {
        self.liquid_density = liquid_density;
    }
}

// TODO
#[test]
fn test_particle_force_enerator() {}
