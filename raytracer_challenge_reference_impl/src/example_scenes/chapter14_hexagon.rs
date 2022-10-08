extern crate num_cpus;

use crate::prelude::{
    Camera, CameraOps, Color, ColorOps, Cylinder, Group, Light, MaterialOps, Matrix, MatrixOps, PointLight, Shape,
    ShapeArr, ShapeIdx, ShapeOps, Sphere, Tuple, Tuple4D, World, WorldOps, BLUE, CYAN, GREEN, PURPLE, RED, YELLOW,
};
use std::error::Error;
use std::f64::consts::PI;

fn hexagon_corner(idx: usize) -> Shape {
    let mut corner = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    let trans = &Matrix::translation(0.0, 0.0, -1.0) * &Matrix::scale(0.25, 0.25, 0.25);
    corner.set_transformation(trans);
    corner.get_material_mut().set_color(get_color(idx));
    corner.set_part_of_group(true);
    corner
}

fn hexagon_edge(idx: usize) -> Shape {
    let mut edge = Cylinder::new();
    edge.set_minimum(0.0);
    edge.set_maximum(1.0);
    let mut edge = Shape::new_cylinder(edge, "cylinder".to_string());
    let trans = &Matrix::translation(0.0, 0.0, -1.0) * &Matrix::rotate_y(-PI / 6.0);
    let trans = &trans * &Matrix::rotate_z(-PI / 2.0);
    let trans = &trans * &Matrix::scale(0.25, 1.0, 0.25);

    edge.set_transformation(trans);
    edge.get_material_mut().set_color(get_color(idx));
    edge.set_part_of_group(true);
    edge
}

fn hexagon_side(shapes: &mut ShapeArr, idx: usize) -> ShapeIdx {
    let side_idx = Group::new_part_of_group(shapes);
    Group::add_child(shapes, side_idx, hexagon_corner(idx));
    Group::add_child(shapes, side_idx, hexagon_edge(idx));

    side_idx
}

fn get_color(idx: usize) -> Color {
    match idx {
        0 => CYAN,
        1 => RED,
        2 => BLUE,
        3 => GREEN,
        4 => YELLOW,
        5 => PURPLE,
        _ => panic!("should never come here"),
    }
}

fn hexagon(shapes: &mut ShapeArr) -> ShapeIdx {
    let hexagon = Group::new(shapes, "hexagon".to_string());

    for i in 0..5 {
        let side_idx = hexagon_side(shapes, i as usize);
        let side = shapes.get_mut(side_idx).unwrap();
        let trans = Matrix::rotate_y(i as f64 * PI / 6.0);
        side.set_transformation(trans);
        println!("i = {}", i);
        Group::add_child_idx(shapes, hexagon, side_idx);
    }

    hexagon
}

pub fn chapter14_hexagon(width: usize, height: usize) -> (World, Camera) {
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
