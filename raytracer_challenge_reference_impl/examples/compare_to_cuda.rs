#![feature(stmt_expr_attributes)]

extern crate num_cpus;

use std::error::Error;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use raytracer_challenge_reference_impl::prelude::*;

fn main_debug() -> Result<(), Box<dyn Error>> {
    let width = 384;
    let height = 216;

    let filename = "compare_to_cuda_no_aa.ppm";

    let (world, camera) = setup_world(width, height);
    let start = Instant::now();
    let mut canvas = Canvas::new(camera.get_hsize(), camera.get_vsize());

    let n_samples = camera.get_antialiasing_size();
    let mut jitter_matrix = Vec::new();
    if n_samples == 2 {
        jitter_matrix = vec![
            -1.0 / 4.0,
            1.0 / 4.0,
            1.0 / 4.0,
            1.0 / 4.0,
            -1.0 / 4.0,
            -1.0 / 4.0,
            1.0 / 4.0,
            -3.0 / 4.0,
        ];
    }

    if n_samples == 3 {
        let two_over_six = 2.0 / 6.0;
        #[rustfmt::skip]
            jitter_matrix = vec![
            -two_over_six,
            two_over_six,
            0.0,
            two_over_six,
            two_over_six,
            two_over_six,
            -two_over_six,
            0.0,
            0.0,
            0.0,
            two_over_six,
            0.0,
            -two_over_six,
            -two_over_six,
            0.0,
            -two_over_six,
            two_over_six,
            -two_over_six,
        ];
    }

    let height = camera.get_vsize();
    let width = camera.get_hsize();

    let x = 235;
    let y = 70;

    let mut color = BLACK;
    if camera.get_antialiasing() {
        // Accumulate light for N samples.
        for sample in 0..n_samples {
            let delta_x = jitter_matrix[2 * sample] * camera.get_pixel_size();
            let delta_y = jitter_matrix[2 * sample + 1] * camera.get_pixel_size();
            let r = Camera::ray_for_pixel_anti_aliasing(&camera, x, y, delta_x, delta_y);
            color = color + World::color_at(&world, &r, MAX_REFLECTION_RECURSION_DEPTH);
        }
        color = color / n_samples as f32;
        println!("with AA    color at ({}/{}): {:?}\n\n\n", x, y, color);
    } else {
        let r = Camera::ray_for_pixel(&camera, x, y);
        color = World::color_at(&world, &r, MAX_REFLECTION_RECURSION_DEPTH);
        println!("no  AA    color at ({}/{}): {:?}\n\n\n", x, y, color);
    }

    let x = 236;
    let y = 70;

    let mut color = BLACK;
    if camera.get_antialiasing() {
        // Accumulate light for N samples.
        for sample in 0..n_samples {
            let delta_x = jitter_matrix[2 * sample] * camera.get_pixel_size();
            let delta_y = jitter_matrix[2 * sample + 1] * camera.get_pixel_size();
            let r = Camera::ray_for_pixel_anti_aliasing(&camera, x, y, delta_x, delta_y);
            color = color + World::color_at(&world, &r, MAX_REFLECTION_RECURSION_DEPTH);
        }
        color = color / n_samples as f32;
        println!("with AA    color at ({}/{}): {:?}\n\n\n", x, y, color);
    } else {
        let r = Camera::ray_for_pixel(&camera, x, y);
        color = World::color_at(&world, &r, MAX_REFLECTION_RECURSION_DEPTH);
        println!("no  AA    color at ({}/{}): {:?}\n\n\n", x, y, color);
    }
    canvas.write_pixel(x, y, color);
    canvas.write_ppm(filename)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let width = 384;
    let height = 216;

    let width = 600;
    let height = 500;

    let filename = "compare_to_cuda_no_aa.ppm";

    let (world, camera) = setup_world(width, height);

    let start = Instant::now();
    let num_cores = num_cpus::get() + 1;

    println!("using {} cores", num_cores);

    let canvas = Canvas::new(camera.get_hsize(), camera.get_vsize());
    let data = Arc::new(Mutex::new(canvas));

    let mut children = vec![];

    let act_y: usize = 0;
    let act_y_mutex = Arc::new(Mutex::new(act_y));

    for _i in 0..num_cores {
        let n_samples = camera.get_antialiasing_size();
        let mut jitter_matrix = Vec::new();
        if n_samples == 2 {
            jitter_matrix = vec![
                -1.0 / 4.0,
                1.0 / 4.0,
                1.0 / 4.0,
                1.0 / 4.0,
                -1.0 / 4.0,
                -1.0 / 4.0,
                1.0 / 4.0,
                -3.0 / 4.0,
            ];
        }

        if n_samples == 3 {
            let two_over_six = 2.0 / 6.0;
            #[rustfmt::skip]
                jitter_matrix = vec![
                -two_over_six,
                two_over_six,
                0.0,
                two_over_six,
                two_over_six,
                two_over_six,
                -two_over_six,
                0.0,
                0.0,
                0.0,
                two_over_six,
                0.0,
                -two_over_six,
                -two_over_six,
                0.0,
                -two_over_six,
                two_over_six,
                -two_over_six,
            ];
        }

        let cloned_data = Arc::clone(&data);
        let cloned_act_y = Arc::clone(&act_y_mutex);
        let height = camera.get_vsize();
        let width = camera.get_hsize();

        let c_clone = camera.clone();
        let w_clone = world.clone();

        children.push(thread::spawn(move || {
            let mut y: usize = 0;

            println!(
                "camera height / width  {}/{}     thread_id {:?}",
                height,
                width,
                thread::current().id()
            );

            while *cloned_act_y.lock().unwrap() < height {
                if y < height {
                    let mut acty = cloned_act_y.lock().unwrap();
                    y = *acty;
                    *acty = *acty + 1;
                }
                for x in 0..width {
                    let mut color = BLACK;
                    if c_clone.get_antialiasing() {
                        // Accumulate light for N samples.
                        for sample in 0..n_samples {
                            let delta_x = jitter_matrix[2 * sample] * c_clone.get_pixel_size();
                            let delta_y = jitter_matrix[2 * sample + 1] * c_clone.get_pixel_size();

                            let r = Camera::ray_for_pixel_anti_aliasing(&c_clone, x, y, delta_x, delta_y);

                            color = color + World::color_at(&w_clone, &r, MAX_REFLECTION_RECURSION_DEPTH);
                        }
                        color = color / n_samples as f32;
                    // println!("with AA    color at ({}/{}): {:?}", x, y, color);
                    } else {
                        let r = Camera::ray_for_pixel(&c_clone, x, y);
                        color = World::color_at(&w_clone, &r, MAX_REFLECTION_RECURSION_DEPTH);
                    }

                    let mut canvas = cloned_data.lock().unwrap();
                    canvas.write_pixel(x, y, color);
                }
            }
            thread::current().id()
        }));
    }
    for child in children {
        let dur = Instant::now() - start;
        println!("child finished {:?}   run for {:?}", child.join().unwrap(), dur);
    }
    let dur = Instant::now() - start;
    println!("multi core duration: {:?}  ", dur,);

    let c = data.lock().unwrap();
    c.write_ppm(filename)?;

    Ok(())
}

fn setup_world_chapter14<'a>(width: usize, height: usize) -> (World, Camera) {
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

    let mut stripe = StripePattern::new();
    stripe.set_color_a(Color::new(1.0, 0.0, 0.0));
    stripe.set_color_b(Color::new(0.1, 1.0, 0.0));
    stripe.set_transformation(Matrix::scale(0.6, 0.6, 0.6));
    let stripe = Pattern::StripePattern(stripe);

    let mut cube = Cube::new();
    let c_trans = Matrix::translation(2.5, 2.5, -1.0);
    let c_scale = Matrix::scale(0.5, 0.5, 0.25);
    cube.set_transformation(c_scale * c_trans);
    cube.get_material_mut().set_pattern(stripe);
    cube.get_material_mut().set_transparency(0.5);

    let mut checker = Checker3DPattern::new();
    checker.set_color_a(Color::new(1.0, 0.0, 0.0));
    checker.set_color_b(Color::new(0.7, 0.0, 0.0));
    let p = Pattern::Checker3DPattern(checker);

    let mut cylinder = Cylinder::new();
    let c_trans = Matrix::translation(1.5, 1.0, -0.75);
    let c_scale = Matrix::scale(0.5, 0.5, 0.25);
    cylinder.set_transformation(c_trans * c_scale);
    cylinder.get_material_mut().set_pattern(p);
    cylinder.get_material_mut().set_transparency(0.0);
    cylinder.set_minimum(0.0);
    cylinder.set_minimum(1.0);

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

    let mut c = Camera::new(width, height, PI / 4.0);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -6.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
    ));
    (w, c)
}

fn setup_world(w: usize, h: usize) -> (World, Camera) {
    let mut floor = Sphere::new();
    floor.set_transformation(Matrix::scale(10.0, 0.01, 10.0));
    floor.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    floor.get_material_mut().set_specular(0.0);

    let mut left_wall = Sphere::new();
    left_wall.set_transformation(
        &(&(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(-PI / 4.0)) * &Matrix::rotate_x(PI / 2.0))
            * &Matrix::scale(10.0, 0.01, 10.),
    );
    left_wall.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    left_wall.get_material_mut().set_specular(0.0);

    let mut right_wall = Sphere::new();
    right_wall.set_transformation(
        &(&(&Matrix::translation(0.0, 0.0, 5.0) * &Matrix::rotate_y(PI / 4.0)) * &Matrix::rotate_x(PI / 2.0))
            * &Matrix::scale(10.0, 0.01, 10.0),
    );
    right_wall.get_material_mut().set_color(Color::new(1.0, 0.9, 0.9));
    right_wall.get_material_mut().set_specular(0.0);

    let mut middle = Sphere::new();
    middle.set_transformation(Matrix::translation(-0.5, 1.0, 0.5));
    middle.get_material_mut().set_color(Color::new(0.1, 1.0, 0.5));
    middle.get_material_mut().set_diffuse(0.7);
    middle.get_material_mut().set_specular(0.3);

    let mut right = Sphere::new();
    right.set_transformation(&Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scale(0.5, 0.5, 0.5));
    right.get_material_mut().set_color(Color::new(0.5, 1.0, 0.1));
    right.get_material_mut().set_diffuse(0.7);
    right.get_material_mut().set_specular(0.3);

    let mut left = Sphere::new();
    left.set_transformation(&Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scale(0.333, 0.333, 0.333));
    left.get_material_mut().set_color(Color::new(1.0, 0.8, 0.1));
    left.get_material_mut().set_diffuse(0.7);
    left.get_material_mut().set_specular(0.3);

    let pl = PointLight::new(Tuple4D::new_point(-1.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut world = World::new();
    world.set_light(l);
    world.add_shape(Shape::new(ShapeEnum::Sphere(floor)));
    world.add_shape(Shape::new(ShapeEnum::Sphere(left_wall)));
    world.add_shape(Shape::new(ShapeEnum::Sphere(right_wall)));
    world.add_shape(Shape::new(ShapeEnum::Sphere(middle)));
    world.add_shape(Shape::new(ShapeEnum::Sphere(left)));
    world.add_shape(Shape::new(ShapeEnum::Sphere(right)));

    let mut c = Camera::new(w, h, PI / 3.0);
    c.calc_pixel_size();
    c.set_antialiasing(false);
    c.set_antialiasing_size(3);

    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -5.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
    ));

    (world, c)
}
