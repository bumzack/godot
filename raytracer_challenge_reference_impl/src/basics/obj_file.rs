use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use crate::math::{Tuple, Tuple4D};
use crate::prelude::{Shape, ShapeEnum, Triangle};
 use crate::shape::{Group};

pub struct Parser {
    vertices: Vec<Tuple4D>,
    triangles: Vec<Triangle>,
}

impl Parser {
    fn new(vertices: Vec<Tuple4D>, triangles: Vec<Triangle>) -> Parser {
        Parser {
            vertices,
            triangles,
        }
    }

    fn get_vertices(&self) -> &Vec<Tuple4D> {
        &self.vertices
    }

    fn get_triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

    fn get_default_group(&self, name: String) -> Shape {
        let n = name.clone();
         let mut  g = Group::new( name);

        for (idx,t) in self.get_triangles().iter().enumerate() {
            let t1 = t.clone();
            let n = format!("group: {}  idx {}", &n, idx);
            let triangle = Shape::new_part_of_group(ShapeEnum::Triangle(t1),n);
            Group::add_child(  &mut g,   triangle);
        }
        g
    }
}

pub trait ObjFileOps {
    fn parse_obj_file<'a>(filename: &'a str) -> Parser;
}

impl ObjFileOps for Parser {
    fn parse_obj_file<'a>(filename: &'a str) -> Parser {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        if let Ok(lines) = read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                match line {
                    Ok(ref l) => {
                        if l.trim().is_empty() {
                            continue;
                        }
                        let mut iter = l.as_str().split_whitespace();
                        let command = iter.next().unwrap();
                        match command {
                            "v" => {
                                let x = iter.next().unwrap();
                                let y = iter.next().unwrap();
                                let z = iter.next().unwrap();

                                vertices.push(Tuple4D::new_point(str::parse::<f64>(x).unwrap(), str::parse::<f64>(y).unwrap(), str::parse::<f64>(z).unwrap()));
                            }
                            "f" => {
                                let p1_idx = str::parse::<usize>(iter.next().unwrap()).unwrap();
                                let p2_idx = str::parse::<usize>(iter.next().unwrap()).unwrap();
                                let p3_idx = str::parse::<usize>(iter.next().unwrap()).unwrap();

                                let p1 = vertices.get(p1_idx - 1).unwrap();
                                let p2 = vertices.get(p2_idx - 1).unwrap();
                                let p3 = vertices.get(p3_idx - 1).unwrap();

                                let t = Triangle::new(p1.clone(), p2.clone(), p3.clone());
                                triangles.push(t);
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {}
                }

                // res.push(ip);
            }
        }
        Parser::new(vertices, triangles)
    }
}


// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[cfg(test)]
mod tests {
    use crate::math::{assert_tuple, Tuple};

    use super::*;

    // page 213
    // Ignoring unrecognized lines
    #[test]
    fn test_gibberish() {
        let filename = "./test_files/test_gibberish.obj";
        let parser = Parser::parse_obj_file(&filename);


        assert_eq!(parser.triangles.len(), 0);
        assert_eq!(parser.vertices.len(), 0);
    }

    // page 214
    // Vertex records
    #[test]
    fn test_vertex_records() {
        let filename = "./test_files/vertex_records.obj";
        let parser = Parser::parse_obj_file(&filename);


        let v1_expected = Tuple4D::new_point(-1.0, 1.0, 0.0);
        let v2_expected = Tuple4D::new_point(-1.0, 0.5, 0.0);
        let v3_expected = Tuple4D::new_point(1.0, 0.0, 0.0);
        let v4_expected = Tuple4D::new_point(1.0, 1.0, 0.0);

        assert_eq!(parser.vertices.len(), 4);
        assert_tuple(&parser.get_vertices()[0], &v1_expected);
        assert_tuple(&parser.get_vertices()[1], &v2_expected);
        assert_tuple(&parser.get_vertices()[2], &v3_expected);
        assert_tuple(&parser.get_vertices()[3], &v4_expected);
    }

    // page 214
    // parsing triangle faces
    #[test]
    fn test_triangle_faces() {
        let filename = "./test_files/triangle_records.obj";
        let parser = Parser::parse_obj_file(&filename);


        let v1_expected = Tuple4D::new_point(-1.0, 1.0, 0.0);
        let v2_expected = Tuple4D::new_point(-1.0, 0.0, 0.0);
        let v3_expected = Tuple4D::new_point(1.0, 0.0, 0.0);
        let v4_expected = Tuple4D::new_point(1.0, 1.0, 0.0);

        assert_eq!(parser.vertices.len(), 4);
        assert_tuple(&parser.get_vertices()[0], &v1_expected);
        assert_tuple(&parser.get_vertices()[1], &v2_expected);
        assert_tuple(&parser.get_vertices()[2], &v3_expected);
        assert_tuple(&parser.get_vertices()[3], &v4_expected);

        let t1 = parser.get_triangles().get(0).unwrap();
        let t2 = parser.get_triangles().get(1).unwrap();
        assert_tuple(t1.get_p1(), &v1_expected);
        assert_tuple(t1.get_p2(), &v2_expected);
        assert_tuple(t1.get_p3(), &v3_expected);

        assert_tuple(t2.get_p1(), &v1_expected);
        assert_tuple(t2.get_p2(), &v3_expected);
        assert_tuple(t2.get_p3(), &v4_expected);
    }
}