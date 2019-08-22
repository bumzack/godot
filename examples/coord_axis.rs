extern crate num_cpus;

use std::error::Error;
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use raytracer_challenge::basics::camera::{Camera, CameraOps};
use raytracer_challenge::basics::canvas::{Canvas, CanvasOps};
use raytracer_challenge::basics::color::{Color, ColorOps};
use raytracer_challenge::light::light::LightEnum;
use raytracer_challenge::light::pointlight::PointLight;
use raytracer_challenge::material::material::MaterialOps;
use raytracer_challenge::math::matrix::{Matrix, MatrixOps};
use raytracer_challenge::math::tuple4d::{Tuple, Tuple4D};
use raytracer_challenge::patterns::patterns::Pattern;
use raytracer_challenge::patterns::stripe_patterns::StripePattern;
use raytracer_challenge::shape::plane::Plane;
use raytracer_challenge::shape::shape::{Shape, ShapeEnum};
use raytracer_challenge::world::world::{World, WorldOps, MAX_REFLECTION_RECURSION_DEPTH};

fn main() -> Result<(), Box<dyn Error>> {
    let width = 1280;
    let height = 720;

    let width = 320;
    let height = 200;

    let (w, c) = setup_world(width, height);

    // multi core
    //    let start = Instant::now();
    //    let canvas = Camera::render_multi_core(&c, &w, 4);
    //    canvas.write_ppm("chapter12_multi_core.ppm")?;
    //    let dur = Instant::now() - start;
    //    println!("multi core duration: {:?}", dur);

    let multi_core = true;
    let single_core = false;

    if single_core {
        // single core
        let start = Instant::now();
        // let canvas = Camera::render_debug(&c, &w, 226, 241);
        let canvas = Camera::render(&c, &w);
        let dur = Instant::now() - start;
        println!("single core duration: {:?}", dur);
        canvas.write_ppm("coord_axis_single.ppm")?;
    }

    if multi_core {
        let start = Instant::now();

        let num_cores = num_cpus::get();
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
                        //let color = Color::new(1.0,1.0,1.0);
                        // TODO: wtf ?!
                        // if color.r != 0.0 || color.g != 0.0 || color.b != 0.0 {}
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

fn setup_world<'a>(width: usize, height: usize) -> (World<'a>, Camera) {
    let pl = PointLight::new(Tuple4D::new_point(-1.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = LightEnum::PointLight(pl);

    let mut w = World::new();
    w.set_light(l);
    w.add_floor();
    w.add_x_axis();

    let mut c = Camera::new(width, height, PI / 2.0);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -6.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
    ));
    (w, c)
}
