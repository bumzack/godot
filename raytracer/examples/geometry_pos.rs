use std::f32::consts::PI;

use raytracer::prelude::*;

fn main() {
    let (backend, mut world, mut camera) = setup_world();

    let p1 = Tuple4D::new_point(-2.0, 0.0, -2.0);
    let p2 = Tuple4D::new_point(2.0, 0.0, -2.0);
    let p3 = Tuple4D::new_point(2.0, 0.0, 2.0);
    let p4 = Tuple4D::new_point(-2.0, 0.0, 2.0);
    let p5 = Tuple4D::new_point(-2.0, 0.0, -2.0);

    let p6 = Tuple4D::new_point(-2.0, 2.0, -2.0);
    let p7 = Tuple4D::new_point(2.0, 2.0, -2.0);
    let p8 = Tuple4D::new_point(2.0, 2.0, 2.0);
    let p9 = Tuple4D::new_point(-2.0, 2.0, 2.0);
    let p10 = Tuple4D::new_point(-2.0, 2.0, -2.0);

    let points = vec![&p1, &p2, &p3, &p4, &p5, &p6, &p7, &p8, &p9, &p10];

    //  create_spheres(&mut world, &points);

    add_floor(&mut world);
    add_cylinder(&mut world, &points);

    render_and_save_world(&backend, &mut world, &camera, "geom_pos1.png");

    render_multiple_scene(&backend, &mut world, &mut camera);
}

fn add_cylinder(world: &mut World, points: &Vec<&Tuple4D>) {
    let radius = 0.05;

    for i in 0..points.len() - 1 {
        let c = cylinder_between_two_points(points[i], points[i + 1], radius);
        let mut c = Shape::new(ShapeEnum::Cylinder(c));
        c.set_casts_shadow(false);
        world.add_shape(c);
    }
}

fn render_multiple_scene(backend: &BackendCpu, mut world: &mut World, camera: &mut Camera) {
    let camera_from = Tuple4D::new_point(2.5, 3.0, -3.0);
    let camera_to = Tuple4D::new_point(1.0, 0.0, -1.0);
    let camera_up = Tuple4D::new_vector(0.0, 1.0, 0.0);
    camera.set_transformation(Matrix::view_transform(&camera_from, &camera_to, &camera_up));
    render_and_save_world(&backend, &mut world, &camera, "geom_pos2.png");

    let camera_from = Tuple4D::new_point(2.5, 3.0, 3.0);
    let camera_to = Tuple4D::new_point(1.0, 0.0, 1.0);
    let camera_up = Tuple4D::new_vector(0.0, 0.5, 0.0);
    camera.set_transformation(Matrix::view_transform(&camera_from, &camera_to, &camera_up));
    render_and_save_world(&backend, &mut world, &camera, "geom_pos3.png");
}

fn create_spheres(world: &mut World, points: &Vec<&Tuple4D>) {
    let scale_factor = 0.05;
    let m_scale = Matrix::scale(scale_factor, scale_factor, scale_factor);

    points.iter().for_each(|p| {
        let m_trans = Matrix::translation(p.x, p.y, p.z);
        let m = &m_trans * &m_scale;

        let mut s = Sphere::new();

        s.set_transformation(m);
        s.get_material_mut().set_color(Color::new(1.0, 0.0, 0.0));
        let s = Shape::new(ShapeEnum::Sphere(s));
        world.add_shape(s);
    });
}

fn render_and_save_world(backend: &BackendCpu, world: &mut World, camera: &Camera, filename: &str) {
    let canvas = backend.render_world_multi_core(world, &camera);
    let filename = format!(
        "/Users/bumzack/stoff/rust/raytracer-challenge/raytracer/examples/{}",
        filename
    );
    canvas.unwrap().write_png(&filename).unwrap();
}

fn setup_world() -> (BackendCpu, World, Camera) {
    let width = 1280;
    let height = 720;
    let backend = BackendCpu::new();

    let (mut world, mut camera) = setup_world_coord_axes(width, height, false);
    // add_floor(&mut world);

    let camera_from = Tuple4D::new_point(1.9, 3.0, -6.0);
    let camera_to = Tuple4D::new_point(0.0, 0.0, 0.0);
    let camera_up = Tuple4D::new_vector(0.0, 1.0, 0.0);
    camera.set_transformation(Matrix::view_transform(&camera_from, &camera_to, &camera_up));

    let light_pos = Tuple4D::new_point(2.0, 5.0, -2.0);
    let pl = PointLight::new(light_pos, Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);
    world.set_light(l);

    (backend, world, camera)
}

pub fn add_floor(world: &mut World) {
    // floor
    let mut floor = Cube::new();
    floor.get_material_mut().set_color(Color::new(0.0, 0.3, 0.0));
    floor.get_material_mut().set_ambient(0.3);
    floor.get_material_mut().set_diffuse(0.6);
    floor.get_material_mut().set_specular(0.0);
    floor.get_material_mut().set_reflective(0.1);

    let m_scale = Matrix::scale(3.0, 0.01, 10.0);
    let m_trans = Matrix::translation(0.0, 0.0, 0.0);
    floor.set_transformation(m_trans * m_scale);
    let mut floor = Shape::new(ShapeEnum::Cube(floor));
    floor.set_casts_shadow(false);

    world.add_shape(floor);
}

pub fn setup_world_coord_axes(width: usize, height: usize, show_axis_shperes: bool) -> (World, Camera) {
    let radius = 0.05;
    let len = 0.5;

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

    let sphere_scale = Matrix::scale(radius, radius, radius);
    let m_translate = Matrix::translation(0.0, 0.00, 1.0);
    sphere_z.set_transformation(&sphere_scale * &m_translate);
    let mut sphere_z = Shape::new(ShapeEnum::Sphere(sphere_z));
    sphere_z.set_casts_shadow(false);

    // sphere to test y axis
    let mut sphere_y = Sphere::new();
    sphere_y.get_material_mut().set_color(Color::new(0.0, 1.0, 0.0));

    let m_translate = Matrix::translation(0.0, 1.00, 0.0);
    sphere_y.set_transformation(&sphere_scale * &m_translate);
    let mut sphere_y = Shape::new(ShapeEnum::Sphere(sphere_y));
    sphere_y.set_casts_shadow(false);

    // sphere to test y axis
    let mut sphere_x = Sphere::new();
    sphere_x.get_material_mut().set_color(Color::new(1.0, 0.0, 0.0));

    let m_translate = Matrix::translation(1.0, 0.00, 0.0);
    sphere_x.set_transformation(&sphere_scale * &m_translate);
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

    let mut c = Camera::new(width, height, 1.0);
    c.set_antialiasing(false);
    c.set_calc_reflection(false);
    c.set_calc_refraction(false);
    c.calc_pixel_size();
    (w, c)
}
