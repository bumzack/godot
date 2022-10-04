extern crate num_cpus;

use raytracer_challenge_reference_impl::prelude::*;
use std::error::Error;
use std::f64::consts::PI;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 1280;
    let height = 800;

    let (world, camera) = setup_world(width, height);
    let start = Instant::now();
    let canvas = Camera::render_multi_core(&camera, &world);
    let dur = Instant::now() - start;
    println!("multi core duration: {:?}", dur);
    canvas.write_png("chapter14_multi_core_no_AA.png")?;

    Ok(())
}

fn setup_world(width: usize, height: usize) -> (World, Camera) {
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

    let mut pattern = Checker3DPattern::new();
    pattern.set_color_a(Color::new(0.1, 0.8, 0.4));
    pattern.set_color_a(Color::new(0.8, 0.2, 0.2));
    let   checker_3d = Pattern::new(PatternEnum::Checker3DPatternEnum(pattern));
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
    middle.get_material_mut().set_reflective(1.3);
    middle.get_material_mut().set_refractive_index(1.3);

    let mut right = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    right.set_transformation(&Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scale(0.5, 0.5, 0.5));
    right.get_material_mut().set_color(Color::new(0.5, 1.0, 0.1));
    right.get_material_mut().set_diffuse(0.7);
    right.get_material_mut().set_specular(0.3);
    middle.get_material_mut().set_reflective(1.8);
    middle.get_material_mut().set_refractive_index(1.8);

    let mut left = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    left.set_transformation(&Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scale(0.333, 0.333, 0.333));
    left.get_material_mut().set_color(Color::new(1., 0., 0.));
    left.get_material_mut().set_diffuse(0.7);
    left.get_material_mut().set_specular(0.3);

    let mut pattern = Checker3DPattern::new();
    pattern.set_color_a(Color::new(1.0, 0.0, 1.0));
    pattern.set_color_a(Color::new(0.1, 0.1, 1.0));
    let   checker_3d = Pattern::new(PatternEnum::Checker3DPatternEnum(pattern));

    let mut cube = Shape::new_cube(Cube::new(), "cube".to_string());
    let c_trans = Matrix::translation(-2.0, 2.0, -1.75);
    let c_scale = Matrix::scale(0.5, 0.5, 0.25);
    let c_rot = Matrix::rotate_y(PI / 5.0);
    let c_trans2 = Matrix::translation(-2.0, 2.0, -1.75);
    let m = c_scale * c_trans;
    let m = c_rot * m;
    let m = c_trans2 * m;
    cube.set_transformation(m);
    cube.get_material_mut().set_pattern(checker_3d);
    cube.get_material_mut().set_transparency(1.5);

    let mut pattern = Checker3DPattern::new();
    pattern.set_color_a(Color::new(1.0, 0.0, 0.0));
    pattern.set_color_b(Color::new(0.0, 1.0, 1.0));
    let   checker = Pattern::new(PatternEnum::Checker3DPatternEnum(pattern));

    let mut cylinder = Cylinder::new();
    cylinder.set_minimum(1.0);
    cylinder.set_maximum(2.0);
    let mut cylinder = Shape::new_cylinder(cylinder, "cylinder".to_string());
    let c_trans = Matrix::translation(1.5, 1.0, -0.75);
    // let c_scale = Matrix::scale(2.0, 0.5, 0.25);
    cylinder.set_transformation(c_trans);
    cylinder.get_material_mut().set_pattern(checker);
    cylinder.get_material_mut().set_transparency(1.5);

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
    w.add_shape(cube);
    w.add_shape(cylinder);

    let mut c = Camera::new(width, height, PI / 2.0);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -6.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
