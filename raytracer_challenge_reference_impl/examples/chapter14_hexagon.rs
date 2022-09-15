extern crate num_cpus;

use std::error::Error;
use std::f64::consts::PI;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 2048;
    let height = 2048;

    // let width = 320;
    // let height = 200;

    let (world, camera) = setup_world(width, height);
    let start = Instant::now();
    let canvas = Camera::render_multi_core(&camera, &world);
    let dur = Instant::now() - start;

    println!("multi core duration: {:?}", dur);

    let aa = match camera.get_antialiasing() {
        true => format!("with_AA_{}", camera.get_antialiasing_size()),
        false => "no_AA".to_string(),
    };
    let filename = &format!("./chapter14_hexagon_{}_{}_{}.png", camera.get_hsize(), camera.get_vsize(), aa);

    canvas.write_png(&filename)?;
    Ok(())
}

fn hexagon_corner(idx: usize) -> Shape {
    let mut corner = Sphere::new();
    let trans = &Matrix::translation(0.0, 0.0, -1.0) * &Matrix::scale(0.25, 0.25, 0.25);
    corner.set_transformation(trans);
    corner.get_material_mut().set_color(get_color(idx));
    Shape::new(ShapeEnum::Sphere(corner))
}

fn hexagon_edge(idx: usize) -> Shape {
    let mut edge = Cylinder::new();
    edge.set_minimum(0.0);
    edge.set_maximum(1.0);
    let trans = &Matrix::translation(0.0, 0.0, -1.0) * &Matrix::rotate_y(-PI / 6.0);
    let trans = &trans * &Matrix::rotate_z(-PI / 2.0);
    let trans = &trans * &Matrix::scale(0.25, 1.0, 0.25);

    edge.set_transformation(trans);
    edge.get_material_mut().set_color(get_color(idx));
    Shape::new(ShapeEnum::Cylinder(edge))
}

fn hexagon_side(shapes: &mut ShapeArr, idx: usize) -> ShapeIdx {
    let side_idx = Group::new(shapes);
    Group::add_child(shapes, side_idx, hexagon_corner(idx));
    Group::add_child(shapes, side_idx, hexagon_edge(idx));

    side_idx
}

fn get_color(idx: usize) -> Color {
    let red = Color::new(1.0, 0.0, 0.0);
    let yellow = Color::new(1.0, 1.0, 0.0);
    let brown = Color::new(1.0, 0.5, 0.0);
    let green = Color::new(0.0, 1.0, 0.0);
    let cyan = Color::new(0.0, 1.0, 1.0);
    let blue = Color::new(0.0, 0.0, 1.0);
    let purple = Color::new(1.0, 0.0, 1.0);
    let white = Color::new(1.0, 1.0, 1.0);

    match idx {
        0 => cyan,
        1 => red,
        2 => blue,
        3 => green,
        4 => yellow,
        5 => purple,
        _ => panic!("should never come here"),
    }
}

fn hexagon(shapes: &mut ShapeArr) -> ShapeIdx {
    let hexagon = Group::new(shapes);

    for i in 0..6 {
        let side = hexagon_side(shapes, i as usize);
        let mut side = shapes.get_mut(side).unwrap();
        let trans = Matrix::rotate_y(i as f64 * PI / 3.0);
        side.set_transformation(trans);
        println!("i = {}", i)
    }

    hexagon
}

fn setup_world(width: usize, height: usize) -> (World, Camera) {
    let mut w = World::new();
    let _hexagon = hexagon(w.get_shapes_mut());

    let pl = PointLight::new(Tuple4D::new_point(5.0, 8.0, -9.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    w.add_light(l);

    let mut c = Camera::new(width, height, 0.3);
    c.set_antialiasing(true);
    c.set_antialiasing_size(3);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(2.0, 4.0, -9.0),
        &Tuple4D::new_point(0.0, 0.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
