use math::prelude::*;

#[derive(Clone, Debug, Copy)]
pub struct Particle {
    position: Tuple4D,
    velocity: Tuple4D,
    acceleration: Tuple4D,
    force_accum: Tuple4D,
    damping: f32,
    inverse_mass: f32,
    pub mass: f32,
    id: i32,
}

pub trait ParticleOps {
    fn set_position(&mut self, p: Tuple4D);
    fn set_velocity(&mut self, v: Tuple4D);
    fn set_acceleration(&mut self, a: Tuple4D);

    fn get_position(&self) -> &Tuple4D;
    fn get_position_mut(&mut self) -> &Tuple4D;

    fn get_velocity(&self) -> &Tuple4D;
    fn get_velocity_mut(&mut self) -> &Tuple4D;

    fn get_acceleration(&self) -> &Tuple4D;
    fn get_acceleration_mut(&mut self) -> &Tuple4D;

    fn set_damping(&mut self, d: f32);
    fn get_damping(self) -> f32;

    fn set_inverse_mass(&mut self, m: f32);
    fn get_inverse_mass(self) -> f32;
    fn get_mass(self) -> f32;
    fn has_finite_mass(&self) -> bool;

    fn integrate(&mut self, duration: f32);

    fn clear_accumulator(&mut self);

    fn add_force(&mut self, f: &Tuple4D);

    fn set_id(&mut self, id: i32);
    fn get_id(self) -> i32;
}

impl ParticleOps for Particle {
    fn set_position(&mut self, p: Tuple4D) {
        self.position = p;
    }

    fn set_velocity(&mut self, v: Tuple4D) {
        self.velocity = v;
    }

    fn set_acceleration(&mut self, a: Tuple4D) {
        self.acceleration = a;
    }

    fn get_position(&self) -> &Tuple4D {
        &self.position
    }

    fn get_position_mut(&mut self) -> &Tuple4D {
        &mut self.position
    }

    fn get_velocity(&self) -> &Tuple4D {
        &self.velocity
    }

    fn get_velocity_mut(&mut self) -> &Tuple4D {
        &mut self.velocity
    }

    fn get_acceleration(&self) -> &Tuple4D {
        &self.acceleration
    }

    fn get_acceleration_mut(&mut self) -> &Tuple4D {
        &mut self.acceleration
    }

    fn set_damping(&mut self, d: f32) {
        self.damping = d;
    }

    fn get_damping(self) -> f32 {
        self.damping
    }

    fn set_inverse_mass(&mut self, m: f32) {
        self.inverse_mass = m;
    }

    fn get_inverse_mass(self) -> f32 {
        self.inverse_mass
    }

    fn get_mass(self) -> f32 {
        if self.inverse_mass > 0.0 {
            return 1.0 / self.inverse_mass;
        }
        // TODO ???
        -1.0
    }

    fn has_finite_mass(&self) -> bool {
        self.inverse_mass > 0.0
    }

    fn integrate(&mut self, duration: f32) {
        assert!(duration > 0.0);

        // update lin. position
        self.position = &self.position + &(&self.velocity * duration);

        // wor out acceleration from force
        let mut resulting_acc = Tuple4D::new_vector_from(&self.acceleration);
        resulting_acc = resulting_acc + self.force_accum * self.inverse_mass;

        // update linear velocity from acc.
        self.velocity = &self.velocity + &(resulting_acc * duration);

        // impose drag
        self.velocity = &self.velocity * self.damping.powf(duration);

        // clear forces
        self.clear_accumulator();
    }

    fn clear_accumulator(&mut self) {
        self.force_accum.x = 0.0;
        self.force_accum.y = 0.0;
        self.force_accum.z = 0.0;
    }

    fn add_force(&mut self, f: &Tuple4D) {
        println!();
        println!(
            "Particle::add_force:  force_accum = {:?}, adding f = {:?}",
            self.force_accum, f
        );
        self.force_accum = &self.force_accum + &f;
        println!("Particle::add_force:   AFTERwards force_accum = {:?}", self.force_accum);
        println!();
    }

    fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    fn get_id(self) -> i32 {
        self.id
    }
}

impl Particle {
    // seems good default values?
    pub fn new() -> Particle {
        Particle {
            position: Tuple4D::new_point(0.0, 0.0, 0.0),
            velocity: Tuple4D::new_vector(0.0, 0.0, 0.0),
            acceleration: Tuple4D::new_vector(0.0, 0.0, 0.0),
            force_accum: Tuple4D::new_vector(0.0, 0.0, 0.0),
            damping: 0.999,
            inverse_mass: 0.0,
            mass: 0.0,
            id: -1,
        }
    }
}

// TODO
#[test]
fn test_particle_integrate() {
    let velocity_initial_expected = Tuple4D::new_vector(0.0, 0.0, 0.0);
    let velocity_after_integration_expected = Tuple4D::new_vector(1.0, 1.0, 1.0);

    let mut p = Particle::new();
    // p.set_velocity()
    // p.set_inverse_mass(2.0;

    //    assert_Tuple4D(p.get_velocity(), &velocity_after_integration_expected);
    //
    //    // execute one integration step
    //    p.integrate(2.0);
    //
    //    assert_Tuple4D(p.get_velocity(), &velocity_after_integration_expected);
}
