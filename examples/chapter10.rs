use std::error::Error;
use std::f64::consts::PI;
use std::time::Instant;

use raytracer_challenge::basics::camera::{Camera, CameraOps};
use raytracer_challenge::basics::canvas::CanvasOps;
use raytracer_challenge::basics::color::{Color, ColorOps};
use raytracer_challenge::light::light::Light;
use raytracer_challenge::light::pointlight::PointLight;
use raytracer_challenge::material::material::MaterialOps;
use raytracer_challenge::math::matrix::{Matrix, MatrixOps};
use raytracer_challenge::math::tuple4d::{Tuple, Tuple4D};
use raytracer_challenge::patterns::checker3d_patterns::Checker3DPattern;
use raytracer_challenge::patterns::gradient_patterns::GradientPattern;
use raytracer_challenge::patterns::patterns::Pattern;
use raytracer_challenge::patterns::ring_patterns::RingPattern;
use raytracer_challenge::shape::plane::{Plane, PlaneOps};
use raytracer_challenge::shape::shape::Shape;
use raytracer_challenge::shape::sphere::{Sphere, SphereOps};
use raytracer_challenge::world::world::{World, WorldOps};

fn main() -> Result<(), Box<dyn Error>> {
    let mut floor = Plane::new();
    let mut p: GradientPattern = GradientPattern::new();
    p.set_color_a(Color::new(1.0, 0.0, 0.0));
    p.set_color_a(Color::new(1.0, 0.0, 1.0));
    let m = Matrix::rotate_y(PI / 4.0);
    p.set_transformation(m);
    let p = Pattern::GradientPattern(p);
    floor.get_material_mut().set_pattern(p);

    let mut p = RingPattern::new();
    p.set_color_a(Color::new(0.5, 0.0, 0.0));
    p.set_color_a(Color::new(0.5, 0.0, 0.8));
    let m = Matrix::rotate_x(PI / 4.0);
    p.set_transformation(m);
    let p = Pattern::RingPattern(p);

    let mut left_wall = Plane::new();
    left_wall.set_transformation(
        &(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(-PI / 4.0)) * &Matrix::rotate_x(PI / 2.0),
    );
    left_wall.get_material_mut().set_pattern(p);

    let mut checker_3d = Checker3DPattern::new();
    checker_3d.set_color_a(Color::new(0.1, 0.8, 0.4));
    checker_3d.set_color_a(Color::new(0.8, 0.2, 0.2));
    let p = Pattern::Checker3DPattern(checker_3d);

    let mut right_wall = Plane::new();
    right_wall.set_transformation(
        &(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(PI / 4.0)) * &Matrix::rotate_x(PI / 2.0),
    );
    right_wall.get_material_mut().set_pattern(p);

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
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.set_light(l);
    w.add_shape(Shape::Plane(floor));
    w.add_shape(Shape::Plane(left_wall));
    w.add_shape(Shape::Plane(right_wall));
    w.add_shape(Shape::Sphere(middle));
    w.add_shape(Shape::Sphere(left));
    w.add_shape(Shape::Sphere(right));

    let mut c = Camera::new(120, 100, PI / 3.0);
    c.calc_pixel_size();

    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -5.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
    ));

    let start = Instant::now();
    let canvas = Camera::render(&c, &w);
    canvas.write_ppm("chapter10.ppm")?;
    let dur = Instant::now() - start;

    println!("DONE in {:?}", dur);

    Ok(())
}