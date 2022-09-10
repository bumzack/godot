extern crate num_cpus;

use std::collections::HashMap;
use std::error::Error;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 800;
    let height = 400;

    let (w, c) = setup_world(width, height);

    let start = Instant::now();

    let num_cores = num_cpus::get() + 1;
    println!("using {} cores", num_cores);
    let canvas = Canvas::new(c.get_hsize(), c.get_vsize());

    let data = Arc::new(Mutex::new(canvas));
    let mut children = vec![];
    let act_y: usize = 0;
    let act_y_mutex = Arc::new(Mutex::new(act_y));

    for _i in 0..num_cores {
        let cloned_data = Arc::clone(&data);
        let cloned_act_y = Arc::clone(&act_y_mutex);
        let height = c.get_vsize();
        let width = c.get_hsize();
        println!("camera height / width  {}/{}", height, width);

        let c_clone = c.clone();
        let w_clone = w.clone();

        children.push(thread::spawn(move || {
            let mut y: usize = 0;
            while *cloned_act_y.lock().unwrap() < height {
                if y < height {
                    let mut acty = cloned_act_y.lock().unwrap();
                    y = *acty;
                    *acty = *acty + 1;
                }
                for x in 0..width {
                    let r = Camera::ray_for_pixel(&c_clone, x, y);
                    let color = World::color_at(&w_clone, &r, MAX_REFLECTION_RECURSION_DEPTH);
                    let mut canvas = cloned_data.lock().unwrap();
                    canvas.write_pixel(x, y, color);
                }
            }
        }));
    }

    for child in children {
        let _ = child.join();
    }
    let dur = Instant::now() - start;
    println!("multi core duration: {:?}", dur);
    let c = data.lock().unwrap();
    c.write_png("./bonus_cube_mapping.png")?;
    println!("file exported");
    Ok(())
}

fn setup_world(width: usize, height: usize) -> (World, Camera) {
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

    let mut cube = Cube::new();
    cube.get_material_mut().set_pattern(p);
    cube.get_material_mut().set_ambient(0.2);
    cube.get_material_mut().set_specular(0.);
    cube.get_material_mut().set_diffuse(0.8);

    let pl = PointLight::new(Tuple4D::new_point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.set_light(l);

    let trans = Matrix::translation(-6.0, 2., 0.0);
    let rot_x = Matrix::rotate_x(0.7854);
    let rot_y = Matrix::rotate_y(0.7854);
    let transform = &(&rot_y * &rot_x) * &trans;
    cube.set_transformation(transform);
    w.add_shape(Shape::new(ShapeEnum::Cube(cube.clone())));

    let trans = Matrix::translation(-2.0, 2., 0.0);
    let rot_x = Matrix::rotate_x(0.7854);
    let rot_y = Matrix::rotate_y(2.356);
    let transform = &(&rot_y * &rot_x) * &trans;
    cube.set_transformation(transform);
    w.add_shape(Shape::new(ShapeEnum::Cube(cube.clone())));

    let trans = Matrix::translation(2.0, 2., 0.0);
    let rot_x = Matrix::rotate_x(0.7854);
    let rot_y = Matrix::rotate_y(3.927);
    let transform = &(&rot_y * &rot_x) * &trans;
    cube.set_transformation(transform);
    w.add_shape(Shape::new(ShapeEnum::Cube(cube.clone())));

    let trans = Matrix::translation(6.0, 2., 0.0);
    let rot_x = Matrix::rotate_x(0.7854);
    let rot_y = Matrix::rotate_y(5.4978);
    let transform = &(&rot_y * &rot_x) * &trans;
    cube.set_transformation(transform);
    w.add_shape(Shape::new(ShapeEnum::Cube(cube.clone())));

    let trans = Matrix::translation(-6.0, -2., 0.0);
    let rot_x = Matrix::rotate_x(-0.7854);
    let rot_y = Matrix::rotate_y(0.7854);
    let transform = &(&rot_y * &rot_x) * &trans;
    cube.set_transformation(transform);
    w.add_shape(Shape::new(ShapeEnum::Cube(cube.clone())));

    let trans = Matrix::translation(-2.0, -2., 0.0);
    let rot_x = Matrix::rotate_x(-0.7854);
    let rot_y = Matrix::rotate_y(2.3562);
    let transform = &(&rot_y * &rot_x) * &trans;
    cube.set_transformation(transform);
    w.add_shape(Shape::new(ShapeEnum::Cube(cube.clone())));

    let trans = Matrix::translation(2.0, -2., 0.0);
    let rot_x = Matrix::rotate_x(-0.7854);
    let rot_y = Matrix::rotate_y(3.927);
    let transform = &(&rot_y * &rot_x) * &trans;
    cube.set_transformation(transform);
    w.add_shape(Shape::new(ShapeEnum::Cube(cube.clone())));

    let trans = Matrix::translation(6.0, -2., 0.0);
    let rot_x = Matrix::rotate_x(-0.7854);
    let rot_y = Matrix::rotate_y(5.4978);
    let transform = &(&rot_y * &rot_x) * &trans;
    cube.set_transformation(transform);
    w.add_shape(Shape::new(ShapeEnum::Cube(cube)));

    let mut c = Camera::new(width, height, 0.8);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 0.0, -20.0),
        &Tuple4D::new_point(0.0, 0.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
