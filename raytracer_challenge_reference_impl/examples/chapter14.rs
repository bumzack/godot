extern crate num_cpus;

use raytracer_challenge_reference_impl::prelude::*;
use std::error::Error;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 640;
    let height = 480;

    let (w, c) = setup_world(width, height);

    let start = Instant::now();

    let num_cores = num_cpus::get() + 1;
    println!("using {} cores", num_cores);
    let mut canvas = Canvas::new(c.get_hsize(), c.get_vsize());

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
    c.write_ppm("chapter14_multi_core_no_AA.ppm")?;

    Ok(())
}

fn setup_world(width: usize, height: usize) -> (World, Camera) {
    let mut floor = Plane::new();
    let mut p: GradientPattern = GradientPattern::new();
    p.set_color_a(Color::new(1.0, 0.0, 0.0));
    p.set_color_a(Color::new(1.0, 0.0, 1.0));
    let m = Matrix::rotate_y(PI / 4.0);
    p.set_transformation(m);
    let p = Pattern::GradientPattern(p);
    floor.get_material_mut().set_pattern(p);
    let mut p = RingPattern::new();
    p.set_color_a(Color::new(0.5, 0.0, 0.0));
    p.set_color_a(Color::new(0.5, 0.0, 0.8));
    let m = Matrix::rotate_x(PI / 4.0);
    p.set_transformation(m);
    let p = Pattern::RingPattern(p);
    let mut left_wall = Plane::new();
    left_wall.set_transformation(
        &(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(-PI / 4.0)) * &Matrix::rotate_x(PI / 2.0),
    );
    left_wall.get_material_mut().set_pattern(p);
    let mut checker_3d = Checker3DPattern::new();
    checker_3d.set_color_a(Color::new(0.1, 0.8, 0.4));
    checker_3d.set_color_a(Color::new(0.8, 0.2, 0.2));
    let p = Pattern::Checker3DPattern(checker_3d);
    let mut right_wall = Plane::new();
    right_wall.set_transformation(
        &(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(PI / 4.0)) * &Matrix::rotate_x(PI / 2.0),
    );
    right_wall.get_material_mut().set_pattern(p);
    let mut middle = Sphere::new();
    middle.set_transformation(Matrix::translation(-0.5, 1.0, 0.5));
    middle.get_material_mut().set_color(Color::new(0.1, 1.0, 0.5));
    middle.get_material_mut().set_diffuse(0.7);
    middle.get_material_mut().set_specular(0.3);
    middle.get_material_mut().set_reflective(1.3);
    middle.get_material_mut().set_refractive_index(1.3);
    let mut right = Sphere::new();
    right.set_transformation(&Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scale(0.5, 0.5, 0.5));
    right.get_material_mut().set_color(Color::new(0.5, 1.0, 0.1));
    right.get_material_mut().set_diffuse(0.7);
    right.get_material_mut().set_specular(0.3);
    middle.get_material_mut().set_reflective(1.8);
    middle.get_material_mut().set_refractive_index(1.8);

    let mut left = Sphere::new();
    left.set_transformation(&Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scale(0.333, 0.333, 0.333));
    left.get_material_mut().set_color(Color::new(1., 0., 0.));
    left.get_material_mut().set_diffuse(0.7);
    left.get_material_mut().set_specular(0.3);

    let mut checker_3d = Checker3DPattern::new();
    checker_3d.set_color_a(Color::new(1.0, 0.0, 1.0));
    checker_3d.set_color_a(Color::new(0.1, 0.1, 1.0));
    let p = Pattern::Checker3DPattern(checker_3d);

    let mut cube = Cube::new();
    let c_trans = Matrix::translation(-2.0, 2.0, -1.75);
    let c_scale = Matrix::scale(0.5, 0.5, 0.25);
    let c_rot = Matrix::rotate_y(PI/5.0);
    let c_trans2 = Matrix::translation(-2.0, 2.0, -1.75);
    let m = c_scale * c_trans;
    let m = c_rot * m;
    let m = c_trans2 * m;
    cube.set_transformation(m);
    cube.get_material_mut().set_pattern(p);
    cube.get_material_mut().set_transparency(1.5);

    let mut checker = Checker3DPattern::new();
    checker.set_color_a(Color::new(1.0, 0.0, 0.0));
    checker.set_color_b(Color::new(0.0, 1.0, 1.0));
    let p = Pattern::Checker3DPattern(checker);

    let mut cylinder = Cylinder::new();
    let c_trans = Matrix::translation(1.5, 1.0, -0.75);
    // let c_scale = Matrix::scale(2.0, 0.5, 0.25);
    cylinder.set_transformation(c_trans);
    cylinder.get_material_mut().set_pattern(p);
    cylinder.get_material_mut().set_transparency(1.5);
    cylinder.set_minimum(1.0);
    cylinder.set_minimum(2.0);

    let pl = PointLight::new(Tuple4D::new_point(-1.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.set_light(l);
    w.add_shape(Shape::new(ShapeEnum::Plane(floor)));
    w.add_shape(Shape::new(ShapeEnum::Plane(left_wall)));
    w.add_shape(Shape::new(ShapeEnum::Plane(right_wall)));
    w.add_shape(Shape::new(ShapeEnum::Sphere(middle)));
    w.add_shape(Shape::new(ShapeEnum::Sphere(left)));
    w.add_shape(Shape::new(ShapeEnum::Sphere(right)));
    w.add_shape(Shape::new(ShapeEnum::Cube(cube)));
    w.add_shape(Shape::new(ShapeEnum::Cylinder(cylinder)));

    let mut c = Camera::new(width, height, PI / 2.0);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -6.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
