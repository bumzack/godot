use crate::prelude::patterns::Pattern;
use crate::prelude::*;
use crate::DEBUG;

pub const REFRACTION_VACUUM: f32 = 1.0;
pub const REFRACTION_AIR: f32 = 1.00029;
pub const REFRACTION_WATER: f32 = 1.333;
pub const REFRACTION_GLASS: f32 = 1.52;
pub const REFRACTION_DIAMOND: f32 = 2.417;

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    color: Color,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
    pattern: Option<Pattern>,
    reflective: f32,
    transparency: f32,
    refractive_index: f32,
}

pub trait MaterialOps {
    fn new() -> Material;

    fn lightning(
        material: &Material,
        object: &Shape,
        light: &mut Light,
        point: &Tuple4D,
        eye: &Tuple4D,
        n: &Tuple4D,
        intensity: f32,
    ) -> Color;

    fn set_color(&mut self, c: Color);
    fn set_diffuse(&mut self, d: f32);
    fn set_specular(&mut self, s: f32);
    fn set_shininess(&mut self, s: f32);
    fn set_ambient(&mut self, a: f32);

    fn get_diffuse(&self) -> f32;
    fn get_specular(&self) -> f32;
    fn get_shininess(&self) -> f32;
    fn get_ambient(&self) -> f32;

    fn get_color(&self) -> &Color;

    fn set_pattern(&mut self, p: Pattern);
    fn get_pattern(&self) -> &Option<Pattern>;

    fn get_reflective(&self) -> f32;
    fn set_reflective(&mut self, a: f32);

    fn get_transparency(&self) -> f32;

    fn set_transparency(&mut self, transparency: f32);
    fn get_refractive_index(&self) -> f32;

    fn set_refractive_index(&mut self, refractive_index: f32);
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
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }

    fn lightning(
        material: &Material,
        object: &Shape,
        light: &mut Light,
        point: &Tuple4D,
        eye: &Tuple4D,
        n: &Tuple4D,
        intensity: f32,
    ) -> Color {
        let c: Color;

        // TODO: a lot of color copying here ...
        match material.get_pattern() {
            None => c = &material.color * light.get_intensity(),
            Some(pattern) => c = pattern.color_at_object(object, point),
        }

        // ambient
        let effective_color = &c * light.get_intensity();
        let ambient = &effective_color * material.ambient;

        let mut sum = BLACK;

        // create the sample points for the different lights
        let mut samples = Vec::new();

        for v in 0..light.get_vsteps() {
            for u in 0..light.get_usteps() {
                let mut l = light.clone();
                samples.push(World::point_on_light(&mut l, u, v));
            }
        }

        for sample in samples.iter() {
            let mut specular;
            let mut diffuse;

            let light_v = Tuple4D::normalize(&(sample - point));
            let light_dot_normal = &light_v ^ &n;

            if light_dot_normal < 0.0 || intensity == 0.0 {
                specular = BLACK;
                diffuse = BLACK;
            } else {
                diffuse = &effective_color * material.diffuse * light_dot_normal;
                diffuse.fix_nan();
                let reflect_v = Tuple4D::reflect(&light_v, &n) * (-1.0);
                let reflect_dot_eye = &reflect_v ^ eye;

                specular = BLACK;
                if reflect_dot_eye > 0.0 {
                    if DEBUG {
                        println!(
                            "specular  BEFORE check     {:?}           point = {:?}",
                            specular, point
                        );
                        println!(
                            "specular {:?}        reflect_dot_eye = {:?}    point = {:?}",
                            specular, reflect_dot_eye, point
                        );

                        println!(
                            "refelct_dot_eye = reflect_v ^ eye = {:?} ^  {:?} = {:?}      shininess = {}",
                            reflect_v, eye, reflect_dot_eye, material.shininess
                        );
                    }

                    let factor = reflect_dot_eye.powf(material.shininess);
                    //                    if factor > MAX/2.0 {
                    //                        factor = MAX / 2.0;
                    //                    }
                    specular = light.get_intensity() * material.specular * factor;

                    if DEBUG {
                        println!(
                            "specular {:?}       factor = {} , light.intensity = {:?}    point = {:?}",
                            specular,
                            factor,
                            light.get_intensity(),
                            point
                        );
                    }

                    assert_valid_color(&specular);
                    specular.fix_nan();
                    if DEBUG {
                        println!("specular  AFTER check     {:?}         point = {:?}", specular, point);
                    }
                }
            }
            sum = &sum + &diffuse;
            sum = &sum + &specular;
        }
        assert_valid_color(&ambient);

        assert_valid_color(&sum);
        sum.replace_inf_with_max();
        assert_valid_color(&sum);

        ambient + sum / light.get_samples() as f32 * intensity
    }

    fn set_color(&mut self, c: Color) {
        self.color = c;
    }

    fn set_diffuse(&mut self, d: f32) {
        self.diffuse = d;
    }

    fn set_specular(&mut self, s: f32) {
        self.specular = s;
    }

    fn set_shininess(&mut self, s: f32) {
        self.shininess = s;
    }

    fn set_ambient(&mut self, a: f32) {
        self.ambient = a;
    }

    fn get_diffuse(&self) -> f32 {
        self.diffuse
    }

    fn get_specular(&self) -> f32 {
        self.specular
    }

    fn get_shininess(&self) -> f32 {
        self.shininess
    }

    fn get_ambient(&self) -> f32 {
        self.ambient
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

    fn get_reflective(&self) -> f32 {
        self.reflective
    }

    fn set_reflective(&mut self, a: f32) {
        self.reflective = a;
    }

    fn get_transparency(&self) -> f32 {
        self.transparency
    }

    fn set_transparency(&mut self, transparency: f32) {
        self.transparency = transparency;
    }

    fn get_refractive_index(&self) -> f32 {
        self.refractive_index
    }

    fn set_refractive_index(&mut self, refractive_index: f32) {
        self.refractive_index = refractive_index;
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::SQRT_2;

    use crate::prelude::stripe_patterns::StripePattern;

    use super::*;

    fn setup() -> (Material, Tuple4D) {
        let m = Material::new();
        let p = Tuple4D::new_point(0.0, 0.0, 0.0);
        (m, p)
    }

    // page 85
    #[test]
    fn test_material_new() {
        let m = Material::new();
        assert_float(m.ambient, 0.1);
        assert_float(m.diffuse, 0.9);
        assert_float(m.specular, 0.9);
        assert_float(m.shininess, 200.0);

        assert_float(m.reflective, 0.0);
        assert_float(m.transparency, 0.0);
        assert_float(m.refractive_index, 1.0);

        assert_color(&m.color, &WHITE);
    }

    // page 86
    #[test]
    fn test_material_lightning_eye_between_light_and_surface() {
        let s = Sphere::new();
        let dummy_obj = Shape::new(ShapeEnum::Sphere(s));

        let (m, p) = setup();

        let eye_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let l = PointLight::new(Tuple4D::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = Material::lightning(&m, &dummy_obj, &mut Light::PointLight(l), &p, &eye_v, &normal_v, 1.0);

        let result_expected = Color::new(1.9, 1.9, 1.9);
        assert_color(&result, &result_expected);
    }

    // page 86
    #[test]
    fn test_material_lightning_eye_offset_45() {
        let s = Sphere::new();
        let dummy_obj = Shape::new(ShapeEnum::Sphere(s));

        let (m, p) = setup();

        let eye_v = Tuple4D::new_vector(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let l = PointLight::new(Tuple4D::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = Material::lightning(&m, &dummy_obj, &mut Light::PointLight(l), &p, &eye_v, &normal_v, 1.0);

        let result_expected = Color::new(1.0, 1.0, 1.0);
        assert_color(&result, &result_expected);
    }

    // page 87
    #[test]
    fn test_material_lightning_light_opposite_eye() {
        let s = Sphere::new();
        let dummy_obj = Shape::new(ShapeEnum::Sphere(s));

        let (m, p) = setup();

        let eye_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let l = PointLight::new(Tuple4D::new_point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = Material::lightning(&m, &dummy_obj, &mut Light::PointLight(l), &p, &eye_v, &normal_v, 1.0);
        let result_expected = Color::new(0.7363961, 0.7363961, 0.7363961);

        println!("result = {:?}", result);
        println!("result_Expected = {:?}", result_expected);

        assert_color(&result, &result_expected);
    }

    // page 87
    #[test]
    fn test_material_lightning_eye_in_path_of_reflecting_vector() {
        let s = Sphere::new();
        let dummy_obj = Shape::new(ShapeEnum::Sphere(s));

        let (m, p) = setup();

        let eye_v = Tuple4D::new_vector(0.0, -SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let l = PointLight::new(Tuple4D::new_point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = Material::lightning(&m, &dummy_obj, &mut Light::PointLight(l), &p, &eye_v, &normal_v, 1.0);
        let result_expected = Color::new(1.6363853, 1.6363853, 1.6363853);

        println!("result = {:?}", result);
        println!("result_Expected = {:?}", result_expected);

        assert_color(&result, &result_expected);
    }

    // page 88
    #[test]
    fn test_material_lightning_light_behind_surface() {
        let s = Sphere::new();
        let dummy_obj = Shape::new(ShapeEnum::Sphere(s));

        let (m, p) = setup();

        let eye_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let l = PointLight::new(Tuple4D::new_point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        let result = Material::lightning(&m, &dummy_obj, &mut Light::PointLight(l), &p, &eye_v, &normal_v, 1.0);
        let result_expected = Color::new(0.1, 0.1, 0.1);
        assert_color(&result, &result_expected);
    }

    // page 110 - shadows - yeah!
    #[test]
    fn test_material_lightning_with_surface_in_shadow() {
        let s = Sphere::new();
        let object = Shape::new(ShapeEnum::Sphere(s));

        let (material, point) = setup();

        let eye = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let normal_v = Tuple4D::new_vector(0.0, 0.0, -1.0);

        let position = Tuple4D::new_point(0.0, 0.0, -10.0);
        let intensity = Color::new(1.0, 1.0, 1.0);
        let l = PointLight::new(position, intensity);
        let mut light = Light::PointLight(l).clone();
        let in_shadow = 0.0;

        let result = Material::lightning(&material, &object, &mut light, &point, &eye, &normal_v, in_shadow);

        let result_expected = Color::new(0.1, 0.1, 0.1);
        assert_color(&result, &result_expected);
    }

    // page 129
    #[test]
    fn test_material_with_pattern() {
        let s = Sphere::new();
        let dummy_obj = Shape::new(ShapeEnum::Sphere(s));
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
        let mut pl = Light::PointLight(l);

        let p1 = Tuple4D::new_point(0.9, 0.0, 0.0);
        let c1 = Material::lightning(&m, &dummy_obj, &mut pl, &p1, &eye_v, &normal_v, 1.0);
        let c1_expected = Color::new(1.0, 1.0, 1.0);
        assert_color(&c1, &c1_expected);

        let p2 = Tuple4D::new_point(1.1, 0.0, 0.0);
        let c2 = Material::lightning(&m, &dummy_obj, &mut pl, &p2, &eye_v, &normal_v, 1.0);
        let c2_expected = Color::new(0.0, 0.0, 0.0);
        assert_color(&c2, &c2_expected);
    }

    // page 143
    #[test]
    fn test_material_precomputing_reflection_vector() {
        let p = Plane::new();
        let shape = Shape::new(ShapeEnum::Plane(p));

        let p = Tuple4D::new_point(0.0, 1.0, -1.0);
        let o = Tuple4D::new_vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0);
        let r = Ray::new(p, o);
        let i = Intersection::new(SQRT_2, &shape);

        let comps = Intersection::prepare_computations(&i, &r, &IntersectionList::new());

        let reflection_vector_expected = Tuple4D::new_vector(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0);
        assert_tuple(comps.get_reflected_vector(), &reflection_vector_expected);
    }

    // bonus: soft shadows // area light
    fn test_area_lights_lighning_uses_light_intensity_helper(intensity: f32, expected_result: Color) {
        let w = default_world_soft_shadows();

        let point = Tuple4D::new_point(0.0, 0.0, -1.0);
        let eye_vector = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let normal_vector = Tuple4D::new_vector(0.0, 0.0, -1.0);

        let material = w.get_shapes()[0].get_material();
        let s1 = &w.get_shapes()[0];

        let mut l = w.get_light()[0].clone();
        let result = Material::lightning(material, s1, &mut l, &point, &eye_vector, &normal_vector, intensity);

        println!("result            {:?}", result);
        println!("expected_result   {:?}", expected_result);

        assert_color(&result, &expected_result);
    }

    // lightning uses light intensity
    #[test]
    fn test_area_lights_lighning_uses_light_intensity() {
        let result = Color::new(1.0, 1.0, 1.0);
        test_area_lights_lighning_uses_light_intensity_helper(1.0, result);

        let result = Color::new(0.55, 0.55, 0.55);
        test_area_lights_lighning_uses_light_intensity_helper(0.5, result);

        let result = Color::new(0.1, 0.1, 0.1);
        test_area_lights_lighning_uses_light_intensity_helper(0.0, result);
    }

    // bonus:  Scenario Outline: lighting() samples the area light
    fn test_area_lights_lightning_samples_area_light_helper(point: Tuple4D, expected_result: Color) {
        let corner = Tuple4D::new_point(-0.5, -0.5, -5.0);
        let v1 = Tuple4D::new_vector(1.0, 0.0, 0.0);
        let v2 = Tuple4D::new_vector(0.0, 1.0, 0.0);

        let usteps = 2;
        let vsteps = 2;

        let intensity = Color::new(1.0, 1.0, 1.0);
        let arealight = AreaLight::new(corner, v1, usteps, v2, vsteps, intensity, Sequence::new(vec![0.5]));
        let mut light = Light::AreaLight(arealight);

        let mut sphere = Sphere::new();
        sphere.get_material_mut().set_ambient(0.1);
        sphere.get_material_mut().set_diffuse(0.9);
        sphere.get_material_mut().set_specular(0.0);
        sphere.get_material_mut().set_color(Color::new(1.0, 1.0, 1.0));
        let sphere = Shape::new(ShapeEnum::Sphere(sphere));

        let eye = Tuple4D::new_point(0.0, 0.0, -5.0);
        let eye_vec = Tuple4D::normalize(&(&eye - &point));
        let normal_v = Tuple4D::new_vector(point.x, point.y, point.z);

        let result = Material::lightning(
            sphere.get_material(),
            &sphere,
            &mut light,
            &point,
            &eye_vec,
            &normal_v,
            1.0,
        );

        println!("actual                {:?}", &result);
        println!("expected_result       {:?}", &expected_result);

        assert_color(&result, &expected_result);
    }

    // bonus:  Scenario Outline: lighting() samples the area light
    #[test]
    fn test_area_lights_lightning_samples_area_light() {
        let point = Tuple4D::new_point(0.0, 0.0, -1.0);
        let color = Color::new(0.99650484, 0.99650484, 0.99650484);
        test_area_lights_lightning_samples_area_light_helper(point, color);

        let point = Tuple4D::new_point(0.0, 0.7071, -0.7071);
        let color = Color::new(0.62318283, 0.62318283, 0.62318283);
        test_area_lights_lightning_samples_area_light_helper(point, color);
    }
}
