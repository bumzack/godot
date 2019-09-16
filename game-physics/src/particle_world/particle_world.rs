use crate::{
    Particle, ParticleContact, ParticleContactGenerator, ParticleContactGeneratorOps, ParticleContactResolver,
    ParticleContactResolverOps, ParticleForceRegistry, ParticleForceRegistryOps, ParticleOps,
};

pub struct ParticleWorld {
    particles: Vec<Particle>,
    contact_generators: Vec<ParticleContactGenerator>,
    calculate_iterations: bool,
    particle_force_registry: ParticleForceRegistry,
    particle_contact_resolver: ParticleContactResolver,
    contacts: Vec<ParticleContact>,
    max_contacts: usize,
    iterations: usize,
}

pub trait ParticleWorldOps {
    fn generate_contacts(&mut self, registry: &ParticleForceRegistry) -> usize;
    fn integrate(&mut self, duration: f32);
    fn run_physics(&mut self, duration: f32, registry: &ParticleForceRegistry);
    fn start_frame(&mut self);

    fn get_particles(&self) -> &Vec<Particle>;
    fn get_contact_generators(&self) -> &Vec<ParticleContactGenerator>;
    fn get_particle_force_registry(&self) -> &ParticleForceRegistry;

    fn add_contact_generator(&mut self, contact_generator: ParticleContactGenerator);
}

impl ParticleWorldOps for ParticleWorld {
    fn generate_contacts(&mut self, registry: &ParticleForceRegistry) -> usize {
        let mut limit = self.max_contacts;
        let mut res: Vec<ParticleContact> = Vec::new();

        for cg in &mut self.contact_generators.iter_mut() {
            let mut new_contacts = cg.add_contact(registry, limit).unwrap();
            limit -= new_contacts.len();
            // TODO: exit closure?!
            //            if limit <= 0 {
            //                break;
            //            }
            res.append(new_contacts.as_mut());
        }
        self.max_contacts - limit
    }

    fn integrate(&mut self, duration: f32) {
        self.particles.iter_mut().for_each(|p| p.integrate(duration));
    }

    fn run_physics(&mut self, duration: f32, registry: &ParticleForceRegistry) {
        self.particle_force_registry.update_forces(duration);
        self.integrate(duration);
        let used_contacts = self.generate_contacts(registry);
        if used_contacts > 0 {
            if self.calculate_iterations {
                self.particle_contact_resolver.set_iterations(used_contacts * 2);
                self.particle_contact_resolver.resolve_contacts(
                    &mut self.contacts,
                    used_contacts,
                    duration,
                    &mut self.particle_force_registry,
                );
            }
        }
    }

    fn start_frame(&mut self) {
        self.particles.iter_mut().for_each(|p| p.clear_accumulator());
    }

    fn get_particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    fn get_contact_generators(&self) -> &Vec<ParticleContactGenerator> {
        &self.contact_generators
    }

    fn get_particle_force_registry(&self) -> &ParticleForceRegistry {
        &self.particle_force_registry
    }

    fn add_contact_generator(&mut self, contact_generator: ParticleContactGenerator) {
        self.contact_generators.push(contact_generator);
    }
}

impl ParticleWorld {
    pub fn new(max_contacts: usize, iterations: usize) -> ParticleWorld {
        ParticleWorld {
            particles: Vec::new(),
            contact_generators: Vec::new(),
            calculate_iterations: false,
            particle_force_registry: ParticleForceRegistry::new(),
            particle_contact_resolver: ParticleContactResolver::new(),
            contacts: Vec::new(),
            max_contacts,
            iterations,
        }
    }
}
