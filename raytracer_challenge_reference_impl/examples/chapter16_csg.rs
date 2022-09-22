extern crate num_cpus;

use std::error::Error;
use std::mem::transmute;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let size_factor = 2.0;
    let antialiasing = true;
    let antialiasing_size = 3;
    let arealight_u = 8;
    let arealight_v = 8;

    let (world, camera) = setup_world_csg(size_factor, antialiasing, antialiasing_size, arealight_u, arealight_v);
    let start = Instant::now();
    let canvas = Camera::render_multi_core(&camera, &world);
    // let canvas = Camera::render_debug(&camera, &world, 308, 254);
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
        "./chapter_16_csg_{}x{}_{}_arealight_{}x{}.png",
        camera.get_hsize(),
        camera.get_vsize(),
        aa,
        arealight_u,
        arealight_v
    );

    canvas.write_png(filename.as_str())?;
    println!("wrote file {}", filename);

    Ok(())
}

fn setup_world_csg<'a>(
    size_factor: f64,
    antialiasing: bool,
    antialiasing_size: usize,
    usteps: usize,
    vsteps: usize,
) -> (World, Camera) {
    let width = (400 as f64 * size_factor) as usize;
    let height = (160 as f64 * size_factor) as usize;

    let corner = Tuple4D::new_point(-1.0, 2.0, 4.0);
    let uvec = Tuple4D::new_vector(2.0, 0.0, 0.0);
    let vvec = Tuple4D::new_vector(0.0, 2.0, 0.0);
    let intensity = Color::new(1.5, 1.5, 1.5);
    let area_light = AreaLight::new(corner, uvec, usteps, vvec, vsteps, intensity, Sequence::new(vec![]));
    let area_light = Light::AreaLight(area_light);

    // ---- CUBE -------
    let mut c = Cube::new();
    c.get_material_mut().set_color(Color::new(1.5, 1.5, 1.5));
    c.get_material_mut().set_ambient(1.0);
    c.get_material_mut().set_diffuse(0.0);
    c.get_material_mut().set_specular(0.0);
    c.get_material_mut().set_shininess(100.0);

    let m_trans = Matrix::translation(0.0, 3.0, 4.0);
    let m_scale = Matrix::scale(1.0, 1.0, 0.01);
    let m = &m_trans * &m_scale;

    c.set_transformation(m);
    let mut cube = Shape::new(ShapeEnum::CubeEnum(c));
    cube.set_casts_shadow(false);

    // ---- PLANE -------
    let mut plane = Plane::new();
    plane.get_material_mut().set_color(Color::new(1., 1., 1.));
    plane.get_material_mut().set_ambient(0.025);
    plane.get_material_mut().set_diffuse(0.67);
    plane.get_material_mut().set_specular(0.0);
    plane.get_material_mut().set_shininess(200.0);

    let plane = Shape::new(ShapeEnum::PlaneEnum(plane));

    // ---- SPHERE 1 -------
    let mut sphere1 = Sphere::new();
    sphere1.get_material_mut().set_color(Color::new(1.0, 0., 0.));
    sphere1.get_material_mut().set_ambient(0.1);
    sphere1.get_material_mut().set_diffuse(0.6);
    sphere1.get_material_mut().set_specular(0.0);
    sphere1.get_material_mut().set_reflective(0.3);
    sphere1.get_material_mut().set_shininess(200.0);
    sphere1.get_material_mut().set_transparency(0.2);
    sphere1.get_material_mut().set_refractive_index(0.2);

    let m_trans = Matrix::translation(0.5, 0.5, 0.0);
    let m_scale = Matrix::scale(0.5, 0.5, 0.5);
    let m = &m_trans * &m_scale;

    sphere1.set_transformation(m);
    let sphere1 = Shape::new(ShapeEnum::SphereEnum(sphere1));

    // ---- SPHERE 2 -------
    let mut sphere2 = Sphere::new();
    sphere2.get_material_mut().set_color(Color::new(0.5, 0.5, 1.0));
    sphere2.get_material_mut().set_ambient(0.1);
    sphere2.get_material_mut().set_diffuse(0.6);
    sphere2.get_material_mut().set_specular(0.0);
    sphere2.get_material_mut().set_reflective(0.3);
    sphere2.get_material_mut().set_shininess(200.0);
    sphere2.get_material_mut().set_transparency(0.2);
    sphere2.get_material_mut().set_refractive_index(0.2);

    let m_trans = Matrix::translation(0.25, 0.25, 0.0);
    let m_scale = Matrix::scale(0.3, 0.3, 0.3);
    let m = &m_trans * &m_scale;

    sphere2.set_transformation(m);
    let sphere2 = Shape::new(ShapeEnum::SphereEnum(sphere2));

    let mut w = World::new();
    let csg = Csg::new(w.get_shapes_mut(), "first_csg".to_string(), CsgOp::INTERSECTION);
    Csg::add_child(w.get_shapes_mut(), csg, sphere1, sphere2);

    w.add_light(area_light);
    w.add_shape(cube);
    w.add_shape(plane);
    // w.add_shape(sphere1);
    // w.add_shape(sphere2);

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
