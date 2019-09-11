#![feature(stmt_expr_attributes)]

use std::time::Instant;

use raytracer::Camera;
use raytracer::CameraOps;
use raytracer::Color;
use raytracer::ColorOps;
use raytracer::Cube;
use raytracer::Light;
use raytracer::MaterialOps;
use raytracer::Matrix;
use raytracer::MatrixOps;
use raytracer::Plane;
use raytracer::PointLight;
use raytracer::Shape;
use raytracer::ShapeEnum;
use raytracer::ShapeOps;
use raytracer::Sphere;
use raytracer::Tuple;
use raytracer::Tuple4D;
use raytracer_lib_std::World;
use raytracer_lib_std::WorldOps;

pub fn setup_world_shadow_glamour(size_factor: f32, antialiasing: bool, antialiasing_size: usize) -> (World, Camera) {
    let width = (400 as f32 * size_factor) as usize;
    let height = (160 as f32 * size_factor) as usize;

    let pl = PointLight::new(Tuple4D::new_point(-1.0, 2.0, 4.0), Color::new(1.5, 1.5, 1.5));
    let l = Light::PointLight(pl);

    // ---- CUBE -------
    let mut c = Cube::new();
    c.get_material_mut().set_color(Color::new(1., 0.5, 0.2));
    c.get_material_mut().set_ambient(1.0);
    c.get_material_mut().set_diffuse(0.0);
    c.get_material_mut().set_specular(0.0);

    let m_trans = Matrix::translation(0.0, 3.0, 4.0);
    let m_scale = Matrix::scale(1.0, 1.0, 0.01);
    let m = &m_trans * &m_scale;

    c.set_transformation(m);
    let mut cube = Shape::new(ShapeEnum::Cube(c));
    cube.set_casts_shadow(false);

    // ---- PLANE -------
    let mut plane = Plane::new();
    plane.get_material_mut().set_color(Color::new(1., 1., 1.));
    plane.get_material_mut().set_ambient(0.025);
    plane.get_material_mut().set_diffuse(0.67);
    plane.get_material_mut().set_specular(0.0);

    let plane = Shape::new(ShapeEnum::Plane(plane));

    // ---- SPHERE 1 -------
    let mut sphere1 = Sphere::new();
    sphere1.get_material_mut().set_color(Color::new(1.0, 0., 0.));
    sphere1.get_material_mut().set_ambient(0.1);
    sphere1.get_material_mut().set_diffuse(0.6);
    sphere1.get_material_mut().set_specular(0.0);
    sphere1.get_material_mut().set_reflective(0.3);

    let m_trans = Matrix::translation(0.5, 0.5, 0.0);
    let m_scale = Matrix::scale(0.5, 0.5, 0.5);
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

    let m_trans = Matrix::translation(-0.25, 0.33, 0.0);
    let m_scale = Matrix::scale(0.33, 0.33, 0.33);
    let m = &m_trans * &m_scale;

    sphere2.set_transformation(m);
    let sphere2 = Shape::new(ShapeEnum::Sphere(sphere2));

    let mut w = World::new();
    w.set_light(l);
    w.add_shape(cube);
    w.add_shape(plane);
    w.add_shape(sphere1);
    w.add_shape(sphere2);

    let mut c = Camera::new(width, height, 0.78540);
    c.set_antialiasing(antialiasing);
    c.set_antialiasing_size(antialiasing_size);

    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(-3.0, 1., 2.5),
        &Tuple4D::new_point(0.0, 0.5, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
