#[cfg(feature = "cuda")]
extern crate rustacuda_core;

use math::Matrix;
use raytracer_lib_std::Canvas;
use utils::prelude::ObjModel;

use crate::render_context::RenderContext;
use crate::vertex::Vertex;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<usize>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: vec![],
            indices: vec![],
        }
    }

    pub fn read_obj_file(filename: &str) -> Result<Mesh, Box<dyn std::error::Error>> {
        println!("reading file {:?}", filename);
        let model = ObjModel::read_file(filename)?;
        let model = model.to_indexed_model();

        let mut vertices = vec![];

        for i in 0..model.positions().len() {
            let v = Vertex::new(
                *model.positions().get(i).unwrap(),
                *model.tex_coords().get(i).unwrap(),
                *model.normals().get(i).unwrap(),
            );
            vertices.push(v)
        }

        let indices = model.indices();

        println!("success reading file {:?}", filename);

        Ok(Mesh { vertices, indices })
    }

    pub fn draw(&self, context: &mut RenderContext, view_projection: &Matrix, transform: &Matrix, texture: &Canvas) {
        let mvp = view_projection * transform;
        for i in (0..self.indices.len() - 3).step_by(3) {
            // println!("draw_traingle:   i = {}, self.vertices.len() = {}", i, self.vertices.len());
            let idx1 = self.indices.get(i).unwrap();
            let idx2 = self.indices.get(i+1).unwrap();
            let idx3 = self.indices.get(i+2).unwrap();
            context.draw_triangle(
                &self.vertices.get(*idx1).unwrap().transform(&mvp, transform),
                &self.vertices.get(*idx2).unwrap().transform(&mvp, transform),
                &self.vertices.get(*idx3).unwrap().transform(&mvp, transform),
                texture,
            )
        }
    }
}
