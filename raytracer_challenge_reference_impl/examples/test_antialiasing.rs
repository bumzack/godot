#![feature(stmt_expr_attributes)]

extern crate num_cpus;

use std::error::Error;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use raytracer_challenge_reference_impl::basics::camera::{Camera, CameraOps};
use raytracer_challenge_reference_impl::basics::canvas::{Canvas, CanvasOps};
use raytracer_challenge_reference_impl::basics::color::{Color, ColorOps, BLACK};
use raytracer_challenge_reference_impl::light::light::LightEnum;
use raytracer_challenge_reference_impl::light::pointlight::PointLight;
use raytracer_challenge_reference_impl::math::matrix::{Matrix, MatrixOps};
use raytracer_challenge_reference_impl::math::tuple4d::{Tuple, Tuple4D};
use raytracer_challenge_reference_impl::world::world::{World, WorldOps, MAX_REFLECTION_RECURSION_DEPTH};

fn main() -> Result<(), Box<dyn Error>> {
    let width = 3840;
    let height = 2160;

    let width = 800;
    let height = 600;

    let antialiasing = true;
    let antialiasing_size = 3;
    let filename;
    if antialiasing {
        filename = format!(
            "test_with_anti_aliasing_size_{}_wxh_{}x{}_multi_core.ppm",
            antialiasing_size, width, height
        );
    } else {
        filename = format!("test_no_anti_noaliasing_wxh_{}x{}_multi_core.ppm", width, height);
    }

    let (world, camera) = setup_world(width, height, antialiasing, antialiasing_size);

    let start = Instant::now();
    let num_cores = num_cpus::get();

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

fn setup_world<'a>(width: usize, height: usize, antialiasing: bool, antialiasing_size: usize) -> (World<'a>, Camera) {
    let pl = PointLight::new(Tuple4D::new_point(-1.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = LightEnum::PointLight(pl);

    let mut w = World::new();
    w.set_light(l);
    w.add_floor();
    w.add_x_axis();

    let mut c = Camera::new(width, height, PI / 2.0);
    c.set_antialiasing(antialiasing);
    c.set_antialiasing_size(antialiasing_size);

    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -6.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
    ));
    (w, c)
}
