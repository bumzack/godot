use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::indexed_model::IndexedModel;
use crate::math::{Tuple, Tuple4D};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
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

    pub fn read_file(filename: &str) -> Result<ObjModel, Box<dyn std::error::Error>> {
       println!("read_file {}", filename);
        let mut obj_model = ObjModel::new();

        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = line?;

            let tokens: Vec<&str> = l.split(" ").collect();
            let tokens: Vec<String> = tokens.iter().map(|t| t.to_string()).filter(|t| !t.is_empty()).collect();

            if tokens.len() == 0 || tokens[0].eq("#") {
                continue;
            } else if tokens[0].eq("v") {
                obj_model.positions.push(Tuple4D::new(
                    tokens[1].parse::<f32>().unwrap(),
                    tokens[2].parse::<f32>().unwrap(),
                    tokens[3].parse::<f32>().unwrap(),
                    1.0,
                ));
            } else if tokens[0].eq("vt") {
                obj_model.tex_coords.push(Tuple4D::new(
                    tokens[1].parse::<f32>().unwrap(),
                    1.0 - tokens[2].parse::<f32>().unwrap(),
                    0.0,
                    0.0,
                ));
            } else if tokens[0].eq("vn") {
                obj_model.normals.push(Tuple4D::new(
                    tokens[1].parse::<f32>().unwrap(),
                    tokens[2].parse::<f32>().unwrap(),
                    tokens[3].parse::<f32>().unwrap(),
                    0.0,
                ));
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

    pub fn to_indexed_model(self) -> IndexedModel {
        let mut result = IndexedModel::new();
        let mut normal_model = IndexedModel::new();

        let mut result_index_map: HashMap<ObjIndex, usize> = HashMap::new();
        let mut normal_index_map: HashMap<usize, usize> = HashMap::new();
        let mut index_map: HashMap<usize, usize> = HashMap::new();

        println!("to_indexed_model      self.indices.len() = {}", self.indices.len());
        println!("to_indexed_model      self.tex_coords.len() = {}", self.tex_coords.len());
        println!("to_indexed_model      self.normals.len() = {}", self.normals.len());
        println!("to_indexed_model      self.positions.len() = {}", self.positions.len());

        for i in 0..self.indices.len() {
            let current_index = self.indices.get(i).unwrap().clone();
            let current_position = self.positions.get(current_index.vertex_index).unwrap();
            let current_tex_coord;
            let current_normal;

            if self.has_tex_coords {
                current_tex_coord = *self.tex_coords.get(current_index.tex_coord_index).unwrap();
            } else {
                current_tex_coord = Tuple4D::new_point(0.0, 0.0, 0.0);
            }

            if self.has_normals {
                current_normal = *self.normals.get(current_index.normal_index).unwrap();
            } else {
                current_normal = Tuple4D::new_vector(0.0, 0.0, 0.0);
            }

            let model_vertex_index;

            if !result_index_map.contains_key(&current_index) {
                model_vertex_index = result.positions().len();
                result_index_map.insert(current_index, model_vertex_index);

                result.positions_mut().push(*current_position);
                result.tex_coords_mut().push(current_tex_coord);
                if self.has_normals {
                    result.normals_mut().push(current_normal);
                }
            } else {
                model_vertex_index = *result_index_map.get(&current_index).unwrap();
            }

            let normal_model_index;
            if !normal_index_map.contains_key(&current_index.vertex_index) {
                normal_model_index = normal_model.positions().len();
                normal_index_map.insert(current_index.vertex_index, normal_model_index);

                normal_model.positions_mut().push(*current_position);
                normal_model.tex_coords_mut().push(current_tex_coord);
                normal_model.normals_mut().push(current_normal);
                normal_model.tangents_mut().push(Tuple4D::new_vector(0.0, 0.0, 0.0));
            } else {
                normal_model_index = *normal_index_map.get(&current_index.vertex_index).unwrap();
            }

            result.indices_mut().push(model_vertex_index);
            normal_model.indices_mut().push(normal_model_index);
            index_map.insert(model_vertex_index, normal_model_index);
        }

        if !self.has_normals {
            normal_model.calc_normals();

            for i in 0..result.positions().len() {
                let n = normal_model.normals().get(*index_map.get(&i).unwrap()).unwrap();
                result.normals_mut().push(*n);
            }
        }

        normal_model.calc_tangents();
        for i in 0..result.positions().len() {
            let t = normal_model.tangents().get(*index_map.get(&i).unwrap()).unwrap();
            result.tangents_mut().push(*t);
        }

        result
    }

    pub fn parse_obj_index(&mut self, token: &str) {
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

//impl Eq for ObjIndex {}
//
//impl PartialEq for ObjIndex {
//    fn eq(&self, other: &Self) -> bool {
//        self.vertex_index == other.vertex_index && self.tex_coord_index == other.tex_coord_index && self.normal_index == other.normal_index
//    }
//}
//
