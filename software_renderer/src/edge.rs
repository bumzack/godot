#[cfg(feature = "cuda")]
extern crate rustacuda_core;

use crate::gradient::Gradient;
use crate::vertex::Vertex;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Edge {
    x: f32,
    x_step: f32,
    y_start: i32,
    y_end: i32,
    tex_coord_x: f32,
    tex_coord_x_step: f32,
    tex_coord_y: f32,
    tex_coord_y_step: f32,
    one_over_z: f32,
    one_over_z_step: f32,
    depth: f32,
    depth_step: f32,
    light_amt: f32,
    light_amt_step: f32,
}

impl Edge {
    pub fn new(
        gradient: Gradient,
        min_y_vert: Vertex,
        max_y_vert: Vertex,
        min_y_vert_index: usize,
    ) -> Edge {
        let y_start = min_y_vert.y().ceil() as i32;
        let y_end = max_y_vert.y().ceil() as i32;

        let x_dist = max_y_vert.x() - min_y_vert.x();
        let y_dist = max_y_vert.y() - min_y_vert.y();

        let y_prestep = y_start as f32 - min_y_vert.y();
        let x_step = x_dist / y_dist;
        let x = min_y_vert.x() + y_prestep * x_step;
        let x_prestep = x - min_y_vert.x();

        let tex_coord_x = gradient.tex_coord_x(min_y_vert_index)
            + gradient.tex_coord_x_xstep() * x_prestep
            + gradient.tex_coord_x_ystep() * y_prestep;
        let tex_coord_x_step = gradient.tex_coord_x_ystep() + gradient.tex_coord_x_xstep() * x_step;

        let tex_coord_y = gradient.tex_coord_y(min_y_vert_index)
            + gradient.tex_coord_y_xstep() * x_prestep
            + gradient.tex_coord_y_ystep() * y_prestep;
        let tex_coord_y_step = gradient.tex_coord_y_ystep() + gradient.tex_coord_y_xstep() * x_step;

        let one_over_z = gradient.one_over_z(min_y_vert_index)
            + gradient.one_over_z_xstep() * x_prestep
            + gradient.one_over_z_ystep() * y_prestep;
        let one_over_z_step = gradient.one_over_z_ystep() + gradient.one_over_z_xstep() * x_step;

        let depth =
            gradient.depth(min_y_vert_index) + gradient.depth_xstep() * x_prestep + gradient.depth_ystep() * y_prestep;
        let depth_step = gradient.depth_ystep() + gradient.depth_xstep() * x_step;

        let light_amt = gradient.light_amp(min_y_vert_index)
            + gradient.light_amp_xstep() * x_prestep
            + gradient.light_amp_ystep() * y_prestep;
        let light_amt_step = gradient.light_amp_ystep() + gradient.light_amp_xstep() * x_step;

        Edge {
            x,
            x_step,
            y_start,
            y_end,
            tex_coord_x,
            tex_coord_x_step,
            tex_coord_y,
            tex_coord_y_step,
            one_over_z,
            one_over_z_step,
            depth,
            depth_step,
            light_amt,
            light_amt_step,
        }
    }

    pub fn step(&mut self) {
        self.x += self.x_step;
        self.tex_coord_x += self.tex_coord_x_step;
        self.tex_coord_y += self.tex_coord_y_step;
        self.one_over_z += self.one_over_z_step;
        self.depth += self.depth_step;
        self.light_amt += self.light_amt_step;
    }
}
