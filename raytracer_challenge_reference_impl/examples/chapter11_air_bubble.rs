use std::error::Error;
use std::f64::consts::PI;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 1200;
    let height = 7200;

    let pov = 0.45;
    let antialiasing = true;
    let antialiasing_size = 3;

    let (world, camera) = setup_world(width, height, pov, antialiasing, antialiasing_size);

    let aa = match camera.get_antialiasing() {
        true => format!("with_AA_{}", camera.get_antialiasing_size()),
        false => "no_AA".to_string(),
    };
    let filename = &format!(
        "./chapter11_air_bubble_{}x{}_{}.png",
        camera.get_hsize(),
        camera.get_vsize(),
        aa
    );
    println!("filename {}", filename);

    let start = Instant::now();
    let canvas = Camera::render_multi_core(&camera, &world);
    let dur = Instant::now() - start;
    println!("multi core duration: {:?}", dur);

    canvas.write_png(filename)?;
    println!("wrote file {}", filename);

    println!("DONE");

    Ok(())
}

fn setup_world(width: i32, height: i32, pov: f64, anitaliasing: bool, anitaliasing_size: usize) -> (World, Camera) {
    let mut checker_pattern = Checker3DPattern::new();
    checker_pattern.set_color_a(Color::new(0.15, 0.15, 0.15));
    checker_pattern.set_color_a(Color::new(0.85, 0.85, 0.85));

    let mut wall_material = Material::new();
    wall_material.set_pattern(Pattern::new(PatternEnum::Checker3DPatternEnum(checker_pattern)));
    wall_material.set_ambient(0.8);
    wall_material.set_diffuse(0.2);
    wall_material.set_specular(0.0);

    let wall_trans = &Matrix::rotate_x(1.5708) * &Matrix::translation(0., 0., 10.);

    // wall
    let mut wall = Shape::new(ShapeEnum::PlaneEnum(Plane::new()));
    wall.set_transformation(Matrix::rotate_y(0.13145));
    wall.set_material(wall_material);

    // bg glass_ball
    let mut glass_ball = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
    glass_ball.get_material_mut().set_ambient(0.0);
    glass_ball.get_material_mut().set_diffuse(0.0);
    glass_ball.get_material_mut().set_specular(0.9);
    glass_ball.get_material_mut().set_shininess(300.0);
    glass_ball.get_material_mut().set_reflective(0.9);
    glass_ball.get_material_mut().set_transparency(0.9);
    glass_ball.get_material_mut().set_refractive_index(1.5);

    //hollow_center
    let mut hollow_center = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
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

    let mut c = Camera::new(width as usize, height as usize, pov);
    c.calc_pixel_size();
    c.set_antialiasing(anitaliasing);
    c.set_antialiasing_size(anitaliasing_size);

    c.set_transformation(Matrix::view_transform(
        //&Tuple4D::new_point(4.0, 4.0, -6.0),
        &Tuple4D::new_point(0., 0.0, -5.0),
        &Tuple4D::new_point(0.0, 0.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
