#[cfg(feature = "cpu_single_core")]
use raytracer::BackendCpuSingleCore;

#[cfg(feature = "cpu_multi_core")]
use raytracer::BackendCpuMultiCore;

#[cfg(feature = "cuda")]
use raytracer::BackendCuda;

#[cfg(feature = "wasm")]
use raytracer::BackendWasm;

use raytracer::{Backend, BackendEnum, BackendOps};
use raytracer_lib_std::CanvasOpsStd;
use std::error::Error;

pub mod chapter14_with_aa;
pub mod compare_to_cuda;
pub mod dummy_world;
pub mod shadow_glamour_shot;
pub mod test_soft_shadow_aka_area_light;

fn main() -> Result<(), Box<dyn Error>> {
    let backend = Backend::new();

    println!("available Backends:   {}", backend.get_available_backends().len());
    backend
        .get_available_backends()
        .iter()
        .for_each(|b| println!("backend: {}", b));

    let w = 1280;
    let h = 720;
    let size_factor = 1.0;
    let anitaliasing = true;
    let antialiasing_size = 3;

    let mut backend_name: String = "".to_string();

    #[cfg(feature = "cpu_single_core")]
    let (backend, backend_name) = get_single_core(&backend);

    #[cfg(feature = "cpu_multi_core")]
    let (backend, backend_name) = get_multi_core(&backend);

    #[cfg(feature = "cuda")]
    let (backend, backend_name) = get_cuda(&backend);

    #[cfg(feature = "wasm")]
    let (backend, backend_name) = get_wasm(&backend);

    run_stuff(
        backend,
        backend_name,
        w,
        h,
        size_factor,
        anitaliasing,
        antialiasing_size,
    );

    Ok(())
}

#[cfg(feature = "cpu_single_core")]
fn get_single_core(b: &Backend) -> (Box<dyn BackendOps>, String) {
    let backend = b.get_backend(BackendEnum::CpuSingleCore).unwrap();
    let backend_name = "cpu_single_core".to_string();
    (backend, backend_name)
}

#[cfg(feature = "cpu_multi_core")]
fn get_multi_core(b: &Backend) -> (Box<dyn BackendOps>, String) {
    let backend = b.get_backend(BackendEnum::CpuMultiCore).unwrap();
    let backend_name = "cpu_multi_core ".to_string();
    (backend, backend_name)
}

#[cfg(feature = "cuda")]
fn get_cuda(b: &Backend) -> (Box<dyn BackendOps>, String) {
    let backend = b.get_backend(BackendEnum::Cuda).unwrap();
    let backend_name = "CUDA".to_string();
    (backend, backend_name)
}

#[cfg(feature = "wasm")]
fn get_wasm() -> (Box<dyn BackendOps>, String) {
    let backend = backend.get_backend(BackendEnum::Wasm).unwrap();
    let backend_name = "WASM".to_string();
    (backend, backend_name)
}

fn run_stuff(
    backend: Box<dyn BackendOps>,
    backend_name: String,
    w: usize,
    h: usize,
    size_factor: f32,
    anitaliasing: bool,
    antialiasing_size: usize,
) {
    run_chapter14_with_aa(&backend, &backend_name, w, h);
    run_compare_to_cuda(&backend, &backend_name, w, h);
    run_shadow_glamour_shot(&backend, &backend_name, size_factor, anitaliasing, antialiasing_size);
    run_soft_shadow(&backend, &backend_name, size_factor, anitaliasing, antialiasing_size);
}

fn run_chapter14_with_aa(b: &Box<dyn BackendOps>, backend_name: &String, w: usize, h: usize) {
    let filename_cpu_single = format!("{}_chapter14_with_aa_{}x{}.png", backend_name, w, h);
    let (mut world, c) = chapter14_with_aa::setup_world_chapter14_with_aa(w, h);
    println!(
        "{}",
        format!(
            "---------- {}    chapter14_with_aa_  --------------------",
            backend_name
        )
    );
    let canvas = b.render_world(&mut world, &c);
    canvas.unwrap().write_png(&filename_cpu_single).unwrap();
}

fn run_compare_to_cuda(b: &Box<dyn BackendOps>, backend_name: &String, w: usize, h: usize) {
    let filename_cpu_single = format!("{}_compare_to_cuda_cpu_{}x{}.png", backend_name, w, h);
    let (mut world, c) = compare_to_cuda::setup_world_compare_to_cuda(w, h);
    println!(
        "{}",
        format!(
            "\n\n---------- {}    compare_to_cuda   --------------------",
            backend_name
        )
    );
    let canvas = b.render_world(&mut world, &c);
    canvas.unwrap().write_png(&filename_cpu_single).unwrap();
}

fn run_shadow_glamour_shot(
    b: &Box<dyn BackendOps>,
    backend_name: &String,
    size_factor: f32,
    antialiasing: bool,
    antialiasing_size: usize,
) {
    let filename = format!(
        "{}_shadow_glamour_shot_{:2}x{}x{}.png",
        backend_name, size_factor, antialiasing, antialiasing_size
    );
    let (mut world, c) = shadow_glamour_shot::setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);
    println!(
        "{}",
        format!(
            "\n\n---------- {}   shadow_glamour_shot  --------------------",
            backend_name
        )
    );
    let canvas = b.render_world(&mut world, &c);
    canvas.unwrap().write_png(&filename).unwrap();
}

fn run_soft_shadow(
    b: &Box<dyn BackendOps>,
    backend_name: &String,
    size_factor: f32,
    antialiasing: bool,
    antialiasing_size: usize,
) {
    let filename = format!(
        "{}_test_soft_shadow_{:2}x{}x{}.png",
        backend_name, size_factor, antialiasing, antialiasing_size
    );
    let (mut world, c) =
        test_soft_shadow_aka_area_light::setup_world_shadow_glamour(size_factor, antialiasing, antialiasing_size);
    println!(
        "{}",
        format!(
            "\n\n---------- {}    test_soft_shadow --------------------",
            backend_name
        )
    );
    let canvas = b.render_world(&mut world, &c);
    canvas.unwrap().write_png(&filename).unwrap();
}
