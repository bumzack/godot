extern crate num_cpus;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 800;
    let height = 400;

    let (w, c) = setup_world(width, height);

    let start = Instant::now();
    let canvas = Camera::render_multi_core(&c, &w);
    let dur = Instant::now() - start;
    println!("multi core duration: {:?}", dur);
    canvas.write_png("./bonus_cube_mapping.png")?;
    println!("file exported");
    Ok(())
}

fn setup_world(width: usize, height: usize) -> (World, Camera) {
    let pl3 = PointLight::new(Tuple4D::new_point(100.0, 0.0, -100.0), Color::new(0.25, 0.25, 0.25));
    let l3 = Light::PointLight(pl3);

    let pl2 = PointLight::new(Tuple4D::new_point(-100.0, 0.0, -100.0), Color::new(0.25, 0.25, 0.25));
    let l2 = Light::PointLight(pl2);

    let pl = PointLight::new(Tuple4D::new_point(0.0, 100.0, -100.0), Color::new(0.25, 0.25, 0.25));
    let l = Light::PointLight(pl);

    let pl1 = PointLight::new(Tuple4D::new_point(0.0, -100.0, -100.0), Color::new(0.25, 0.25, 0.25));
    let l1 = Light::PointLight(pl1);

    let mut w = World::new();
    w.add_light(l);
    w.add_light(l1);
    w.add_light(l2);
    w.add_light(l3);

    let trans = Matrix::translation(-6.0, 2., 0.0);
    let rot_x = Matrix::rotate_x(0.7854);
    let rot_y = Matrix::rotate_y(0.7854);
    let transform = &(&rot_y * &rot_x) * &trans;

    let mut cube1 = cube();
    cube1.set_transformation(transform);
    w.add_shape(cube1);

    let trans = Matrix::translation(-2.0, 2., 0.0);
    let rot_x = Matrix::rotate_x(0.7854);
    let rot_y = Matrix::rotate_y(2.356);
    let transform = &(&rot_y * &rot_x) * &trans;
    let mut cube2 = cube();
    cube2.set_transformation(transform);
    w.add_shape(cube2);

    let trans = Matrix::translation(2.0, 2., 0.0);
    let rot_x = Matrix::rotate_x(0.7854);
    let rot_y = Matrix::rotate_y(3.927);
    let transform = &(&rot_y * &rot_x) * &trans;
    let mut cube3 = cube();
    cube3.set_transformation(transform);
    w.add_shape(cube3);

    let trans = Matrix::translation(6.0, 2., 0.0);
    let rot_x = Matrix::rotate_x(0.7854);
    let rot_y = Matrix::rotate_y(5.4978);
    let transform = &(&rot_y * &rot_x) * &trans;
    let mut cube4 = cube();
    cube4.set_transformation(transform);
    w.add_shape(cube4);

    // let trans = Matrix::translation(-6.0, -2., 0.0);
    // let rot_x = Matrix::rotate_x(-0.7854);
    // let rot_y = Matrix::rotate_y(0.7854);
    // let transform = &(&rot_y * &rot_x) * &trans;
    // let mut cube5 = cube( );
    // cube5.set_transformation(transform);
    // w.add_shape(Shape::new(ShapeEnum::CubeEnum(cube5)));
    //
    // let trans = Matrix::translation(-2.0, -2., 0.0);
    // let rot_x = Matrix::rotate_x(-0.7854);
    // let rot_y = Matrix::rotate_y(2.3562);
    // let transform = &(&rot_y * &rot_x) * &trans;
    // let mut cube6 = cube( );
    // cube6.set_transformation(transform);
    // w.add_shape(Shape::new(ShapeEnum::CubeEnum(cube6)));
    //
    // let trans = Matrix::translation(2.0, -2., 0.0);
    // let rot_x = Matrix::rotate_x(-0.7854);
    // let rot_y = Matrix::rotate_y(3.927);
    // let transform = &(&rot_y * &rot_x) * &trans;
    // let mut cube7 = cube( );
    // cube7.set_transformation(transform);
    // w.add_shape(Shape::new(ShapeEnum::CubeEnum(cube7)));
    //
    // let trans = Matrix::translation(6.0, -2., 0.0);
    // let rot_x = Matrix::rotate_x(-0.7854);
    // let rot_y = Matrix::rotate_y(5.4978);
    // let transform = &(&rot_y * &rot_x) * &trans;
    // let mut cube8 = cube( );
    // cube8.set_transformation(transform);
    // w.add_shape(Shape::new(ShapeEnum::CubeEnum(cube8)));

    let mut c = Camera::new(width, height, 0.8);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, -5.0, -20.0),
        &Tuple4D::new_point(0.0, 0.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}

fn cube() -> Shape {
    let red = Color::new(1.0, 0.0, 0.0);
    let yellow = Color::new(1.0, 1.0, 0.0);
    let brown = Color::new(1.0, 0.5, 0.0);
    let green = Color::new(0.0, 1.0, 0.0);
    let cyan = Color::new(0.0, 1.0, 1.0);
    let blue = Color::new(0.0, 0.0, 1.0);
    let purple = Color::new(1.0, 0.0, 1.0);
    let white = Color::new(1.0, 1.0, 1.0);

    let left = CubeChecker::new(yellow, cyan, red, blue, brown);
    let front = CubeChecker::new(cyan, red, yellow, brown, green);
    let right = CubeChecker::new(red, yellow, purple, green, white);
    let back = CubeChecker::new(green, purple, cyan, white, blue);
    let up = CubeChecker::new(brown, cyan, purple, red, yellow);
    let down = CubeChecker::new(purple, brown, green, blue, white);

    let mut cube_map: HashMap<CubeFace, CubeChecker> = HashMap::new();
    cube_map.insert(CubeFace::LEFT, left);
    cube_map.insert(CubeFace::RIGHT, right);
    cube_map.insert(CubeFace::UP, up);
    cube_map.insert(CubeFace::DOWN, down);
    cube_map.insert(CubeFace::FRONT, front);
    cube_map.insert(CubeFace::BACK, back);

    let cube_checker = CubeTexturePattern::new(cube_map);

    let p = Pattern::CubeTextPattern(cube_checker);

    let mut cube = Shape::new(ShapeEnum::CubeEnum(Cube::new()));
    cube.get_material_mut().set_pattern(p);
    cube.get_material_mut().set_ambient(0.2);
    cube.get_material_mut().set_specular(0.0);
    cube.get_material_mut().set_diffuse(0.8);
    cube
}
