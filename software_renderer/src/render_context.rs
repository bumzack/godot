#[cfg(feature = "cuda")]
extern crate rustacuda_core;

use crate::vertex::Vertex;
use math::{Matrix, MatrixOps};
use raytracer_lib_std::{Canvas, CanvasOps};
use crate::edge::Edge;
use crate::gradient::Gradient;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct RenderContext {
    z_buffer: Vec<f32>,
    canvas: Canvas,
}

impl RenderContext {
    pub fn new(width: usize, height: usize) -> RenderContext {
        RenderContext {
            z_buffer: vec![],
            canvas: Canvas::new(width, height),
        }
    }

    pub fn width(&self) -> usize {
        self.canvas.get_width()
    }

    pub fn height(&self) -> usize {
        self.canvas.get_height()
    }

    pub fn clear_depth_buffer(&mut self) {
        self.z_buffer.iter_mut().for_each(|v| *v = std::f32::MAX)
    }

    pub fn draw_triangle(&mut self, v1: &Vertex, v2: &Vertex, v3: &Vertex, texture: &Canvas) {
        if v1.is_inside_view_frustum() && v2.is_inside_view_frustum() && v3.is_inside_view_frustum() {
            self.fill_triangle(v1, v2, v3, texture);
        }

        let mut vertices = vec![v1, v2, v3];
        let mut auxillary_list = vec![];

        if self.clip_polygon_axis(&mut vertices, &mut auxillary_list, 0)
            && self.clip_polygon_axis(&mut vertices, &mut auxillary_list, 1)
            && self.clip_polygon_axis(&mut vertices, &mut auxillary_list, 2)
        {
            let initial_vertex = vertices.get(0).unwrap();

            for i in 1..vertices.len() - 1 {
                self.fill_triangle(
                    initial_vertex,
                    vertices.get(i).unwrap(),
                    vertices.get(i + 1).unwrap(),
                    texture,
                );
            }
        }
    }

    fn clip_polygon_axis(
        &mut self,
        vertices: &mut Vec<&Vertex>,
        auxillary_list: &mut Vec<&Vertex>,
        component_index: usize,
    ) -> bool {
        self.clip_polygon_component(vertices, component_index, 1.0, auxillary_list);
        vertices.clear();

        if auxillary_list.is_empty() {
            return false;
        }
        self.clip_polygon_component(auxillary_list, component_index, -1.0, vertices);
        auxillary_list.clear();

        !vertices.is_empty()
    }

    fn clip_polygon_component(
        &self,
        vertices: &mut Vec<&Vertex>,
        component_index: usize,
        component_factor: f32,
        result: &mut Vec<&Vertex>,
    ) {
        let mut previous_vertex = vertices.get(vertices.len() - 1).unwrap();
        let mut previous_component = previous_vertex.get(component_index) * component_factor;
        let mut previous_inside = previous_component <= previous_vertex.w();

        vertices.iter().for_each(|current_vertex| {
            let current_component = current_vertex.get(component_index) * component_factor;
            let current_inside = current_component <= current_vertex.w();

            if current_inside ^ previous_inside {
                let lerp_amt = (previous_vertex.w() - previous_component)
                    / ((previous_vertex.w() - previous_component) - (current_vertex.w() - current_component));
                result.push(&previous_vertex.lerp(&current_vertex, lerp_amt));
            }
            if current_inside {
                result.push(current_vertex);
            }
            previous_vertex = current_vertex;
            previous_component = current_component;
            previous_inside = current_inside;
        });
    }

    fn fill_triangle(&mut self, v1: &Vertex, v2: &Vertex, v3: &Vertex, texture: &Canvas) {
        let screen_space_transform =
            Matrix::init_screen_space_transform(self.canvas.get_width() / 2, self.canvas.get_height() / 2);
        let identity = Matrix::new_identity_4x4();

        let mut min_y_vert = v1.transform(&screen_space_transform, &identity).perspective_divide();
        let mut mid_y_vert = v2.transform(&screen_space_transform, &identity).perspective_divide();
        let mut max_y_vert = v3.transform(&screen_space_transform, &identity).perspective_divide();

        if min_y_vert.triangle_area_times_two(&max_y_vert, &mid_y_vert) >= 0.0 {
            return;
        }
        if max_y_vert.y() < mid_y_vert.y() {
            let temp = max_y_vert;
            max_y_vert = mid_y_vert;
            mid_y_vert = temp;
        }

        if mid_y_vert.y() < min_y_vert.y() {
            let temp = mid_y_vert;
            mid_y_vert = min_y_vert;
            min_y_vert = temp;
        }

        if max_y_vert.y() < mid_y_vert.y() {
            let temp = max_y_vert;
            max_y_vert = mid_y_vert;
            mid_y_vert = temp;
        }

        self.scan_triangle(&min_y_vert, &mid_y_vert, &max_y_vert, min_y_vert.triangle_area_times_two(&max_y_vert, &mid_y_vert) >= 0.0, texture);
    }

    fn scan_triangle(&mut self, min_y_vert: &Vertex, mid_y_vert: &Vertex, max_y_vert: &Vertex, handedness: bool, texture: &Canvas) {
        let gradient = Gradient::new(min_y_vert, mid_y_vert, max_y_vert);

        let top_to_bottom = Edge::new(&gradient, min_y_vert, max_y_vert, 0);
        let top_to_middle = Edge::new(&gradient, min_y_vert, mid_y_vert, 0);
        let middle_to_bottom = Edge::new(&gradient, mid_y_vert, max_y_vert, 1);

        self.scan_edges(&gradient, &top_to_bottom, &top_to_middle, handedness, texture);
        self.scan_edges(&gradient, &top_to_bottom, &middle_to_bottom, handedness, texture);
    }

    fn scan_edges(&mut self, gradients: &Gradient, a: &Edge, b: &Edge, handedness: bool, texture: &Canvas) {
        let mut left = a;
        let mut right = b;

        if handedness {
            let temp = left;
            left = right;
            right = temp;
        }

        let y_start = b.y_start();
        let y_end = b.y_end();

        for j in y_start..y_end {
            self.draw_scan_line(gradients, left, right, j, texture);
            left.step();
            right.step();
        }
    }

    fn draw_scan_line(&mut self, gradients: &Gradient, left: &Edge, right: &Edge, j: i32, texture: &Canvas) {
        let x_min = left.x().ceil() as i32;
        let x_max = right.x().ceil() as i32;

        let x_prestep = x_min as f32 - left.x();

        let tex_coord_x_xstep = gradients.tex_coord_x_xstep();
        let tex_coord_y_xstep = gradients.tex_coord_y_xstep();
        let one_over_z_xstep = gradients.one_over_z_xstep();
        let depth_xstep = gradients.depth_xstep();
        let light_amp_xstep = gradients.light_amp_xstep();

        let mut tex_coord_x = left.tex_coord_x() + tex_coord_x_xstep * x_prestep;
        let mut tex_coord_y = left.tex_coord_y() + tex_coord_y_xstep * x_prestep;
        let mut one_over_z = left.one_over_z() + one_over_z_xstep * x_prestep;
        let mut depth = left.depth() + depth_xstep * x_prestep;
        let mut light_amt = left.light_amt() + light_amp_xstep * x_prestep;

        for i in x_min..x_max {
            let index: usize = (i + j * self.canvas.get_width() as i32) as usize;

            if depth < self.z_buffer[index] {
                self.z_buffer[index] = depth;
                let z = 1.0 / one_over_z;

                let src_x = (tex_coord_x * z * (texture.get_width() - 1) as f32 + 0.5) as i32;
                let src_y = (tex_coord_y * z * (texture.get_height() - 1) as f32 + 0.5) as i32;

                self.copy_pixel(i, j, src_x, src_y, texture, light_amt);
            }

            one_over_z += one_over_z_xstep;
            tex_coord_x += tex_coord_x_xstep;
            tex_coord_y += tex_coord_y_xstep;
            depth += depth_xstep;
            light_amt += light_amp_xstep;
        }
    }

    fn copy_pixel(&mut self, dest_x: i32, dest_y: i32, src_x: i32, src_y: i32, src: &Canvas, light_amt: f32) {
        self.canvas.write_pixel(dest_x as usize, dest_y as usize, src.pixel_at(src_x as usize, src_y as usize).color * light_amt);
    }
}
