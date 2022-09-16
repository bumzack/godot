use std::error::Error;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let size_factor = 5.0;
    let antialiasing = true;
    let antialiasing_size = 3;

    let (world, camera) = setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);
    let aa = match camera.get_antialiasing() {
        true => format!("with_AA_{}", camera.get_antialiasing_size()),
        false => "no_AA".to_string(),
    };
    let filename = &format!(
        "./test_softshadow_multiple_light_{}_{}_{}.png",
        camera.get_hsize(),
        camera.get_vsize(),
        aa
    );

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
    canvas.write_png(filename.as_str())?;
    println!("wrote file {}", filename);
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
    let intensity = Color::new(1.0, 1.0, 1.0);
    let area_light = AreaLight::new(corner, uvec, usteps, vvec, vsteps, intensity, Sequence::new(vec![]));
    let area_light = Light::AreaLight(area_light);

    let corner2 = Tuple4D::new_point(-1.0, 2.2, 4.2);
    let intensity2 = Color::new(1.1, 1.2, 1.2);
    let area_light2 = AreaLight::new(corner2, uvec, usteps, vvec, vsteps, intensity2, Sequence::new(vec![]));
    let area_light2 = Light::AreaLight(area_light2);

    let corner3 = Tuple4D::new_point(1.0, 1.8, 3.0);
    let intensity3 = Color::new(1.0, 1.3, 1.3);
    let area_light3 = AreaLight::new(corner3, uvec, usteps, vvec, vsteps, intensity3, Sequence::new(vec![]));
    let area_light3 = Light::AreaLight(area_light3);

    // let corner3 = Tuple4D::new_point(-1.5, 5.0, 4.5);
    // let intensity = Color::new(0.1, 0.2, 0.1);
    // let point_light = PointLight::new(corner3, intensity);
    // let pl = Light::PointLight(point_light);
    //
    // let corner4 = Tuple4D::new_point(-1.5, 5.0, -4.5);
    // let intensity4 = Color::new(0.1, 0.2, 0.1);
    // let point_light4 = PointLight::new(corner4, intensity4);
    // let pl4 = Light::PointLight(point_light4);
    //
    // let corner5 = Tuple4D::new_point(-1.0, 2.5, 4.5);
    // let intensity5 = Color::new(0.1, 0.2, 0.1);
    // let point_light5 = PointLight::new(corner5, intensity5);
    // let pl5 = Light::PointLight(point_light5);
    //
    // let corner6 = Tuple4D::new_point(-1.0, 4.5, 4.5);
    // let intensity6 = Color::new(0.1, 0.2, 0.1);
    // let point_light6 = PointLight::new(corner6, intensity6);
    // let pl6 = Light::PointLight(point_light6);

    // ---- CUBE -------
    let mut c = Cube::new();
    c.get_material_mut().set_color(Color::new(1.5, 1.5, 1.5));
    c.get_material_mut().set_ambient(1.0);
    c.get_material_mut().set_diffuse(0.0);
    c.get_material_mut().set_specular(0.0);

    let m_trans = Matrix::translation(0.0, 3.0, 4.0);
    let m_scale = Matrix::scale(1.0, 1.0, 0.01);
    let m = &m_trans * &m_scale;

    c.set_transformation(m);
    let mut cube = Shape::new(ShapeEnum::Cube(c));
    cube.set_casts_shadow(true);

    // ---- PLANE -------
    let mut plane = Plane::new();
    plane.get_material_mut().set_color(Color::new(1., 1., 1.));
    plane.get_material_mut().set_ambient(0.025);
    plane.get_material_mut().set_diffuse(0.67);
    plane.get_material_mut().set_specular(0.0);

    let plane = Shape::new(ShapeEnum::Plane(plane));

    // ---- SPHERE 1 -------
    let mut sphere1 = Sphere::new();
    sphere1.get_material_mut().set_color(Color::new(1.0, 0., 0.));
    sphere1.get_material_mut().set_ambient(0.1);
    sphere1.get_material_mut().set_diffuse(0.6);
    sphere1.get_material_mut().set_specular(0.0);
    sphere1.get_material_mut().set_reflective(0.3);

    let m_trans = Matrix::translation(0.5, 0.5, 0.0);
    let m_scale = Matrix::scale(0.5, 0.5, 0.5);
    let m = &m_trans * &m_scale;

    sphere1.set_transformation(m);
    let sphere1 = Shape::new(ShapeEnum::Sphere(sphere1));

    // ---- SPHERE 2 -------
    let mut sphere2 = Sphere::new();
    sphere2.get_material_mut().set_color(Color::new(0.5, 0.5, 1.0));
    sphere2.get_material_mut().set_ambient(0.1);
    sphere2.get_material_mut().set_diffuse(0.6);
    sphere2.get_material_mut().set_specular(0.0);
    sphere2.get_material_mut().set_reflective(0.3);

    let m_trans = Matrix::translation(-0.25, 0.33, 0.0);
    let m_scale = Matrix::scale(0.33, 0.33, 0.33);
    let m = &m_trans * &m_scale;

    sphere2.set_transformation(m);
    let sphere2 = Shape::new(ShapeEnum::Sphere(sphere2));

    let mut w = World::new();
    w.add_light(area_light);
    w.add_light(area_light2);
    w.add_light(area_light3);
    // w.add_light(pl);
    // w.add_light(pl4);
    // w.add_light(pl5);
    // w.add_light(pl6);

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
