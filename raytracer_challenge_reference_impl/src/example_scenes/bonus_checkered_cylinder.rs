use std::error::Error;
use std::f64::consts::PI;

use crate::prelude::{
    uv_checkers, Camera, CameraOps, Color, ColorOps, Cylinder, CylinderTexturePattern, Light, MaterialOps, Matrix,
    MatrixOps, Pattern, PatternEnum, PointLight, Shape, ShapeOps, Tuple, Tuple4D, World, WorldOps,
};

pub fn bonus_checkered_cylinder(width: usize, height: usize) -> (World, Camera) {
    let checker = uv_checkers(16, 8, Color::new(0.0, 0.5, 0.0), Color::new(1.0, 1.0, 1.0));
    let checker_3d = CylinderTexturePattern::new(checker);
    let p = Pattern::new(PatternEnum::CylinderTexturePatternEnum(checker_3d));

    let trans = Matrix::translation(0.0, -0.5, 0.0);
    let scale = Matrix::scale(1.0, PI, 1.0);
    let p_transformed = &scale * &trans;

    let mut cylinder = Cylinder::new();
    cylinder.set_minimum(0.0);
    cylinder.set_maximum(1.0);
    let mut cylinder = Shape::new_cylinder(cylinder, "cylinder".to_string());
    cylinder.set_transformation(p_transformed);
    cylinder.get_material_mut().set_pattern(p);
    cylinder.get_material_mut().set_ambient(0.1);
    cylinder.get_material_mut().set_specular(0.4);
    cylinder.get_material_mut().set_diffuse(0.8);
    cylinder.get_material_mut().set_shininess(15.0);

    let pl = PointLight::new(Tuple4D::new_point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let mut w = World::new();
    w.add_light(l);
    w.add_shape(cylinder);

    let mut c = Camera::new(width, height, 0.5);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(0.0, 3.0, -10.0),
        &Tuple4D::new_point(0.0, 0.0, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
