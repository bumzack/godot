extern crate num_cpus;

use std::error::Error;
use std::f64::consts::PI;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 3840;
    let height = 2160;

    let width = 800;
    let height = 600;

    let antialiasing = true;
    let antialiasing_size = 3;

    let (world, camera) = setup_world(width, height, antialiasing, antialiasing_size);

    let aa = match camera.get_antialiasing() {
        true => format!("with_AA_{}", camera.get_antialiasing_size()),
        false => "no_AA".to_string(),
    };
    let filename = &format!(
        "./chapter_16_csg_{}x{}_{}.png",
        camera.get_hsize(),
        camera.get_vsize(),
        aa,
    );
    let canvas = Camera::render_multi_core_tiled(&camera, &world, 10, 10);
    canvas.write_png(filename.as_str())?;

    Ok(())
}

fn setup_world(width: usize, height: usize, antialiasing: bool, antialiasing_size: usize) -> (World, Camera) {
    let pl = PointLight::new(Tuple4D::new_point(-1.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.add_light(l);
    w.add_floor();
    w.add_x_axis();

    let mut c = Camera::new(width, height, PI / 2.0);
    c.set_antialiasing(antialiasing);
    c.set_antialiasing_size(antialiasing_size);

    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -6.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
