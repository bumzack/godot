use crate::prelude::{
    uv_checkers, Camera, CameraOps, Color, ColorOps, Light, MaterialOps, Matrix, MatrixOps, Pattern, PatternEnum,
    PointLight, Shape, ShapeOps, Sphere, SphereTexturePattern, Tuple, Tuple4D, World, WorldOps,
};

pub fn bonus_checkered_sphere(width: usize, height: usize) -> (World, Camera) {
    let checker = uv_checkers(20, 10, Color::new(0.0, 0.5, 0.0), Color::new(1.0, 1.0, 1.0));
    let p = Pattern::new(PatternEnum::SphereTexturePatternEnum(SphereTexturePattern::new(
        checker,
    )));

    let mut sphere = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    sphere.get_material_mut().set_pattern(p);
    sphere.get_material_mut().set_ambient(0.1);
    sphere.get_material_mut().set_specular(0.4);
    sphere.get_material_mut().set_diffuse(0.6);
    sphere.get_material_mut().set_shininess(10.0);

    let pl = PointLight::new(Tuple4D::new_point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.add_light(l);
    w.add_shape(sphere);

    let mut c = Camera::new(width, height, 0.5);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 0.0, -5.0),
        &Tuple4D::new_point(0.0, 0.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
