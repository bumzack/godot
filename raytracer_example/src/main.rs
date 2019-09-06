use std::error::Error;

use raytracer::{Backend, BackendCpu, BackendCuda};
use raytracer_lib_std::canvas::canvas::CanvasOps;

pub mod compare_to_cuda_world;
pub mod dummy_world;

fn main() -> Result<(), Box<dyn Error>> {
    let w = 1280;
    let h = 720;

    #[cfg(feature = "cuda")]
        run_cuda(w, h);

    #[cfg(not(feature = "cuda"))]
        run_cpu(w, h);


    Ok(())
}

fn run_cuda(w: usize, h: usize) {
    let filename_cuda = format!("cuda_{}x{}.png", w, h);
    let (mut w, c) = compare_to_cuda_world::setup_world(w, h);
    println!("\n\n---------- CUDA   --------------------");
    let backend_cuda = BackendCuda::new();
    let canvas = backend_cuda.render_world(&mut w, &c);
    canvas.unwrap().write_png(&filename_cuda);
}

fn run_cpu(w: usize, h: usize) {
    let filename_cpu_single = format!("cpu_single_{}x{}.png", w, h);
    let (mut world, c) = compare_to_cuda_world::setup_world(w, h);
    println!("\n\n---------- single core CPU    --------------------");
    let backend_cpu = BackendCpu::new();
    let canvas = backend_cpu.render_world(&mut world, &c);
    canvas.unwrap().write_png(&filename_cpu_single);

    let filename_cpu_multi = format!("cpu_multi_core_{}x{}.png", w, h);
    let (mut world, c) = compare_to_cuda_world::setup_world(w, h);
    println!("\n\n---------- multi core  CPU    --------------------");
    let mut backend_cpu = BackendCpu::new();
    backend_cpu.enable_multicore();
    let canvas = backend_cpu.render_world(&mut world, &c);
    canvas.unwrap().write_png(&filename_cpu_multi);
}




