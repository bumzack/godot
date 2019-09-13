use crate::collision::particle_contact::ParticleContact;
use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::particle::particle::Particle;

pub(crate) type ParticleIdx = usize;
pub(crate) type ParticleForceGeneratorOpsIdx = usize;

pub(crate) type ParticleForceGeneratorOpsContainer = Vec<Box<dyn ParticleForceGeneratorOps>>;
pub(crate) type ParticleContainer = Vec<Particle>;

pub(crate) type ParticleContactsContainer = Vec<ParticleContact>;
