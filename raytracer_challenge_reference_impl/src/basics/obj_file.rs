use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use crate::math::{Tuple, Tuple4D};
use crate::prelude::{Shape, ShapeEnum, ShapeIdx, Triangle};
use crate::shape::Group;
use crate::world::ShapeArr;

pub struct Parser {
    vertices: Vec<Tuple4D>,
    normals: Vec<Tuple4D>,
    triangles: Vec<Triangle>,
    named_groups: BTreeMap<String, Vec<Triangle>>,
}

impl Parser {
    fn new(
        vertices: Vec<Tuple4D>,
        triangles: Vec<Triangle>,
        named_groups: BTreeMap<String, Vec<Triangle>>,
        normals: Vec<Tuple4D>,
    ) -> Parser {
        Parser {
            vertices,
            triangles,
            named_groups,
            normals,
        }
    }

    fn get_vertices(&self) -> &Vec<Tuple4D> {
        &self.vertices
    }

    fn get_normals(&self) -> &Vec<Tuple4D> {
        &self.normals
    }

    fn get_triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

    fn get_named_groups(&self) -> &BTreeMap<String, Vec<Triangle>> {
        &self.named_groups
    }

    pub fn get_groups(&self, name: String, shapes: &mut ShapeArr) -> Vec<ShapeIdx> {
        let mut res = vec![];
        let n = name.clone();
        if !self.get_triangles().is_empty() {
            let g = Group::new(shapes, name);
            for (idx, t) in self.get_triangles().iter().enumerate() {
                let t1 = t.clone();
                let n = format!("group: {}  idx {}", &n, idx);
                let triangle = Shape::new_part_of_group(ShapeEnum::Triangle(t1), n);
                Group::add_child(shapes, g, triangle);
            }
            res.push(g);
        }
        if !self.get_named_groups().is_empty() {
            for (key, val) in self.get_named_groups().iter() {
                println!("key: {key} val: {:?}", val);
            }
            for (group_name, triangles) in self.get_named_groups().iter() {
                let g = Group::new(shapes, group_name.to_string());
                for (idx, t) in triangles.iter().enumerate() {
                    let t1 = t.clone();
                    let n = format!("group: {}  idx {}", &group_name, idx);
                    let triangle = Shape::new_part_of_group(ShapeEnum::Triangle(t1), n);
                    Group::add_child(shapes, g, triangle);
                }
                res.push(g);
            }
        }

        res
    }
}

pub trait ObjFileOps {
    fn parse_obj_file<'a>(filename: &'a str) -> Parser;
}

impl ObjFileOps for Parser {
    fn parse_obj_file<'a>(filename: &'a str) -> Parser {
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut triangles = Vec::new();
        let mut group_name = String::new();
        let mut groups = BTreeMap::new();
        match read_lines(filename) {
            Ok(lines) => {
                println!("ok");
                for line in lines {
                    match line {
                        Ok(ref l) => {
                            if l.trim().is_empty() {
                                // println!("skipping empty line    '{}'", &l);
                                continue;
                            }
                            println!("line    '{}'", &l);
                            let mut iter = l.as_str().split_whitespace();
                            let command = iter.next().unwrap();
                            match command {
                                "v" => {
                                    let x = iter.next().unwrap();
                                    let y = iter.next().unwrap();
                                    let z = iter.next().unwrap();

                                    let vertex = Tuple4D::new_point(
                                        str::parse::<f64>(x).unwrap(),
                                        str::parse::<f64>(y).unwrap(),
                                        str::parse::<f64>(z).unwrap(),
                                    );
                                    // println!("got a vertex   {:?}", &vertex);
                                    vertices.push(vertex);
                                }
                                "vn" => {
                                    let x = iter.next().unwrap();
                                    let y = iter.next().unwrap();
                                    let z = iter.next().unwrap();

                                    let normal = Tuple4D::new_vector(
                                        str::parse::<f64>(x).unwrap(),
                                        str::parse::<f64>(y).unwrap(),
                                        str::parse::<f64>(z).unwrap(),
                                    );
                                    // println!("got a vertex   {:?}", &vertex);
                                    normals.push(normal);
                                }
                                "f" => {
                                    let indices =
                                        iter.into_iter().map(|idx| str::parse::<usize>(idx).unwrap()).collect();
                                    // for i in &indices {
                                    //     println!("index {}", i);
                                    // }
                                    if group_name.is_empty() {
                                        fan_triangulation(&indices, &vertices, &mut triangles);
                                    } else {
                                        let mut group_triangles = Vec::new();
                                        fan_triangulation(&indices, &vertices, &mut group_triangles);

                                        if groups.get(&group_name).is_some() {
                                            let t: &mut Vec<Triangle> = groups.get_mut(&group_name).unwrap();
                                            t.append(&mut group_triangles);
                                        } else {
                                            groups.insert(group_name.to_string(), group_triangles);
                                        }
                                    }
                                }
                                "g" => {
                                    group_name = iter.next().unwrap().to_string();
                                    // println!("got a groupname    {:?}", &group_name);
                                }
                                _ => {}
                            }
                        }
                        Err(err) => {
                            println!("error unpacking line   err {}", err);
                            panic!("error unpacking line   err {}", err);
                        }
                    }
                }
            }
            Err(err) => {
                println!("can't open file {} erro {}", filename, err);
                panic!("cant open file {}  error {}", filename, err);
            }
        }
        Parser::new(vertices, triangles, groups, normals)
    }
}

fn fan_triangulation(indices: &Vec<usize>, vertices: &Vec<Tuple4D>, triangles: &mut Vec<Triangle>) {
    //  ¯\_(ツ)_/¯
    // FIXME this should be possible without the if
    if indices.len() == 3 {
        let p1 = vertices.get(indices.get(0).unwrap() - 1).unwrap();
        let p2 = vertices.get(indices.get(1).unwrap() - 1).unwrap();
        let p3 = vertices.get(indices.get(2).unwrap() - 1).unwrap();
        let t = Triangle::new(p1.clone(), p2.clone(), p3.clone());
        println!("triangle from 3 indices {:?}", &t);
        triangles.push(t);
    } else {
        for i in 2..indices.len() {
            let p1 = vertices.get(0).unwrap();
            let p2 = vertices.get(i - 1).unwrap();
            let p3 = vertices.get(i).unwrap();
            let t = Triangle::new(p1.clone(), p2.clone(), p3.clone());
            println!("triangle from more indices {:?}", &t);
            triangles.push(t);
        }
    }
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::math::{assert_tuple, Tuple};
    use crate::prelude::ShapeOps;

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

        let mut shapes = vec![];
        let vec1 = parser.get_groups("testgroup".to_string(), &mut shapes);
        let group = vec1.get(0).unwrap();
        let group = shapes.get(*group as usize).unwrap();

        println!("group {:?}", group);
        Group::print_tree(&shapes, 0, 0);

        let triangle1 = group.get_children().get(0).unwrap();
        let triangle2 = group.get_children().get(1).unwrap();

        let triangle1 = shapes.get(*triangle1 as usize).unwrap();
        let triangle2 = shapes.get(*triangle2 as usize).unwrap();

        let triangle1 = match triangle1.get_shape() {
            ShapeEnum::Triangle(t) => t,
            _ => panic!("unexpected shape"),
        };
        let triangle2 = match triangle2.get_shape() {
            ShapeEnum::Triangle(t) => t,
            _ => panic!("unexpected shape"),
        };

        assert_tuple(triangle1.get_p1(), &v1_expected);
        assert_tuple(triangle1.get_p2(), &v2_expected);
        assert_tuple(triangle1.get_p3(), &v3_expected);

        assert_tuple(triangle2.get_p1(), &v1_expected);
        assert_tuple(triangle2.get_p2(), &v3_expected);
        assert_tuple(triangle2.get_p3(), &v4_expected);
    }

    // page 215
    // parsing triangle faces
    #[test]
    fn test_parsing_polygon_data() {
        let filename = "./test_files/polygon_data.obj";
        let parser = Parser::parse_obj_file(&filename);

        let v1_expected = Tuple4D::new_point(-1.0, 1.0, 0.0);
        let v2_expected = Tuple4D::new_point(-1.0, 0.0, 0.0);
        let v3_expected = Tuple4D::new_point(1.0, 0.0, 0.0);
        let v4_expected = Tuple4D::new_point(1.0, 1.0, 0.0);
        let v5_expected = Tuple4D::new_point(0.0, 2.0, 0.0);

        assert_eq!(parser.vertices.len(), 5);
        assert_tuple(&parser.get_vertices()[0], &v1_expected);
        assert_tuple(&parser.get_vertices()[1], &v2_expected);
        assert_tuple(&parser.get_vertices()[2], &v3_expected);
        assert_tuple(&parser.get_vertices()[3], &v4_expected);
        assert_tuple(&parser.get_vertices()[4], &v5_expected);

        let t1 = parser.get_triangles().get(0).unwrap();
        let t2 = parser.get_triangles().get(1).unwrap();
        let t3 = parser.get_triangles().get(2).unwrap();

        assert_tuple(t1.get_p1(), &v1_expected);
        assert_tuple(t1.get_p2(), &v2_expected);
        assert_tuple(t1.get_p3(), &v3_expected);

        assert_tuple(t2.get_p1(), &v1_expected);
        assert_tuple(t2.get_p2(), &v3_expected);
        assert_tuple(t2.get_p3(), &v4_expected);

        assert_tuple(t3.get_p1(), &v1_expected);
        assert_tuple(t3.get_p2(), &v4_expected);
        assert_tuple(t3.get_p3(), &v5_expected);

        // group
        let mut shapes = vec![];
        let group = parser.get_groups("testgroup".to_string(), &mut shapes)[0];
        let group = shapes.get(group).unwrap();

        println!("group {:?}", group);
        Group::print_tree(&shapes, 0, 0);

        let triangle1 = group.get_children().get(0).unwrap();
        let triangle2 = group.get_children().get(1).unwrap();
        let triangle3 = group.get_children().get(2).unwrap();

        let triangle1 = shapes.get(*triangle1 as usize).unwrap();
        let triangle2 = shapes.get(*triangle2 as usize).unwrap();
        let triangle3 = shapes.get(*triangle3 as usize).unwrap();

        let triangle1 = match triangle1.get_shape() {
            ShapeEnum::Triangle(t) => t,
            _ => panic!("unexpected shape"),
        };
        let triangle2 = match triangle2.get_shape() {
            ShapeEnum::Triangle(t) => t,
            _ => panic!("unexpected shape"),
        };
        let triangle3 = match triangle3.get_shape() {
            ShapeEnum::Triangle(t) => t,
            _ => panic!("unexpected shape"),
        };

        assert_tuple(triangle1.get_p1(), &v1_expected);
        assert_tuple(triangle1.get_p2(), &v2_expected);
        assert_tuple(triangle1.get_p3(), &v3_expected);

        assert_tuple(triangle2.get_p1(), &v1_expected);
        assert_tuple(triangle2.get_p2(), &v3_expected);
        assert_tuple(triangle2.get_p3(), &v4_expected);

        assert_tuple(triangle3.get_p1(), &v1_expected);
        assert_tuple(triangle3.get_p2(), &v4_expected);
        assert_tuple(triangle3.get_p3(), &v5_expected);
    }

    // page 217
    // Triangles in groups
    #[test]
    fn test_triangles_in_groups() {
        let filename = "./test_files/group_data.obj";
        let parser = Parser::parse_obj_file(&filename);

        let v1_expected = Tuple4D::new_point(-1.0, 1.0, 0.0);
        let v2_expected = Tuple4D::new_point(-1.0, 0.0, 0.0);
        let v3_expected = Tuple4D::new_point(1.0, 0.0, 0.0);
        let v4_expected = Tuple4D::new_point(1.0, 1.0, 0.0);

        assert_eq!(parser.vertices.len(), 4);

        // group
        let mut shapes = vec![];
        let vec1 = parser.get_groups("testgroup".to_string(), &mut shapes);
        println!("######################################################################");
        Group::print_tree(&shapes, 0, 0);
        println!("######################################################################");
        Group::print_tree(&shapes, 2, 0);
        println!("######################################################################");

        let group1 = vec1.get(0).unwrap();
        let group1 = shapes.get(*group1 as usize).unwrap();
        let x1 = group1.get_name().as_ref().unwrap();
        assert_eq!(*x1, "FirstGroup".to_string());

        println!("group1 {:?}", group1);

        let group2 = vec1.get(1).unwrap();
        let group2 = shapes.get(*group2 as usize).unwrap();
        let x = group2.get_name().as_ref().unwrap();
        assert_eq!(*x, "SecondGroup".to_string());

        let triangle1 = group1.get_children().get(0).unwrap();
        let triangle2 = group2.get_children().get(0).unwrap();

        let triangle1 = shapes.get(*triangle1 as usize).unwrap();
        let triangle2 = shapes.get(*triangle2 as usize).unwrap();

        let triangle1 = match triangle1.get_shape() {
            ShapeEnum::Triangle(t) => t,
            _ => panic!("unexpected shape"),
        };
        let triangle2 = match triangle2.get_shape() {
            ShapeEnum::Triangle(t) => t,
            _ => panic!("unexpected shape"),
        };

        assert_tuple(triangle1.get_p1(), &v1_expected);
        assert_tuple(triangle1.get_p2(), &v2_expected);
        assert_tuple(triangle1.get_p3(), &v3_expected);

        assert_tuple(triangle2.get_p1(), &v1_expected);
        assert_tuple(triangle2.get_p2(), &v3_expected);
        assert_tuple(triangle2.get_p3(), &v4_expected);
    }

    // page 217
    // Triangles in groups - with a second triangle in the first group
    #[test]
    fn test_triangles_in_groups_additional_triangle() {
        let filename = "./test_files/group_data_multiple_triangles.obj";
        let parser = Parser::parse_obj_file(&filename);

        let v1_expected = Tuple4D::new_point(-1.0, 1.0, 0.0);
        let v2_expected = Tuple4D::new_point(-1.0, 0.0, 0.0);
        let v3_expected = Tuple4D::new_point(1.0, 0.0, 0.0);
        let v4_expected = Tuple4D::new_point(1.0, 1.0, 0.0);

        assert_eq!(parser.vertices.len(), 4);

        // group
        let mut shapes = vec![];
        let vec1 = parser.get_groups("testgroup".to_string(), &mut shapes);
        println!("######################################################################");
        Group::print_tree(&shapes, 0, 0);
        println!("######################################################################");
        Group::print_tree(&shapes, 3, 0);
        println!("######################################################################");

        let group1 = vec1.get(0).unwrap();
        let group1 = shapes.get(*group1 as usize).unwrap();
        let x1 = group1.get_name().as_ref().unwrap();
        assert_eq!(*x1, "FirstGroup".to_string());

        println!("group1 {:?}", group1);
        Group::print_tree(&shapes, 0, 0);

        let group2 = vec1.get(1).unwrap();
        let group2 = shapes.get(*group2 as usize).unwrap();
        let x = group2.get_name().as_ref().unwrap();
        assert_eq!(*x, "SecondGroup".to_string());

        let triangle1 = group1.get_children().get(0).unwrap();
        let triangle1a = group1.get_children().get(1).unwrap();

        let triangle2 = group2.get_children().get(0).unwrap();

        let triangle1 = shapes.get(*triangle1 as usize).unwrap();
        let triangle1a = shapes.get(*triangle1a as usize).unwrap();
        let triangle2 = shapes.get(*triangle2 as usize).unwrap();

        let triangle1 = match triangle1.get_shape() {
            ShapeEnum::Triangle(t) => t,
            _ => panic!("unexpected shape"),
        };
        let triangle1a = match triangle1a.get_shape() {
            ShapeEnum::Triangle(t) => t,
            _ => panic!("unexpected shape"),
        };
        let triangle2 = match triangle2.get_shape() {
            ShapeEnum::Triangle(t) => t,
            _ => panic!("unexpected shape"),
        };

        assert_tuple(triangle1.get_p1(), &v1_expected);
        assert_tuple(triangle1.get_p2(), &v2_expected);
        assert_tuple(triangle1.get_p3(), &v3_expected);

        assert_tuple(triangle1a.get_p1(), &v2_expected);
        assert_tuple(triangle1a.get_p2(), &v3_expected);
        assert_tuple(triangle1a.get_p3(), &v4_expected);

        assert_tuple(triangle2.get_p1(), &v1_expected);
        assert_tuple(triangle2.get_p2(), &v3_expected);
        assert_tuple(triangle2.get_p3(), &v4_expected);
    }

    // page 223
    // Vertex normal records
    #[test]
    fn test_vertex_normal_records() {
        let filename = "./test_files/vertex_normal_records.obj";
        let parser = Parser::parse_obj_file(&filename);

        let n1_expected = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let n2_expected = Tuple4D::new_vector(0.707, 0.0, -0.707);
        let n3_expected = Tuple4D::new_vector(1.0, 2.0, 3.0);

        assert_eq!(parser.get_normals().len(), 3);

        assert_tuple(&parser.get_normals()[0], &n1_expected);
        assert_tuple(&parser.get_normals()[1], &n2_expected);
        assert_tuple(&parser.get_normals()[2], &n3_expected);
    }
}
