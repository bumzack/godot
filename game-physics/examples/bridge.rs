use game_physics::ParticleLinkEnum::CableEnum;
use game_physics::ParticleRod;
use game_physics::prelude::*;
use math::prelude::*;

const GRAVITY: Tuple4D = Tuple4D {
    x: 0.0,
    y: -9.81,
    z: 0.0,
    w: 0.0,
};

const BASE_MASS: f32 = 1.0;
const EXTRA_MASS: f32 = 10.0;

struct BridgeDemo {
    supports: Vec<usize>,
    cables: Vec<usize>,
    rods: Vec<usize>,
    mass_pos: Tuple4D,
    mass_display_pos: Tuple4D,
    world: ParticleWorld,
    // ground_contact_generator: GroundContact,
}

impl BridgeDemo {
    pub fn new(max_contacts: usize) -> BridgeDemo {
        BridgeDemo {
            supports: Vec::new(),
            cables: Vec::new(),
            rods: Vec::new(),
            mass_pos: Tuple4D::empty(),
            mass_display_pos: Tuple4D::empty(),
            world: ParticleWorld::new(max_contacts, 0),
            //  ground_contact_generator: GroundContact::new(),
        }
    }

    pub fn add_supports(&mut self, pcc: usize) -> usize {
        self.supports.push(pcc);
        self.supports.len() - 1
    }

    pub fn add_cable(&mut self, pc: usize) -> usize {
        self.cables.push(pc);
        self.cables.len() - 1
    }

    pub fn add_rod(&mut self, pr: usize) -> usize {
        self.rods.push(pr);
        self.rods.len() - 1
    }

    pub fn get_particle_world_mut(&mut self) -> &mut ParticleWorld {
        &mut self.world
    }

    pub fn get_particle_world(&self) -> &ParticleWorld {
        &self.world
    }

    pub fn set_mass_pos(&mut self, pos: Tuple4D) {
        self.mass_pos = pos;
    }

    pub fn update_additional_mass(&mut self, particle_count: usize, mass: f32, registry: &mut ParticleForceRegistry) {
        for i in 0..particle_count {
            let particle = &mut registry.get_particle_mut(i);
            particle.set_mass(mass);
        }

        let mut x = self.mass_pos.x.round() as i32;
        let mut xp = math::fmod(self.mass_pos.x, 1.0);
        if x < 0 {
            x = 0;
            xp = 0.0;
        }
        if x >= 5 {
            x = 5;
            xp = 0.0;
        }

        let mut z = self.mass_pos.z.round() as i32;
        let mut zp = math::fmod(self.mass_pos.z, 1.0);
        if z < 0 {
            z = 0;
            zp = 0.0;
        }
        if z >= 1 {
            z = 1;
            zp = 0.0;
        }
        self.mass_display_pos.x = 0.0;
        self.mass_display_pos.y = 0.0;
        self.mass_display_pos.z = 0.0;

        let idx = x * 2 + z;
        let prop = (1.0 - xp) * (1.0 - zp);
        let particle = &mut registry.get_particle_mut(idx as usize);

        particle.set_mass(BASE_MASS + EXTRA_MASS * prop as f32);
        let mut mdp = self.get_mass_display_pos();
        let mdp = mdp + &(particle.get_position() * prop as f32);
        self.set_mass_display_pos(mdp);

        if xp > 0.0 {
            let idx = x * 2 + z + 2;
            let particle = &mut registry.get_particle_mut(idx as usize);

            let prop = xp * (1.0 - zp);
            let mut mdp = self.get_mass_display_pos();
            let mdp = mdp + &(particle.get_position() * prop as f32);
            self.set_mass_display_pos(mdp);

            if zp > 0.0 {
                let idx = x * 2 + z + 3;
                let particle = &mut registry.get_particle_mut(idx as usize);

                let prop = xp * zp;
                let mut mdp = self.get_mass_display_pos();
                let mdp = mdp + &(particle.get_position() * prop as f32);
                self.set_mass_display_pos(mdp);
            }
        }

        if zp > 0.0 {
            let idx = x * 2 + z + 1;
            let particle = &mut registry.get_particle_mut(idx as usize);

            let prop = (1.0 - xp) * zp;
            let mut mdp = self.get_mass_display_pos();
            let mdp = mdp + &(particle.get_position() * prop as f32);
            self.set_mass_display_pos(mdp);
        }
    }

    pub fn set_mass_display_pos(&mut self, pos: Tuple4D) {
        self.mass_display_pos = pos;
    }

    pub fn get_mass_display_pos(&self) -> &Tuple4D {
        &self.mass_display_pos
    }
}

fn main() {
    let particle_count = 12;
    let rod_count = 6;
    let cable_count = 10;
    let support_count = 12;
    let max_contacts = 1000;

    let mut bridge_demo = BridgeDemo::new(max_contacts);
    let mut registry = ParticleForceRegistry::new();

    for _i in 0..particle_count * 10 {
        registry.add_particle(Particle::new());
    }

    let ground_contact = GroundContact::new();
    let ground_contact =
        ParticleContactGenerator::new(ParticleContactGeneratorEnum::GroundContactGeneratorEnum(ground_contact));

    bridge_demo
        .get_particle_world_mut()
        .add_contact_generator(ground_contact);

    // Create the masses and connections.
    bridge_demo.set_mass_pos(Tuple4D::new_point(0.0, 0.0, 0.5));
    for i in 0..particle_count {
        // let x = (i % 12) / 2;
        let mut particle = &mut registry.get_particles_mut()[i];
        particle.set_position(Tuple4D::new_point(
            (i as f32 / 2.0) * 2.0 - 5.0,
            4.0,
            (i as f32 % 2.0) * 2.0 - 1.0,
        ));
        particle.set_velocity(Tuple4D::new_vector(0.0, 0.0, 0.0));
        particle.set_damping(0.9);
        particle.set_acceleration(GRAVITY);
        particle.clear_accumulator();
    }

    // add the links
    for i in 0..cable_count {
        let mut cable = ParticleCable::new();
        cable.set_particle0(Some(i));
        cable.set_particle1(Some(i + 2));
        cable.set_max_length(1.9);
        cable.set_restitution(0.3);
        let cable = ParticleContactGenerator::new(ParticleContactGeneratorEnum::ParticleCableEnum(cable));
        let idx = bridge_demo.get_particle_world_mut().add_contact_generator(cable);
        bridge_demo.add_cable(idx);
    }

    // add the supports
    for i in 0..support_count {
        let mut cable_constraint = ParticleCableConstraint::new();
        cable_constraint.set_particle0(Some(i));
        cable_constraint.set_anchor(Tuple4D::new_point(
            (i as f32 / 2.0) * 2.2 - 5.5,
            6.0,
            (i as f32 % 2.0) * 1.6 - 0.8,
        ));
        if i < 6 {
            cable_constraint.set_max_lengths((i as f32 / 2.0) * 0.5 + 3.0);
        } else {
            cable_constraint.set_max_lengths(5.5 - (i as f32 / 2.0) * 0.5);
        }
        cable_constraint.set_restitution(0.5);

        let cable_constraint = ParticleContactGenerator::new(
            ParticleContactGeneratorEnum::ParticleCableConstraintEnum(cable_constraint),
        );

        let idx = bridge_demo
            .get_particle_world_mut()
            .add_contact_generator(cable_constraint);
        bridge_demo.add_supports(idx);
    }

    // add the rods
    for i in 0..rod_count {
        let mut particle_rod = ParticleRod::new();
        particle_rod.set_particle0(Some(i * 2));
        particle_rod.set_particle1(Some(i * 2 + 1));

        particle_rod.set_length(2.0);

        let particle_rod = ParticleContactGenerator::new(ParticleContactGeneratorEnum::ParticleRodEnum(particle_rod));

        let idx = bridge_demo.get_particle_world_mut().add_contact_generator(particle_rod);
        bridge_demo.add_rod(idx);
    }

    bridge_demo.update_additional_mass(particle_count, BASE_MASS, &mut registry);

    bridge_demo.get_particle_world_mut().start_frame(&mut registry);

    let cnt_frames = 5;
    let duration = 0.5;
    for i in 0..cnt_frames {
        bridge_demo.get_particle_world().render_world(i, &registry);

        println!("-----------------------------");
        println!("------frame {}  -----------------------", i);
        println!("-----------------------------\n\n");
        bridge_demo
            .get_particle_world_mut()
            .run_physics(duration, &mut registry);
        println!("\n\n");
    }
}
