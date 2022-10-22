use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::printer::PrinterContext;
use graphviz_rust::{exec, parse};
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::ops::{Add, Mul};

use petgraph::dot::{Config, Dot};
use petgraph::prelude::*;
use petgraph::Graph;

use crate::ValueRefV1;

pub fn draw_graph<T>(root: ValueRefV1<T>, filename: String)
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default + Debug,
{
    let graph = crate_petgraph(root);

    let graphviz_graph = parse(&format!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel]))).unwrap();
    let graph_svg = exec(
        graphviz_graph,
        &mut PrinterContext::default(),
        vec![CommandArg::Format(Format::Svg)],
    )
    .unwrap();

    save_svg(graph_svg, filename);
}

pub fn crate_petgraph<T>(root: ValueRefV1<T>) -> Graph<String, String>
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default + Debug,
{
    let mut g: petgraph::Graph<String, String> = Graph::new();
    let s = format_value_v1(&root);
    let node_index = g.add_node(s);
    add_nodes_petgraph(&mut g, &root, node_index);

    g
}

fn format_value_v1<T>(root: &ValueRefV1<T>) -> String
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default + Debug,
{
    format!(
        "{} | {:?} | {}",
        root.borrow().label(),
        root.borrow().data(),
        root.borrow().op()
    )
}

fn add_nodes_petgraph<T>(graph: &mut Graph<String, String>, node: &ValueRefV1<T>, parent_index: NodeIndex)
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default + Debug,
{
    for c in node.borrow().children() {
        let s = format_value_v1(&c);
        let child_index = graph.add_node(s);
        graph.add_edge(child_index, parent_index, "".to_string());
        add_nodes_petgraph(graph, c, child_index);
    }
}

fn save_svg(svg: String, filename: String) {
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

    let f = format!("/Users/bumzack/stoff/rust/godot/micrograd-rs/src/{}.png", filename);
    pixmap.save_png(f).unwrap();
}

mod test {
    use super::*;

    #[test]
    fn test_add() {
        let a = ValueRefV1::new_value(2.0 as f64, "a".to_string());
        let b = ValueRefV1::new_value(3.0, "b".to_string());

        let mut x = &a + &b;
        x.set_label("x".to_string());

        draw_graph(x, "test".to_string());
    }
}
