extern crate num_cpus;

use std::error::Error;
use std::f64::consts::PI;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 320;
    let height = 200;

    let (mut world, mut camera) = setup_world_coord_axes(width, height, false);
    add_floor(&mut world);
    //  add_borders(&mut world);

    // from the top -> 2D View in -y direction
    let camera_from = Tuple4D::new_point(2.0, 5.0, -2.0);
    let camera_to = Tuple4D::new_point(0.0, 0.0, 0.0);
    let camera_up = Tuple4D::new_vector(0.0, 1.0, 0.0);
    camera.set_transformation(Matrix::view_transform(&camera_from, &camera_to, &camera_up));

    let mut light_pos = Tuple4D::from(camera_from);
    light_pos.x = 2.0;
    light_pos.y = 10.0;
    light_pos.z = -5.0;

    let pl = PointLight::new(light_pos, Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);
    world.add_light(l);

    let filename = format!("test_coord_axes_{}_{}.png", width, height);

    let start = Instant::now();
    let canvas = Camera::render_multi_core(&camera, &world);
    let dur = Instant::now() - start;
    println!("multi core duration: {:?}", dur);
    canvas.write_png(&filename)?;

    Ok(())
}

pub fn add_floor(world: &mut World) {
    // floor
    let mut floor = Cube::new();
    floor.get_material_mut().set_color(Color::new(1.0, 1.0, 1.0));
    floor.get_material_mut().set_ambient(0.3);
    floor.get_material_mut().set_diffuse(0.6);
    floor.get_material_mut().set_specular(0.0);
    floor.get_material_mut().set_reflective(0.1);

    let m_scale = Matrix::scale(3.0, 0.01, 10.0);
    //  let m_scam_transle = Matrix::translation(0.0, -1.5, 0.0);
    floor.set_transformation(m_scale);
    let mut floor = Shape::new(ShapeEnum::Cube(floor));
    floor.set_casts_shadow(false);

    world.add_shape(floor);
}

pub fn add_borders(world: &mut World) {
    let length = 1.0;
    let height = 0.1;
    let thick = 0.01;
    let distance_from_z_axis = 1.0;

    // left border
    let mut left_border = Cube::new();
    left_border.get_material_mut().set_color(Color::new(1.0, 0.0, 1.0));

    let m_trans = Matrix::translation(-distance_from_z_axis, height, length);
    let m_rot = Matrix::new_identity_4x4();
    let m_scale = Matrix::scale(thick, height, length);
    let m = &m_trans * &(m_rot * m_scale);
    left_border.set_transformation(m);
    let mut left_border = Shape::new(ShapeEnum::Cube(left_border));
    left_border.set_casts_shadow(false);

    let mut right_border = Cube::new();
    right_border.get_material_mut().set_color(Color::new(1.0, 1.0, 0.0));

    let m_trans = Matrix::translation(distance_from_z_axis, height, length);
    let m_rot = Matrix::new_identity_4x4();
    let m_scale = Matrix::scale(thick, height, length);
    let m = &m_trans * &(m_rot * m_scale);
    right_border.set_transformation(m);
    let mut right_border = Shape::new(ShapeEnum::Cube(right_border));
    right_border.set_casts_shadow(false);

    world.add_shape(left_border);
    world.add_shape(right_border);
}

pub fn setup_world_coord_axes(width: usize, height: usize, show_axis_shperes: bool) -> (World, Camera) {
    let radius = 0.05;
    let len = 0.8;

    let mut x_axis = Cylinder::new();
    x_axis.set_minimum(0.0);
    x_axis.set_maximum(1.0);
    x_axis.set_closed(true);
    x_axis.get_material_mut().set_color(Color::new(1.0, 0.0, 0.0));
    x_axis.get_material_mut().set_ambient(0.3);
    x_axis.get_material_mut().set_diffuse(0.6);
    x_axis.get_material_mut().set_specular(0.0);
    x_axis.get_material_mut().set_reflective(0.1);

    let m_rot = Matrix::rotate_z(-PI / 2.0);
    let m_trans = Matrix::translation(0.0, 0.0, 0.0);
    let m_scale = Matrix::scale(radius, len, radius);
    let m = &m_trans * &(m_rot * m_scale);
    x_axis.set_transformation(m);
    let mut x_axis = Shape::new(ShapeEnum::Cylinder(x_axis));
    x_axis.set_casts_shadow(false);

    // y axis
    let mut y_axis = Cylinder::new();
    y_axis.set_minimum(0.0);
    y_axis.set_maximum(1.0);
    y_axis.set_closed(true);
    y_axis.get_material_mut().set_color(Color::new(0.0, 1.0, 0.0));
    y_axis.get_material_mut().set_ambient(0.3);
    y_axis.get_material_mut().set_diffuse(0.6);
    y_axis.get_material_mut().set_specular(0.0);
    y_axis.get_material_mut().set_reflective(0.1);

    let m_rot = Matrix::rotate_y(PI / 2.0);
    let m_trans = Matrix::translation(0.0, 0.0, 0.0);
    let m_scale = Matrix::scale(radius, len, radius);
    let m = &m_trans * &(m_rot * m_scale);
    y_axis.set_transformation(m);
    let mut y_axis = Shape::new(ShapeEnum::Cylinder(y_axis));
    y_axis.set_casts_shadow(false);

    // z axis
    let mut z_axis = Cylinder::new();
    z_axis.set_minimum(0.0);
    z_axis.set_maximum(1.0);
    z_axis.set_closed(true);
    z_axis.get_material_mut().set_color(Color::new(0.0, 0.0, 1.0));
    z_axis.get_material_mut().set_ambient(0.3);
    z_axis.get_material_mut().set_diffuse(0.6);
    z_axis.get_material_mut().set_specular(0.0);
    z_axis.get_material_mut().set_reflective(0.1);

    let m_rot = Matrix::rotate_x(PI / 2.0);
    let m_trans = Matrix::translation(0.0, 0.0, 0.0);
    let m_scale = Matrix::scale(radius, len, radius);
    let m = &m_trans * &(m_rot * m_scale);
    z_axis.set_transformation(m);
    let mut z_axis = Shape::new(ShapeEnum::Cylinder(z_axis));
    z_axis.set_casts_shadow(false);

    // sphere to test Z axis
    let mut sphere_z = Sphere::new();
    sphere_z.get_material_mut().set_color(Color::new(0.0, 0.0, 1.0));
    sphere_z.get_material_mut().set_ambient(0.3);

    let m_translate = Matrix::translation(0.0, 0.00, 1.0);
    sphere_z.set_transformation(m_translate);
    let mut sphere_z = Shape::new(ShapeEnum::Sphere(sphere_z));
    sphere_z.set_casts_shadow(false);

    // sphere to test y axis
    let mut sphere_y = Sphere::new();
    sphere_y.get_material_mut().set_color(Color::new(0.0, 1.0, 0.0));

    let m_translate = Matrix::translation(0.0, 1.00, 0.0);
    sphere_y.set_transformation(m_translate);
    let mut sphere_y = Shape::new(ShapeEnum::Sphere(sphere_y));
    sphere_y.set_casts_shadow(false);

    // sphere to test y axis
    let mut sphere_x = Sphere::new();
    sphere_x.get_material_mut().set_color(Color::new(1.0, 0.0, 0.0));

    let m_translate = Matrix::translation(1.0, 0.00, 0.0);
    sphere_x.set_transformation(m_translate);
    let mut sphere_x = Shape::new(ShapeEnum::Sphere(sphere_x));
    sphere_x.set_casts_shadow(false);

    let mut w = World::new();
    w.add_shape(x_axis);
    w.add_shape(y_axis);
    w.add_shape(z_axis);

    if show_axis_shperes {
        w.add_shape(sphere_x);
        w.add_shape(sphere_y);
        w.add_shape(sphere_z);
    }

    let mut c = Camera::new(width, height, 0.6);
    c.set_antialiasing(false);

    c.calc_pixel_size();
    (w, c)
}
