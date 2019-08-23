use std::error::Error;
use std::f32::consts::PI;
use std::time::Instant;

use raytracer_challenge_reference_impl::basics::camera::{Camera, CameraOps};
use raytracer_challenge_reference_impl::basics::canvas::CanvasOps;
use raytracer_challenge_reference_impl::basics::color::{Color, ColorOps};
use raytracer_challenge_reference_impl::light::light::LightEnum;
use raytracer_challenge_reference_impl::light::pointlight::PointLight;
use raytracer_challenge_reference_impl::material::material::MaterialOps;
use raytracer_challenge_reference_impl::math::matrix::{Matrix, MatrixOps};
use raytracer_challenge_reference_impl::math::tuple4d::{Tuple, Tuple4D};
use raytracer_challenge_reference_impl::patterns::checker3d_patterns::Checker3DPattern;
use raytracer_challenge_reference_impl::patterns::gradient_patterns::GradientPattern;
use raytracer_challenge_reference_impl::patterns::patterns::Pattern;
use raytracer_challenge_reference_impl::patterns::ring_patterns::RingPattern;
use raytracer_challenge_reference_impl::shape::cube::{Cube, CubeOps};
use raytracer_challenge_reference_impl::shape::plane::{Plane, PlaneOps};
use raytracer_challenge_reference_impl::shape::shape::{Shape, ShapeEnum};
use raytracer_challenge_reference_impl::shape::sphere::{Sphere, SphereOps};
use raytracer_challenge_reference_impl::world::world::{World, WorldOps};

fn main() -> Result<(), Box<dyn Error>> {
    let (w, c) = setup_world(320, 200);

    // single core
    let start = Instant::now();
    // let canvas = Camera::render_debug(&c, &w, 226, 241);
    let canvas = Camera::render(&c, &w);
    canvas.write_ppm("chapter12.ppm")?;
    let dur = Instant::now() - start;
    println!("single core duration: {:?}", dur);

    // multi core
    //    let start = Instant::now();
    //    let canvas = Camera::render_multi_core(&c, &w, 4);
    //    canvas.write_ppm("chapter12_multi_core.ppm")?;
    //    let dur = Instant::now() - start;
    //    println!("multi core duration: {:?}", dur);

    Ok(())
}

fn setup_world<'a>(width: usize, height: usize) -> (World<'a>, Camera) {
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
    middle.get_material_mut().set_reflective(1.3);
    middle.get_material_mut().set_refractive_index(1.3);
    let mut right = Sphere::new();
    right.set_transformation(&Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scale(0.5, 0.5, 0.5));
    right.get_material_mut().set_color(Color::new(0.5, 1.0, 0.1));
    right.get_material_mut().set_diffuse(0.7);
    right.get_material_mut().set_specular(0.3);
    middle.get_material_mut().set_reflective(1.8);
    middle.get_material_mut().set_refractive_index(1.8);
    let mut left = Sphere::new();
    left.set_transformation(&Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scale(0.333, 0.333, 0.333));
    left.get_material_mut().set_color(Color::new(1., 0., 0.));
    left.get_material_mut().set_diffuse(0.7);
    left.get_material_mut().set_specular(0.3);
    let mut checker_3d = Checker3DPattern::new();
    checker_3d.set_color_a(Color::new(1.0, 0.0, 1.0));
    checker_3d.set_color_a(Color::new(0.1, 0.1, 1.0));
    let p = Pattern::Checker3DPattern(checker_3d);
    let mut cube = Cube::new();
    let c_trans = Matrix::translation(-2.5, 0.33, -0.75);
    let c_scale = Matrix::scale(2.0, 0.5, 0.25);
    cube.set_transformation(c_scale * c_trans);
    cube.get_material_mut().set_pattern(p);
    cube.get_material_mut().set_transparency(1.5);
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
    w.add_shape(Shape::new(ShapeEnum::Cube(cube), "cube"));

    let mut c = Camera::new(width, height, PI / 3.0);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -5.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
    ));
    (w, c)
}
