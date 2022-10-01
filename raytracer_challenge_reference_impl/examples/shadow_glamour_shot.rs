extern crate num_cpus;

use std::error::Error;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let size_factor = 2.0;

    let antialiasing = true;
    let antialiasing_size = 3;
    let filename;
    if antialiasing {
        filename = format!(
            "ref_impl_glamour_world_aliasing_size_{}_multi_core.png",
            antialiasing_size
        );
    } else {
        filename = format!("ref_impl_glamour_world_no_anti_noaliasing_multi_core.png",);
    }

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
    canvas.write_png(filename.as_str())?;

    Ok(())
}

fn setup_world_shadow_glamour<'a>(size_factor: f64, antialiasing: bool, antialiasing_size: usize) -> (World, Camera) {
    let width = (400 as f64 * size_factor) as usize;
    let height = (160 as f64 * size_factor) as usize;

    let pl = PointLight::new(Tuple4D::new_point(-1.0, 2.0, 4.0), Color::new(1.5, 1.5, 1.5));
    let l = Light::PointLight(pl);

    // ---- CUBE -------
    let mut cube = Shape::new(ShapeEnum::CubeEnum(Cube::new()));
    cube.get_material_mut().set_color(Color::new(1., 0.5, 0.2));
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
    w.add_light(l);
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
