extern crate num_cpus;

use std::error::Error;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
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
    canvas.write_png("./checker_cylinder_bonus.png")?;
    println!("file exported");
    Ok(())
}

fn setup_world(width: usize, height: usize) -> (World, Camera) {
    let checker = uv_checkers(16, 8, Color::new(0.0, 0.5, 0.0), Color::new(1.0, 1.0, 1.0));
    let checker_3d = CylinderTexturePattern::new(checker);
    let p = Pattern::CylinderTexturePattern(checker_3d);

    let trans = Matrix::translation(0.0, -0.5, 0.0);
    let scale = Matrix::scale(1.0, PI, 1.0);
    let p_transformed = &scale * &trans;

    let mut cylinder = Cylinder::new();
    cylinder.set_transformation(p_transformed);
    cylinder.set_minimum(0.0);
    cylinder.set_maximum(1.0);
    cylinder.get_material_mut().set_pattern(p);
    cylinder.get_material_mut().set_ambient(0.1);
    cylinder.get_material_mut().set_specular(0.4);
    cylinder.get_material_mut().set_diffuse(0.8);
    cylinder.get_material_mut().set_shininess(15.0);

    let pl = PointLight::new(Tuple4D::new_point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.add_light(l);
    w.add_shape(Shape::new(ShapeEnum::Cylinder(cylinder)));

    let mut c = Camera::new(width, height, 0.5);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 0.0, -10.0),
        &Tuple4D::new_point(0.0, 0.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
