use raytracer_lib_no_std::MAX_REFLECTION_RECURSION_DEPTH;
use raytracer_lib_no_std::{Camera, CameraOps, Color, ColorOps, Light, Pixel, Ray, Shape, BLACK};
use raytracer_lib_std::{World, WorldOps};

pub fn calc_pixel<F>(
    world: &World,
    c: &Camera,
    f: &F,
    n_samples: usize,
    jitter_matrix: &Vec<f32>,
    lights: &Vec<Light>,
    p: &mut Pixel,
) -> ()
where
    F: Fn(*const Shape, usize, *const Light, usize, &Ray, i32, bool, bool, bool) -> Color,
{
    let x = p.x;
    let y = p.y;
    let mut color;
    if c.get_antialiasing() {
        color = calc_pixel_antialiasing(world, c, f, n_samples, jitter_matrix, &lights, x, y);
    } else {
        color = calc_pixel_no_antialiasing(world, c, f, &lights, x, y);
    }
    set_pixel_color(p, &mut color);
}

fn set_pixel_color(p: &mut Pixel, color: &mut Color) {
    color.clamp_color();
    p.color.r = color.r;
    p.color.g = color.g;
    p.color.b = color.b;
}

fn calc_pixel_no_antialiasing<F>(world: &World, c: &Camera, f: &F, lights: &&Vec<Light>, x: usize, y: usize) -> Color
where
    F: Fn(*const Shape, usize, *const Light, usize, &Ray, i32, bool, bool, bool) -> Color,
{
    let r = Camera::ray_for_pixel(c, x, y);
    let color = f(
        world.get_shapes().as_ptr(),
        world.get_shapes().len(),
        lights.as_ptr(),
        lights.len(),
        &r,
        MAX_REFLECTION_RECURSION_DEPTH,
        c.get_calc_reflection(),
        c.get_calc_refraction(),
        c.get_calc_shadows(),
    );
    color
}

fn calc_pixel_antialiasing<F>(
    world: &World,
    c: &Camera,
    f: &F,
    n_samples: usize,
    jitter_matrix: &Vec<f32>,
    lights: &&Vec<Light>,
    x: usize,
    y: usize,
) -> Color
where
    F: Fn(*const Shape, usize, *const Light, usize, &Ray, i32, bool, bool, bool) -> Color,
{
    let mut color = BLACK;
    // Accumulate light for N samples.
    for sample in 0..(n_samples * n_samples) {
        let delta_x = jitter_matrix[2 * sample] * c.get_pixel_size();
        let delta_y = jitter_matrix[2 * sample + 1] * c.get_pixel_size();
        let r = Camera::ray_for_pixel_anti_aliasing(c, x, y, delta_x, delta_y);
        let c = f(
            world.get_shapes().as_ptr(),
            world.get_shapes().len(),
            lights.as_ptr(),
            lights.len(),
            &r,
            MAX_REFLECTION_RECURSION_DEPTH,
            c.get_calc_reflection(),
            c.get_calc_refraction(),
            c.get_calc_shadows(),
        );
        color = c + color;
    }
    color = color / (n_samples * n_samples) as f32;
    color
}

pub fn get_antialiasing_params(c: &Camera) -> (usize, Vec<f32>) {
    let n_samples = c.get_antialiasing_size();
    let mut jitter_matrix = Vec::new();
    if n_samples == 2 {
        jitter_matrix = vec![
            -1.0 / 4.0,
            1.0 / 4.0,
            1.0 / 4.0,
            1.0 / 4.0,
            -1.0 / 4.0,
            -1.0 / 4.0,
            1.0 / 4.0,
            -3.0 / 4.0,
        ];
    }
    if n_samples == 3 {
        let two_over_six = 2.0 / 6.0;
        jitter_matrix = vec![
            -two_over_six,
            two_over_six,
            0.0,
            two_over_six,
            two_over_six,
            two_over_six,
            -two_over_six,
            0.0,
            0.0,
            0.0,
            two_over_six,
            0.0,
            -two_over_six,
            -two_over_six,
            0.0,
            -two_over_six,
            two_over_six,
            -two_over_six,
        ];
    }

    (n_samples, jitter_matrix)
}
