use std::f32::consts::PI;

use crate::basics::color::Color;
use crate::basics::{Canvas, CanvasOps};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple4D;
use crate::math::Tuple;
use crate::patterns::spherical_map;
use crate::prelude::ShapeOps;
use crate::shape::shape::Shape;

#[derive(PartialEq, Debug, Clone)]
pub struct ImageTexturePattern {
    image: Canvas,
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
}

impl ImageTexturePattern {
    pub fn new(image: Canvas) -> ImageTexturePattern {
        ImageTexturePattern {
            image,
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
        }
    }

    pub fn pattern_at(pattern: &ImageTexturePattern, p: &Tuple4D) -> Color {
        let (u, v) = spherical_map(p);
        ImageTexturePattern::uv_pattern_at(&pattern.image, u, v)
    }

    pub fn uv_pattern_at(image: &Canvas, u: f32, v: f32) -> Color {
        let v = 1.0 - v;
        let x = u * (image.get_width() as f32 - 1.0);
        let y = v * (image.get_height() as f32 - 1.0);
        let color = image.pixel_at(x as usize, y as usize).color;
        println!(
            "x  = {}, y =  {}    color = {:?}      w {}, h {}",
            x,
            y,
            color,
            image.get_width(),
            image.get_height()
        );
        image.pixel_at(x as usize, y as usize).color
    }

    pub fn color_at_object(pattern: &ImageTexturePattern, shape: &Shape, world_point: &Tuple4D) -> Color {
        let object_point = shape.get_inverse_transformation() * world_point;
        let pattern_point = pattern.get_inverse_transformation() * &object_point;
        ImageTexturePattern::pattern_at(pattern, &pattern_point)
    }

    pub fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix = Matrix::invert(&m).unwrap();
        self.transformation_matrix = m;
    }

    pub fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }

    pub fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }
}
