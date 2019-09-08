extern crate num_cpus;

use std::error::Error;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use raytracer_challenge_reference_impl::prelude::*;

mod coord_axes;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 1280;
    let height = 720;

    let width = 320;
    let height = 200;

    let (mut world, mut camera) = coord_axes::setup_world_coord_axes(width, height, false);
    coord_axes::add_floor(&mut world);
    coord_axes::add_borders(&mut world);


    let multi_core = true;
    let single_core = false;


    let mut z: f32 = -2.0;
    let amplitude = 0.8;
    let light_camera_distance_y = 15.0;
    // from the top -> 2D View in -y direction
    let mut camera_from = Tuple4D::new_point(3.0, 1., z - 5.0);
    let mut camera_to = Tuple4D::new_point(0.0, 0.0, 0.0);
    let mut camera_up = Tuple4D::new_point(0.0, 1.0, 1.0);
    camera.set_transformation(Matrix::view_transform(&camera_from, &camera_to, &camera_up));


    let mut light_pos = Tuple4D::from(camera_from);
    light_pos.y += light_camera_distance_y;
    light_pos.x = -light_pos.x+0.5;
    light_pos.z = -light_pos.z +0.5;

    let pl = PointLight::new(light_pos, Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);
    world.set_light(l);


    let filename = format!(
        "test_coord_axes_{}_{}.png",
        width, height
    );

    if single_core {
        // single core
        let start = Instant::now();
        // let canvas = Camera::render_debug(&c, &w, 226, 241);
        let canvas = Camera::render(&camera, &world);
        let dur = Instant::now() - start;
        println!("single core duration: {:?}", dur);
        canvas.write_ppm("coord_axis_single.ppm")?;
    }

    if multi_core {
        let start = Instant::now();

        let num_cores = num_cpus::get();
        println!("using {} cores", num_cores);
        let canvas = Canvas::new(camera.get_hsize(), camera.get_vsize());

        let data = Arc::new(Mutex::new(canvas));
        let mut children = vec![];
        let act_y: usize = 0;
        let act_y_mutex = Arc::new(Mutex::new(act_y));

        for _i in 0..num_cores {
            let cloned_data = Arc::clone(&data);
            let cloned_act_y = Arc::clone(&act_y_mutex);
            let height = camera.get_vsize();
            let width = camera.get_hsize();
            println!("camera height / width  {}/{}", height, width);

            let c_clone = camera.clone();
            let w_clone = world.clone();

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
        c.write_ppm("coord_axis_multi.ppm")?;
    }

    Ok(())
}
