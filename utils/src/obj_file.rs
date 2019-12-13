use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::indexed_model::IndexedModel;
use crate::math::{Tuple, Tuple4D};

#[derive(Debug)]
pub struct ObjIndex {
    vertex_index: usize,
    tex_coord_index: usize,
    normal_index: usize,
}

#[derive(Debug)]
pub struct ObjModel {
    positions: Vec<Tuple4D>,
    tex_coords: Vec<Tuple4D>,
    normals: Vec<Tuple4D>,
    indices: Vec<ObjIndex>,
    has_tex_coords: bool,
    has_normals: bool,
}

impl ObjModel {
    fn new() -> ObjModel {
        ObjModel {
            positions: vec![],
            tex_coords: vec![],
            normals: vec![],
            indices: vec![],
            has_tex_coords: false,
            has_normals: false,
        }
    }

    pub fn positions(&self) -> &Vec<Tuple4D> {
        &self.positions
    }

    pub fn tex_coords(&self) -> &Vec<Tuple4D> {
        &self.tex_coords
    }

    pub fn normals(&self) -> &Vec<Tuple4D> {
        &self.normals
    }

    pub fn indices(&self) -> &Vec<ObjIndex> {
        &self.indices
    }

    pub fn has_tex_coords(&self) -> bool {
        self.has_tex_coords
    }

    pub fn has_normals(&self) -> bool {
        self.has_normals
    }
}

pub trait ObjModelOps {
    fn read_file(filename: &str) -> Result<ObjModel, Box<dyn std::error::Error>>;
    fn to_indexed_model(&self) -> IndexedModel;
    fn parse_obj_index(&mut self, token: &str);
}

impl ObjModelOps for ObjModel {
    fn read_file(filename: &str) -> Result<ObjModel, Box<dyn std::error::Error>> {
        let mut obj_model = ObjModel::new();

        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = line?;

            let tokens: Vec<&str> = l.split(" ").collect();
            let tokens: Vec<String> = tokens.iter()
                .map(|t| t.to_string())
                .filter(|t| !t.is_empty())
                .collect();

            if tokens.len() == 0 || tokens[0].eq("#") {
                continue;
            } else if tokens[0].eq("v") {
                obj_model.positions.push(Tuple4D::new(tokens[1].parse::<f32>().unwrap(),
                                                      tokens[2].parse::<f32>().unwrap(),
                                                      tokens[3].parse::<f32>().unwrap(), 1.0));
            } else if tokens[0].eq("vt") {
                obj_model.tex_coords.push(Tuple4D::new(tokens[1].parse::<f32>().unwrap(),
                                                       1.0 - tokens[2].parse::<f32>().unwrap(),
                                                       0.0, 0.0));
            } else if tokens[0].eq("vn") {
                obj_model.normals.push(Tuple4D::new(tokens[1].parse::<f32>().unwrap(),
                                                    tokens[2].parse::<f32>().unwrap(),
                                                    tokens[3].parse::<f32>().unwrap(), 0.0));
            } else if tokens[0].eq("f") {
                for i in 0..tokens.len() - 3 {
                    obj_model.parse_obj_index(&tokens[1]);
                    obj_model.parse_obj_index(&tokens[2 + i]);
                    obj_model.parse_obj_index(&tokens[3 + i]);
                }
            }
        }

        Ok(obj_model)
    }

    fn to_indexed_model(&self) -> IndexedModel {
        unimplemented!()
    }

    fn parse_obj_index(&mut self, token: &str) {
        let t = token.to_string();
        let values: Vec<&str> = t.split("/").collect();

        let mut tex_coord_index = 0;
        let mut normal_index = 0;

        let vertex_index = values[0].parse::<usize>().unwrap() - 1;

        if values.len() > 1 {
            self.has_tex_coords = true;
            tex_coord_index = values[1].parse::<usize>().unwrap() - 1;
        }
        if values.len() > 2 {
            self.has_normals = true;
            normal_index = values[2].parse::<usize>().unwrap() - 1;
        }

        self.indices.push(ObjIndex {
            vertex_index,
            tex_coord_index,
            normal_index,
        });
    }
}

