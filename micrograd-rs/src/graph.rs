use std::ops::{Add, Mul};

use graphviz_rust::attributes::*;
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::printer::PrinterContext;
use graphviz_rust::{exec, parse};
use resvg;
use usvg;

use crate::ValueRefV1;

pub fn draw_graph<T>(root: ValueRefV1<T>)
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default,
{
    let mut g = graph!(id!("id");
         node!("nod"),
         subgraph!("sb";
             edge!(node_id!("a") => subgraph!(;
                node!("n";
                NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::egg))
            ))
        ),
        edge!(node_id!("a1") => node_id!(esc "a2"))
    );
    let graph_svg = exec(g, &mut PrinterContext::default(), vec![CommandArg::Format(Format::Svg)]).unwrap();

    save_svg(graph_svg);
}

struct AGraphNode {
    label: String,
    value: String,
}

struct AEdge<'a> {
    from: &'a AGraphNode,
    to: &'a AGraphNode,
}

fn trace<T>(root: &ValueRefV1<T>)
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default,
{
    let mut nodes: Vec<AGraphNode> = vec![];
    let mut edges: Vec<AEdge> = vec![];
}

fn build<T>(value: ValueRefV1<T>, nodes: &mut Vec<AGraphNode>, edges: &mut Vec<AEdge>)
where
    T: Clone + Add + Mul + Add<Output = T> + Mul<Output = T> + Default,
{
    // if !nodes.contains(&n) {
    //     let n = GraphNode {
    //         label: value.borrow().get_label(),
    //         value: format!("{}", value.borrow().data()),
    //     };
    //     nodes.push(n);
    //     for c in value.borrow().get_children().iter {
    //         edges.push(c, n);
    //     }
    // }
}

fn save_svg(svg: String) {
    let mut opt = usvg::Options::default();
    // Get file's absolute directory.
    opt.resources_dir = std::fs::canonicalize("test.svg")
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
    pixmap
        .save_png("/Users/gsc/stoff/lernen/godot/micrograd-rs/src/test.png")
        .unwrap();
}
