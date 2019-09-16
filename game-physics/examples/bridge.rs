use game_physics::ParticleContactGeneratorEnum::GroundContactGeneratorEnum;
use game_physics::ParticleRod;
use game_physics::prelude::*;
use math::prelude::*;

struct BridgeDemo {
    supports: Vec<ParticleCableConstraint>,
    cables: Vec<ParticleCable>,
    rods: Vec<ParticleRod>,
    mass_pos: Tuple4D,
    mass_display_pos: Tuple4D,
    world: ParticleWorld,
    ground_contact_generator: GroundContact,
}

impl BridgeDemo {
    pub fn new(particle_count: usize) -> BridgeDemo {
        BridgeDemo {
            supports: Vec::new(),
            cables: Vec::new(),
            rods: Vec::new(),
            mass_pos: Tuple4D::empty(),
            mass_display_pos: Tuple4D::empty(),
            world: ParticleWorld::new(particle_count, 0),
            ground_contact_generator: GroundContact::new(),
        }
    }

    pub fn add_cable_constraint(&mut self, pcc: ParticleCableConstraint) -> usize {
        self.supports.push(pcc);
        self.supports.len() - 1
    }

    pub fn add_particle_cable(&mut self, pc: ParticleCable) -> usize {
        self.cables.push(pc);
        self.cables.len() - 1
    }

    pub fn add_particle_rod(&mut self, pr: ParticleRod) -> usize {
        self.rods.push(pr);
        self.rods.len() - 1
    }

    pub fn get_particle_world_mut(&mut self) -> &mut ParticleWorld {
        &mut self.world
    }
}

fn main() {
    let particle_count = 10;
    let mut bridge_demo = BridgeDemo::new(10 * 10);
    let mut particle_force_registry = ParticleForceRegistry::new();

    for i in 0..particle_count * 10 {
        particle_force_registry.add_particle(Particle::new());
    }

    let ground_contact = GroundContact::new();
    let ground_contact =
        ParticleContactGenerator::new(ParticleContactGeneratorEnum::GroundContactGeneratorEnum(ground_contact));
    bridge_demo
        .get_particle_world_mut()
        .add_contact_generator(ground_contact);
}
