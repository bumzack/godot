use crate::basics::canvas::{Canvas, CanvasOps};
use crate::basics::ray::Ray;
use crate::basics::ray::RayOps;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;
use crate::world::world::{MAX_REFLECTION_RECURSION_DEPTH, World};
use crate::world::world::WorldOps;

#[derive(Clone, Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Matrix,
    half_view: f64,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

pub trait CameraOps {
    fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera;

    fn get_hsize(&self) -> usize;
    fn get_vsize(&self) -> usize;
    fn get_field_of_view(&self) -> f64;
    fn get_transform(&self) -> &Matrix;
    fn get_pixel_size(&self) -> f64;
    fn get_half_width(&self) -> f64;
    fn get_half_height(&self) -> f64;

    fn calc_pixel_size(&mut self);

    fn set_transformation(&mut self, m: Matrix);

    fn ray_for_pixel(c: &Camera, x: usize, y: usize) -> Ray;

    fn render(c: &Camera, w: &World) -> Canvas;
    fn render_multi_core(c: &Camera, w: &World, num_cores: i32) -> Canvas;
    fn render_debug(c: &Camera, w: &World, x: usize, y: usize) -> Canvas;
}

impl CameraOps for Camera {
    fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let c = Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::new_identity_4x4(),
            half_view: 0.0,
            half_width: 0.0,
            half_height: 0.0,
            pixel_size: 0.0,
        };
        c
    }

    fn get_hsize(&self) -> usize {
        self.hsize
    }

    fn get_vsize(&self) -> usize {
        self.vsize
    }

    fn get_field_of_view(&self) -> f64 {
        self.field_of_view
    }

    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn calc_pixel_size(&mut self) {
        self.half_view = (self.field_of_view / 2.0).tan();
        let aspect = self.hsize as f64 / (self.vsize as f64);

        if aspect >= 1.0 {
            self.half_width = self.half_view;
            self.half_height = self.half_view / aspect;
        } else {
            self.half_width = self.half_view * aspect;
            self.half_height = self.half_view;
        }

        self.pixel_size = self.half_width * 2.0 / (self.hsize as f64);
    }

    fn get_pixel_size(&self) -> f64 {
        self.pixel_size
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.transform = m;
    }

    fn ray_for_pixel(c: &Camera, x: usize, y: usize) -> Ray {
        let x_offset = (x as f64 + 0.5) * c.get_pixel_size();
        let y_offset = (y as f64 + 0.5) * c.get_pixel_size();

        let world_x = c.get_half_width() - x_offset;
        let world_y = c.get_half_height() - y_offset;
        // TODO: we unwrap here silently ...

        let camera_transform_inv =
            Matrix::invert(c.get_transform()).expect("ray_for_pixel:  cant calculate the inverse");

        // use vector, but is it a vector ?
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

    fn get_half_width(&self) -> f64 {
        self.half_width
    }

    fn get_half_height(&self) -> f64 {
        self.half_height
    }

    fn render(c: &Camera, w: &World) -> Canvas {
        let mut canvas = Canvas::new(c.get_hsize(), c.get_vsize());

        for y in 0..c.get_vsize() {
            for x in 0..c.get_hsize() {
                let r = Camera::ray_for_pixel(c, x, y);
                println!("render point  {}/{}", x, y);
                let color = World::color_at(w, &r, MAX_REFLECTION_RECURSION_DEPTH);
                // TODO: wtf ?!
                if color.r != 0.0 || color.g != 0.0 || color.b != 0.0 {}
                canvas.write_pixel(x, y, color);
            }
            // println!("render line  {}", y);
        }
        canvas
    }

    fn render_multi_core(c: &Camera, w: &World, num_cores: i32) -> Canvas {
        //        let mut canvas = Canvas::new(c.get_hsize(), c.get_vsize());
        //
        //        let data = Arc::new(Mutex::new(canvas));
        //        let mut children = vec![];
        //        let act_y: usize = 0;
        //        let act_y_mutex = Arc::new(Mutex::new(act_y));
        //
        //        for _i in 0..num_cores {
        //            let cloned_data = Arc::clone(&data);
        //            let cloned_act_y = Arc::clone(&act_y_mutex);
        //            let height = c.get_hsize();
        //            let width = c.get_vsize();
        //
        //            let c_clone = c.clone();
        //            let w_clone = w.clone();
        //
        //            children.push(thread::spawn(move || {
        //                let mut y: usize = 0;
        //                while *cloned_act_y.lock().unwrap() < height {
        //                    if y < height {
        //                        let mut acty = cloned_act_y.lock().unwrap();
        //                        y = *acty;
        //                        *acty = *acty + 1;
        //                    }
        //                    for x in 0..width {
        //                        let r = Camera::ray_for_pixel(&c_clone, x, y);
        //                        println!("render point  {}/{}", x, y);
        //                        let color = World::color_at(&w_clone, &r, MAX_REFLECTION_RECURSION_DEPTH);
        //                        // TODO: wtf ?!
        //                        if color.r != 0.0 || color.g != 0.0 || color.b != 0.0 {}
        //                        let mut canvas = cloned_data.lock().unwrap();
        //                        canvas.write_pixel(x, y, color);
        //                    }
        //                }
        //            }));
        //        }
        //
        //        for child in children {
        //            let _ = child.join();
        //        }
        //
        //        let c = data.lock().unwrap();
        //        *c
        Canvas::new(c.get_hsize(), c.get_vsize())
    }

    fn render_debug(c: &Camera, w: &World, x: usize, y: usize) -> Canvas {
        println!("DEBUG render point  {}/{}", x, y);

        let mut canvas = Canvas::new(c.get_hsize(), c.get_vsize());
        let r = Camera::ray_for_pixel(c, x, y);
        let c = World::color_at(w, &r, MAX_REFLECTION_RECURSION_DEPTH);
        if c.r != 0.0 || c.g != 0.0 || c.b != 0.0 {}
        canvas.write_pixel(x, y, c);
        canvas
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use crate::basics::color::{Color, ColorOps};
    use crate::math::common::{assert_color, assert_float, assert_matrix, assert_tuple};
    use crate::world::world::default_world;

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

    // page 104
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
        let color = image.pixel_at(5, 5);
        let c_expected = Color::new(0.38066, 0.47583, 0.2855);
        assert_color(color, &c_expected);
    }

    //
    // copy of sphere::test_ray_sphere_intersection()  but uses the render method
    //    #[test]
    //    fn test_ray_sphere_intersection_render() {
    //        let mut w = World::new();
    //
    //        let light_pos = Tuple4D::new_point(-10.0, 10., -10.0);
    //        let light_intensity = Color::new(1.0, 1.0, 1.0);
    //        let pl = PointLight::new(light_pos, light_intensity);
    //        let light = Light::PointLight(pl);
    //        w.set_light(light);
    //
    //        let mut s1 = Sphere::new();
    //        let shape1 = Shape::Sphere(s1);
    //
    //        w.add_shape(shape1);
    //
    //        let from = Tuple4D::new_point(0.0, 0.0, -5.0);
    //        let to = Tuple4D::new_point(0.0, 0.0, 0.0);
    //        let up = Tuple4D::new_vector(0.0, 1.0, 0.0);
    //
    //        let mut c = Camera::new(11, 11, PI / 2.0);
    //        c.set_transformation(Matrix::view_transform(&from, &to, &up));
    //
    //        let image = Camera::render(&c, &w);
    //        // println!("image = {:#?}", image);
    //
    //        let c = image.pixel_at(5, 5);
    //        let c_expected = Color::new(0.38066, 0.47583, 0.2855);
    //
    //        println!("c = {:#?}", c);
    //        println!("c_expected = {:#?}", c_expected);
    //        assert_color(c, &c_expected);
    //    }
}
