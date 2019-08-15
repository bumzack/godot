use crate::basics::color::Color;
use crate::basics::color::ColorOps;
use crate::basics::color::BLACK;
use crate::light::light::{Light, LightOps};
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;
use crate::patterns::patterns::Pattern;
use crate::shape::shape::Shape;

#[derive(Clone, Debug)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    pattern: Option<Pattern>,
    reflective: f64,
}

pub trait MaterialOps {
    fn new() -> Material;

    fn lightning(
        material: &Material,
        object: &Shape,
        light: &Light,
        point: &Tuple4D,
        eye: &Tuple4D,
        n: &Tuple4D,
        in_shadow: bool,
    ) -> Color;

    fn set_color(&mut self, c: Color);
    fn set_diffuse(&mut self, d: f64);
    fn set_specular(&mut self, s: f64);
    fn set_shininess(&mut self, s: f64);
    fn set_ambient(&mut self, a: f64);

    fn get_color(&self) -> &Color;

    fn set_pattern(&mut self, p: Pattern);
    fn get_pattern(&self) -> &Option<Pattern>;

    fn get_reflective(&self) -> f64;
    fn set_reflective(&mut self, a: f64);
}

impl MaterialOps for Material {
    fn new() -> Material {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
            reflective: 0.0,
        }
    }

    fn lightning(
        material: &Material,
        object: &Shape,
        light: &Light,
        point: &Tuple4D,
        eye: &Tuple4D,
        n: &Tuple4D,
        in_shadow: bool,
    ) -> Color {
        let c: Color;
        // TODO: a lot of color copying here ...
        if material.get_pattern().is_some() {
            c = material.get_pattern().as_ref().unwrap().color_at_object(object, point);
        } else {
            c = Color::from_color(&material.color);
        }
        let effective_color = &c * light.get_intensity();
        let light_v = Tuple4D::normalize(&(light.get_position() - &point));
        let ambient = &effective_color * material.ambient;
        let light_dot_normal = &light_v ^ &n;
        let mut diffuse = BLACK;
        let mut specular = BLACK;
        if light_dot_normal >= 0.0 {
            diffuse = &effective_color * material.diffuse * light_dot_normal;
            let reflect_v = Tuple4D::reflect(&(light_v * (-1.0)), &n);
            let reflect_dot_eye = &reflect_v ^ eye;
            specular = BLACK;

            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = light.get_intensity() * material.specular * factor;
            }
        }
        if in_shadow {
            ambient
        } else {
            ambient + diffuse + specular
        }
    }

    fn set_color(&mut self, c: Color) {
        self.color = c;
    }

    fn set_diffuse(&mut self, d: f64) {
        self.diffuse = d;
    }

    fn set_specular(&mut self, s: f64) {
        self.specular = s;
    }

    fn set_shininess(&mut self, s: f64) {
        self.shininess = s;
    }

    fn set_ambient(&mut self, a: f64) {
        self.ambient = a;
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn set_pattern(&mut self, p: Pattern) {
        self.pattern = Some(p);
    }

    fn get_pattern(&self) -> &Option<Pattern> {
        &self.pattern
    }

    fn get_reflective(&self) -> f64 {
        self.reflective
    }

    fn set_reflective(&mut self, a: f64) {
        self.reflective = a;
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use crate::basics::color::WHITE;
    use crate::basics::intersection::{Intersection, IntersectionListOps, IntersectionOps};
    use crate::basics::ray::{Ray, RayOps};
    use crate::light::pointlight::PointLight;
    use crate::math::common::{assert_color, assert_float, assert_tuple};
    use crate::patterns::stripe_patterns::StripePattern;
    use crate::shape::plane::{Plane, PlaneOps};
    use crate::shape::sphere::{Sphere, SphereOps};

    use super::*;
    use crate::math::matrix::MatrixOps;
    use crate::world::world::WorldOps;

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
        assert_float(m.reflective, 0.0);

        assert_color(&m.color, &WHITE);
    }

    #[test]
    fn test_material_lightning_eye_between_light_and_surface() {
        let s = Sphere::new();
        let dummy_obj = Shape::Sphere(s);

        let (m, p) = setup();

        let eye_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let l = PointLight::new(Tuple4D::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = Material::lightning(&m, &dummy_obj, &Light::PointLight(l), &p, &eye_v, &normal_v, false);

        let result_expected = Color::new(1.9, 1.9, 1.9);
        println!(
            "test_material_lightning_perpendicular  result = {:#?}, expected = {:#?}",
            result, result_expected
        );

        assert_color(&result, &result_expected);
    }

    #[test]
    fn test_material_lightning_eye_offset_45() {
        let s = Sphere::new();
        let dummy_obj = Shape::Sphere(s);

        let (m, p) = setup();

        let eye_v = Tuple4D::new_vector(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let l = PointLight::new(Tuple4D::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = Material::lightning(&m, &dummy_obj, &Light::PointLight(l), &p, &eye_v, &normal_v, false);

        let result_expected = Color::new(1.0, 1.0, 1.0);
        assert_color(&result, &result_expected);
    }

    #[test]
    fn test_material_lightning_light_opposite_eye() {
        let s = Sphere::new();
        let dummy_obj = Shape::Sphere(s);

        let (m, p) = setup();

        let eye_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let l = PointLight::new(Tuple4D::new_point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = Material::lightning(&m, &dummy_obj, &Light::PointLight(l), &p, &eye_v, &normal_v, false);

        let result_expected = Color::new(0.7364, 0.7364, 0.7364);
        assert_color(&result, &result_expected);
    }

    #[test]
    fn test_material_lightning_eye_in_path_of_reflecting_vector() {
        let s = Sphere::new();
        let dummy_obj = Shape::Sphere(s);

        let (m, p) = setup();

        let eye_v = Tuple4D::new_vector(0.0, -SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let l = PointLight::new(Tuple4D::new_point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = Material::lightning(&m, &dummy_obj, &Light::PointLight(l), &p, &eye_v, &normal_v, false);

        let result_expected = Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928);
        println!(
            "test_material_lightning_eye_in_reflecing_path  result = {:#?}, expected = {:#?}",
            result, result_expected
        );
        assert_color(&result, &result_expected);
    }

    #[test]
    fn test_material_lightning_light_behind_surface() {
        let s = Sphere::new();
        let dummy_obj = Shape::Sphere(s);

        let (m, p) = setup();

        let eye_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let l = PointLight::new(Tuple4D::new_point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        let result = Material::lightning(&m, &dummy_obj, &Light::PointLight(l), &p, &eye_v, &normal_v, false);

        let result_expected = Color::new(0.1, 0.1, 0.1);
        assert_color(&result, &result_expected);
    }

    // page 110 - shadows - yeah!
    #[test]
    fn test_material_lightning_with_surface_in_shadow() {
        let s = Sphere::new();
        let dummy_obj = Shape::Sphere(s);

        let (m, p) = setup();

        let eye_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let l = PointLight::new(Tuple4D::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;

        let result = Material::lightning(&m, &dummy_obj, &Light::PointLight(l), &p, &eye_v, &normal_v, in_shadow);

        let result_expected = Color::new(0.1, 0.1, 0.1);
        assert_color(&result, &result_expected);
    }

    // page 129
    #[test]
    fn test_material_with_pattern() {
        let s = Sphere::new();
        let dummy_obj = Shape::Sphere(s);
        let stripe_pattern = StripePattern::new();
        let pattern = Pattern::StripePattern(stripe_pattern);

        let mut m = Material::new();
        m.set_pattern(pattern);
        m.set_ambient(1.0);
        m.set_diffuse(0.0);
        m.set_specular(0.0);

        let eye_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let l = PointLight::new(Tuple4D::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let pl = Light::PointLight(l);

        let p1 = Tuple4D::new_point(0.9, 0.0, 0.0);
        let c1 = Material::lightning(&m, &dummy_obj, &pl, &p1, &eye_v, &normal_v, false);
        let c1_expected = Color::new(1.0, 1.0, 1.0);
        assert_color(&c1, &c1_expected);

        let p2 = Tuple4D::new_point(1.1, 0.0, 0.0);
        let c2 = Material::lightning(&m, &dummy_obj, &pl, &p2, &eye_v, &normal_v, false);
        let c2_expected = Color::new(0.0, 0.0, 0.0);
        assert_color(&c2, &c2_expected);
    }

    // page 143
    #[test]
    fn test_material_precomputing_reflection_vector() {
        let p = Plane::new();
        let shape = Shape::Plane(p);

        let p = Tuple4D::new_point(0.0, 1.0, -1.0);
        let o = Tuple4D::new_vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0);
        let r = Ray::new(p, o);
        let i = Intersection::new(SQRT_2, &shape);

        let comps = Intersection::prepare_computations(&i, &r);

        let reflection_vector_expected = Tuple4D::new_vector(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0);
        assert_tuple(comps.get_reflected_vector(), &reflection_vector_expected);
    }
}
