use crate::prelude::{
    Camera, CameraOps, Checker3DPattern, Color, ColorOps, Light, Material, MaterialOps, Matrix, MatrixOps, Pattern,
    PatternEnum, Plane, PointLight, Shape, ShapeOps, Sphere, Tuple, Tuple4D, World, WorldOps,
};
use std::error::Error;

pub fn chapter11_air_bubble(width: usize, height: usize) -> (World, Camera) {
    let mut checker_pattern = Checker3DPattern::new();
    checker_pattern.set_color_a(Color::new(0.15, 0.15, 0.15));
    checker_pattern.set_color_b(Color::new(0.85, 0.85, 0.85));

    let mut wall_material = Material::new();
    wall_material.set_pattern(Pattern::new(PatternEnum::Checker3DPatternEnum(checker_pattern)));
    wall_material.set_ambient(0.8);
    wall_material.set_diffuse(0.2);
    wall_material.set_specular(0.0);

    let wall_trans = &Matrix::rotate_x(1.5708) * &Matrix::translation(0., 0., 10.);

    // wall
    let mut wall = Shape::new_plane(Plane::new(), "plane".to_string());
    wall.set_transformation(wall_trans);
    wall.set_material(wall_material);

    // bg glass_ball
    let mut glass_ball = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    glass_ball.get_material_mut().set_ambient(0.0);
    glass_ball.get_material_mut().set_diffuse(0.0);
    glass_ball.get_material_mut().set_specular(0.9);
    glass_ball.get_material_mut().set_shininess(300.0);
    glass_ball.get_material_mut().set_reflective(0.9);
    glass_ball.get_material_mut().set_transparency(0.9);
    glass_ball.get_material_mut().set_refractive_index(1.5);

    //hollow_center
    let mut hollow_center = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    let trans = Matrix::scale(0.5, 0.5, 0.5);
    hollow_center.set_transformation(trans);
    hollow_center.get_material_mut().set_ambient(0.0);
    hollow_center.get_material_mut().set_diffuse(0.0);
    hollow_center.get_material_mut().set_specular(0.9);
    hollow_center.get_material_mut().set_shininess(300.0);
    hollow_center.get_material_mut().set_reflective(0.9);
    hollow_center.get_material_mut().set_transparency(0.9);
    hollow_center.get_material_mut().set_refractive_index(1.0000034);

    let pl = PointLight::new(Tuple4D::new_point(-20., 80.0, -20.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.add_light(l);
    //   w.add_light(    l);

    w.add_shape(wall);
    w.add_shape(glass_ball);
    w.add_shape(hollow_center);

    let mut c = Camera::new(width as usize, height as usize, 0.45);
    c.calc_pixel_size();
    c.set_antialiasing(true);
    c.set_antialiasing_size(3);

    c.set_from(Tuple4D::new_point(4.0, 4.0, -6.0));
    c.set_to(Tuple4D::new_point(0.0, 0.0, 0.0));
    c.set_up(Tuple4D::new_vector(0.0, 1.0, 0.0));

    (w, c)
}
