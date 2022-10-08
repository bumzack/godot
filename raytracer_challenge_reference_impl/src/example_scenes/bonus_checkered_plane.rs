use crate::prelude::{
    uv_checkers, Camera, CameraOps, Color, ColorOps, Light, MaterialOps, Matrix, MatrixOps, Pattern, PatternEnum,
    Plane, PlaneTexturePattern, PointLight, Shape, ShapeOps, Tuple, Tuple4D, World, WorldOps,
};
use std::error::Error;

pub fn bonus_checkered_plane(width: usize, height: usize) -> (World, Camera) {
    let checker = uv_checkers(2, 2, Color::new(0.0, 0.5, 0.0), Color::new(1.0, 1.0, 1.0));
    let plane_checker = Pattern::new(PatternEnum::PlaneTexturePatternEnum(PlaneTexturePattern::new(checker)));

    let mut plane = Shape::new_plane(Plane::new(), "plane".to_string());
    plane.get_material_mut().set_pattern(plane_checker);
    plane.get_material_mut().set_ambient(0.1);
    plane.get_material_mut().set_specular(0.0);
    plane.get_material_mut().set_diffuse(0.9);

    let pl = PointLight::new(Tuple4D::new_point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.add_light(l);
    w.add_shape(plane);

    let mut c = Camera::new(width, height, 0.50);
    c.set_antialiasing(true);
    c.set_antialiasing_size(3);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(1.0, 2.0, -5.0),
        &Tuple4D::new_point(0.0, 0.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
