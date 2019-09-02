use std::error::Error;

use raytracer::{Backend, BackendCpu};
use raytracer::CanvasOps;

pub mod compare_to_cuda_world;
pub mod dummy_world;

fn main() -> Result<(), Box<dyn Error>> {
    let w = 80;
    let h = 60;

    //let filename_cuda = format!("cuda_{}x{}.png", w, h);
    let filename_cpu_single = format!("cpu_single_{}x{}.png", w, h);

    let (mut w, c) = compare_to_cuda_world::setup_world(w, h);

//    println!("\n\n---------- CUDA   --------------------");
//    let backend_cuda = BackendCuda::new();
//    let canvas = backend_cuda.render_world(&mut w, &c)?;
//    canvas.write_png(&filename_cuda);

    println!("\n\n---------- CPU    --------------------");
    let backend_cpu = BackendCpu::new();
    let canvas = backend_cpu.render_world(&mut w, &c)?;
    canvas.write_png(&filename_cpu_single)?;

    Ok(())
}
