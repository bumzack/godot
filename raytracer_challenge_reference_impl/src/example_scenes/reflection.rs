use std::f64::consts::PI;

use crate::prelude::PatternEnum::RingPatternEnum;
use crate::prelude::{
    Camera, CameraOps, Color, ColorOps, Light, MaterialOps, Matrix, MatrixOps, Pattern, PatternEnum, Plane, PointLight,
    RingPattern, Shape, ShapeOps, Sphere, StripePattern, Tuple, Tuple4D, World, WorldOps,
};

pub fn reflection(width: usize, height: usize) -> (World, Camera) {
    let mut floor = Shape::new_plane(Plane::new(), "plane".to_string());
    let mut ring_pattern = RingPattern::new();
    ring_pattern.set_color_a(Color::new(1.0, 0.9, 0.9));
    ring_pattern.set_color_b(Color::new(0.1, 0.3, 0.3));
    let ring_pattern = Pattern::new(RingPatternEnum(ring_pattern));
    //let m = Matrix::rotate_y(PI / 4.0);
    // p.set_transformation(m);
    floor.get_material_mut().set_pattern(ring_pattern);
    floor.get_material_mut().set_specular(0.0);
    floor.set_casts_shadow(false);

    let mut ring_pattern = RingPattern::new();
    ring_pattern.set_color_a(Color::new(0.2, 0.2, 0.2));
    ring_pattern.set_color_b(Color::new(0.9, 0.6, 0.9));
    let ring_pattern = Pattern::new(RingPatternEnum(ring_pattern));

    let mut ball1 = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    ball1.set_transformation(Matrix::translation(-0.5, 1.0, 0.5));
    ball1.get_material_mut().set_pattern(ring_pattern);
    ball1.get_material_mut().set_color(Color::new(0.1, 1.0, 0.5));
    ball1.get_material_mut().set_diffuse(0.7);
    ball1.get_material_mut().set_specular(0.3);
    // ball1.get_material_mut().set_reflective(1.3);
    // ball1.get_material_mut().set_refractive_index(1.3);

    let mut stripe_pattern = StripePattern::new();
    stripe_pattern.set_color_a(Color::new(0.1, 1.0, 0.5));
    stripe_pattern.set_color_b(Color::new(0.1, 0.5, 0.2));
    let mut stripe_pattern = Pattern::new(PatternEnum::StripePatternEnum(stripe_pattern));
    stripe_pattern.set_transformation(&Matrix::rotate_y(PI / 5.0) * &Matrix::scale(0.25, 0.25, 0.25));

    let mut ball2 = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    ball2.set_transformation(&Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scale(0.5, 0.5, 0.5));
    ball2.get_material_mut().set_color(Color::new(0.5, 1.0, 0.1));
    ball2.get_material_mut().set_diffuse(0.7);
    ball2.get_material_mut().set_specular(0.3);
    ball2.get_material_mut().set_pattern(stripe_pattern);
    // ball2.get_material_mut().set_reflective(1.8);
    // ball2.get_material_mut().set_refractive_index(1.8);

    let mut stripe_pattern = StripePattern::new();
    stripe_pattern.set_color_a(Color::new(1.0, 0.8, 0.1));
    stripe_pattern.set_color_b(Color::new(0.5, 0.4, 0.1));
    let mut stripe_pattern = Pattern::new(PatternEnum::StripePatternEnum(stripe_pattern));
    stripe_pattern.set_transformation(
        &(&Matrix::rotate_x(PI / 2.0) * &Matrix::rotate_y(PI / 5.0)) * &Matrix::scale(0.25, 0.25, 0.25),
    );

    let mut ball3 = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    ball3.set_transformation(&Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scale(0.33, 0.33, 0.33));
    ball3.get_material_mut().set_diffuse(0.7);
    ball3.get_material_mut().set_specular(0.3);
    ball3.get_material_mut().set_pattern(stripe_pattern);
    // ball2.get_material_mut().set_reflective(1.8);
    // ball2.get_material_mut().set_refractive_index(1.8);#
    // let checker_3d = Pattern::new(PatternEnum::Checker3DPattern(pattern));

    let pl = PointLight::new(Tuple4D::new_point(-1.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.add_light(l);
    w.add_shape(ball1);
    w.add_shape(ball2);
    w.add_shape(ball3);

    let mut c = Camera::new(width, height, PI / 3.0);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 1.5, -1.5),
        &Tuple4D::new_point(0.0, 1.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
