#[cfg(feature = "cuda")]
extern crate rustacuda_core;

use crate::vertex::Vertex;
use math::{Tuple, Tuple4D};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Gradient {
    tex_coord_x: [f32; 3],
    tex_coord_y: [f32; 3],
    one_over_z: [f32; 3],
    depth: [f32; 3],
    light_amp: [f32; 3],
    tex_coord_x_xstep: f32,
    tex_coord_x_ystep: f32,
    tex_coord_y_xstep: f32,
    tex_coord_y_ystep: f32,
    one_over_z_xstep: f32,
    one_over_z_ystep: f32,
    depth_xstep: f32,
    depth_ystep: f32,
    light_amp_xstep: f32,
    light_amp_ystep: f32,
}

impl Gradient {
    pub fn new(min_y_vert: &Vertex, mid_y_vert: &Vertex, max_y_vert: &Vertex) -> Gradient {
        let one_over_dx = 1.0
            / (((mid_y_vert.x() - max_y_vert.x()) * (min_y_vert.y() - max_y_vert.y()))
                - ((min_y_vert.x() - max_y_vert.x()) * (mid_y_vert.y() - max_y_vert.y())));

        let one_over_dy = -one_over_dx;

        let mut one_over_z = [0.0; 3];
        let mut tex_coord_x = [0.0; 3];
        let mut tex_coord_y = [0.0; 3];
        let mut depth = [0.0; 3];
        let mut light_amp = [0.0; 3];

        depth[0] = min_y_vert.z();
        depth[1] = mid_y_vert.z();
        depth[2] = max_y_vert.z();

        let light_dir = Tuple4D::new_vector(0.0, 0.0, 1.0);

        light_amp[0] = saturate(min_y_vert.normal() ^ &light_dir) * 0.9 + 0.1;
        light_amp[1] = saturate(mid_y_vert.normal() ^ &light_dir) * 0.9 + 0.1;
        light_amp[2] = saturate(max_y_vert.normal() ^ &light_dir) * 0.9 + 0.1;

        one_over_z[0] = 1.0 / min_y_vert.w();
        one_over_z[1] = 1.0 / mid_y_vert.w();
        one_over_z[2] = 1.0 / max_y_vert.w();

        tex_coord_x[0] = min_y_vert.tex_coords().get_x() * one_over_z[0];
        tex_coord_x[1] = mid_y_vert.tex_coords().get_x() * one_over_z[1];
        tex_coord_x[2] = max_y_vert.tex_coords().get_x() * one_over_z[2];

        tex_coord_y[0] = min_y_vert.tex_coords().get_y() * one_over_z[0];
        tex_coord_y[1] = mid_y_vert.tex_coords().get_y() * one_over_z[1];
        tex_coord_y[2] = max_y_vert.tex_coords().get_y() * one_over_z[2];

        let tex_coord_x_xstep = calc_x_step(tex_coord_x, min_y_vert, mid_y_vert, max_y_vert, one_over_dx);
        let tex_coord_x_ystep = calc_y_step(tex_coord_x, min_y_vert, mid_y_vert, max_y_vert, one_over_dy);

        let tex_coord_y_xstep = calc_x_step(tex_coord_y, min_y_vert, mid_y_vert, max_y_vert, one_over_dx);
        let tex_coord_y_ystep = calc_y_step(tex_coord_y, min_y_vert, mid_y_vert, max_y_vert, one_over_dy);

        let one_over_z_xstep = calc_x_step(one_over_z, min_y_vert, mid_y_vert, max_y_vert, one_over_dx);
        let one_over_z_ystep = calc_y_step(one_over_z, min_y_vert, mid_y_vert, max_y_vert, one_over_dy);

        let depth_xstep = calc_x_step(depth, min_y_vert, mid_y_vert, max_y_vert, one_over_dx);
        let depth_ystep = calc_y_step(depth, min_y_vert, mid_y_vert, max_y_vert, one_over_dy);

        let light_amp_xstep = calc_x_step(light_amp, min_y_vert, mid_y_vert, max_y_vert, one_over_dx);
        let light_amp_ystep = calc_y_step(light_amp, min_y_vert, mid_y_vert, max_y_vert, one_over_dy);

        Gradient {
            tex_coord_x,
            tex_coord_y,
            one_over_z,
            depth,
            light_amp,
            tex_coord_x_xstep,
            tex_coord_x_ystep,
            tex_coord_y_xstep,
            tex_coord_y_ystep,
            one_over_z_xstep,
            one_over_z_ystep,
            depth_xstep,
            depth_ystep,
            light_amp_xstep,
            light_amp_ystep,
        }
    }

    pub fn tex_coord_x(&self, loc: usize) -> f32 {
        self.tex_coord_x[loc]
    }
    pub fn tex_coord_y(&self, loc: usize) -> f32 {
        self.tex_coord_y[loc]
    }
    pub fn one_over_z(&self, loc: usize) -> f32 {
        self.one_over_z[loc]
    }
    pub fn depth(&self, loc: usize) -> f32 {
        self.depth[loc]
    }
    pub fn light_amp(&self, loc: usize) -> f32 {
        self.light_amp[loc]
    }

    pub fn tex_coord_x_xstep(&self) -> f32 {
        self.tex_coord_x_xstep
    }
    pub fn tex_coord_x_ystep(&self) -> f32 {
        self.tex_coord_x_ystep
    }

    pub fn tex_coord_y_xstep(&self) -> f32 {
        self.tex_coord_y_xstep
    }
    pub fn tex_coord_y_ystep(&self) -> f32 {
        self.tex_coord_y_ystep
    }
    pub fn one_over_z_xstep(&self) -> f32 {
        self.one_over_z_xstep
    }
    pub fn one_over_z_ystep(&self) -> f32 {
        self.one_over_z_ystep
    }
    pub fn depth_xstep(&self) -> f32 {
        self.depth_xstep
    }
    pub fn depth_ystep(&self) -> f32 {
        self.depth_ystep
    }
    pub fn light_amp_xstep(&self) -> f32 {
        self.light_amp_xstep
    }
    pub fn light_amp_ystep(&self) -> f32 {
        self.light_amp_ystep
    }
}

fn saturate(value: f32) -> f32 {
    if value > 1.0 {
        return 1.0;
    }
    if value < 0.0 {
        return 0.0;
    }
    value
}

fn calc_x_step(
    values: [f32; 3],
    min_y_vert: &Vertex,
    mid_y_vert: &Vertex,
    max_y_vert: &Vertex,
    one_over_dx: f32,
) -> f32 {
    (((values[1] - values[2]) * (min_y_vert.y() - max_y_vert.y()))
        - ((values[0] - values[2]) * (mid_y_vert.y() - max_y_vert.y())))
        * one_over_dx
}

fn calc_y_step(
    values: [f32; 3],
    min_y_vert: &Vertex,
    mid_y_vert: &Vertex,
    max_y_vert: &Vertex,
    one_over_dy: f32,
) -> f32 {
    (((values[1] - values[2]) * (min_y_vert.x() - max_y_vert.x()))
        - ((values[0] - values[2]) * (mid_y_vert.x() - max_y_vert.x())))
        * one_over_dy
}
