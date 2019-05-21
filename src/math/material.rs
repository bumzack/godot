use std::f32::consts::SQRT_2;

use crate::math::color::{BLACK, WHITE};
use crate::math::color::Color;
use crate::math::color::ColorOps;
use crate::math::common::{assert_color, assert_float, assert_tuple};
use crate::math::light::LightOps;
use crate::math::light::PointLight;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug)]
pub struct Material {
    color: Color,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

pub trait MaterialOps {
    fn new() -> Material;
    fn lighting(material: &Material, light: &PointLight, point: &Tuple4D, eye: &Tuple4D, n: &Tuple4D) -> Color;
    fn set_color(&mut self, c: Color);
}

impl MaterialOps for Material {
    fn new() -> Material {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    fn lighting(material: &Material, light: &PointLight, point: &Tuple4D, eye: &Tuple4D, n: &Tuple4D) -> Color {
        let effective_color = &material.color * &light.intensitiy;
        let light_v = Tuple4D::normalize(&(&light.position - &point));
        let ambient = &effective_color * material.ambient;
        let light_dot_normal = &light_v ^ &n;
        let mut diffuse = BLACK;
        let mut specular = BLACK;
        if light_dot_normal >= 0.0 {
            diffuse = &effective_color * material.diffuse * light_dot_normal;
            let reflect_v = Tuple4D::reflect(&(-light_v), &n);
            let reflect_dot_eye = &reflect_v ^ eye;
            let mut specular = BLACK;

            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = &light.intensitiy * material.specular * factor;
            }
        }
        ambient + diffuse + specular
    }

    fn set_color(&mut self, c: Color) {
        self.color = c;
    }
}

fn setup() -> (Material, Tuple4D) {
    let m = Material::new();
    let p = Tuple4D::new_point(0.0, 0.0, 0.0);
    (m, p)
}


#[test]
fn test_material_new() {
    let m = Material::new();
    assert_float(m.ambient, 0.1);
    assert_float(m.specular, 0.9);
    assert_float(m.diffuse, 0.9);
    assert_float(m.shininess, 200.0);

    assert_color(&m.color, &WHITE);
}


#[test]
fn test_material_lightning_eye_between_light_and_surface() {
    let (m, p) = setup();

    let eye_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
    let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
    let l = PointLight::new(Tuple4D::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let result = Material::lighting(&m, &l, &p, &eye_v, &normal_v);

    let result_expected = Color::new(1.9, 1.9, 1.9);
    println!("test_material_lightning_perpendicular  result = {:#?}, expected = {:#?}", result, result_expected);

    assert_color(&result, &result_expected);
}

#[test]
fn test_material_lightning_eye_offset_45() {
    let (m, p) = setup();

    let eye_v = Tuple4D::new_vector(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
    let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
    let l = PointLight::new(Tuple4D::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let result = Material::lighting(&m, &l, &p, &eye_v, &normal_v);

    let result_expected = Color::new(1.0, 1.0, 1.0);
    assert_color(&result, &result_expected);
}


#[test]
fn test_material_lightning_light_opposite_eye() {
    let (m, p) = setup();

    let eye_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
    let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
    let l = PointLight::new(Tuple4D::new_point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let result = Material::lighting(&m, &l, &p, &eye_v, &normal_v);

    let result_expected = Color::new(0.7364, 0.7364, 0.7364);
    assert_color(&result, &result_expected);
}


#[test]
fn test_material_lightning_eye_in_path_of_reflecting_vector() {
    let (m, p) = setup();

    let eye_v = Tuple4D::new_vector(0.0, -SQRT_2 / 2.0, -SQRT_2 / 2.0);
    let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
    let l = PointLight::new(Tuple4D::new_point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let result = Material::lighting(&m, &l, &p, &eye_v, &normal_v);

    let result_expected = Color::new(1.6364, 1.6364, 1.6364);
    println!("test_material_lightning_eye_in_reflecing_path  result = {:#?}, expected = {:#?}", result, result_expected);
    assert_color(&result, &result_expected);
}


#[test]
fn test_material_lightning_light_behind_surface() {
    let (m, p) = setup();

    let eye_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
    let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
    let l = PointLight::new(Tuple4D::new_point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

    let result = Material::lighting(&m, &l, &p, &eye_v, &normal_v);

    let result_expected = Color::new(0.1, 0.1, 0.1);
    assert_color(&result, &result_expected);
}






