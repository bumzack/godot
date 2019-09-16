use crate::force::particle_force_generator::ParticleForceGeneratorOps;

pub(crate) type ParticleIdx = usize;
pub(crate) type ParticleForceGeneratorOpsIdx = usize;

pub(crate) type ParticleForceGeneratorOpsContainer = Vec<Box<dyn ParticleForceGeneratorOps>>;
