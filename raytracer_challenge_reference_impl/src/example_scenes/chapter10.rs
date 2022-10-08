use std::error::Error;
use std::f64::consts::PI;

use crate::prelude::{
    Camera, CameraOps, CanvasOpsStd, Checker3DPattern, Color, ColorOps, GradientPattern, Light, MaterialOps, Matrix,
    MatrixOps, Pattern, PatternEnum, Plane, PointLight, RingPattern, Shape, ShapeOps, Sphere, Tuple, Tuple4D, World,
    WorldOps,
};

pub fn chapter10(width: usize, height: usize) -> (World, Camera) {
    let mut floor = Shape::new_plane(Plane::new(), "plane".to_string());
    let mut gradient_pattern = GradientPattern::new();
    gradient_pattern.set_color_a(Color::new(1.0, 0.0, 0.0));
    gradient_pattern.set_color_a(Color::new(1.0, 0.0, 1.0));
    let mut p = Pattern::new(PatternEnum::GradientPatternEnum(gradient_pattern));
    let m = Matrix::rotate_y(PI / 4.0);
    p.set_transformation(m);
    floor.get_material_mut().set_pattern(p);

    let mut ring_pattern = RingPattern::new();
    ring_pattern.set_color_a(Color::new(0.5, 0.0, 0.0));
    ring_pattern.set_color_a(Color::new(0.5, 0.0, 0.8));
    let mut p = Pattern::new(PatternEnum::RingPatternEnum(ring_pattern));
    let m = Matrix::rotate_x(PI / 4.0);
    p.set_transformation(m);

    let mut left_wall = Shape::new_plane(Plane::new(), "plane".to_string());
    left_wall.set_transformation(
        &(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(-PI / 4.0)) * &Matrix::rotate_x(PI / 2.0),
    );
    left_wall.get_material_mut().set_pattern(p);

    let mut checker3dpattern = Checker3DPattern::new();
    checker3dpattern.set_color_a(Color::new(0.1, 0.8, 0.4));
    checker3dpattern.set_color_a(Color::new(0.8, 0.2, 0.2));
    let checker_3d = Pattern::new(PatternEnum::Checker3DPatternEnum(checker3dpattern));

    let mut right_wall = Shape::new_plane(Plane::new(), "plane".to_string());
    right_wall.set_transformation(
        &(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(PI / 4.0)) * &Matrix::rotate_x(PI / 2.0),
    );
    right_wall.get_material_mut().set_pattern(checker_3d);

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

    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -5.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));

    (w, c)
}
