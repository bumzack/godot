extern crate num_cpus;

use std::error::Error;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let size_factor = 3.0;
    let antialiasing = true;
    let antialiasing_size = 3;

    let (world, camera) = setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);
    let start = Instant::now();
    let canvas = Camera::render_multi_core(&camera, &world);
    let dur = Instant::now() - start;
    if camera.get_antialiasing() {
        println!(
            "multi core duration: {:?} with AA size = {}",
            dur,
            camera.get_antialiasing_size()
        );
    } else {
        println!("multi core duration: {:?}, no AA", dur);
    }
    let aa = match camera.get_antialiasing() {
        true => format!("with_AA_{}", camera.get_antialiasing_size()),
        false => "no_AA".to_string(),
    };
    let filename = &format!(
        "./test_soft_shadow_aka_area_lights_{}_{}_{}.png",
        camera.get_hsize(),
        camera.get_vsize(),
        aa
    );

    canvas.write_png(&filename)?;
    println!("written file {}", filename);

    Ok(())
}

fn setup_world_shadow_glamour<'a>(size_factor: f64, antialiasing: bool, antialiasing_size: usize) -> (World, Camera) {
    let width = (400 as f64 * size_factor) as usize;
    let height = (160 as f64 * size_factor) as usize;

    let corner = Tuple4D::new_point(-1.0, 2.0, 4.0);
    let uvec = Tuple4D::new_vector(2.0, 0.0, 0.0);
    let vvec = Tuple4D::new_vector(0.0, 2.0, 0.0);
    let usteps = 16;
    let vsteps = 16;
    let intensity = Color::new(1.5, 1.5, 1.5);
    let area_light = AreaLight::new(corner, uvec, usteps, vvec, vsteps, intensity, Sequence::new(vec![]));
    let area_light = Light::AreaLight(area_light);

    // ---- CUBE -------
    let mut cube = Shape::new(ShapeEnum::CubeEnum(Cube::new()));
    cube.get_material_mut().set_color(Color::new(1.5, 1.5, 1.5));
    cube.get_material_mut().set_ambient(1.0);
    cube.get_material_mut().set_diffuse(0.0);
    cube.get_material_mut().set_specular(0.0);

    let m_trans = Matrix::translation(0.0, 3.0, 4.0);
    let m_scale = Matrix::scale(1.0, 1.0, 0.01);
    let m = &m_trans * &m_scale;

    cube.set_transformation(m);
    cube.set_casts_shadow(false);

    // ---- PLANE -------
    let mut plane = Shape::new(ShapeEnum::PlaneEnum(Plane::new()));
    plane.get_material_mut().set_color(Color::new(1., 1., 1.));
    plane.get_material_mut().set_ambient(0.025);
    plane.get_material_mut().set_diffuse(0.67);
    plane.get_material_mut().set_specular(0.0);

    // ---- SPHERE 1 -------
    let mut sphere1 = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
    sphere1.get_material_mut().set_color(Color::new(1.0, 0., 0.));
    sphere1.get_material_mut().set_ambient(0.1);
    sphere1.get_material_mut().set_diffuse(0.6);
    sphere1.get_material_mut().set_specular(0.0);
    sphere1.get_material_mut().set_reflective(0.3);

    let m_trans = Matrix::translation(0.5, 0.5, 0.0);
    let m_scale = Matrix::scale(0.5, 0.5, 0.5);
    let m = &m_trans * &m_scale;

    sphere1.set_transformation(m);

    // ---- SPHERE 2 -------
    let mut sphere2 = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
    sphere2.get_material_mut().set_color(Color::new(0.5, 0.5, 1.0));
    sphere2.get_material_mut().set_ambient(0.1);
    sphere2.get_material_mut().set_diffuse(0.6);
    sphere2.get_material_mut().set_specular(0.0);
    sphere2.get_material_mut().set_reflective(0.3);

    let m_trans = Matrix::translation(-0.25, 0.33, 0.0);
    let m_scale = Matrix::scale(0.33, 0.33, 0.33);
    let m = &m_trans * &m_scale;

    sphere2.set_transformation(m);

    let mut w = World::new();
    w.add_light(area_light);
    w.add_shape(cube);
    w.add_shape(plane);
    w.add_shape(sphere1);
    w.add_shape(sphere2);

    let mut c = Camera::new(width, height, 0.78540);
    c.set_antialiasing(antialiasing);
    c.set_antialiasing_size(antialiasing_size);

    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(-3.0, 1., 2.5),
        &Tuple4D::new_point(0.0, 0.5, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
