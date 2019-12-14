#[cfg(feature = "cuda")]
extern crate rustacuda_core;

use raytracer_lib_std::{Canvas, CanvasOps};

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
}
