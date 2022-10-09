use std::f64::consts::PI;

use crate::prelude::{
    Camera, CameraOps, Color, ColorOps, Light, MaterialOps, Matrix, MatrixOps, PointLight, Shape, ShapeOps, Sphere,
    Tuple, Tuple4D, World, WorldOps,
};

pub fn chapter07(width: usize, height: usize) -> (World, Camera) {
    let mut floor = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    floor.set_transformation(Matrix::scale(10.0, 0.01, 10.0));
    floor.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    floor.get_material_mut().set_specular(0.0);

    let mut left_wall = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    left_wall.set_transformation(
        &(&(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(-PI / 4.0)) * &Matrix::rotate_x(PI / 2.0))
            * &Matrix::scale(10.0, 0.01, 10.),
    );
    left_wall.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    left_wall.get_material_mut().set_specular(0.0);

    let mut right_wall = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    right_wall.set_transformation(
        &(&(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(PI / 4.0)) * &Matrix::rotate_x(PI / 2.0))
            * &Matrix::scale(10.0, 0.01, 10.0),
    );
    right_wall.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    right_wall.get_material_mut().set_specular(0.0);

    let mut middle = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    middle.set_transformation(Matrix::translation(-0.5, 1.0, 0.5));
    middle.get_material_mut().set_color(Color::new(0.1, 1.0, 0.5));
    middle.get_material_mut().set_diffuse(0.7);
    middle.get_material_mut().set_specular(0.3);

    let mut right = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    right.set_transformation(&Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scale(0.5, 0.5, 0.5));
    right.get_material_mut().set_color(Color::new(0.5, 1.0, 0.1));
    right.get_material_mut().set_diffuse(0.7);
    right.get_material_mut().set_specular(0.3);

    let mut left = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    left.set_transformation(&Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scale(0.333, 0.333, 0.333));
    left.get_material_mut().set_color(Color::new(1.0, 0.8, 0.1));
    left.get_material_mut().set_diffuse(0.7);
    left.get_material_mut().set_specular(0.3);

    let pl = PointLight::new(Tuple4D::new_point(-1.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.add_light(l);

    w.add_shape(floor);
    w.add_shape(left_wall);
    w.add_shape(right_wall);
    w.add_shape(middle);
    w.add_shape(left);
    w.add_shape(right);

    let mut c = Camera::new(width, height, PI / 3.0);
    c.calc_pixel_size();

    c.set_from(Tuple4D::new_point(0.0, 1.5, -5.0));
    c.set_to(Tuple4D::new_point(0.0, 1.0, 0.0));
    c.set_up(Tuple4D::new_vector(0.0, 1.0, 0.0));

    (w, c)
}
