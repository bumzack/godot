#![feature(stmt_expr_attributes)]

extern crate num_cpus;

use std::error::Error;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use raytracer_challenge::basics::camera::{Camera, CameraOps};
use raytracer_challenge::basics::canvas::{Canvas, CanvasOps};
use raytracer_challenge::basics::color::{BLACK, Color, ColorOps};
use raytracer_challenge::light::light::LightEnum;
use raytracer_challenge::light::pointlight::PointLight;
use raytracer_challenge::material::material::MaterialOps;
use raytracer_challenge::math::matrix::{Matrix, MatrixOps};
use raytracer_challenge::math::tuple4d::{Tuple, Tuple4D};
use raytracer_challenge::patterns::patterns::Pattern;
use raytracer_challenge::patterns::stripe_patterns::StripePattern;
use raytracer_challenge::shape::plane::Plane;
use raytracer_challenge::shape::shape::{Shape, ShapeEnum};
use raytracer_challenge::world::world::{World, WorldOps, MAX_REFLECTION_RECURSION_DEPTH};
use raytracer_challenge::shape::sphere::{Sphere, SphereOps};
use raytracer_challenge::patterns::checker3d_patterns::Checker3DPattern;
use raytracer_challenge::shape::cube::{Cube, CubeOps};
use raytracer_challenge::basics::ray::RayOps;


fn main() -> Result<(), Box<dyn Error>> {
    let width = 800;
    let height = 600;
    let filename = "compare_to_cuda.ppm";

    let (world, camera) = setup_world(width, height);

    let start = Instant::now();
    let num_cores = num_cpus::get();

    println!("using {} cores", num_cores);

    let mut canvas = Canvas::new(camera.get_hsize(), camera.get_vsize());

    println!("camera height / width  {}/{}     thread_id {:?}", height, width, thread::current().id());

    let x = 300;
    let y = 240;


    let mut color = BLACK;

    let r = Camera::ray_for_pixel(&camera, x, y);
    color = World::color_at(&world, &r, MAX_REFLECTION_RECURSION_DEPTH);
    println!("ray at ( {} / {} )   origin = {:?},    direction = {:?}", x, y, r.get_origin(), r.get_direction());
    println!("color  at ( {} / {} )   c = {:?},     ", x, y, color);

    canvas.write_pixel(x, y, color);

    let dur = Instant::now() - start;
    println!("multi core duration: {:?}  ", dur, );


    Ok(())
}


fn setup_world<'a>(w: usize, h: usize) -> (World<'a>, Camera) {
    let mut floor = Sphere::new();
    floor.set_transformation(Matrix::scale(10.0, 0.01, 10.0));
    floor.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    floor.get_material_mut().set_specular(0.0);

    let mut left_wall = Sphere::new();
    left_wall.set_transformation(
        &(&(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(-PI / 4.0)) * &Matrix::rotate_x(PI / 2.0))
            * &Matrix::scale(10.0, 0.01, 10.),
    );
    left_wall.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    left_wall.get_material_mut().set_specular(0.0);

    let mut right_wall = Sphere::new();
    right_wall.set_transformation(
        &(&(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(PI / 4.0)) * &Matrix::rotate_x(PI / 2.0))
            * &Matrix::scale(10.0, 0.01, 10.0),
    );
    right_wall.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    right_wall.get_material_mut().set_specular(0.0);

    let mut middle = Sphere::new();
    middle.set_transformation(Matrix::translation(-0.5, 1.0, 0.5));
    middle.get_material_mut().set_color(Color::new(0.1, 1.0, 0.5));
    middle.get_material_mut().set_diffuse(0.7);
    middle.get_material_mut().set_specular(0.3);

    let mut right = Sphere::new();
    right.set_transformation(&Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scale(0.5, 0.5, 0.5));
    right.get_material_mut().set_color(Color::new(0.5, 1.0, 0.1));
    right.get_material_mut().set_diffuse(0.7);
    right.get_material_mut().set_specular(0.3);

    let mut left = Sphere::new();
    left.set_transformation(&Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scale(0.333, 0.333, 0.333));
    left.get_material_mut().set_color(Color::new(1.0, 0.8, 0.1));
    left.get_material_mut().set_diffuse(0.7);
    left.get_material_mut().set_specular(0.3);

    let pl = PointLight::new(Tuple4D::new_point(-1.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = LightEnum::PointLight(pl);

    let mut world = World::new();
    world.set_light(l);
    world.add_shape(Shape::new(ShapeEnum::Sphere(floor), "floor"));
    world.add_shape(Shape::new(ShapeEnum::Sphere(left_wall), "left_wall"));
    world.add_shape(Shape::new(ShapeEnum::Sphere(right_wall), "v"));
    world.add_shape(Shape::new(ShapeEnum::Sphere(middle), "middle"));
    world.add_shape(Shape::new(ShapeEnum::Sphere(left), "left"));
    world.add_shape(Shape::new(ShapeEnum::Sphere(right), "right"));

    let mut c = Camera::new(w, h, PI / 3.0);
    c.calc_pixel_size();

    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -5.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
    ));

    (world, c)
}

