use std::error::Error;

use raytracer::{Backend, BackendCpu};
#[cfg(feature = "cuda")]
use raytracer::BackendCuda;
use raytracer_lib_std::canvas::canvas::CanvasOps;

pub mod chapter14_with_aa;
pub mod compare_to_cuda;
pub mod dummy_world;
pub mod shadow_glamour_shot;
pub mod test_soft_shadow_aka_area_light;

fn main() -> Result<(), Box<dyn Error>> {
    let w = 3840;
    let h = 2160;
    let size_factor = 5.0;
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
    let backend_gpu = BackendCuda::new();
    run_cpu_chapter14_with_aa(&backend_gpu, true, w, h);
    run_cpu_compare_to_cuda(&backend_gpu, true, w, h);
    run_cpu_shadow_glamour_shot(&backend_gpu, true, size_factor, anitaliasing, antialiasing_size);
    run_cpu_soft_shadow(&backend_gpu, true, size_factor, anitaliasing, antialiasing_size);
}

fn run_cpu_stuff(w: usize, h: usize, size_factor: f32, anitaliasing: bool, antialiasing_size: usize) {
    let backend_cpu = BackendCpu::new();
    run_cpu_chapter14_with_aa(&backend_cpu, false, w, h);
    run_cpu_compare_to_cuda(&backend_cpu, false, w, h);
    run_cpu_shadow_glamour_shot(&backend_cpu, false, size_factor, anitaliasing, antialiasing_size);
    run_cpu_soft_shadow(&backend_cpu, false, size_factor, anitaliasing, antialiasing_size);
}

fn run_cpu_chapter14_with_aa(b: &dyn Backend, is_cuda: bool, w: usize, h: usize) {
    let backend_name = match is_cuda {
        true => "CUDA",
        false => "CPU",
    };

    if is_cuda {
        let filename_cpu_single = format!("{}_chapter14_with_aa_cpu_{}x{}.png", backend_name, w, h);
        let (mut world, c) = chapter14_with_aa::setup_world_chapter14_with_aa(w, h);
        println!(
            "{}",
            format!("---------- CUDA    chapter14_with_aa_  --------------------")
        );
        let canvas = b.render_world(&mut world, &c);
        canvas.unwrap().write_png(&filename_cpu_single).unwrap();
    } else {
        let filename_cpu_multi = format!("{}_chapter14_with_aa_cpu_multi_core_{}x{}.png", backend_name, w, h);
        let (mut world, c) = chapter14_with_aa::setup_world_chapter14_with_aa(w, h);
        println!("---------- multi core  CPU    --------------------");
        let canvas = b.render_world_multi_core(&mut world, &c);
        canvas.unwrap().write_png(&filename_cpu_multi).unwrap();
    }
}

fn run_cpu_compare_to_cuda(b: &dyn Backend, is_cuda: bool, w: usize, h: usize) {
    let backend_name = match is_cuda {
        true => "cuda",
        false => "cpu",
    };
    if is_cuda {
        let filename_cpu_single = format!("{}_compare_to_cuda_cpu_{}x{}.png", backend_name, w, h);
        let (mut world, c) = compare_to_cuda::setup_world_compare_to_cuda(w, h);
        println!(
            "{}",
            format!("\n\n---------- CUDA    compare_to_cuda   --------------------")
        );
        let canvas = b.render_world(&mut world, &c);
        canvas.unwrap().write_png(&filename_cpu_single).unwrap();
    } else {
        let filename_cpu_multi = format!("{}_compare_to_cuda_cpu_multi_core_{}x{}.png", backend_name, w, h);
        let (mut world, c) = compare_to_cuda::setup_world_compare_to_cuda(w, h);
        println!("\n\n---------- multi core  CPU    --------------------");
        let canvas = b.render_world_multi_core(&mut world, &c);
        canvas.unwrap().write_png(&filename_cpu_multi).unwrap();
    }
}

fn run_cpu_shadow_glamour_shot(
    b: &dyn Backend,
    is_cuda: bool,
    size_factor: f32,
    antialiasing: bool,
    antialiasing_size: usize,
) {
    let backend_name = match is_cuda {
        true => "cuda",
        false => "cpu",
    };

    if is_cuda {
        let filename_cpu_single = format!(
            "{}_shadow_glamour_shot_cpu_{:2}x{}x{}.png",
            backend_name, size_factor, antialiasing, antialiasing_size
        );
        let (mut world, c) =
            shadow_glamour_shot::setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);
        println!(
            "{}",
            format!("\n\n---------- CUDA    shadow_glamour_shot  --------------------")
        );
        let canvas = b.render_world(&mut world, &c);
        canvas.unwrap().write_png(&filename_cpu_single).unwrap();
    } else {
        let filename_cpu_multi = format!(
            "{}_shadow_glamour_shot_cpu_multi_core_{:2}x{}x{}.png",
            backend_name, size_factor, antialiasing, antialiasing_size
        );
        let (mut world, c) =
            shadow_glamour_shot::setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);
        println!("\n\n---------- multi core  CPU    --------------------");
        let canvas = b.render_world_multi_core(&mut world, &c);
        canvas.unwrap().write_png(&filename_cpu_multi).unwrap();
    }
}

fn run_cpu_soft_shadow(b: &dyn Backend, is_cuda: bool, size_factor: f32, antialiasing: bool, antialiasing_size: usize) {
    let backend_name = match is_cuda {
        true => "cuda",
        false => "cpu",
    };

    if is_cuda {
        let filename_cpu_single = format!(
            "{}_test_soft_shadow_cpu_{:2}x{}x{}.png",
            backend_name, size_factor, antialiasing, antialiasing_size
        );
        let (mut world, c) =
            test_soft_shadow_aka_area_light::setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);
        println!(
            "{}",
            format!("\n\n---------- CUDA    test_soft_shadow --------------------")
        );
        let canvas = b.render_world(&mut world, &c);
        canvas.unwrap().write_png(&filename_cpu_single).unwrap();
    } else {
        let filename_cpu_multi = format!(
            "{}_test_soft_shadow_cpu_multi_core_{:2}x{}x{}.png",
            backend_name, size_factor, antialiasing, antialiasing_size
        );
        let (mut world, c) =
            test_soft_shadow_aka_area_light::setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);
        println!("\n\n---------- multi core  CPU    --------------------");
        let canvas = b.render_world_multi_core(&mut world, &c);
        canvas.unwrap().write_png(&filename_cpu_multi).unwrap();
    }
}
