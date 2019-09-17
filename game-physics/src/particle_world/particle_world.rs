use std::f32::consts::PI;

use crate::{
    ParticleContact, ParticleContactGenerator, ParticleContactGeneratorOps, ParticleContactResolver,
    ParticleContactResolverOps, ParticleForceRegistry, ParticleForceRegistryOps, ParticleOps,
};
use crate::raytracer::prelude::*;

pub struct ParticleWorld {
    contact_generators: Vec<ParticleContactGenerator>,
    calculate_iterations: bool,
    particle_contact_resolver: ParticleContactResolver,
    contacts: Vec<ParticleContact>,
    max_contacts: usize,
    iterations: usize,
}

pub trait ParticleWorldOps {
    fn generate_contacts(&mut self, registry: &ParticleForceRegistry) -> usize;
    fn integrate(&mut self, duration: f32, registry: &mut ParticleForceRegistry);
    fn run_physics(&mut self, duration: f32, registry: &mut ParticleForceRegistry);
    fn start_frame(&mut self, registry: &mut ParticleForceRegistry);

    //    fn get_particles(&self) -> &Vec<Particle>;
    //    fn get_particles_mut(&mut self) -> &mut Vec<Particle>;

    // fn get_contact_generators(&self) -> &Vec<ParticleContactGenerator>;
    fn add_contact_generator(&mut self, contact_generator: ParticleContactGenerator) -> usize;

    fn render_world(&self ,cnt: usize,  registry: &ParticleForceRegistry);
}

impl ParticleWorldOps for ParticleWorld {
    fn generate_contacts(&mut self, registry: &ParticleForceRegistry) -> usize {
        let mut limit = self.max_contacts;
        let mut res: Vec<ParticleContact> = Vec::new();

        for cg in &mut self.contact_generators.iter_mut() {
            // TODO: if let ...
            let mut new_contacts = match cg.add_contact(registry, limit) {
                Some(c) => c,
                None => vec![],
            };
            limit -= new_contacts.len();
            // TODO: exit closure?!
            //            if limit <= 0 {
            //                break;
            //            }
            res.append(new_contacts.as_mut());
        }
        self.max_contacts - limit
    }

    fn integrate(&mut self, duration: f32, registry: &mut ParticleForceRegistry) {
        registry
            .get_particles_mut()
            .iter_mut()
            .for_each(|p| p.integrate(duration));
    }

    fn run_physics(&mut self, duration: f32, registry: &mut ParticleForceRegistry) {
        registry.update_forces(duration);
        self.integrate(duration, registry);
        let used_contacts = self.generate_contacts(registry);
        if used_contacts > 0 {
            if self.calculate_iterations {
                self.particle_contact_resolver.set_iterations(used_contacts * 2);
                self.particle_contact_resolver.resolve_contacts(
                    &mut self.contacts,
                    used_contacts,
                    duration,
                    registry,
                );
            }
        }
    }

    fn start_frame(&mut self, registry: &mut ParticleForceRegistry) {
        registry
            .get_particles_mut()
            .iter_mut()
            .for_each(|p| p.clear_accumulator());
    }

    //    fn get_particles(&self) -> &Vec<Particle> {
    //        &self.particles
    //    }
    //
    //    fn get_particles_mut(&mut self) -> &mut Vec<Particle> {
    //        &mut self.particles
    //    }

//    fn get_contact_generators(&self) -> &Vec<ParticleContactGenerator> {
//        &self.contact_generators
//    }
//
//    fn get_particle_force_registry(&self) -> &ParticleForceRegistry {
//        &self.particle_force_registry
//    }

    fn add_contact_generator(&mut self, contact_generator: ParticleContactGenerator) -> usize {
        self.contact_generators.push(contact_generator);
        self.contact_generators.len() - 1
    }

    fn render_world(&self, cnt: usize, registry: &ParticleForceRegistry) {
        let particle_scale = 0.1;
        let particle_color = Color::new(1.0, 0.0,0.0);

        let rod_scale = 0.1;
        let rod_color = Color::new(0.0, 1.0,0.0);

        let support_scale = 0.1;
        let support_color = Color::new(0.0, 0.0,1.0);


        let width = 240;
        let height = 200;
        let b = BackendCpu::new();

        let mut world = World::new();

        let pl = PointLight::new(Tuple4D::new_point(-1.0, 20.0, -20.0), Color::new(1.0, 1.0, 1.0));
        let l = Light::PointLight(pl);
        world.set_light(l);

        let mut c = Camera::new(width, height, PI / 5.5);
        c.calc_pixel_size();
        c.set_transformation(Matrix::view_transform(
            &Tuple4D::new_point(0.0, 10.5, -15.0),
            &Tuple4D::new_point(0.0, 1.0, 0.0),
            &Tuple4D::new_vector(0.0, 1.0, 0.0),
        ));
        c.set_antialiasing(false);
        c.set_calc_reflection(false);
        c.set_calc_refraction(false);

        // floor
        let mut floor = Plane::new();
        let m = Matrix::rotate_y(PI / 4.0);
        floor.set_transformation(m);
        floor.get_material_mut().set_color(WHITE);
        let floor = Shape::new(ShapeEnum::Plane(floor));
        world.add_shape(floor);


        for p in registry.get_particles().iter() {
           let mut  sphere = Sphere::new();
            let m_trans = Matrix::translation(p.get_position().x, p.get_position().y, p.get_position().z);
            let m_scale = Matrix::scale(particle_scale,particle_scale, particle_scale);
            sphere.set_transformation(m_trans * m_scale);
            sphere.get_material_mut().set_color(particle_color.clone());
            let sphere = Shape::new(ShapeEnum::Sphere(sphere));
            world.add_shape(sphere);
        }

        let filename  = format!("bridge_{}.png", cnt);
        let canvas = b.render_world_multi_core(&mut world, &c);
        canvas.unwrap().write_png(&filename).unwrap();
    }
}

impl ParticleWorld {
    pub fn new(max_contacts: usize, iterations: usize) -> ParticleWorld {
        ParticleWorld {
            //  particles: Vec::new(),
            contact_generators: Vec::new(),
            calculate_iterations: true,
            particle_contact_resolver: ParticleContactResolver::new(),
            contacts: Vec::new(),
            max_contacts,
            iterations,
        }
    }
}
