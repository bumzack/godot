use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
// source for graph impl
// https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
use std::ops::{Add, Mul};
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::{exec, parse};
use graphviz_rust::printer::PrinterContext;

use petgraph::dot::{Config, Dot};
use petgraph::Graph;
use petgraph::prelude::*;

use crate::ValueRefV1;

pub fn draw_graph<T>(root: ValueRefV1<T>,filename:String)
    where
        T: Clone + Add + Mul + Add<Output=T> + Mul<Output=T> + Default + Debug,
{
    let graph = crate_petgraph(root);

    let graphviz_graph  =parse(&format!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel]))).unwrap();
    let graph_svg = exec(graphviz_graph, &mut PrinterContext::default(), vec![
        CommandArg::Format(Format::Svg),
    ]).unwrap();

    save_svg(graph_svg, filename);
}


pub fn crate_petgraph<T>(root: ValueRefV1<T>) -> Graph<String, String>
    where
        T: Clone + Add + Mul + Add<Output=T> + Mul<Output=T> + Default + Debug,
{
    let mut g: petgraph::Graph<String, String> = Graph::new();
    let s = format!("label {:?}, data: {:?}", root.borrow().data(), root.borrow().label());
    let node_index = g.add_node(s);
    add_nodes_petgraph(&mut g, &root, node_index);

    g
}

fn add_nodes_petgraph<T>(graph: &mut Graph<String, String>, node: &ValueRefV1<T>, parent_index: NodeIndex)
    where
        T: Clone + Add + Mul + Add<Output=T> + Mul<Output=T> + Default + Debug,
{
    for c in node.borrow().children() {
        let s = format!("label {:?}, data: {:?}", c.borrow().data(), c.borrow().label());
        let child_index = graph.add_node(s);
        graph.add_edge(child_index, parent_index, "".to_string());
        add_nodes_petgraph(graph, c, child_index);
    }
}


pub fn crate_graph<T>(root: ValueRefV1<T>) -> NNGraph
    where
        T: Clone + Add + Mul + Add<Output=T> + Mul<Output=T> + Default + Debug,
{
    let mut g = NNGraph::new();

    let value = format!("label {:?}", root.borrow().data());
    let label = format!("label {}", root.borrow().label());
    let grad = "grad".to_string();

    let node_index = g.add_node(label, value, grad);

    add_nodes(&mut g, &root, node_index);

    g
}

fn add_nodes<T>(graph: &mut NNGraph, node: &ValueRefV1<T>, parent_index: NNNodeIndex)
    where
        T: Clone + Add + Mul + Add<Output=T> + Mul<Output=T> + Default + Debug,
{
    for c in node.borrow().children() {
        let value = format!("label {:?}", c.borrow().data());
        let label = format!("label {}", c.borrow().label());
        let grad = "grad".to_string();

        let child_index = graph.add_node(label, value, grad);

        graph.add_edge(child_index, parent_index);
        add_nodes(graph, c, child_index);
    }
}

pub type NNNodeIndex = usize;
pub type NNEdgeIndex = usize;

#[derive(Debug)]
pub struct NNGraph {
    nodes: Vec<NNNode>,
    edges: Vec<NNEdge>,
}

#[derive(Debug)]
struct NNNode {
    first_outgoing_edge: Option<NNEdgeIndex>,
    label: String,
    value: String,
    grad: String,
}

#[derive(Debug)]
struct NNEdge {
    target: NNNodeIndex,
    next_outgoing_edge: Option<NNEdgeIndex>,
}

impl NNGraph {
    pub fn new() -> NNGraph {
        NNGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, label: String, value: String, grad: String) -> NNNodeIndex {
        let index = self.nodes.len();
        let node = NNNode {
            first_outgoing_edge: None,
            label,
            value,
            grad,
        };
        self.nodes.push(node);
        index
    }

    pub fn add_edge(&mut self, source: NNNodeIndex, target: NNNodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        let edge = NNEdge {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        };
        self.edges.push(edge);
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn successors(&self, source: NNNodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_index: first_outgoing_edge,
        }
    }
}

pub struct Successors<'graph> {
    graph: &'graph NNGraph,
    current_edge_index: Option<NNNodeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NNNodeIndex;

    fn next(&mut self) -> Option<NNNodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

//
// fn trace<T>(root: &ValueRefV1<T>)
// where
//     T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default,
// {
//     let mut nodes: Vec<AGraphNode> = vec![];
//     let mut edges: Vec<AEdge> = vec![];
// }
//
// fn build<T>(value: ValueRefV1<T>, nodes: &mut Vec<AGraphNode>, edges: &mut Vec<AEdge>)
// where
//     T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default,
// {
//     // if !nodes.contains(&n) {
//     //     let n = GraphNode {
//     //         label: value.borrow().get_label(),
//     //         value: format!("{}", value.borrow().data()),
//     //     };
//     //     nodes.push(n);
//     //     for c in value.borrow().get_children().iter {
//     //         edges.push(c, n);
//     //     }
//     // }
// }

fn save_svg(svg: String,filename:String) {
    let mut opt = usvg::Options::default();
    // Get file's absolute directory.
    let filename_svg = &format!("{}.svg", &filename);
    opt.resources_dir = std::fs::canonicalize(filename_svg)
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));
    opt.fontdb.load_system_fonts();

    let rtree = usvg::Tree::from_str(&svg, &opt.to_ref()).unwrap();

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &rtree,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
        .unwrap();

    let f = format!("/Users/bumzack/stoff/rust/godot/micrograd-rs/src/{}.png",filename);
    pixmap
        .save_png(f)
        .unwrap();
}

mod test {
    use super::*;

    #[test]
    fn test_graph() {
        let mut graph = NNGraph::new();

        let n0 = graph.add_node("".to_string(), "".to_string(), "".to_string());
        let n1 = graph.add_node("".to_string(), "".to_string(), "".to_string());
        let n2 = graph.add_node("".to_string(), "".to_string(), "".to_string());
        let n3 = graph.add_node("".to_string(), "".to_string(), "".to_string());

        graph.add_edge(n0, n1); // e0
        graph.add_edge(n1, n2); // e1
        graph.add_edge(n0, n3); // e2
        graph.add_edge(n3, n2); // e3

        let successors: Vec<NNNodeIndex> = graph.successors(n0).collect();
        assert_eq!(&successors[..], &[n3, n1]);
    }

    #[test]
    fn test_add() {
        let a = ValueRefV1::new_value(2.0 as f64, "a".to_string());
        let b = ValueRefV1::new_value(3.0, "b".to_string());

        let mut x = &a + &b;
        x.set_label("x".to_string());

        draw_graph(x, "test".to_string());
    }





}
