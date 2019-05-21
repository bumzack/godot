use std::fs::File;
use std::io::{Error, Write};

use crate::math::canvas::{Canvas, CanvasOps};
use crate::math::color::Color;
use crate::math::color::ColorOps;
use crate::math::commonshape::CommonShape;
use crate::math::intersection::{Intersection, IntersectionListOps, IntersectionOps};
use crate::math::light::Light;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::ray::Ray;
use crate::math::ray::RayOps;
use crate::math::sphere::{Sphere, SphereOps};
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;

pub struct Scene {
    width: usize,
    height: usize,
    shapes: Vec<CommonShape>,
    origin: Tuple4D,
    canvas:Canvas,
}

pub trait SceneOps<'a> {
    fn new(width: usize, height: usize) -> Scene;
    fn set_camera(&mut self, o: Tuple4D);
    fn add_light(&mut self, light: Light);
    fn write_ppm(&self, filename: &'a str) -> Result<(), Error>;
    fn render_scene(&mut self);

}

impl<'a> SceneOps<'a> for Scene {
    fn new(width: usize, height: usize) -> Scene {
       Scene {
           width,
           height,
           shapes: Vec::new(),
           origin: Tuple4D::new_point(0.0,0.0,-10.0),
           canvas: Canvas::new(width,height),
       }
    }

    fn set_camera(&mut self, o: Tuple4D) {
        unimplemented!()
    }

    fn add_light(&mut self, light: Light) {
        unimplemented!()
    }

    fn write_ppm(&self, filename: &'a str) -> Result<(), Error> {
        self.canvas.write_ppm(filename)
    }

    fn render_scene(&mut self) {
        let ray_origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let wall_z = 10.0;
        let wall_size = 7.0;
        let canvas_pixel = self.width;
        let pixel_size = wall_size / canvas_pixel as f32;
        let half = wall_size / 2.0;
//        let mut c = Canvas::new(canvas_pixel, canvas_pixel);
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

                let r = Ray::new(ray_origin.clone(), d);

                let xs = Intersection::intersect(&o, &r);
                let res = xs.hit();
                match res {
                    Some(_) => self.canvas.write_pixel(x, y, color.clone()),
                    None => {}
                }
            }
        }
    }
}





