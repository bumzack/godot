use utils::prelude::{ObjModel, ObjModelOps};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terrain = ObjModel::read_file("./res/terrain2.obj")?;

    println!("terrain.indices = {:?}", terrain.indices().len());
    println!("terrain.normals = {:?}", terrain.normals().len());
    println!("terrain.positions = {:?}", terrain.positions().len());
    println!("terrain.has_normals = {:?}", terrain.has_normals());
    println!("terrain.has_tex_coords = {:?}", terrain.has_tex_coords());

    Ok(())
}
