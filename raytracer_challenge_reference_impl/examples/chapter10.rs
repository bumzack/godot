use std::error::Error;
use std::f64::consts::PI;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

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
    w.add_light(l);
    let mut shape = Shape::new(ShapeEnum::PlaneEnum(floor));
    // shape.set_casts_shadow(false);
    w.add_shape(shape);
    w.add_shape(Shape::new(ShapeEnum::PlaneEnum(left_wall)));
    w.add_shape(Shape::new(ShapeEnum::PlaneEnum(right_wall)));
    let mut shape3 = Shape::new(ShapeEnum::SphereEnum(middle));
    // shape3.set_casts_shadow(false);
    w.add_shape(shape3);
    let mut shape1 = Shape::new(ShapeEnum::SphereEnum(left));
    shape1.set_casts_shadow(false);
    w.add_shape(shape1);
    let mut shape2 = Shape::new(ShapeEnum::SphereEnum(right));
    shape2.set_casts_shadow(false);
    w.add_shape(shape2);

    let mut c = Camera::new(800, 600, PI / 3.0);
    c.calc_pixel_size();

    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -5.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));

    let start = Instant::now();
    let canvas = Camera::render_multi_core(&c, &w);
    let dur = Instant::now() - start;
    println!("multi core duration: {:?}", dur);

    let aa = match c.get_antialiasing() {
        true => format!("with_AA_{}", c.get_antialiasing_size()),
        false => "no_AA".to_string(),
    };
    let filename = &format!("./chapter10_{}x{}_{} .png", c.get_hsize(), c.get_vsize(), aa);

    canvas.write_png(filename)?;
    println!("written file {}", filename);

    Ok(())
}
