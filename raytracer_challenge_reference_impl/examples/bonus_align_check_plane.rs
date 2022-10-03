extern crate num_cpus;

use std::error::Error;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 400;
    let height = 400;

    let (w, camera) = setup_world(width, height);
    let start = Instant::now();
    let canvas = Camera::render_multi_core(&camera, &w);
    let dur = Instant::now() - start;
    println!("multi core duration: {:?}", dur);
    let aa = match camera.get_antialiasing() {
        true => format!("with_AA_{}", camera.get_antialiasing_size()),
        false => "no_AA".to_string(),
    };
    let filename = &format!(
        "./bonus_align_checker_plane_{}x{}_{}.png",
        camera.get_hsize(),
        camera.get_vsize(),
        aa
    );
    println!("filename {}", filename);
    canvas.write_png(filename)?;
    println!("file exported");
    Ok(())
}

fn setup_world(width: usize, height: usize) -> (World, Camera) {
    let main = Color::new(1.0, 1.0, 1.0);
    let ul = Color::new(1.0, 0.0, 0.0);
    let ur = Color::new(1.0, 1.0, 0.0);
    let bl = Color::new(0.0, 1.0, 0.0);
    let br = Color::new(0.0, 1.0, 1.0);

    let cube_checker = CubeChecker::new(main, ul, ur, bl, br);
    let texture = AlignCheckTexturePattern::new(cube_checker);

    let p = Pattern::new(PatternEnum::AlignCheckTexturePatternEnum(texture));

    let mut plane = Shape::new(ShapeEnum::PlaneEnum(Plane::new()));
    plane.get_material_mut().set_pattern(p);
    plane.get_material_mut().set_ambient(0.1);
    plane.get_material_mut().set_diffuse(0.8);

    let pl = PointLight::new(Tuple4D::new_point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.add_light(l);
    w.add_shape(plane);

    let mut c = Camera::new(width, height, 0.9);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(1.0, 2.0, -5.0),
        &Tuple4D::new_point(0.0, 0.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
