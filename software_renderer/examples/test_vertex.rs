
use math::{Tuple4D, Tuple};
use raytracer_lib_std::{Canvas, CanvasOpsStd};
use software_renderer::prelude::{Vertex, Gradient, Edge};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pos1 = Tuple4D::new_point(1.0, 2.0, 3.0);
    let pos2 = Tuple4D::new_point(2.0, 3.0, 4.0);
    let pos3 = Tuple4D::new_point(3.0, 4.0, 5.0);
    let tex_coords = Tuple4D::new_point(2.0, 2.0, 3.0);
    let normal = Tuple4D::new_vector(3.0, 2.0, 3.0);
    let v1 = Vertex::new(pos1, tex_coords.clone(), normal.clone());
    let v2 = Vertex::new(pos2, tex_coords.clone(), normal.clone());
    let v3 = Vertex::new(pos3, tex_coords.clone(), normal.clone());

    let gradient = Gradient::new(&v1, &v2, &v3);
    let edge = Edge::new(gradient, v1, v3, 0);
    println!("edge = {:?}", edge);

    let bitmap = Canvas::read_bitmap("./res/bricks.jpg")?;

    Ok(())
}
