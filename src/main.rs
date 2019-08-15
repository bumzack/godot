use std::error::Error;
use std::f64::consts::{PI, SQRT_2};

use crate::basics::camera::{Camera, CameraOps};
use crate::basics::canvas::CanvasOps;
use crate::basics::color::Color;
use crate::basics::color::ColorOps;
use crate::basics::ray::RayOps;
use crate::light::light::Light;
use crate::light::pointlight::PointLight;
use crate::material::material::MaterialOps;
use crate::math::common::{assert_color, assert_tuple};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;
use crate::shape::shape::Shape;
use crate::shape::sphere::{Sphere, SphereOps};
use crate::world::world::{default_world, World, WorldOps};

mod basics;
mod light;
mod material;
mod math;
mod shape;
mod world;

fn main2() -> Result<(), Box<dyn Error>> {
    let mut w = default_world();

    let shapes = w.get_shapes();
    let outer_shape = shapes.get(0).unwrap();
    let inner_shape = shapes.get(1).unwrap();

    println!(
        "outer_shape.get_tranformation             {:#?}",
        outer_shape.get_transformation()
    );
    println!(
        "outer_shape.get_inverse_transformation    {:#?}",
        outer_shape.get_inverse_transformation()
    );
    println!(
        "inner_shape.get_tranformation             {:#?}",
        inner_shape.get_transformation()
    );
    println!(
        "inner_shape.get_inverse_transformation    {:#?}",
        inner_shape.get_inverse_transformation()
    );

    let mut c = Camera::new(11, 11, PI / 2.0);
    c.calc_pixel_size();

    let from = Tuple4D::new_point(0.0, 0.0, -5.0);
    let to = Tuple4D::new_point(0.0, 0.0, 0.0);
    let up = Tuple4D::new_vector(0.0, 1.0, 0.0);

    c.set_transformation(Matrix::view_transform(&from, &to, &up));

    let image = Camera::render(&c, &w);
    // println!("image = {:#?}", image);

    let color = image.pixel_at(5, 5);
    let c_expected = Color::new(0.38066, 0.47583, 0.2855);

    println!("color = {:#?}", color);
    println!("c_expected = {:#?}", c_expected);
    assert_color(color, &c_expected);

    Ok(())
}

fn main_wall_ball() -> Result<(), Box<dyn Error>> {
    let mut floor = Sphere::new();
    floor.set_transformation(Matrix::scale(10.0, 0.01, 10.0));
    floor.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    floor.get_material_mut().set_specular(0.0);

    let mut left_wall = Sphere::new();
    left_wall.set_transformation(
        &(&(&(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::scale(10.0, 0.01, 10.0)) * &Matrix::rotate_y(-PI / 4.0))
            * &Matrix::rotate_x(PI / 2.0))
            * &Matrix::scale(10.0, 0.1, 10.),
    );
    left_wall.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    left_wall.get_material_mut().set_specular(0.0);

    let mut right_wall = Sphere::new();
    right_wall.set_transformation(
        &(&(&(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::scale(10.0, 0.01, 10.0)) * &Matrix::rotate_y(PI / 4.0))
            * &Matrix::rotate_x(PI / 2.0))
            * &Matrix::scale(10.0, 0.1, 10.),
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

    let pl = PointLight::new(Tuple4D::new_point(-1.0, 10.0, 10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.set_light(l);
    w.add_shape(Shape::Sphere(floor));
    w.add_shape(Shape::Sphere(left_wall));
    w.add_shape(Shape::Sphere(right_wall));
    w.add_shape(Shape::Sphere(middle));
    w.add_shape(Shape::Sphere(left));
    w.add_shape(Shape::Sphere(right));

    let mut c = Camera::new(100, 100, PI / 3.0);
    c.calc_pixel_size();

    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -5.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
    ));

    let canvas = Camera::render(&c, &w);
    canvas.write_ppm("wusch.ppm")?;

    println!("DONE");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut c = Camera::new(201, 101, PI / 2.0);
    c.calc_pixel_size();

    c.set_transformation(Matrix::rotate_y(PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0));
    let r = Camera::ray_for_pixel(&c, 100, 50);

    println!("&r.get_origin() = {:#?}", &r.get_origin());
    println!("&r.get_direction()= {:#?}", &r.get_direction());

    assert_tuple(&r.get_origin(), &Tuple4D::new_point(0.0, 2.0, -5.0));
    assert_tuple(
        &r.get_direction(),
        &Tuple4D::new_vector(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0),
    );

    Ok(())
}
