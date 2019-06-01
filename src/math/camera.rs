use std::f32::consts::{PI, SQRT_2};

use crate::math::canvas::{Canvas, CanvasOps};
use crate::math::color::Color;
use crate::math::color::ColorOps;
use crate::math::common::{assert_color, assert_float, assert_matrix, assert_tuple};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::ray::{Ray, RayOps};
use crate::math::tuple4d::ORIGIN;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;
use crate::math::world::default_world;
use crate::math::world::World;
use crate::math::world::WorldOps;

#[derive(Clone, Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    transform: Matrix,
    half_view: f32,
    half_width: f32,
    half_height: f32,
    pixel_size: f32,
}

pub trait CameraOps {
    fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera;

    fn get_hsize(&self) -> usize;
    fn get_vsize(&self) -> usize;
    fn get_field_of_view(&self) -> f32;
    fn get_transform(&self) -> &Matrix;
    fn get_pixel_size(&self) -> f32;
    fn get_half_width(&self) -> f32;
    fn get_half_height(&self) -> f32;

    fn calc_pixel_size(&mut self);

    fn set_transformation(&mut self, m: Matrix);

    fn ray_for_pixel(c: &Camera, x: usize, y: usize) -> Ray;

    fn render(c: &Camera, w: &World) -> Canvas;
}

impl CameraOps for Camera {
    fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera {
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::new_identity_4x4(),
            half_view: 0.0,
            half_width: 0.0,
            half_height: 0.0,
            pixel_size: 0.0,
        }
    }

    fn get_hsize(&self) -> usize {
        self.hsize
    }

    fn get_vsize(&self) -> usize {
        self.vsize
    }

    fn get_field_of_view(&self) -> f32 {
        self.field_of_view
    }

    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn calc_pixel_size(&mut self) {
        self.half_view = (self.field_of_view / 2.0).tan();
        let aspect = self.hsize as f32 / (self.vsize as f32);

        if aspect >= 1.0 {
            self.half_width = self.half_view;
            self.half_height = self.half_view / aspect;
        } else {
            self.half_width = self.half_view * aspect;
            self.half_height = self.half_view;
        }

        self.pixel_size = self.half_width * 2.0 / (self.hsize as f32);
    }

    fn get_pixel_size(&self) -> f32 {
        self.pixel_size
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.transform = m;
    }

    fn ray_for_pixel(c: &Camera, x: usize, y: usize) -> Ray {
        let x_offset = (x as f32 + 0.5) * c.get_pixel_size();
        let y_offset = (y as f32 + 0.5) * c.get_pixel_size();

        let world_x = c.get_half_width() - x_offset;
        let world_y = c.get_half_height() - y_offset;
        // TODO: we unwrap here silently ...
        let pixel = Matrix::invert(c.get_transform()).unwrap() * Tuple4D::new_point(world_x, world_y, -1.0);
        let origin = &Matrix::invert(c.get_transform()).unwrap() * &ORIGIN;
        let direction = Tuple4D::normalize(&(pixel - ORIGIN));
        Ray::new(origin, direction)
    }

    fn get_half_width(&self) -> f32 {
        self.half_width
    }

    fn get_half_height(&self) -> f32 {
        self.half_height
    }

    fn render(c: &Camera, w: &World) -> Canvas {
        let mut canvas = Canvas::new(c.get_hsize(), c.get_vsize());

        for y in 0..c.get_hsize() {
            for x in 0..c.get_vsize() {
                let r = Camera::ray_for_pixel(c, x, y);
                let c = World::color_at(w, &r);
                canvas.write_pixel(x, y, c);
            }
        }
        canvas
    }
}

#[test]
fn test_camera_new() {
    let mut c = Camera::new(160, 120, PI / SQRT_2);
    c.calc_pixel_size();

    assert_eq!(c.get_hsize(), 160);
    assert_eq!(c.get_vsize(), 120);

    assert_float(c.get_field_of_view(), PI / SQRT_2);
    assert_matrix(c.get_transform(), &Matrix::new_identity_4x4());
}

#[test]
fn test_camera_pixel_size_horizontal() {
    let mut c = Camera::new(200, 125, PI / 2.0);
    c.calc_pixel_size();
    assert_float(c.get_pixel_size(), 0.01);
}

#[test]
fn test_camera_pixel_size_vertical() {
    let mut c = Camera::new(125, 200, PI / 2.0);
    c.calc_pixel_size();
    assert_float(c.get_pixel_size(), 0.01);
}


#[test]
fn test_camera_ray_for_pixel_center() {
    let mut c = Camera::new(201, 101, PI / 2.0);
    c.calc_pixel_size();

    let r = Camera::ray_for_pixel(&c, 100, 50);

    assert_tuple(&r.get_origin(), &Tuple4D::new_point(0.0, 0.0, 0.0));
    assert_tuple(&r.get_direction(), &Tuple4D::new_vector(0.0, 0.0, -1.0));
}

#[test]
fn test_camera_ray_for_pixel_canvas_corner() {
    let mut c = Camera::new(201, 101, PI / 2.0);
    c.calc_pixel_size();

    let r = Camera::ray_for_pixel(&c, 0, 0);

    assert_tuple(&r.get_origin(), &Tuple4D::new_point(0.0, 0.0, 0.0));
    assert_tuple(&r.get_direction(), &Tuple4D::new_vector(0.66519, 0.33259, -0.66851));
}

#[test]
fn test_camera_ray_for_pixel_transformed_camera() {
    let mut c = Camera::new(201, 101, PI / 2.0);
    c.calc_pixel_size();

    c.set_transformation(Matrix::rotate_y(PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0));
    let r = Camera::ray_for_pixel(&c, 100, 50);

    assert_tuple(&r.get_origin(), &Tuple4D::new_point(0.0, 2.0, -5.0));
    assert_tuple(&r.get_direction(), &Tuple4D::new_vector(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0));
}

#[test]
fn test_camera_render() {
    let w = default_world();
    let mut c = Camera::new(11, 11, PI / 2.0);
    c.calc_pixel_size();

    let from = Tuple4D::new_point(0.0, 0.0, -5.0);
    let to = Tuple4D::new_point(0.0, 0.0, 0.0);
    let up = Tuple4D::new_vector(0.0, 1.0, 0.0);

    c.set_transformation(Matrix::view_transform(&from, &to, &up));

    let image = Camera::render(&c, &w);
    println!("image = {:#?}", image);

    let c = image.pixel_at(5, 5);
    let c_expected = Color::new(0.38066, 0.47583, 0.2855);

    println!("c = {:#?}", c);
    println!("c_expected = {:#?}", c_expected);
    assert_color(c, &c_expected);
}
