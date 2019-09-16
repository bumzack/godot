use crate::{ParticleCableConstraint, ParticleContact, ParticleForceRegistry, ParticleRodConstraint};

pub enum ParticleConstraintEnum {
    ParticleCableConstraintEnum(ParticleCableConstraint),
    ParticleRodConstraintEnum(ParticleRodConstraint),
}

// #[derive(Debug)]
pub struct ParticleConstraint {
    particle_constraint: ParticleConstraintEnum,
}

pub trait ParticleConstraintOps {
    fn add_contact(&mut self, registry: &ParticleForceRegistry, limit: usize) -> Option<Vec<ParticleContact>>;

    fn current_length(&self, registry: &ParticleForceRegistry) -> f32;
}

impl ParticleConstraintOps for ParticleConstraint {
    fn add_contact(&mut self, registry: &ParticleForceRegistry, limit: usize) -> Option<Vec<ParticleContact>> {
        match self.particle_constraint {
            ParticleConstraintEnum::ParticleCableConstraintEnum(ref mut particle_cable_constraint) => {
                particle_cable_constraint.add_contact(registry, limit)
            }
            ParticleConstraintEnum::ParticleRodConstraintEnum(ref mut particle_rod_constraint) => {
                particle_rod_constraint.add_contact(registry, limit)
            }
        }
    }

    fn current_length(&self, registry: &ParticleForceRegistry) -> f32 {
        match self.particle_constraint {
            ParticleConstraintEnum::ParticleCableConstraintEnum(ref particle_cable_constraint) => {
                particle_cable_constraint.current_length(registry)
            }
            ParticleConstraintEnum::ParticleRodConstraintEnum(ref particle_rod_constraint) => {
                particle_rod_constraint.current_length(registry)
            }
        }
    }
}

impl ParticleConstraint {
    pub fn new(particle_constraint: ParticleConstraintEnum) -> ParticleConstraint {
        ParticleConstraint { particle_constraint }
    }
}
