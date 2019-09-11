use raytracer::prelude::*;

use std::f32::consts::PI;

pub fn setup_world_2_walls_and_tilts(width: usize, height: usize) -> (World, Camera) {
    // ---- CUBE -------
    let mut c = Cube::new();
    c.get_material_mut().set_color(Color::new(1., 0.5, 0.2));
    c.get_material_mut().set_ambient(1.0);
    c.get_material_mut().set_diffuse(0.0);
    c.get_material_mut().set_specular(0.0);

    let m_trans = Matrix::translation(0.0, 3.0, 40.0);
    let m_scale = Matrix::scale(1.0, 1.0, 0.01);
    let m = &m_trans * &m_scale;

    c.set_transformation(m);
    let mut cube = Shape::new(ShapeEnum::Cube(c));
    cube.set_casts_shadow(false);

    // ---- PLANE -------
    let mut plane = Plane::new();
    plane.get_material_mut().set_color(Color::new(0., 0.5, 0.));
    plane.get_material_mut().set_ambient(0.025);
    plane.get_material_mut().set_diffuse(0.67);
    plane.get_material_mut().set_specular(0.0);

    let mut plane = Shape::new(ShapeEnum::Plane(plane));
    plane.set_casts_shadow(false);

    // ---- SPHERE 1 -------
    let mut sphere1 = Sphere::new();
    sphere1.get_material_mut().set_color(Color::new(1.0, 0., 0.));
    sphere1.get_material_mut().set_ambient(0.1);
    sphere1.get_material_mut().set_diffuse(0.6);
    sphere1.get_material_mut().set_specular(0.0);
    sphere1.get_material_mut().set_reflective(0.3);

    let m_trans = Matrix::translation(0.0, 0.0, 0.0);
    let m_scale = Matrix::scale(0.2, 0.2, 0.2);
    let m = &m_trans * &m_scale;

    sphere1.set_transformation(m);
    let sphere1 = Shape::new(ShapeEnum::Sphere(sphere1));

    // ---- SPHERE 2 -------
    let mut sphere2 = Sphere::new();
    sphere2.get_material_mut().set_color(Color::new(0.5, 0.5, 1.0));
    sphere2.get_material_mut().set_ambient(0.1);
    sphere2.get_material_mut().set_diffuse(0.6);
    sphere2.get_material_mut().set_specular(0.0);
    sphere2.get_material_mut().set_reflective(0.3);

    let m_trans = Matrix::translation(-0.0, 0.0, 0.0);
    let m_scale = Matrix::scale(0.1, 0.1, 0.3);
    let m = &m_trans * &m_scale;

    sphere2.set_transformation(m);
    let sphere2 = Shape::new(ShapeEnum::Sphere(sphere2));

    let mut sphere2_clone = sphere2.clone();
    let m_trans = Matrix::translation(1.0, 0.0, 0.0);
    let m_scale = Matrix::scale(0.1, 0.1, 0.1);
    let m = &m_trans * &m_scale;
    sphere2_clone.set_transformation(m);

    let mut sphere2_clone_clone = sphere2.clone();
    let m_trans = Matrix::translation(1.0, 0.0, 3.0);
    let m_scale = Matrix::scale(0.3, 0.3, 0.3);
    let m = &m_trans * &m_scale;
    sphere2_clone_clone.set_transformation(m);

    // -- -left border
    let m_trans = Matrix::translation(-1.0, 0.5, -0.25);
    let m_scale = Matrix::scale(0.01, 0.2, 2.0);
    let mut border_left = Cube::new();
    border_left.set_transformation(m_trans * m_scale);
    border_left.get_material_mut().set_color(Color::new(0.5, 0.5, 0.5));
    border_left.get_material_mut().set_ambient(0.3);
    border_left.get_material_mut().set_diffuse(0.6);
    border_left.get_material_mut().set_specular(0.0);
    border_left.get_material_mut().set_reflective(0.1);
    let mut border_left = Shape::new(ShapeEnum::Cube(border_left));
    border_left.set_casts_shadow(false);

    // -- -right border
    let m_trans = Matrix::translation(1.0, 0.5, -0.15);
    let m_scale = Matrix::scale(0.01, 0.2, 2.0);
    let mut border_right = Cube::new();
    border_right.set_transformation(m_trans * m_scale);
    border_right.get_material_mut().set_color(Color::new(0.5, 0.5, 0.5));
    border_right.get_material_mut().set_ambient(0.3);
    border_right.get_material_mut().set_diffuse(0.6);
    border_right.get_material_mut().set_specular(0.0);
    border_right.get_material_mut().set_reflective(0.1);
    let mut border_right = Shape::new(ShapeEnum::Cube(border_right));
    border_right.set_casts_shadow(false);

    let left_tilt = -30.0 * PI / 180.0;
    let right_tilt = 15.0 * PI / 180.0;

    // left tilt
    let mut border_left_tilt_left = border_left.clone();
    let m_trans = Matrix::translation(0., 1.5, 2.38);
    let m_rot = Matrix::rotate_y(left_tilt);
    let m_scale = Matrix::scale(0.02, 0.2, 1.5);
    // let m = m_rot * m_scale;
    let m = &m_rot * &(m_scale * m_trans);
    border_left_tilt_left.set_transformation(m);

    let mut border_right_tilt_left = border_right.clone();
    let m_trans = Matrix::translation(95.0, 0.0, 1.83);
    let m_rot = Matrix::rotate_y(left_tilt);
    let m_scale = Matrix::scale(0.02, 0.2, 1.5);
    // let m = m_rot * m_scale;
    let m = &m_rot * &(m_scale * m_trans);
    border_right_tilt_left.set_transformation(m);

    // tight tilt
    let mut border_left_tilt_right = border_left.clone();
    let m_trans = Matrix::translation(-3.6, 0., 5.1);
    let m_rot = Matrix::rotate_y(right_tilt);
    let m_scale = Matrix::scale(0.02, 0.2, 1.5);
    // let m = m_rot * m_scale;
    let m = &m_rot * &(m_trans * m_scale);
    border_left_tilt_right.set_transformation(m);

    let mut border_right_tilt_right = border_right.clone();
    let m_trans = Matrix::translation(-1.55, 0.0, 5.8);
    let m_rot = Matrix::rotate_y(right_tilt);
    let m_scale = Matrix::scale(0.02, 0.2, 1.5);
    // let m = m_rot * m_scale;
    let m = &m_rot * &(m_trans * m_scale);
    border_right_tilt_right.set_transformation(m);

    let mut w = World::new();
    w.add_shape(sphere1);
    w.add_shape(cube);
    w.add_shape(plane);
    w.add_shape(border_left);
    w.add_shape(border_right);
    w.add_shape(border_left_tilt_left);
    w.add_shape(border_right_tilt_left);
    w.add_shape(border_left_tilt_right);
    w.add_shape(border_right_tilt_right);

    w.add_shape(sphere2);
    w.add_shape(sphere2_clone);
    w.add_shape(sphere2_clone_clone);

    let mut c = Camera::new(width, height, 0.50);
    c.set_antialiasing(false);

    c.calc_pixel_size();
    (w, c)
}

pub fn animate(
    width: usize,
    height: usize,
    mut world: &mut World,
    camera: &mut Camera,
    backend: Box<dyn Backend>,
    frames: i32,
    delta: f32,
    is_3d: bool,
    full_raytracing: bool,
    mut x: f32,
) -> () {
    let mut z: f32 = -2.0;
    let amplitude = 0.8;
    let light_camera_distance_y = 90.0;
    // from the top -> 2D View in -y direction
    let mut camera_from = Tuple4D::new_point(0.0, 10., z);
    let mut camera_to = Tuple4D::new_point(0.0, 0.0, z);
    let mut camera_up = Tuple4D::new_point(0.0, 0.0, 1.0);
    if full_raytracing {
        camera.set_calc_reflection(true);
        camera.set_calc_refraction(true);
    }
    if is_3d {
        // for a 3D like view try this
        camera_from = Tuple4D::new_point(1.6, 1.8, -3.0);
        camera_to = Tuple4D::new_point(0.0, 0.0, 0.0);
        camera_up = Tuple4D::new_point(0.0, 1.0, 0.0);
    }
    let mut light_pos = Tuple4D::from(camera_from);
    light_pos.y += light_camera_distance_y;
    let pl = PointLight::new(light_pos, Color::new(1.5, 1.5, 1.5));
    let l = Light::PointLight(pl);
    world.set_light(l);
    let z_start_left_tilt = 2.0;
    let left_tilt = -30.0 * PI / 180.0;
    let z_start_right_tilt = 4.0;
    let right_tilt = 15.0 * PI / 180.0;
    let mut z_sphere = z;
    for i in 0..frames {
        x = amplitude * z_sphere.sin();
        let x_filename = x;

        if z_sphere > z_start_left_tilt {
            // add x from 30° line
            let x_delta = (z_sphere - z_start_left_tilt) * left_tilt.sin();
            x += x_delta;
        }
        if z_sphere > z_start_right_tilt {
            // add x from 15° line
            let x_delta = (z_sphere - z_start_right_tilt) * right_tilt.sin();
            x -= x_delta;
        }

        let m_scale = Matrix::scale(0.2, 0.2, 0.2);
        let m_trans = Matrix::translation(x, 0.0, z_sphere);
        world.get_shapes_mut()[0].set_transformation(m_trans * m_scale);

        camera.set_transformation(Matrix::view_transform(&camera_from, &camera_to, &camera_up));
        let canvas = backend.render_world(&mut world, &camera);

        let filename = format!(
            "/home/bumzack/stoff/rust/raytracer-challenge/create_street/img/wall_with_2_tilts_{}_{}_frame_{:0>8}_dist_{:.6}.png",
            width, height, i, x_filename
        );
        canvas.unwrap().write_png(&filename).unwrap();
        println!("x = {}, z = {}     filename = {}", x, z, filename);
        println!("camera_from  = {:?}, camera_to = {:?}   ", camera_from, camera_to);

        // update coordinates camera
        camera_from.z += delta;
        camera_to.z += delta;

        // update light pos
        let mut light_pos = Tuple4D::from(camera_from);
        light_pos.y += light_camera_distance_y;
        let pl = PointLight::new(light_pos, Color::new(1.5, 1.5, 1.5));
        let l = Light::PointLight(pl);
        world.set_light(l);
        z -= delta;
        z_sphere += delta;
    }
}
