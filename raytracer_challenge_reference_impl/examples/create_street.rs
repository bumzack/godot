use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use raytracer_challenge_reference_impl::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 640;
    let height = 480;
    let filename;
    filename = "create_street_multi_core.ppm";

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
                    let r = Camera::ray_for_pixel(&c_clone, x, y);
                    let color = World::color_at(&w_clone, &r, MAX_REFLECTION_RECURSION_DEPTH);
                    // println!("no AA    color at ({}/{}): {:?}", x, y, color);

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
    c.write_ppm(filename)?;

    Ok(())
}

fn setup_world(width: usize, height: usize) -> (World, Camera) {
    let mut x_axis = Cylinder::new();
    x_axis.set_minimum(0.0);
    x_axis.set_maximum(1.0);
    x_axis.set_closed(true);
    x_axis.get_material_mut().set_color(Color::new(1.0, 0.0, 0.0));
    x_axis.get_material_mut().set_ambient(0.3);
    x_axis.get_material_mut().set_diffuse(0.6);
    x_axis.get_material_mut().set_specular(0.0);
    x_axis.get_material_mut().set_reflective(0.1);
    let x_axis = Shape::new(ShapeEnum::Cylinder(x_axis));

    let mut floor = Cube::new();
    floor.get_material_mut().set_color(Color::new(1.0, 1.0, 1.0));
    floor.get_material_mut().set_ambient(0.3);
    floor.get_material_mut().set_diffuse(0.6);
    floor.get_material_mut().set_specular(0.0);
    floor.get_material_mut().set_reflective(0.1);

    let m_scale = Matrix::scale(2.0, 0.01, 4.0);
    floor.set_transformation(m_scale);

    let floor = Shape::new(ShapeEnum::Cube(floor));

    let mut world = World::new();
    world.add_shape(x_axis);
    world.add_shape(floor);

    let z = -2.0;
    let camera_from = Tuple4D::new_point(3.0, 4., z - 6.0);
    let camera_to = Tuple4D::new_point(0.0, 0.0, z);
    let camera_up = Tuple4D::new_point(0.0, 1.0, 0.0);

    let mut camera = Camera::new(width, height, 0.50);
    camera.set_antialiasing(false);

    camera.set_transformation(Matrix::view_transform(&camera_from, &camera_to, &camera_up));

    let light_camera_distance_y = 90.0;

    let mut light_pos = Tuple4D::from(camera_from);
    light_pos.y += light_camera_distance_y;
    let pl = PointLight::new(light_pos, Color::new(1.5, 1.5, 1.5));
    let l = Light::PointLight(pl);
    world.set_light(l);

    (world, camera)
}
