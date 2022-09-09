extern crate num_cpus;

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let size_factor = 2.0;

    let antialiasing = true;
    let antialiasing_size = 3;
    let filename;
    if antialiasing {
        filename = format!(
            "ref_impl_glamour_world_aliasing_size_{}_multi_core.png",
            antialiasing_size
        );
    } else {
        filename = format!("ref_impl_glamour_world_no_anti_noaliasing_multi_core.png",);
    }

    let (world, camera) = setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);

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
                        // println!("no AA    color at ({}/{}): {:?}", x, y, color);
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
    if camera.get_antialiasing() {
        println!(
            "multi core duration: {:?} with AA size = {}",
            dur,
            camera.get_antialiasing_size()
        );
    } else {
        println!("multi core duration: {:?}, no AA", dur);
    }
    let c = data.lock().unwrap();
    c.write_ppm(filename.as_str())?;

    Ok(())
}

fn main1() -> Result<(), Box<dyn Error>> {
    let size_factor = 1.0;

    let antialiasing = true;
    let antialiasing_size = 3;
    let filename;
    if antialiasing {
        filename = format!("ref_impl_glamour_world_aliasing_size_{}_debug.png", antialiasing_size);
    } else {
        filename = format!("ref_impl_glamour_world_no_anti_noaliasing_debug.png",);
    }

    let (world, camera) = setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);

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

    let start = Instant::now();

    let mut canvas = Canvas::new(camera.get_hsize(), camera.get_vsize());

    let height = camera.get_vsize();
    let width = camera.get_hsize();

    println!(
        "camera height / width  {}/{}     thread_id {:?}",
        height,
        width,
        thread::current().id()
    );

    let x = 159;
    let y = 61;

    let mut color = BLACK;
    // Accumulate light for N samples.
    println!("n_samples   {:?}\n\n\n", n_samples);
    let sample = 2;
    let delta_x = jitter_matrix[2 * sample] * camera.get_pixel_size();
    let delta_y = jitter_matrix[2 * sample + 1] * camera.get_pixel_size();
    let r = Camera::ray_for_pixel_anti_aliasing(&camera, x, y, delta_x, delta_y);
    color = color + World::color_at(&world, &r, MAX_REFLECTION_RECURSION_DEPTH);

    println!("with AA    color at ({}/{}): {:?}\n\n\n", x, y, color);
    color = color / n_samples as f32;
    println!("with AA    color at ({}/{}): {:?}\n\n\n", x, y, color);

    let r = Camera::ray_for_pixel(&camera, x, y);
    color = World::color_at(&world, &r, MAX_REFLECTION_RECURSION_DEPTH);
    println!("no AA    color at ({}/{}): {:?}\n\n\n", x, y, color);

    //    let x = 160;
    //    let y = 61;
    //    let mut color = BLACK;
    //
    //    let mut color = BLACK;
    //    // Accumulate light for N samples.
    //    for sample in 0..n_samples {
    //        let delta_x = jitter_matrix[2 * sample] * camera.get_pixel_size();
    //        let delta_y = jitter_matrix[2 * sample + 1] * camera.get_pixel_size();
    //        let r = Camera::ray_for_pixel_anti_aliasing(&camera, x, y, delta_x, delta_y);
    //        color = color + World::color_at(&world, &r, MAX_REFLECTION_RECURSION_DEPTH);
    //    }
    //    color = color / n_samples as f32;
    //    println!("with AA    color at ({}/{}): {:?}\n\n\n", x, y, color);
    //
    //    let r = Camera::ray_for_pixel(&camera, x, y);
    //    color = World::color_at(&world, &r, MAX_REFLECTION_RECURSION_DEPTH);
    //    println!("no AA    color at ({}/{}): {:?}\n\n\n", x, y, color);
    //
    //    let x = 161;
    //    let y = 61;
    //    let mut color = BLACK;
    //
    //    let mut color = BLACK;
    //    // Accumulate light for N samples.
    //    for sample in 0..n_samples {
    //        let delta_x = jitter_matrix[2 * sample] * camera.get_pixel_size();
    //        let delta_y = jitter_matrix[2 * sample + 1] * camera.get_pixel_size();
    //        let r = Camera::ray_for_pixel_anti_aliasing(&camera, x, y, delta_x, delta_y);
    //        color = color + World::color_at(&world, &r, MAX_REFLECTION_RECURSION_DEPTH);
    //    }
    //    color = color / n_samples as f32;
    //    println!("with AA    color at ({}/{}): {:?}\n\n\n", x, y, color);
    //
    //    let r = Camera::ray_for_pixel(&camera, x, y);
    //    color = World::color_at(&world, &r, MAX_REFLECTION_RECURSION_DEPTH);
    //    println!("no AA    color at ({}/{}): {:?}\n\n\n", x, y, color);
    //
    //    let x = 162;
    //    let y = 61;
    //    let mut color = BLACK;
    //
    //    let mut color = BLACK;
    //    // Accumulate light for N samples.
    //    for sample in 0..n_samples {
    //        let delta_x = jitter_matrix[2 * sample] * camera.get_pixel_size();
    //        let delta_y = jitter_matrix[2 * sample + 1] * camera.get_pixel_size();
    //        let r = Camera::ray_for_pixel_anti_aliasing(&camera, x, y, delta_x, delta_y);
    //        color = color + World::color_at(&world, &r, MAX_REFLECTION_RECURSION_DEPTH);
    //    }
    //    color = color / n_samples as f32;
    //    println!("with AA    color at ({}/{}): {:?}\n\n\n", x, y, color);
    //
    //    let r = Camera::ray_for_pixel(&camera, x, y);
    //    color = World::color_at(&world, &r, MAX_REFLECTION_RECURSION_DEPTH);
    //    println!("no AA    color at ({}/{}): {:?}\n\n\n", x, y, color);

    canvas.write_pixel(x, y, color);

    canvas.write_ppm(filename.as_str())?;

    Ok(())
}

fn setup_world_shadow_glamour<'a>(size_factor: f32, antialiasing: bool, antialiasing_size: usize) -> (World, Camera) {
    let width = (400 as f32 * size_factor) as usize;
    let height = (160 as f32 * size_factor) as usize;

    let pl = PointLight::new(Tuple4D::new_point(-1.0, 2.0, 4.0), Color::new(1.5, 1.5, 1.5));
    let l = Light::PointLight(pl);

    // ---- CUBE -------
    let mut c = Cube::new();
    c.get_material_mut().set_color(Color::new(1., 0.5, 0.2));
    c.get_material_mut().set_ambient(1.0);
    c.get_material_mut().set_diffuse(0.0);
    c.get_material_mut().set_specular(0.0);

    let m_trans = Matrix::translation(0.0, 3.0, 4.0);
    let m_scale = Matrix::scale(1.0, 1.0, 0.01);
    let m = &m_trans * &m_scale;

    c.set_transformation(m);
    let mut cube = Shape::new(ShapeEnum::Cube(c));
    cube.set_casts_shadow(false);

    // ---- PLANE -------
    let mut plane = Plane::new();
    plane.get_material_mut().set_color(Color::new(1., 1., 1.));
    plane.get_material_mut().set_ambient(0.025);
    plane.get_material_mut().set_diffuse(0.67);
    plane.get_material_mut().set_specular(0.0);

    let plane = Shape::new(ShapeEnum::Plane(plane));

    // ---- SPHERE 1 -------
    let mut sphere1 = Sphere::new();
    sphere1.get_material_mut().set_color(Color::new(1.0, 0., 0.));
    sphere1.get_material_mut().set_ambient(0.1);
    sphere1.get_material_mut().set_diffuse(0.6);
    sphere1.get_material_mut().set_specular(0.0);
    sphere1.get_material_mut().set_reflective(0.3);

    let m_trans = Matrix::translation(0.5, 0.5, 0.0);
    let m_scale = Matrix::scale(0.5, 0.5, 0.5);
    let m = &m_trans * &m_scale;

    sphere1.set_transformation(m);
    let sphere1 = Shape::new(ShapeEnum::Sphere(sphere1));

    // ---- SPHERE 2 -------
    let mut sphere2 = Sphere::new();
    sphere2.get_material_mut().set_color(Color::new(0.5, 0.5, 1.0));
    sphere2.get_material_mut().set_ambient(0.1);
    sphere2.get_material_mut().set_diffuse(0.6);
    sphere2.get_material_mut().set_specular(0.0);
    sphere2.get_material_mut().set_reflective(0.3);

    let m_trans = Matrix::translation(-0.25, 0.33, 0.0);
    let m_scale = Matrix::scale(0.33, 0.33, 0.33);
    let m = &m_trans * &m_scale;

    sphere2.set_transformation(m);
    let sphere2 = Shape::new(ShapeEnum::Sphere(sphere2));

    let mut w = World::new();
    w.set_light(l);
    w.add_shape(cube);
    w.add_shape(plane);
    w.add_shape(sphere1);
    w.add_shape(sphere2);

    let mut c = Camera::new(width, height, 0.78540);
    c.set_antialiasing(antialiasing);
    c.set_antialiasing_size(antialiasing_size);

    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(-3.0, 1., 2.5),
        &Tuple4D::new_point(0.0, 0.5, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
