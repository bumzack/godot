use game_physics::{ParticleWorld, ParticleWorldOps};

fn main() {
    let mut particle_world = ParticleWorld::new(1, 1);
    particle_world.start_frame();
    // world.startFrame();

    // Find the duration of the last frame in seconds
    // float duration = (float)TimingData::get().lastFrameDuration * 0.001f;
    // if (duration <= 0.0f) return;

    let duration = 0.0;

    // Run the simulation
    particle_world.run_physics(duration);
}
