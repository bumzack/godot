use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::math::matrix::Matrix;
use crate::prelude::{Canvas, RenderContext, Vertex};
use crate::utils::prelude::ObjModel;

#[derive(Clone, Debug, Default)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<usize>,
}

impl Mesh {
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
        //println!("view_projection  {}", view_projection);
        //println!("transform  {}", transform);
        let mvp = view_projection * transform;
        //println!("mvp  {}", mvp);

        for i in (0..self.indices.len() - 3).step_by(3) {
            // println!("draw_traingle:   i = {}, self.vertices.len() = {}", i, self.vertices.len());
            let idx1 = self.indices.get(i).unwrap();
            let idx2 = self.indices.get(i + 1).unwrap();
            let idx3 = self.indices.get(i + 2).unwrap();

            let v1_orig = &self.vertices.get(*idx1).unwrap();
            let v1 = v1_orig.transform(&mvp, transform);
            let v2_orig = &self.vertices.get(*idx2).unwrap();
            let v2 = v2_orig.transform(&mvp, transform);
            let v3_orig = &self.vertices.get(*idx3).unwrap();
            let v3 = v3_orig.transform(&mvp, transform);

            //            println!("v1 original = {},    v1 transformed = {}", v1_orig, v1);
            //            println!("v2 original = {},    v2 transformed = {}", v2_orig, v2);
            //            println!("v3 original = {},    v3 transformed = {}", v3_orig, v3);
            context.draw_triangle(&v1, &v2, &v3, texture)
        }
    }

    pub fn draw_multi_core(
        &self,
        context: &mut RenderContext,
        view_projection: &Matrix,
        transform: &Matrix,
        texture: &Canvas,
    ) {
        let mvp = view_projection * transform;
        let num_cores = num_cpus::get();

        println!("using {} cores", num_cores);
        let start = Instant::now();

        let data = Arc::new(Mutex::new(context));

        let act_i: usize = 0;
        let act_i_mutex = Arc::new(Mutex::new(act_i));
        let max_i = self.indices.len() - 3;

        let pixels_per_thread = 99;

        let _ = crossbeam::scope(|scope| {
            let mut children = vec![];

            for _i in 0..num_cores {
                let cloned_data = Arc::clone(&data);
                let cloned_act_i = Arc::clone(&act_i_mutex);

                let mvp = mvp.clone();

                children.push(scope.spawn(move |_| {
                    let mut i: usize = 0;
                    let mut cnt_pixels = 0;

                    println!("   thread_id {:?}", thread::current().id());

                    while *cloned_act_i.lock().unwrap() < max_i {
                        cnt_pixels += pixels_per_thread;
                        if i < max_i {
                            let mut acti = cloned_act_i.lock().unwrap();
                            i = *acti;
                            *acti += pixels_per_thread;
                            // println!(
                            //     "   thread_id {:?},    i {}    maxi {}     acti = {},",
                            //     thread::current().id(),
                            //     i,
                            //     max_i,
                            //     acti
                            // );
                        }

                        if i > max_i {
                            i = max_i;
                        }
                        let iterations = if i + pixels_per_thread > max_i {
                            max_i - i
                        } else {
                            pixels_per_thread
                        };

                        for j in (i..i + iterations).step_by(3) {
                            let idx1 = self.indices.get(j).unwrap();
                            let idx2 = self.indices.get(j + 1).unwrap();
                            let idx3 = self.indices.get(j + 2).unwrap();

                            let v1_orig = &self.vertices.get(*idx1).unwrap();
                            let v1 = v1_orig.transform(&mvp, transform);
                            let v2_orig = &self.vertices.get(*idx2).unwrap();
                            let v2 = v2_orig.transform(&mvp, transform);
                            let v3_orig = &self.vertices.get(*idx3).unwrap();
                            let v3 = v3_orig.transform(&mvp, transform);

                            let mut ctx = cloned_data.lock().unwrap();
                            ctx.draw_triangle(&v1, &v2, &v3, texture)
                        }
                    }
                    (thread::current().id(), cnt_pixels)
                }));
            }

            for child in children {
                let dur = Instant::now() - start;
                let (thread_id, cnt_lines) = child.join().unwrap();
                println!(
                    "child thread {:?} finished. run for {:?} , processed {:?} pixels",
                    thread_id, dur, cnt_lines
                );
            }
            let dur = Instant::now() - start;
            println!("draw_mesh_multi_core took {:6.4} ms ", dur.as_millis() / 1000);
            data.lock().unwrap()
        })
        .unwrap();
    }
}
