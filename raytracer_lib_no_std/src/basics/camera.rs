use serde::Deserialize;
use serde::Serialize;

use math::prelude::*;

use crate::{Ray, RayOps};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    transform: Matrix,
    half_view: f32,
    half_width: f32,
    half_height: f32,
    pixel_size: f32,
    antialiasing: bool,
    antialiasing_size: usize, // 2 or 3
    calc_reflection: bool,
    calc_refraction: bool,
    calc_shadows: bool,
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

    fn set_antialiasing(&mut self, aa: bool);
    fn get_antialiasing(&self) -> bool;

    fn set_antialiasing_size(&mut self, size: usize);
    fn get_antialiasing_size(&self) -> usize;

    fn get_calc_reflection(&self) -> bool;
    fn set_calc_reflection(&mut self, calc_reflection: bool);

    fn get_calc_refraction(&self) -> bool;
    fn set_calc_refraction(&mut self, calc_refraction: bool);

    fn get_calc_shadows(&self) -> bool;
    fn set_calc_shadows(&mut self, calc_shadows: bool);

    fn calc_pixel_size(&mut self);

    fn set_transformation(&mut self, m: Matrix);

    fn ray_for_pixel(c: &Camera, x: usize, y: usize) -> Ray;
    fn ray_for_pixel_anti_aliasing(c: &Camera, x: usize, y: usize, x_offset: f32, y_offset: f32) -> Ray;
}

impl CameraOps for Camera {
    fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera {
        let c = Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::new_identity_4x4(),
            half_view: 0.0,
            half_width: 0.0,
            half_height: 0.0,
            pixel_size: 0.0,
            antialiasing: false,
            antialiasing_size: 2,
            calc_reflection: true,
            calc_refraction: true,
            calc_shadows: true,
        };
        c
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
        self.half_view = intri_tan(self.field_of_view / 2.0);
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
        let camera_transform_inv =
            Matrix::invert(c.get_transform()).expect("ray_for_pixel:  cant calculate the inverse");

        let x_offset = (x as f32 + 0.5) * c.get_pixel_size();
        let y_offset = (y as f32 + 0.5) * c.get_pixel_size();

        let world_x = c.get_half_width() - x_offset;
        let world_y = c.get_half_height() - y_offset;
        // println!("no AA    (x/y) = ({}/{})  world_point = ({}/{})", x, y, world_x, world_y);

        let p = Tuple4D::new_point(world_x, world_y, -1.0);

        let o = Tuple4D::new_point(0.0, 0.0, 0.0);

        let pixel = &camera_transform_inv * &p;
        let mut origin = &camera_transform_inv * &o;
        let mut direction = Tuple4D::normalize(&(&pixel - &origin));

        // so the assert in Ray::new don't panic
        origin.w = 1.0;
        direction.w = 0.0;
        Ray::new(origin, direction)
    }

    fn ray_for_pixel_anti_aliasing(c: &Camera, x: usize, y: usize, delta_x: f32, delta_y: f32) -> Ray {
        let camera_transform_inv =
            Matrix::invert(c.get_transform()).expect("ray_for_pixel:  cant calculate the inverse");

        let x_offset = (x as f32 + 0.5) * c.get_pixel_size();
        let y_offset = (y as f32 + 0.5) * c.get_pixel_size();

        let world_x = c.get_half_width() - x_offset + delta_x;
        let world_y = c.get_half_height() - y_offset + delta_y;

        let p = Tuple4D::new_point(world_x, world_y, -1.0);

        let o = Tuple4D::new_point(0.0, 0.0, 0.0);

        let pixel = &camera_transform_inv * &p;
        let mut origin = &camera_transform_inv * &o;
        let mut direction = Tuple4D::normalize(&(&pixel - &origin));

        // so the assert in Ray::new don't panic
        origin.w = 1.0;
        direction.w = 0.0;
        Ray::new(origin, direction)
    }

    fn get_half_width(&self) -> f32 {
        self.half_width
    }

    fn get_half_height(&self) -> f32 {
        self.half_height
    }

    fn set_antialiasing(&mut self, aa: bool) {
        self.antialiasing = aa;
    }

    fn get_antialiasing(&self) -> bool {
        self.antialiasing
    }

    fn set_antialiasing_size(&mut self, size: usize) {
        self.antialiasing_size = size;
    }

    fn get_antialiasing_size(&self) -> usize {
        self.antialiasing_size
    }

    fn get_calc_reflection(&self) -> bool {
        self.calc_reflection
    }

    fn set_calc_reflection(&mut self, calc_reflection: bool) {
        self.calc_reflection = calc_reflection;
    }

    fn get_calc_refraction(&self) -> bool {
        self.calc_refraction
    }

    fn set_calc_refraction(&mut self, calc_refraction: bool) {
        self.calc_refraction = calc_refraction
    }

    fn get_calc_shadows(&self) -> bool {
        self.calc_shadows
    }

    fn set_calc_shadows(&mut self, calc_shadows: bool) {
        self.calc_shadows = calc_shadows;
    }
}

#[cfg(test)]
mod tests {
    use core::f32::consts::{PI, SQRT_2};

    use crate::{assert_float, assert_matrix, assert_tuple, MatrixOps, Tuple4D};
    use crate::basics::color::{Color, ColorOps};

    use super::*;

    // page 101
    #[test]
    fn test_camera_new() {
        let mut c = Camera::new(160, 120, PI / SQRT_2);
        c.calc_pixel_size();

        assert_eq!(c.get_hsize(), 160);
        assert_eq!(c.get_vsize(), 120);

        assert_float(c.get_field_of_view(), PI / SQRT_2);
        assert_matrix(c.get_transform(), &Matrix::new_identity_4x4());
    }

    // page 101 bottom
    #[test]
    fn test_camera_pixel_size_horizontal() {
        let mut c = Camera::new(200, 125, PI / 2.0);
        c.calc_pixel_size();
        assert_float(c.get_pixel_size(), 0.01);
        assert_matrix(c.get_transform(), &Matrix::new_identity_4x4());
    }

    // page 101 bottom part 2
    #[test]
    fn test_camera_pixel_size_vertical() {
        let mut c = Camera::new(125, 200, PI / 2.0);
        c.calc_pixel_size();
        assert_float(c.get_pixel_size(), 0.01);
        assert_matrix(c.get_transform(), &Matrix::new_identity_4x4());
    }

    // page 103 part1
    #[test]
    fn test_camera_ray_for_pixel_center() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.calc_pixel_size();

        let r = Camera::ray_for_pixel(&c, 100, 50);
        let origin_expected = Tuple4D::new_point(0.0, 0.0, 0.0);
        let direction_expected = Tuple4D::new_vector(0.0, 0.0, -1.0);

        assert_tuple(&r.get_origin(), &origin_expected);
        assert_tuple(&r.get_direction(), &direction_expected);
    }

    // page 103 part 2
    #[test]
    fn test_camera_ray_for_pixel_canvas_corner() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.calc_pixel_size();

        let r = Camera::ray_for_pixel(&c, 0, 0);
        let origin_expected = Tuple4D::new_point(0.0, 0.0, 0.0);
        let direction_expected = Tuple4D::new_vector(0.66519, 0.33259, -0.66851);

        assert_tuple(&r.get_origin(), &origin_expected);
        assert_tuple(&r.get_direction(), &direction_expected);
    }

    // page 103 part 3
    #[test]
    fn test_camera_ray_for_pixel_transformed_camera() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.calc_pixel_size();

        let rot_y = Matrix::rotate_y(PI / 4.0);
        let trans = Matrix::translation(0.0, -2.0, 5.0);

        let transform = &rot_y * &trans;
        c.set_transformation(transform);

        let r = Camera::ray_for_pixel(&c, 100, 50);
        let expected_origin = Tuple4D::new_point(0.0, 2.0, -5.0);
        let expected_direction = Tuple4D::new_vector(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0);

        assert_tuple(&r.get_origin(), &expected_origin);
        assert_tuple(&r.get_direction(), &expected_direction);
    }
}
