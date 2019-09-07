#![feature(stmt_expr_attributes)]

use std::f32::consts::PI;
use std::time::Instant;

use raytracer::AreaLight;
use raytracer::Camera;
use raytracer::CameraOps;
use raytracer::Checker3DPattern;
use raytracer::Color;
use raytracer::ColorOps;
use raytracer::Cube;
use raytracer::Cylinder;
use raytracer::GradientPattern;
use raytracer::Light;
use raytracer::MaterialOps;
use raytracer::Matrix;
use raytracer::MatrixOps;
use raytracer::Pattern;
use raytracer::Plane;
use raytracer::PointLight;
use raytracer::RingPattern;
use raytracer::Shape;
use raytracer::ShapeEnum;
use raytracer::ShapeOps;
use raytracer::Sphere;
use raytracer::Tuple;
use raytracer::Tuple4D;
use raytracer_lib_std::World;
use raytracer_lib_std::WorldOps;

pub(crate) fn setup_world_chapter14_with_aa(width: usize, height: usize) -> (World, Camera) {
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
    let c_trans = Matrix::translation(-2.0, 2.0, -1.75);
    let c_scale = Matrix::scale(0.5, 0.5, 0.25);
    cube.set_transformation(c_scale * c_trans);
    cube.get_material_mut().set_pattern(p);
    cube.get_material_mut().set_transparency(1.5);

    let mut checker = Checker3DPattern::new();
    checker.set_color_a(Color::new(1.0, 0.0, 0.0));
    checker.set_color_b(Color::new(0.4, 0.0, 0.0));
    let p = Pattern::Checker3DPattern(checker);

    let mut cylinder = Cylinder::new();
    let c_trans = Matrix::translation(-3.5, 1.0, -0.75);
    // let c_scale = Matrix::scale(2.0, 0.5, 0.25);
    cylinder.set_transformation(c_trans);
    cylinder.get_material_mut().set_pattern(p);
    cylinder.get_material_mut().set_transparency(1.5);
    cylinder.set_minimum(0.0);
    cylinder.set_minimum(1.0);

    let pl = PointLight::new(Tuple4D::new_point(-1.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.set_light(l);
    w.add_shape(Shape::new(ShapeEnum::Plane(floor)));
    w.add_shape(Shape::new(ShapeEnum::Plane(left_wall)));
    w.add_shape(Shape::new(ShapeEnum::Plane(right_wall)));
    w.add_shape(Shape::new(ShapeEnum::Sphere(middle)));
    w.add_shape(Shape::new(ShapeEnum::Sphere(left)));
    w.add_shape(Shape::new(ShapeEnum::Sphere(right)));
    w.add_shape(Shape::new(ShapeEnum::Cube(cube)));
    // w.add_shape(Shape::new(ShapeEnum::Cylinder(cylinder)));

    let mut c = Camera::new(width, height, PI / 4.0);
    c.set_antialiasing(true);
    c.set_antialiasing_size(3);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -6.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
    ));
    (w, c)
}