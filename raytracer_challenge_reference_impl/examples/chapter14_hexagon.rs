extern crate num_cpus;

use std::error::Error;
use std::f64::consts::PI;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    // let width = 2048;
    // let height = 2048;

    let width = 400;
    let height = 400;

    let (world, camera) = setup_world(width, height);

    for s in world.get_shapes() {
        println!("shape {}", s);
    }
    println!("####################");
    Group::print_tree(world.get_shapes(), 0, 0);

    let start = Instant::now();
    // let canvas = Camera::render_debug(&camera, &world, 100, 50 );
    let canvas = Camera::render_multi_core(&camera, &world);
    let dur = Instant::now() - start;

    println!("multi core duration: {:?}", dur);

    let aa = match camera.get_antialiasing() {
        true => format!("with_AA_{}", camera.get_antialiasing_size()),
        false => "no_AA".to_string(),
    };
    let filename = &format!(
        "./chapter14_hexagon_{}_{}_{}.png",
        camera.get_hsize(),
        camera.get_vsize(),
        aa
    );

    canvas.write_png(&filename)?;
    println!("written file {}", filename);

    Ok(())
}

fn hexagon_corner<'a>(idx: usize) -> Shape {
    let mut corner = Sphere::new();
    let trans = &Matrix::translation(0.0, 0.0, -1.0) * &Matrix::scale(0.25, 0.25, 0.25);
    corner.set_transformation(trans);
    // corner.get_material_mut().set_color(get_color(idx));
    corner.get_material_mut().set_color(Color::new(1.0, 0.2, 0.4));
    corner.get_material_mut().set_diffuse(0.8);
    corner.get_material_mut().set_specular(0.6);
    corner.get_material_mut().set_ambient(0.0);
    corner.get_material_mut().set_shininess(50.0);
    corner.get_material_mut().set_reflective(0.3);
    Shape::new_part_of_group(ShapeEnum::Sphere(corner), format!("sphere {}", idx).to_string())
}

fn hexagon_edge<'a>(idx: usize) -> Shape {
    let mut edge = Cylinder::new();
    edge.set_minimum(0.0);
    edge.set_maximum(1.0);
    let trans = &Matrix::translation(0.0, 0.0, -1.0) * &Matrix::rotate_y(-PI / 6.0);
    let trans = &trans * &Matrix::rotate_z(-PI / 2.0);
    let trans = &trans * &Matrix::scale(0.25, 1.0, 0.25);

    edge.set_transformation(trans);
    edge.get_material_mut().set_color(Color::new(1.0, 0.2, 0.4));
    edge.get_material_mut().set_diffuse(0.8);
    edge.get_material_mut().set_specular(0.6);
    edge.get_material_mut().set_ambient(0.0);
    edge.get_material_mut().set_shininess(50.0);
    edge.get_material_mut().set_reflective(0.3);

    //   edge.get_material_mut().set_color(get_color(idx));
    Shape::new_part_of_group(ShapeEnum::Cylinder(edge), format!("cylinder {}", idx).to_string())
}

fn hexagon_side(shapes: &mut ShapeArr, idx: usize) -> ShapeIdx {
    let side_idx = Group::new_part_of_group(shapes, format!("side {}", idx).to_string());
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

fn hexagon<'a>(shapes: &mut ShapeArr) -> ShapeIdx {
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

fn setup_world<'a>(width: usize, height: usize) -> (World, Camera) {
    let mut w = World::new();
    let _hexagon = hexagon(w.get_shapes_mut());
    // let hexagon  = w.get_shapes_mut().get_mut(_hexagon as usize).unwrap();
    //
    // // ~color:(color 1. 0.2 0.4) ~diffuse:0.8 ~specular:0.6 ~ambient:0. ~shininess:50. ~reflective:0.3 () in
    // hexagon.get_material_mut().set_color(Color::new(1.0, 0.2,0.4));
    // hexagon.get_material_mut().set_diffuse(0.8);
    // hexagon.get_material_mut().set_specular(0.6);
    // hexagon.get_material_mut().set_ambient(0.0);
    // hexagon.get_material_mut().set_shininess(50.0);
    // hexagon.get_material_mut().set_reflective(0.3);

    let pl = PointLight::new(Tuple4D::new_point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    w.add_light(l);

    let mut c = Camera::new(width, height, 0.5);
    c.set_antialiasing(true);
    c.set_antialiasing_size(3);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(1.0, 2.0, -5.0),
        &Tuple4D::new_point(0.0, 0.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
