use std::error::Error;
use std::f64::consts::PI;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 3840;
    let height = 2160;

    let pov = 0.8;
    let antialiasing = true;
    let antialiasing_size = 3;
    let arealight_u = 16;
    let arealight_v = 16;
    let (world, camera) = setup_world(
        width,
        height,
        pov,
        antialiasing,
        antialiasing_size,
        arealight_u,
        arealight_v,
    );

    let aa = match camera.get_antialiasing() {
        true => format!("with_AA_{}", camera.get_antialiasing_size()),
        false => "no_AA".to_string(),
    };
    let filename = &format!(
        "./chapter15_suzanne_smoothed_{}x{}_{}_arealight_{}x{}.png",
        camera.get_hsize(),
        camera.get_vsize(),
        aa,
        arealight_u,
        arealight_v
    );
    println!("filename {}", filename);

    let start = Instant::now();
    let canvas = Camera::render_multi_core(&camera, &world);
    let dur = Instant::now() - start;
    println!("multi core duration: {:?}", dur);

    canvas.write_png(filename)?;
    println!("wrote file {}", filename);

    println!("DONE");

    Ok(())
}

fn setup_world(
    width: i32,
    height: i32,
    pov: f64,
    anitaliasing: bool,
    anitaliasing_size: usize,
    arealight_u: usize,
    arealight_v: usize,
) -> (World, Camera) {
    let mut floor = Sphere::new();
    floor.set_transformation(Matrix::scale(20.0, 0.01, 20.0));
    floor.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    floor.get_material_mut().set_specular(0.0);

    let mut left_wall = Sphere::new();
    left_wall.set_transformation(
        &(&(&Matrix::translation(0.0, 0.0, 6.0) * &Matrix::rotate_y(-PI / 4.0)) * &Matrix::rotate_x(PI / 2.0))
            * &Matrix::scale(10.0, 0.01, 10.),
    );
    left_wall.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    left_wall.get_material_mut().set_specular(0.0);

    let mut right_wall = Sphere::new();
    right_wall.set_transformation(
        &(&(&Matrix::translation(0.0, 0.0, 6.0) * &Matrix::rotate_y(PI / 4.0)) * &Matrix::rotate_x(PI / 2.0))
            * &Matrix::scale(10.0, 0.01, 10.0),
    );
    right_wall.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    right_wall.get_material_mut().set_specular(0.0);

    // let mut middle = Sphere::new();
    // middle.set_transformation(Matrix::translation(-0.5, 1.0, 0.5));
    // middle.get_material_mut().set_color(Color::new(0.1, 1.0, 0.5));
    // middle.get_material_mut().set_diffuse(0.7);
    // middle.get_material_mut().set_specular(0.3);
    //
    // let mut right = Sphere::new();
    // right.set_transformation(&Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scale(0.5, 0.5, 0.5));
    // right.get_material_mut().set_color(Color::new(0.5, 1.0, 0.1));
    // right.get_material_mut().set_diffuse(0.7);
    // right.get_material_mut().set_specular(0.3);
    //
    // let mut left = Sphere::new();
    // left.set_transformation(&Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scale(0.333, 0.333, 0.333));
    // left.get_material_mut().set_color(Color::new(1.0, 0.8, 0.1));
    // left.get_material_mut().set_diffuse(0.7);
    // left.get_material_mut().set_specular(0.3);

    let pl = PointLight::new(Tuple4D::new_point(3.0, 4.5, -5.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let corner = Tuple4D::new_point(4.5, 8.0, -9.0);
    let uvec = Tuple4D::new_vector(2.0, 0.0, 0.0);
    let vvec = Tuple4D::new_vector(0.0, 2.0, 0.0);

    let corner = Tuple4D::new_point(0.0, 4.0, -9.0);
    let uvec = Tuple4D::new_vector(2.0, 0.0, 0.0);
    let vvec = Tuple4D::new_vector(0.0, 2.0, -1.5);

    let usteps = 16;
    let intensity = Color::new(1.0, 1.0, 1.0);
    let area_light = AreaLight::new(
        corner,
        uvec,
        arealight_u,
        vvec,
        arealight_v,
        intensity,
        Sequence::new(vec![]),
    );
    let area_light = Light::AreaLight(area_light);

    let mut w = World::new();
      w.add_light(area_light);
    //   w.add_light(    l);

    w.add_shape(Shape::new(ShapeEnum::Sphere(floor)));
    w.add_shape(Shape::new(ShapeEnum::Sphere(left_wall)));
    w.add_shape(Shape::new(ShapeEnum::Sphere(right_wall)));
    // w.add_x_axis();
    // w.add_y_axis();
    // w.add_z_axis();
    // w.add_shape(Shape::new(ShapeEnum::Sphere(middle)));
    // w.add_shape(Shape::new(ShapeEnum::Sphere(left)));
    // w.add_shape(Shape::new(ShapeEnum::Sphere(right)));

    let filename =
        "/Users/bumzack/stoff/rust/godot/raytracer_challenge_reference_impl/downloaded_obj_files/suzanne.obj";
    println!("filename {}", filename);

    let teapot = Parser::parse_obj_file(&filename);

    let teapot_group = teapot.get_groups("teapot".to_string(), w.get_shapes_mut());
    let idx = teapot_group.get(0).unwrap();
    let mut teapot = w.get_shapes_mut().get_mut(*idx as usize).unwrap();

   let trans = &Matrix::translation( 1.5, 1.0, 0.9) * &Matrix::rotate_y(-PI / 2.0);



     teapot.set_transformation(trans);

    println!("teapot_group index {}", teapot_group.get(0).unwrap());

    let mut c = Camera::new(width as usize, height as usize, pov);
    c.calc_pixel_size();
    c.set_antialiasing(anitaliasing);
    c.set_antialiasing_size(anitaliasing_size);

    c.set_transformation(Matrix::view_transform(
        //&Tuple4D::new_point(4.0, 4.0, -6.0),
        &Tuple4D::new_point(3.0, 4.5, -5.0),
        &Tuple4D::new_point(1.0, 2.1, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
