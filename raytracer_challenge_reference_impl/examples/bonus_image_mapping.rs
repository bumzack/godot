extern crate num_cpus;

use std::error::Error;
use std::time::Instant;

use raytracer_challenge_reference_impl::patterns::PatternEnum::ImageTexturePatternEnum;
use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 800;
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
        "./image_texture_bonus_{}x{}_{}.png",
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
    let mut s = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
    s.get_material_mut().set_diffuse(0.9);
    s.get_material_mut().set_specular(0.1);
    s.get_material_mut().set_ambient(0.1);
    s.get_material_mut().set_shininess(10.0);

    let rot_y = Matrix::rotate_y(1.9);
    let translate = Matrix::translation(0.0, 1.1, 0.0);
    let trans = &rot_y * &translate;
    s.set_transformation(trans);

    let mut p = Shape::new(ShapeEnum::PlaneEnum(Plane::new()));
    p.get_material_mut().set_diffuse(0.1);
    p.get_material_mut().set_specular(0.);
    p.get_material_mut().set_ambient(0.);
    p.get_material_mut().set_reflective(0.4);

    let pl = PointLight::new(Tuple4D::new_point(-100.0, 100.0, -100.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let image = Canvas::read_png(
        "/Users/bumzack/stoff/rust/godot/raytracer_challenge_reference_impl/downloaded_obj_files/Earthmap1000x500.jpg",
    )
    .expect("loading image linear_gradient.png");
    let pattern = ImageTexturePattern::new(image);
    let pattern = Pattern::new(ImageTexturePatternEnum(pattern));
    s.get_material_mut().set_pattern(pattern);

    let mut w = World::new();
    w.add_light(l);
    w.add_shape(p);
    w.add_shape(s);

    let mut c = Camera::new(width, height, 0.9);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(1.0, 2.0, -10.0),
        &Tuple4D::new_point(0.0, 1.1, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
