use raytracer::prelude::*;

mod coord_axes;
mod walls_with_2_tilts;

fn main() {
    main_coord_axes();
}

fn main_coord_axes() {
    let width = 640;
    let height = 480;

    let debug = true;

    let (mut world, mut camera) = coord_axes::setup_world_coord_axes(width, height, false);
    coord_axes::add_floor(&mut world);
    coord_axes::add_borders(&mut world);

    let backend = BackendCuda::new();
    // let backend = BackendCpu::new();

//    let x = 82;
//    let y = 10;
//
//
//    let mut camera_from = Tuple4D::new_point(2.0, 2.0, -2.0);
//    let mut camera_to = Tuple4D::new_point(0.0, 0.0, 0.0);
//    let mut camera_up = Tuple4D::new_vector(0.0, 1.0, 0.0);
//
//    let mut light_pos = Tuple4D::new_point(2.0, 7.0, -2.0);
//
//    println!("light pos = {:?}", light_pos);
//
//    let pl = PointLight::new(light_pos, Color::new(1.0, 1.0, 1.0));
//    let l = Light::PointLight(pl);
//    world.set_light(l);
//    camera.set_transformation(Matrix::view_transform(&camera_from, &camera_to, &camera_up));
//    let canvas = backend.render_world_debug(&mut world, &camera, x, y);
//    let filename = format!("./create_street/img/debug_point_{}_{}.png", x, y);


    // real deal


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

    let (frames, delta) = (1, 0.02);
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
