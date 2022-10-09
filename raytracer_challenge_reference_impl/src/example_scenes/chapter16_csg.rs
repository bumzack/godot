use crate::prelude::{
    AreaLight, Camera, CameraOps, Color, ColorOps, Csg, CsgOp, Cube, Light, MaterialOps, Matrix, MatrixOps, Plane,
    Sequence, Shape, ShapeOps, Sphere, Tuple, Tuple4D, World, WorldOps,
};

pub fn chapter16_csg(
    width: usize,
    height: usize,
    anitaliasing: bool,
    anitaliasing_size: usize,
    arealight_u: usize,
    arealight_v: usize,
) -> (World, Camera) {
    let corner = Tuple4D::new_point(-1.0, 2.0, 4.0);
    let uvec = Tuple4D::new_vector(2.0, 0.0, 0.0);
    let vvec = Tuple4D::new_vector(0.0, 2.0, 0.0);
    let intensity = Color::new(1.5, 1.5, 1.5);
    let area_light = AreaLight::new(
        corner,
        uvec,
        arealight_u,
        vvec,
        arealight_v,
        intensity,
        Sequence::new(vec![]),
    );
    let area_light = Light::AreaLight(area_light);

    // ---- CUBE -------
    let mut cube = Shape::new_cube(Cube::new(), "cube".to_string());
    cube.get_material_mut().set_color(Color::new(1.5, 1.5, 1.5));
    cube.get_material_mut().set_ambient(1.0);
    cube.get_material_mut().set_diffuse(0.0);
    cube.get_material_mut().set_specular(0.0);
    cube.get_material_mut().set_shininess(100.0);

    let m_trans = Matrix::translation(0.0, 3.0, 4.0);
    let m_scale = Matrix::scale(1.0, 1.0, 0.01);
    let m = &m_trans * &m_scale;

    cube.set_transformation(m);
    cube.set_casts_shadow(false);

    // ---- PLANE -------
    let mut plane = Shape::new_plane(Plane::new(), "plane".to_string());
    plane.get_material_mut().set_color(Color::new(1., 1., 1.));
    plane.get_material_mut().set_ambient(0.025);
    plane.get_material_mut().set_diffuse(0.67);
    plane.get_material_mut().set_specular(0.0);
    plane.get_material_mut().set_shininess(200.0);

    // ---- SPHERE 1 -------
    let mut sphere1 = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    sphere1.get_material_mut().set_color(Color::new(1.0, 0., 0.));
    sphere1.get_material_mut().set_ambient(0.1);
    sphere1.get_material_mut().set_diffuse(0.6);
    sphere1.get_material_mut().set_specular(0.0);
    sphere1.get_material_mut().set_reflective(0.3);
    sphere1.get_material_mut().set_shininess(200.0);
    sphere1.get_material_mut().set_transparency(0.2);
    sphere1.get_material_mut().set_refractive_index(0.2);

    let m_trans = Matrix::translation(0.5, 0.5, 0.0);
    let m_scale = Matrix::scale(0.5, 0.5, 0.5);
    let m = &m_trans * &m_scale;

    sphere1.set_transformation(m);

    // ---- SPHERE 2 -------
    let mut sphere2 = Shape::new_sphere(Sphere::new(), "sphere".to_string());
    sphere2.get_material_mut().set_color(Color::new(0.5, 0.5, 1.0));
    sphere2.get_material_mut().set_ambient(0.1);
    sphere2.get_material_mut().set_diffuse(0.6);
    sphere2.get_material_mut().set_specular(0.0);
    sphere2.get_material_mut().set_reflective(0.3);
    sphere2.get_material_mut().set_shininess(200.0);
    sphere2.get_material_mut().set_transparency(0.2);
    sphere2.get_material_mut().set_refractive_index(0.2);

    let m_trans = Matrix::translation(0.25, 0.25, 0.0);
    let m_scale = Matrix::scale(0.3, 0.3, 0.3);
    let m = &m_trans * &m_scale;

    sphere2.set_transformation(m);

    let mut w = World::new();
    let csg = Csg::new(w.get_shapes_mut(), "first_csg".to_string(), CsgOp::DIFFERENCE);
    Csg::add_child(w.get_shapes_mut(), csg, sphere1, sphere2);

    w.add_light(area_light);
    w.add_shape(cube);
    w.add_shape(plane);
    // w.add_shape(sphere1);
    // w.add_shape(sphere2);

    let mut c = Camera::new(width, height, 0.78540);
    c.set_antialiasing(anitaliasing);
    c.set_antialiasing_size(anitaliasing_size);

    c.calc_pixel_size();
    c.set_transformation(Matrix::view_transform(
        &Tuple4D::new_point(-3.0, 1., 2.5),
        &Tuple4D::new_point(0.0, 0.5, 0.0),
        &Tuple4D::new_vector(0.0, 1.0, 0.0),
    ));
    (w, c)
}
