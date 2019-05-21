use crate::math::canvas::Canvas;
use crate::math::canvas::CanvasOps;
use crate::math::color::Color;
use crate::math::color::ColorOps;
use crate::math::common::float_equal;
use crate::math::commonshape::CommonShape;
use crate::math::intersection::{Intersection, IntersectionListOps};
use crate::math::intersection::IntersectionOps;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::ray::Ray;
use crate::math::ray::RayOps;
use crate::math::sphere::Sphere;
use crate::math::sphere::SphereOps;
use crate::math::tuple4d::{Tuple, Tuple4D};

mod math;

fn main() {
    let ray_origin = Tuple4D::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixel = 500;
    let pixel_size = wall_size / canvas_pixel as f32;
    let half = wall_size / 2.0;
    let mut c = Canvas::new(canvas_pixel, canvas_pixel);
    let color = Color::new(1.0, 0.0, 0.0);

    let mut s = Sphere::new();
    s.set_transformation(Matrix::scale(0.75,0.5,0.9));
    let o = CommonShape::Sphere(s);

    for y in 0..canvas_pixel {
        let world_y = half - pixel_size * y as f32;

        for x in 0..canvas_pixel {
            let world_x = -half + pixel_size * x as f32;
            let position = Tuple4D::new_point(world_x, world_y, wall_z);

            // TODO: clone here ... :-(
            let d = Tuple4D::normalize(&(position - ray_origin.clone()));
            if x == 49 && y == 49 {
                println!("d = {:#?}", d);
            }
            if x == 48 && y == 48 {
                println!("d = {:#?}", d);
            }
            if x == 50 && y == 50 {
                println!("d = {:#?}", d);
            }
            let r = Ray::new(ray_origin.clone(), d);
            if x == 50 && y == 50 {
                println!("r.origin = {:#?}", r.origin);
                println!("r.direction = {:#?}", r.direction);
            }


            let xs = Intersection::intersect(&o, &r);
            let res = xs.hit();
            match res {
                Some(_) => {
                    println!("got a match: at x={}, y= {}", x, y);
                    c.write_pixel(x, y, color.clone());
                }
                None => {}
            }
        }
    }
    c.write_ppm("red_sphere.ppm");
}
