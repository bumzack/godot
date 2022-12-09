use std::f64::consts::PI;

use crate::prelude::{
    AreaLight, Camera, CameraOps, Color, ColorOps, Light, MaterialOps, Matrix, MatrixOps, Sequence, Shape, ShapeOps,
    Sphere, Tuple, Tuple4D, World, WorldOps,
};

pub fn chapter14_with_aa(
    width: usize,
    height: usize,
    pov: f64,
    anitaliasing: bool,
    anitaliasing_size: usize,
    arealight_u: usize,
    arealight_v: usize,
) -> (World, Camera) {
    let mut floor = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    floor.set_transformation(Matrix::scale(20.0, 0.01, 20.0));
    floor.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    floor.get_material_mut().set_specular(0.0);

    let mut left_wall = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    left_wall.set_transformation(
        &(&(&Matrix::translation(0.0, 0.0, 6.0) * &Matrix::rotate_y(-PI / 4.0)) * &Matrix::rotate_x(PI / 2.0))
            * &Matrix::scale(10.0, 0.01, 10.),
    );
    left_wall.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    left_wall.get_material_mut().set_specular(0.0);

    let mut right_wall = Shape::new_sphere(Sphere::new(), "Sphere".to_string());
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

    // let pl = PointLight::new(Tuple4D::new_point(-151.0, 100.0, -100.0), Color::new(1.0, 1.0, 1.0));
    // let l = Light::PointLight(pl);

    // let corner = Tuple4D::new_point(4.5, 8.0, -9.0);
    // let uvec = Tuple4D::new_vector(2.0, 0.0, 0.0);
    // let vvec = Tuple4D::new_vector(0.0, 2.0, 0.0);

    let corner = Tuple4D::new_point(0.0, 4.0, -9.0);
    let uvec = Tuple4D::new_vector(2.0, 0.0, 0.0);
    let vvec = Tuple4D::new_vector(0.0, 2.0, -1.5);

    //  let usteps = 16;
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
    // w.add_light(    l);

    w.add_shape(floor);
    w.add_shape(left_wall);
    w.add_shape(right_wall);
    // w.add_shape(Shape::new(ShapeEnum::SphereEnum(middle)));
    // w.add_shape(Shape::new(ShapeEnum::SphereEnum(left)));
    // w.add_shape(Shape::new(ShapeEnum::SphereEnum(right)));

    let filename = "/Users/gsc/stoff/lernen/godot/raytracer_challenge_reference_impl/downloaded_obj_files/teapot01.obj";
    println!("filename {}", filename);

    // let teapot = Parser::parse_obj_file(filename);
    //
    // let teapot_group = teapot.get_groups("teapot".to_string(), w.get_shapes_mut());
    // let idx = teapot_group.get(0).unwrap();
    // let teapot = w.get_shapes_mut().get_mut(*idx as usize).unwrap();
    //
    // // let trans = &(&Matrix::rotate_y(-PI/8.0)* &Matrix::rotate_x(-PI/4.0)) * &Matrix::scale(0.4,0.4,0.34) ;
    // let trans = &Matrix::rotate_x(-PI / 4.0) * &Matrix::scale(0.4, 0.4, 0.4);
    // teapot.set_transformation(trans);
    //
    // println!("teapot_group index {}", teapot_group.get(0).unwrap());

    let mut c = Camera::new(width as usize, height as usize, pov);
    c.calc_pixel_size();
    c.set_antialiasing(anitaliasing);
    c.set_antialiasing_size(anitaliasing_size);

    c.set_transformation(Matrix::view_transform(
        //&Tuple4D::new_point(4.0, 4.0, -6.0),
        &Tuple4D::new_point(3.0, 4.5, -5.0),
        &Tuple4D::new_point(1.0, 1.2, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
