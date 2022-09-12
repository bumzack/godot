extern crate num_cpus;

use std::error::Error;
use std::f32::consts::PI;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 400;
    let height = 400;

    let (w, c) = setup_world(width, height);

    let start = Instant::now();
    let canvas = Camera::render_multi_core(&c, &w);
    let dur = Instant::now() - start;
    println!("multi core duration: {:?}", dur);
    canvas.write_png("./checker_plane_bonus.png")?;
    println!("file exported");
    Ok(())
}

fn setup_world(width: usize, height: usize) -> (World, Camera) {
    let checker = uv_checkers(2, 2, Color::new(0.0, 0.5, 0.0), Color::new(1.0, 1.0, 1.0));
    let plane_checker = PlaneTexturePattern::new(checker);
    let p = Pattern::PlaneTexturePattern(plane_checker);

    let mut plane = Plane::new();
    plane.get_material_mut().set_pattern(p);
    plane.get_material_mut().set_ambient(0.1);
    plane.get_material_mut().set_specular(0.0);
    plane.get_material_mut().set_diffuse(0.9);

    let pl = PointLight::new(Tuple4D::new_point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.add_light(l);
    w.add_shape(Shape::new(ShapeEnum::Plane(plane)));

    let mut c = Camera::new(width, height, 0.50);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(1.0, 2.0, -5.0),
        &Tuple4D::new_point(0.0, 0.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
