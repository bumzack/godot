use crate::force::particle_force_registry::ParticleForceRegistry;
use crate::force::particle_force_types::ParticleContainer;
use crate::particle::particle::{Particle, ParticleOps};
use math::prelude::*;

pub trait ParticleForceGeneratorOps: ParticleForceGeneratorOpsClone {
    fn update_force(&self, particle: &mut Particle, duration: f32, all_particles: &ParticleContainer);
}

// this is from some clever guy on the internet - either stackoverflow or rust lang forum
// in case you get a "trait Clone not Implement for trait XXX"  - not struct XXX
pub trait ParticleForceGeneratorOpsClone {
    fn clone_box(&self) -> Box<ParticleForceGeneratorOps>;
}

impl<T> ParticleForceGeneratorOpsClone for T
where
    T: 'static + ParticleForceGeneratorOps + Clone,
{
    fn clone_box(&self) -> Box<ParticleForceGeneratorOps> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<ParticleForceGeneratorOps> {
    fn clone(&self) -> Box<ParticleForceGeneratorOps> {
        self.clone_box()
    }
}
