use std::io::Error;

use crate::math::canvas::{Canvas, CanvasOps};
use crate::math::color::Color;
use crate::math::color::ColorOps;
use crate::math::intersection::{Intersection, IntersectionListOps, IntersectionOps};
use crate::math::light::Light;
use crate::math::material::{Material, MaterialOps};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::pointlight::LightOps;
use crate::math::pointlight::PointLight;
use crate::math::ray::Ray;
use crate::math::ray::RayOps;
use crate::math::shape::Shape;
use crate::math::sphere::{Sphere, SphereOps};
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;

pub struct World {
    width: usize,
    height: usize,
    shapes: Vec<Shape>,
    origin: Tuple4D,
    canvas: Canvas,
    lights: Vec<Light>,
}

pub trait WorldOps<'a> {
    fn new(width: usize, height: usize) -> World;
    fn set_camera(&mut self, o: Tuple4D);
    fn add_light(&mut self, light: Light);
    fn add_shape(&mut self, shape: Shape);
    fn write_ppm(&self, filename: &'a str) -> Result<(), Error>;
    fn render_scene(&mut self);
    fn get_shapes(&self)  -> &Vec<Shape>;
}

impl<'a> WorldOps<'a> for World {
    fn new(width: usize, height: usize) -> World {
        World {
            width,
            height,
            shapes: Vec::new(),
            origin: Tuple4D::new_point(0.0, 0.0, -10.0),
            canvas: Canvas::new(width, height),
            lights: Vec::new(),
        }
    }

    fn set_camera(&mut self, o: Tuple4D) {
        unimplemented!()
    }

    fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    fn get_shapes(&self)  -> &Vec<Shape> {
        &self.shapes
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
        let mut m = Material::new();
        let c = Color::new(1.0, 0.2, 1.0);
        m.set_color(c);

        s.set_material(m);
        s.set_transformation(Matrix::scale(0.75, 0.5, 0.9));
        let o = Shape::Sphere(s);

        let light_color = Color::new(1.0, 1.0, 1.0);
        let light_psoition = Tuple4D::new_point(10.0, 10.0, -10.0);
        let l = PointLight::new(light_psoition, light_color);

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
                    Some(i) => {
                        let p = Ray::position(&r, i.get_t());
                        let shape = match o {
                            Shape::Sphere(ref s) => s,
                        };
                        let normal = shape.normal_at(&p);
                        let color = Material::lighting(&shape.get_material(), &l, &p, &ray_origin, &normal);
                        self.canvas.write_pixel(x, y, color);
                    }
                    None => {}
                }
            }
        }
    }
}


fn default_world() -> World {
    let mut w = World::new(400, 400);

    let light_pos = Tuple4D::new_point(-10.0, 10., -10.0);
    let light_intensity = Color::new(1.0, 1.0, 1.0);
    let pl = PointLight::new(light_pos, light_intensity);
    let light = Light::PointLight(pl);
    w.add_light(light);

    let mut m = Material::new();
    m.set_color(Color::new(0.8, 1., 0.6));
    m.set_diffuse(0.7);
    m.set_specular(0.2);

    let mut s1 = Sphere::new();
    s1.set_material(m);
    let shape1 = Shape::Sphere(s1);

    let m = Matrix::scale(0.5, 0.5, 0.5);
    let mut s2 = Sphere::new();
    s2.set_transformation(m);
    let shape2 = Shape::Sphere(s2);

    w.add_shape(shape1);
    w.add_shape(shape2);

    w
}

#[test]
fn test_default_world() {
    let w = default_world();

    let origin = Tuple4D::new_point(0.0,0.0,-5.0);
    let direction = Tuple4D::new_vector(0.0,0.0,1.0);
    let r = Ray::new(origin, direction);
    let tmp = Intersection::intersect_world(&w, &r);
    let xs = tmp.get_intersections();

    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].get_t(), 4.0);
    assert_eq!(xs[1].get_t(), 4.5);
    assert_eq!(xs[2].get_t(), 5.5);
    assert_eq!(xs[3].get_t(), 6.0);
}



