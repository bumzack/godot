use std::error::Error;
use std::f32::consts::PI;

use raytracer_challenge::basics::camera::{Camera, CameraOps};
use raytracer_challenge::basics::canvas::CanvasOps;
use raytracer_challenge::basics::color::{Color, ColorOps};
use raytracer_challenge::light::light::LightEnum;
use raytracer_challenge::light::pointlight::PointLight;
use raytracer_challenge::material::material::MaterialOps;
use raytracer_challenge::math::matrix::{Matrix, MatrixOps};
use raytracer_challenge::math::tuple4d::{Tuple, Tuple4D};
use raytracer_challenge::shape::plane::{Plane, PlaneOps};
use raytracer_challenge::shape::shape::{Shape, ShapeEnum};
use raytracer_challenge::shape::sphere::{Sphere, SphereOps};
use raytracer_challenge::world::world::{World, WorldOps};

fn main() -> Result<(), Box<dyn Error>> {
    let mut floor = Plane::new();
    floor.get_material_mut().set_color(Color::new(1.0, 0.0, 0.0));
    floor.get_material_mut().set_specular(0.0);

    let mut left_wall = Plane::new();
    left_wall.set_transformation(
        &(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(-PI / 4.0)) * &Matrix::rotate_x(PI / 2.0),
    );
    left_wall.get_material_mut().set_color(Color::new(0.0, 1., 0.0));
    left_wall.get_material_mut().set_specular(0.5);

    let mut right_wall = Plane::new();
    right_wall.set_transformation(
        &(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(PI / 4.0)) * &Matrix::rotate_x(PI / 2.0),
    );
    right_wall.get_material_mut().set_color(Color::new(0.0, 0.0, 1.));
    right_wall.get_material_mut().set_specular(1.0);

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

    let mut w = World::new();
    w.set_light(l);
    w.add_shape(Shape::new(ShapeEnum::Plane(floor), "floor"));
    w.add_shape(Shape::new(ShapeEnum::Plane(left_wall), "left_wall"));
    w.add_shape(Shape::new(ShapeEnum::Plane(right_wall), "right_wall"));
    w.add_shape(Shape::new(ShapeEnum::Sphere(middle), "middle"));
    w.add_shape(Shape::new(ShapeEnum::Sphere(left), "left"));
    w.add_shape(Shape::new(ShapeEnum::Sphere(right), "right"));

    let mut c = Camera::new(120, 100, PI / 3.0);
    c.calc_pixel_size();

    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -5.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
    ));

    let canvas = Camera::render(&c, &w);
    canvas.write_ppm("chapter09.ppm")?;

    println!("DONE");

    Ok(())
}
