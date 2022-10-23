use std::fmt::Debug;
use std::ops::{Add, Mul};

use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::printer::PrinterContext;
use graphviz_rust::{exec, parse};
use petgraph::dot::{Config, Dot};
use petgraph::prelude::*;
use petgraph::Graph;

use crate::micrograd_rs_v2::OpEnumV2;
use crate::ValueRefV2;

pub fn draw_graph(root: ValueRefV2, filename: String) {
    let graph = create_petgraph(root);

    let graphviz_graph = parse(&format!("{}",




                                        Dot::with_config(&graph, &[Config::EdgeNoLabel]))).unwrap();
    let graph_svg = exec(
        graphviz_graph,
        &mut PrinterContext::default(),
        vec![CommandArg::Format(Format::Svg)],
    )
    .unwrap();

    save_svg(graph_svg, filename);
}

pub fn create_petgraph(root: ValueRefV2) -> Graph<String, String> {
    let mut g: petgraph::Graph<String, String> = Graph::new();
    let s = format_value_v2(&root);
    let node_index = g.add_node(s);
    add_nodes_petgraph(&mut g, &root, node_index, root.borrow().op());

    g
}

fn add_nodes_petgraph(graph: &mut Graph<String, String>, node: &ValueRefV2, parent_index: NodeIndex, op: &OpEnumV2) {
    let p_idx = match op {
        // all operations ...
        OpEnumV2::NONE => parent_index,
        _ => {
            let i = graph.add_node(format!("{}", op));
            graph.add_edge(i, parent_index, "".to_string());
            i
        }
    };
    for c in node.borrow().children() {
        let s = format_value_v2(&c);
        let child_index = graph.add_node(s);
        graph.add_edge(child_index, p_idx, "".to_string());
        add_nodes_petgraph(graph, c, child_index, c.borrow().op());
    }
}

fn format_value_v2(root: &ValueRefV2) -> String {
    format!(
        "{} | data {:.4} | grad {:.4} ",
        root.borrow().label(),
        root.borrow().data(),
        root.borrow().grad(),
    )
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
    use crate::micrograd_rs_v2::assert_two_float;
    use crate::{draw_graph, ValueRefV2};

    #[test]
    fn test_add() {
        let a = ValueRefV2::new_value(2.0 as f64, "a".to_string());
        let b = ValueRefV2::new_value(3.0, "b".to_string());

        let mut x = &a + &b;
        x.set_label("x".to_string());

        draw_graph(x, "test_add".to_string());
    }

    // before starting to add grad
    // https://youtu.be/VMj-3S1tku0?t=1875
    #[test]
    pub fn test_video() {
        let a = ValueRefV2::new_value(2.0 as f64, "a".to_string());
        let b = ValueRefV2::new_value(-3.0, "b".to_string());
        let c = ValueRefV2::new_value(10.0, "c".to_string());
        let f = ValueRefV2::new_value(-2.0, "f".to_string());

        let mut e = &a * &b;
        e.set_label("e".to_string());

        let mut d = &e + &c;
        d.set_label("d".to_string());

        let mut l = &d * &f;
        l.set_label("L".to_string());

        println!("a {}", a);
        println!("b {}", b);
        println!("c {}", c);
        println!("d {}", d);
        println!("e {}", e);
        println!("f {}", f);
        println!("l {}", l);

        assert_two_float(*l.borrow().data(), -8.0);

        draw_graph(l, "test_video_before_grad".to_string());
    }
}
