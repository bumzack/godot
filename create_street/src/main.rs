use raytracer::prelude::*;

mod coord_axes;
mod walls_with_2_tilts;

fn main() {
    main_coord_axes();
}

fn main_coord_axes() {
    let width = 320;
    let height = 200;

    let (mut world, mut camera) = coord_axes::setup_world_coord_axes(width, height, false);
    coord_axes::add_floor(&mut world);
    coord_axes::add_borders(&mut world);

    let backend = BackendCuda::new();
    // let backend = BackendCpu::new();

    let (frames, delta) = (10, 0.8);
    //    let (frames, delta) = (25, 0.6);

    let is_3d = true;
    let full_raytracing = false;

    let mut x: f32 = 0.0;

    coord_axes::animate(
        width,
        height,
        &mut world,
        &mut camera,
        Box::new(backend),
        frames,
        delta,
        is_3d,
        full_raytracing,
        x,
    )
}

fn main_wall_with_2_tilsts() {
    let width = 1280;
    let height = 720;

    let (mut world, mut camera) = walls_with_2_tilts::setup_world_2_walls_and_tilts(width, height);

    let backend = BackendCuda::new();

    let (frames, delta) = (420, 0.02);
    //    let (frames, delta) = (25, 0.6);

    let is_3d = false;
    let full_raytracing = true;

    let mut x: f32 = 0.0;

    walls_with_2_tilts::animate(
        width,
        height,
        &mut world,
        &mut camera,
        Box::new(backend),
        frames,
        delta,
        is_3d,
        full_raytracing,
        x,
    )
}
