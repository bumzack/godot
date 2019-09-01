use std::f32::consts::PI;

use raytracer::Camera;
use raytracer::CameraOps;
use raytracer::Color;
use raytracer::ColorOps;
use raytracer::Light;
use raytracer::Material;
use raytracer::MaterialOps;
use raytracer::Matrix;
use raytracer::MatrixOps;
use raytracer::PointLight;
use raytracer::Shape;
use raytracer::ShapeEnum;
use raytracer::Sphere;
use raytracer::SphereOps;
use raytracer::Tuple;
use raytracer::Tuple4D;
use raytracer::World;
use raytracer::WorldOps;

pub fn setup_world(w: usize, h: usize) -> (World, Camera) {
    let mut floor = Sphere::new();
    floor.set_transformation(Matrix::scale(10.0, 0.01, 10.0));
    floor
        .get_material_mut()
        .set_color(Color::new(1.0, 0.9, 0.9));
    floor.get_material_mut().set_specular(0.0);

    let mut left_wall = Sphere::new();
    left_wall.set_transformation(
        &(&(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(-PI / 4.0))
            * &Matrix::rotate_x(PI / 2.0))
            * &Matrix::scale(10.0, 0.01, 10.),
    );
    left_wall
        .get_material_mut()
        .set_color(Color::new(1.0, 0.9, 0.9));
    left_wall.get_material_mut().set_specular(0.0);

    let mut right_wall = Sphere::new();
    right_wall.set_transformation(
        &(&(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(PI / 4.0))
            * &Matrix::rotate_x(PI / 2.0))
            * &Matrix::scale(10.0, 0.01, 10.0),
    );
    right_wall
        .get_material_mut()
        .set_color(Color::new(1.0, 0.9, 0.9));
    right_wall.get_material_mut().set_specular(0.0);

    let mut middle = Sphere::new();
    middle.set_transformation(Matrix::translation(-0.5, 1.0, 0.5));
    middle
        .get_material_mut()
        .set_color(Color::new(0.1, 1.0, 0.5));
    middle.get_material_mut().set_diffuse(0.7);
    middle.get_material_mut().set_specular(0.3);

    let mut right = Sphere::new();
    right.set_transformation(&Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scale(0.5, 0.5, 0.5));
    right
        .get_material_mut()
        .set_color(Color::new(0.5, 1.0, 0.1));
    right.get_material_mut().set_diffuse(0.7);
    right.get_material_mut().set_specular(0.3);

    let mut left = Sphere::new();
    left.set_transformation(
        &Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scale(0.333, 0.333, 0.333),
    );
    left.get_material_mut().set_color(Color::new(1.0, 0.8, 0.1));
    left.get_material_mut().set_diffuse(0.7);
    left.get_material_mut().set_specular(0.3);

    let pl = PointLight::new(
        Tuple4D::new_point(-1.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    );
    let l = Light::PointLight(pl);

    let mut world = World::new();
    world.set_light(l);
    world.add_shape(Shape::new(ShapeEnum::Sphere(floor)));
    world.add_shape(Shape::new(ShapeEnum::Sphere(left_wall)));
    world.add_shape(Shape::new(ShapeEnum::Sphere(right_wall)));
    world.add_shape(Shape::new(ShapeEnum::Sphere(middle)));
    world.add_shape(Shape::new(ShapeEnum::Sphere(left)));
    world.add_shape(Shape::new(ShapeEnum::Sphere(right)));

    let mut c = Camera::new(w, h, PI / 3.0);
    c.calc_pixel_size();
    c.set_antialiasing(true);
    c.set_antialiasing_size(3);

    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -5.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
    ));

    (world, c)
}
