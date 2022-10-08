use image::ImageError;
use std::error::Error;

use crate::prelude::PatternEnum::ImageTexturePatternEnum;
use crate::prelude::{
    Camera, CameraOps, Canvas, CanvasOpsStd, Color, ColorOps, ImageTexturePattern, Light, MaterialOps, Matrix,
    MatrixOps, Pattern, Plane, PointLight, Shape, ShapeOps, Sphere, Tuple, Tuple4D, World, WorldOps,
};

pub fn bonus_image_mapping(width: usize, height: usize) -> Result<(World, Camera), ImageError> {
    let mut s = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    s.get_material_mut().set_diffuse(0.9);
    s.get_material_mut().set_specular(0.1);
    s.get_material_mut().set_ambient(0.1);
    s.get_material_mut().set_shininess(10.0);

    let rot_y = Matrix::rotate_y(1.9);
    let translate = Matrix::translation(0.0, 1.1, 0.0);
    let trans = &translate * &rot_y;
    s.set_transformation(trans);

    let mut p = Shape::new_plane(Plane::new(), "plane".to_string());
    p.get_material_mut().set_diffuse(0.1);
    p.get_material_mut().set_specular(0.);
    p.get_material_mut().set_ambient(0.);
    p.get_material_mut().set_reflective(0.4);
    // p.get_material_mut().set_color(Color::new(1.0, 1.0, 1.0));

    let pl = PointLight::new(Tuple4D::new_point(-100.0, 100.0, -100.0), Color::new(1.0, 1.0, 1.0));
    let l = Light::PointLight(pl);

    let image = Canvas::read_png(
        "/Users/bumzack/stoff/rust/godot/raytracer_challenge_reference_impl/downloaded_obj_files/earthmap1k.jpg",
    )
    .expect("loading image linear_gradient.png");

    image.write_png("blupp.png")?;

    let pattern = ImageTexturePattern::new(image);
    let pattern = Pattern::new(ImageTexturePatternEnum(pattern));
    s.get_material_mut().set_pattern(pattern);

    let mut w = World::new();
    w.add_light(l);
    w.add_shape(p);
    w.add_shape(s);

    let mut c = Camera::new(width, height, 0.8);
    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(1.0, 2.0, -10.0),
        &Tuple4D::new_point(0.0, 1.1, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    Ok((w, c))
}
