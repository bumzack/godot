use raytracer::Backend;

fn main() {
    let backend = Backend::new();

    println!("available Backends:   {}", backend.get_available_backends().len());
    backend
        .get_available_backends()
        .iter()
        .for_each(|b| println!("backend: {}", b));
}
