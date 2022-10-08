use std::error::Error;

use crate::prelude::{
    Camera, CameraOps, Checker3DPattern, Color, ColorOps, Light, Material, MaterialOps, Matrix, MatrixOps, Pattern,
    PatternEnum, Plane, PointLight, Shape, ShapeOps, Sphere, StripePattern, Tuple, Tuple4D, World, WorldOps,
};

pub fn chapter11_01(width: usize, height: usize) -> (World, Camera) {
    let mut wall_material = Material::new();
    wall_material.set_ambient(0.0);
    wall_material.set_diffuse(0.4);
    wall_material.set_specular(0.0);
    wall_material.set_reflective(0.3);

    let mut stripe_pattern = StripePattern::new();
    stripe_pattern.set_color_a(Color::new(0.45, 0.45, 0.45));
    stripe_pattern.set_color_a(Color::new(0.55, 0.55, 0.55));

    // let wall_trans = &Matrix::rotate_y(1.5708) * &Matrix::scale(0.25, 0.25, 0.25);

    // floor
    let mut floor = Shape::new_plane(Plane::new(), "plane".to_string());
    floor.set_transformation(Matrix::rotate_y(0.13145));
    let mut checker_pattern = Checker3DPattern::new();
    checker_pattern.set_color_a(Color::new(0.35, 0.35, 0.35));
    checker_pattern.set_color_a(Color::new(0.65, 0.65, 0.65));
    floor
        .get_material_mut()
        .set_pattern(Pattern::new(PatternEnum::Checker3DPatternEnum(checker_pattern)));
    floor.get_material_mut().set_specular(0.0);
    floor.get_material_mut().set_reflective(0.4);

    // ceiling
    let mut ceiling = Shape::new_plane(Plane::new(), "plane".to_string());
    ceiling.set_transformation(Matrix::translation(0.0, 5., 0.0));
    ceiling.get_material_mut().set_color(Color::new(0.8, 0.8, 0.8));
    ceiling.get_material_mut().set_ambient(0.3);
    ceiling.get_material_mut().set_specular(0.0);

    // west wall
    let mut west_wall = Shape::new_plane(Plane::new(), "plane".to_string());
    let trans = &Matrix::rotate_y(1.5708) * &Matrix::rotate_z(1.5708);
    let trans = &trans * &Matrix::translation(-5.0, 0., 0.0);
    west_wall.set_transformation(trans);
    west_wall.set_material(wall_material.clone());

    // east wall
    let mut east_wall = Shape::new_plane(Plane::new(), "plane".to_string());
    let trans = &Matrix::rotate_y(1.5708) * &Matrix::rotate_z(1.5708);
    let trans = &trans * &Matrix::translation(5.0, 0., 0.0);
    east_wall.set_transformation(trans);
    east_wall.set_material(wall_material.clone());

    // north wall
    let mut north_wall = Shape::new_plane(Plane::new(), "plane".to_string());
    let trans = Matrix::rotate_x(1.5708);
    let trans = &trans * &Matrix::translation(0.0, 0., 5.0);
    north_wall.set_transformation(trans);
    north_wall.set_material(wall_material.clone());

    // south wall
    let mut south_wall = Shape::new_plane(Plane::new(), "plane".to_string());
    let trans = Matrix::rotate_x(1.5708);
    let trans = &trans * &Matrix::translation(0.0, 0., -5.0);
    south_wall.set_transformation(trans);
    south_wall.set_material(wall_material.clone());

    // bg ball1
    let mut bg_ball1 = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    let trans = Matrix::scale(0.4, 0.4, 0.4);
    let trans = &trans * &Matrix::translation(4.6, 0.4, 1.0);
    bg_ball1.set_transformation(trans);
    bg_ball1.get_material_mut().set_color(Color::new(0.8, 0.5, 0.3));
    bg_ball1.get_material_mut().set_shininess(50.0);

    // bg ball2
    let mut bg_ball2 = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    let trans = Matrix::scale(0.3, 0.3, 0.3);
    let trans = &trans * &Matrix::translation(4.7, 0.3, 0.4);
    bg_ball2.set_transformation(trans);
    bg_ball2.get_material_mut().set_color(Color::new(0.9, 0.4, 0.5));
    bg_ball2.get_material_mut().set_shininess(50.0);

    // bg ball3
    let mut bg_ball3 = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    let trans = Matrix::scale(0.5, 0.5, 0.5);
    let trans = &trans * &Matrix::translation(-1., 0.5, 4.5);
    bg_ball3.set_transformation(trans);
    bg_ball3.get_material_mut().set_color(Color::new(0.4, 0.9, 0.6));
    bg_ball3.get_material_mut().set_shininess(50.0);

    // bg ball4
    let mut bg_ball4 = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    let trans = Matrix::scale(0.3, 0.3, 0.3);
    let trans = &trans * &Matrix::translation(-1.7, 0.3, 4.7);
    bg_ball4.set_transformation(trans);
    bg_ball4.get_material_mut().set_color(Color::new(0.4, 0.6, 0.9));
    bg_ball4.get_material_mut().set_shininess(50.0);

    // red sphere
    let mut red_sphere = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    //  let trans  =  Matrix::scale( 0.3,0.3,0.3);
    let trans = Matrix::translation(-0.6, 1., 0.6);
    red_sphere.set_transformation(trans);
    red_sphere.get_material_mut().set_color(Color::new(1., 0.3, 0.2));
    red_sphere.get_material_mut().set_specular(0.4);
    red_sphere.get_material_mut().set_shininess(5.0);

    // blue glass  sphere
    let mut blue_glass_sphere = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    let trans = Matrix::scale(0.7, 0.7, 0.7);
    let trans = &trans * &Matrix::translation(0.6, 0.7, -0.6);
    blue_glass_sphere.set_transformation(trans);
    blue_glass_sphere.get_material_mut().set_color(Color::new(0., 0., 0.2));
    blue_glass_sphere.get_material_mut().set_ambient(0.);
    blue_glass_sphere.get_material_mut().set_diffuse(0.4);
    blue_glass_sphere.get_material_mut().set_specular(0.9);
    blue_glass_sphere.get_material_mut().set_shininess(300.0);
    blue_glass_sphere.get_material_mut().set_reflective(0.9);
    blue_glass_sphere.get_material_mut().set_transparency(0.9);
    blue_glass_sphere.get_material_mut().set_refractive_index(1.5);

    // green glass  sphere
    let mut green_glass_sphere = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    let trans = Matrix::scale(0.5, 0.5, 0.5);
    let trans = &trans * &Matrix::translation(-0.7, 0.5, -0.8);
    green_glass_sphere.set_transformation(trans);
    green_glass_sphere.get_material_mut().set_color(Color::new(0., 0.2, 0.));
    green_glass_sphere.get_material_mut().set_ambient(0.);
    green_glass_sphere.get_material_mut().set_diffuse(0.4);
    green_glass_sphere.get_material_mut().set_specular(0.9);
    green_glass_sphere.get_material_mut().set_shininess(300.0);
    green_glass_sphere.get_material_mut().set_reflective(0.9);
    green_glass_sphere.get_material_mut().set_transparency(0.9);
    green_glass_sphere.get_material_mut().set_refractive_index(1.5);

    let pl = PointLight::new(Tuple4D::new_point(-4.9, 4.9, -1.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.add_light(l);
    //   w.add_light(    l);

    w.add_shape(floor);
    w.add_shape(ceiling);
    w.add_shape(west_wall);
    w.add_shape(east_wall);
    w.add_shape(north_wall);
    w.add_shape(south_wall);
    w.add_shape(bg_ball1);
    w.add_shape(bg_ball2);
    w.add_shape(bg_ball3);
    w.add_shape(bg_ball4);
    w.add_shape(red_sphere);
    w.add_shape(blue_glass_sphere);
    w.add_shape(green_glass_sphere);

    // w.add_x_axis();
    // w.add_y_axis();
    // w.add_z_axis();
    // w.add_shape(Shape::new(ShapeEnum::SphereEnum(middle)));
    // w.add_shape(Shape::new(ShapeEnum::SphereEnum(left)));
    // w.add_shape(Shape::new(ShapeEnum::SphereEnum(right)));

    let mut c = Camera::new(width as usize, height as usize, 1.15);
    c.calc_pixel_size();
    c.set_antialiasing(true);
    c.set_antialiasing_size(3);

    c.set_transformation(Matrix::view_transform(
        //&Tuple4D::new_point(4.0, 4.0, -6.0),
        &Tuple4D::new_point(-2.6, 1.5, -3.9),
        &Tuple4D::new_point(-0.6, 1.0, -0.8),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
