use std::collections::HashMap;

use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::force::particle_force_types::{
    ParticleContainer, ParticleForceGeneratorOpsContainer, ParticleForceGeneratorOpsIdx, ParticleIdx,
};
use crate::particle::particle::{Particle, ParticleOps};
use math::prelude::*;

#[derive(Clone)]
pub struct ParticleForceRegistry {
    particle_force_generators: ParticleForceGeneratorOpsContainer,
    particles: ParticleContainer,
    registry: HashMap<ParticleForceGeneratorOpsIdx, Vec<ParticleIdx>>,
}

pub trait ParticleForceRegistryOps {
    fn add_particle(&mut self, p: Particle) -> ParticleIdx;

    fn add_particle_force_generator(&mut self, g: Box<ParticleForceGeneratorOps>) -> ParticleForceGeneratorOpsIdx;

    fn add_force_for_particle(&mut self, p_idx: ParticleIdx, g_idx: ParticleForceGeneratorOpsIdx);

    // fn remove(&mut self,  gen:vPFG);
    fn clear(&mut self);

    fn update_forces(&mut self, duration: f32);

    fn get_particle(&self, idx: ParticleIdx) -> &Particle;

    fn get_particle_mut(&mut self, idx: ParticleIdx) -> &mut Particle;

    fn get_particle_force_generators(&self, idx: ParticleForceGeneratorOpsIdx) -> &ParticleForceGeneratorOps;

    fn get_particle_force_generators_mut(
        &mut self,
        idx: ParticleForceGeneratorOpsIdx,
    ) -> &mut ParticleForceGeneratorOps;

    fn set_velocity(&mut self, p_idx: ParticleIdx, v: Tuple4D);
}

impl ParticleForceRegistryOps for ParticleForceRegistry {
    fn add_particle(&mut self, p: Particle) -> ParticleIdx {
        self.particles.push(p);
        self.particles.len() - 1
    }

    fn add_particle_force_generator(&mut self, g: Box<ParticleForceGeneratorOps>) -> ParticleForceGeneratorOpsIdx {
        self.particle_force_generators.push(g);
        self.particle_force_generators.len() - 1
    }

    fn add_force_for_particle(&mut self, p_idx: ParticleIdx, g_idx: ParticleForceGeneratorOpsIdx) {
        if !self.registry.contains_key(&g_idx) {
            self.registry.insert(g_idx, Vec::new());
        }
        let v = &mut self.registry.get_mut(&g_idx).unwrap();
        v.push(p_idx);
    }

    fn clear(&mut self) {
        // TODO clear the particles and generators too?
        // self.particle_force_generators.clear();
        // self.particles.clear();
        self.registry.clear();
    }

    fn update_forces(&mut self, duration: f32) {
        for (gen_idx, particles_indices) in self.registry.iter() {
            for p_idx in particles_indices.iter() {
                // TODO: here the BorrowChecker is a PITA
                // we would like to do
                //                let mut p = &mut self.particles.get_mut(*p_idx).unwrap();
                //                println!("update_forces            gen_idx = {}, p_idx = {}", gen_idx, p_idx);
                //                pfg.update_force(p, duration, &self);        // the &self is ugly too, we need a way to pass the "other" particle to the update_force for springs ..
                // --------------------------------------

                // instead we clone the force_generators and particles
                // pass the clone to the mehtod, so it has access to the "other" particle
                let particle_force_generators_clone = self.particle_force_generators.clone();
                let particles_clone = self.particles.clone();

                let pfg = particle_force_generators_clone.get(*gen_idx).unwrap();
                let p = &mut self.particles.get_mut(*p_idx).unwrap();
                println!("update_forces            gen_idx = {}, p_idx = {}", gen_idx, p_idx);
                pfg.update_force(p, duration, &particles_clone);
            }
        }
    }

    fn get_particle(&self, idx: ParticleIdx) -> &Particle {
        // TODO: index check?
        &self.particles[idx]
    }

    fn get_particle_mut(&mut self, idx: usize) -> &mut Particle {
        &mut self.particles[idx]
    }

    fn get_particle_force_generators(&self, idx: ParticleForceGeneratorOpsIdx) -> &ParticleForceGeneratorOps {
        // TODO: index check?
        &*self.particle_force_generators[idx]
    }

    fn get_particle_force_generators_mut(&mut self, idx: usize) -> &mut dyn ParticleForceGeneratorOps {
        &mut *self.particle_force_generators[idx]
    }

    fn set_velocity(&mut self, p_idx: usize, v: Tuple4D) {
        self.particles[p_idx].set_velocity(v);
    }
}

impl ParticleForceRegistry {
    pub fn new() -> ParticleForceRegistry {
        ParticleForceRegistry {
            particle_force_generators: Vec::new(),
            particles: Vec::new(),
            registry: Default::default(),
        }
    }
}

// TODO
#[test]
fn test_force_registry() {}
