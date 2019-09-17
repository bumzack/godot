use crate::{
    GroundContact, ParticleCable, ParticleContact, ParticleContactGeneratorOps, ParticleForceRegistry, ParticleRod,
};

pub enum ParticleLinkEnum {
    GroundContactEnum(GroundContact),
    RodEnum(ParticleRod),
    CableEnum(ParticleCable),
}

// #[derive(Debug)]
pub struct ParticleLink {
    particle_link: ParticleLinkEnum,
}

pub trait ParticleLinkOps {
    fn add_contact(&mut self, registry: &ParticleForceRegistry, limit: usize) -> Option<Vec<ParticleContact>>;

    fn current_length(&self, registry: &ParticleForceRegistry) -> f32;
}

impl ParticleLinkOps for ParticleLink {
    fn add_contact(&mut self, registry: &ParticleForceRegistry, limit: usize) -> Option<Vec<ParticleContact>> {
        match self.particle_link {
            ParticleLinkEnum::GroundContactEnum(ref mut ground_contact) => ground_contact.add_contact(registry, limit),
            ParticleLinkEnum::RodEnum(ref mut particle_rod) => particle_rod.add_contact(registry, limit),
            ParticleLinkEnum::CableEnum(ref mut particle_cable) => particle_cable.add_contact(registry, limit),
        }
    }

    fn current_length(&self, registry: &ParticleForceRegistry) -> f32 {
        match self.particle_link {
            ParticleLinkEnum::GroundContactEnum(ref ground_contact) => unreachable!("should never be called"),
            ParticleLinkEnum::RodEnum(ref particle_rod) => particle_rod.current_length(registry),
            ParticleLinkEnum::CableEnum(ref particle_cable) => particle_cable.current_length(registry),
        }
    }
}

impl ParticleLink {
    pub fn new(particle_link: ParticleLinkEnum) -> ParticleLink {
        ParticleLink { particle_link }
    }
}
