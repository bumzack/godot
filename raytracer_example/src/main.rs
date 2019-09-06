use std::error::Error;

use raytracer::{Backend, BackendCpu};
#[cfg(feature = "cuda")]
use raytracer::BackendCuda;
use raytracer_lib_std::canvas::canvas::CanvasOps;

pub mod dummy_world;
pub mod shadow_glamour_shot;
pub mod chapter14_with_aa;
pub mod test_soft_shadow_aka_area_light;
pub mod compare_to_cuda;

fn main() -> Result<(), Box<dyn Error>> {
    let w = 160;
    let h = 120;
    let size_factor = 0.5;
    let anitaliasing = true;
    let antialiasing_size = 3;


    #[cfg(feature = "cuda")]
        run_cuda_stuff(w, h, size_factor, anitaliasing, antialiasing_size);

    #[cfg(not(feature = "cuda"))]
        run_cpu_stuff(w, h, size_factor, anitaliasing, antialiasing_size);

    Ok(())
}


#[cfg(feature = "cuda")]
fn run_cuda_stuff(w: usize, h: usize, size_factor: f32, anitaliasing: bool, antialiasing_size: usize) {
    let filename_cuda = format!("cuda_{}x{}.png", w, h);
    let (mut w, c) = compare_to_cuda_world::setup_world(w, h);
    println!("\n\n---------- CUDA   --------------------");
    let backend_cuda = BackendCuda::new();
    let canvas = backend_cuda.render_world(&mut w, &c);
    canvas.unwrap().write_png(&filename_cuda);
}

fn run_cpu_stuff(w: usize, h: usize, size_factor: f32, anitaliasing: bool, antialiasing_size: usize) {
    let backend_cpu = BackendCpu::new();
    run_cpu_chapter14_with_aa(&backend_cpu, "cpu", w, h);
    run_cpu_compare_to_cuda(&backend_cpu, "cpu", w, h);
    run_cpu_shadow_glamour_shot(&backend_cpu, "cpu", size_factor, anitaliasing, antialiasing_size);
    run_cpu_soft_shadow(&backend_cpu, "cpu", size_factor, anitaliasing, antialiasing_size);
}

fn run_cpu_chapter14_with_aa(b: &dyn Backend, backend_name: &str, w: usize, h: usize) {
    let filename_cpu_single = format!("chapter14_with_aa_cpu_single_{}x{}.png", w, h);
    let (mut world, c) = chapter14_with_aa::setup_world_chapter14_with_aa(w, h);
    println!("\n\n---------- single core CPU    --------------------");
    let canvas = b.render_world(&mut world, &c);
    canvas.unwrap().write_png(&filename_cpu_single);

    let filename_cpu_multi = format!("chapter14_with_aa_cpu_multi_core_{}x{}.png", w, h);
    let (mut world, c) = chapter14_with_aa::setup_world_chapter14_with_aa(w, h);
    println!("\n\n---------- multi core  CPU    --------------------");
    let canvas = b.render_world_multi_core(&mut world, &c);
    canvas.unwrap().write_png(&filename_cpu_multi);
}

fn run_cpu_compare_to_cuda(b: &dyn Backend, backend_name: &str, w: usize, h: usize) {
    let filename_cpu_single = format!("compare_to_cuda_cpu_single_{}x{}.png", w, h);
    let (mut world, c) = compare_to_cuda::setup_world_compare_to_cuda(w, h);
    println!("\n\n---------- single core CPU    --------------------");
    let canvas = b.render_world(&mut world, &c);
    canvas.unwrap().write_png(&filename_cpu_single);

    let filename_cpu_multi = format!("compare_to_cuda_cpu_multi_core_{}x{}.png", w, h);
    let (mut world, c) = compare_to_cuda::setup_world_compare_to_cuda(w, h);
    println!("\n\n---------- multi core  CPU    --------------------");
    let canvas = b.render_world_multi_core(&mut world, &c);
    canvas.unwrap().write_png(&filename_cpu_multi);
}


fn run_cpu_shadow_glamour_shot(b: &dyn Backend, backend_name: &str, size_factor: f32, antialiasing: bool, antialiasing_size: usize) {
    let filename_cpu_single = format!("shadow_glamour_shotcpu_single_{:2}x{}x{}.png", size_factor, antialiasing, antialiasing_size);
    let (mut world, c) = shadow_glamour_shot::setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);
    println!("\n\n---------- single core CPU    --------------------");
    let canvas = b.render_world(&mut world, &c);
    canvas.unwrap().write_png(&filename_cpu_single);

    let filename_cpu_multi = format!("shadow_glamour_shot_cpu_multi_core_{:2}x{}x{}.png", size_factor, antialiasing, antialiasing_size);
    let (mut world, c) = shadow_glamour_shot::setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);
    println!("\n\n---------- multi core  CPU    --------------------");
    let canvas = b.render_world_multi_core(&mut world, &c);
    canvas.unwrap().write_png(&filename_cpu_multi);
}


fn run_cpu_soft_shadow(b: &dyn Backend, backend_name: &str, size_factor: f32, antialiasing: bool, antialiasing_size: usize) {
    let filename_cpu_single = format!("test_soft_shadow_cpu_single_{:2}x{}x{}.png", size_factor, antialiasing, antialiasing_size);
    let (mut world, c) = test_soft_shadow_aka_area_light::setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);
    println!("\n\n---------- single core CPU    --------------------");
    let canvas = b.render_world(&mut world, &c);
    canvas.unwrap().write_png(&filename_cpu_single);

    let filename_cpu_multi = format!("test_soft_shadow_cpu_multi_core_{:2}x{}x{}.png", size_factor, antialiasing, antialiasing_size);
    let (mut world, c) = test_soft_shadow_aka_area_light::setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);
    println!("\n\n---------- multi core  CPU    --------------------");
    let canvas = b.render_world_multi_core(&mut world, &c);
    canvas.unwrap().write_png(&filename_cpu_multi);
}
