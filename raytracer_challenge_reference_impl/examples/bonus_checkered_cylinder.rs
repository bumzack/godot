extern crate num_cpus;

use std::error::Error;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 400;
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
    c.write_png("./checker_cylinder_bonus.png")?;
    println!("file exported");
    Ok(())
}

fn setup_world(width: usize, height: usize) -> (World, Camera) {
    let checker = uv_checkers(16, 8, Color::new(0.0, 0.5, 0.0), Color::new(1.0, 1.0, 1.0));
    let checker_3d = CylinderTexturePattern::new(checker);
    let p = Pattern::CylinderTexturePattern(checker_3d);

    let trans = Matrix::translation(0.0, -0.5, 0.0);
    let scale = Matrix::scale(1.0, PI, 1.0);
    let p_transformed = &scale * &trans;

    let mut cylinder = Cylinder::new();
    cylinder.set_transformation(p_transformed);
    cylinder.set_minimum(0.0);
    cylinder.set_maximum(1.0);
    cylinder.get_material_mut().set_pattern(p);
    cylinder.get_material_mut().set_ambient(0.1);
    cylinder.get_material_mut().set_specular(0.4);
    cylinder.get_material_mut().set_diffuse(0.8);
    cylinder.get_material_mut().set_shininess(15.0);

    let pl = PointLight::new(Tuple4D::new_point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.set_light(l);
    w.add_shape(Shape::new(ShapeEnum::Cylinder(cylinder)));

    let mut c = Camera::new(width, height, 0.5);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 0.0, -10.0),
        &Tuple4D::new_point(0.0, 0.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
