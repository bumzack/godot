use crate::{
    GroundContact, ParticleCable, ParticleCableConstraint, ParticleConstraintOps, ParticleContact,
    ParticleForceRegistry, ParticleLinkOps, ParticleRod,
};

// #[derive(Debug)]
pub enum ParticleContactGeneratorEnum {
    GroundContactGeneratorEnum(GroundContact),
    ParticleRodEnum(ParticleRod),
    ParticleCableEnum(ParticleCable),
    ParticleCableConstraintEnum(ParticleCableConstraint),
}

// #[derive(Debug)]
pub struct ParticleContactGenerator {
    particle_contact_generator: ParticleContactGeneratorEnum,
}

pub trait ParticleContactGeneratorOps {
    fn add_contact(&mut self, registry: &ParticleForceRegistry, limit: usize) -> Option<Vec<ParticleContact>>;
    //    fn current_length(&self, registry: &ParticleForceRegistry) -> f32;
}

impl ParticleContactGeneratorOps for ParticleContactGenerator {
    fn add_contact(&mut self, registry: &ParticleForceRegistry, limit: usize) -> Option<Vec<ParticleContact>> {
        match self.particle_contact_generator {
            ParticleContactGeneratorEnum::GroundContactGeneratorEnum(ref mut ground_contact) => {
                ground_contact.add_contact(registry, limit)
            }
            ParticleContactGeneratorEnum::ParticleRodEnum(ref mut particle_rod) => {
                particle_rod.add_contact(registry, limit)
            }
            ParticleContactGeneratorEnum::ParticleCableEnum(ref mut particle_cable) => {
                particle_cable.add_contact(registry, limit)
            }
            ParticleContactGeneratorEnum::ParticleCableConstraintEnum(ref mut particle_cable_contraint) => {
                particle_cable_contraint.add_contact(registry, limit)
            }
        }
    }

    //    fn current_length(&self, registry: &ParticleForceRegistry) -> f32 {
    //        match self.particle_contact_generator {
    //            ParticleContactGeneratorEnum::GroundContactGeneratorEnum(ref ground_contact) => ground_contact.current_length(registry),
    //            ParticleContactGeneratorEnum::ParticleRodEnum(ref particle_rod) => particle_rod.current_length(registry),
    //            ParticleContactGeneratorEnum::ParticleCableEnum(ref particle_cable) => particle_cable.current_length(registry),
    //        }
    //    }
}

impl ParticleContactGenerator {
    pub fn new(contact_generator: ParticleContactGeneratorEnum) -> ParticleContactGenerator {
        ParticleContactGenerator {
            particle_contact_generator: contact_generator,
        }
    }
}
