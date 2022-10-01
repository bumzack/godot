extern crate num_cpus;

use std::error::Error;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 2048;
    let height = 2048;

    let (w, c) = setup_world(width, height);

    let start = Instant::now();
    let canvas = Camera::render_multi_core(&c, &w);
    let dur = Instant::now() - start;
    println!("multi core duration: {:?}", dur);
    let aa = match c.get_antialiasing() {
        true => format!("with_AA_{}", c.get_antialiasing_size()),
        false => "no_AA".to_string(),
    };
    let filename = &format!("./checker_plane_bonus_{}_{}_{}.png", width, height, aa);
    canvas.write_png(filename)?;
    println!("file {} exported", filename);
    Ok(())
}

fn setup_world(width: usize, height: usize) -> (World, Camera) {
    let checker = uv_checkers(2, 2, Color::new(0.0, 0.5, 0.0), Color::new(1.0, 1.0, 1.0));
    let plane_checker = PlaneTexturePattern::new(checker);
    let p = Pattern::PlaneTexturePattern(plane_checker);

    let mut plane = Shape::new(ShapeEnum::PlaneEnum(Plane::new()));
    plane.get_material_mut().set_pattern(p);
    plane.get_material_mut().set_ambient(0.1);
    plane.get_material_mut().set_specular(0.0);
    plane.get_material_mut().set_diffuse(0.9);

    let pl = PointLight::new(Tuple4D::new_point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.add_light(l);
    w.add_shape(plane);

    let mut c = Camera::new(width, height, 0.50);
    c.set_antialiasing(true);
    c.set_antialiasing_size(3);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(1.0, 2.0, -5.0),
        &Tuple4D::new_point(0.0, 0.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
