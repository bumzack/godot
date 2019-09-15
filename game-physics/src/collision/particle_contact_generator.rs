use crate::ParticleContact;

#[derive(Clone, Debug)]
pub struct ParticleContactGenerator {}

pub trait ParticleContactGeneratorOps {
    fn add_contact(&self, limit: usize) -> Vec<ParticleContact>;
}

impl ParticleContactGeneratorOps for ParticleContactGenerator {
    fn add_contact(&self, limit: usize) -> Vec<ParticleContact> {
        let res: Vec<ParticleContact> = Vec::new();
        res
    }
}
