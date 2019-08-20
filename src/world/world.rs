use crate::basics::color::BLACK;
use crate::basics::color::Color;
use crate::basics::color::ColorOps;
use crate::basics::intersection::{Intersection, IntersectionList, IntersectionListOps, IntersectionOps};
use crate::basics::precomputed_component::PrecomputedComponent;
use crate::basics::ray::Ray;
use crate::basics::ray::RayOps;
use crate::light::light::{LightEnum, LightOps};
use crate::light::pointlight::PointLight;
use crate::material::material::{Material, MaterialOps};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;
use crate::patterns::patterns::Pattern;
use crate::patterns::stripe_patterns::StripePattern;
use crate::shape::cylinder::{Cylinder, CylinderOps};
use crate::shape::plane::{Plane, PlaneOps};
use crate::shape::shape::{Shape, ShapeEnum};
use crate::shape::sphere::{Sphere, SphereOps};

pub const MAX_REFLECTION_RECURSION_DEPTH: i32 = 10;

#[derive(Clone, Debug)]
pub struct World<'a> {
    shapes: Vec<Shape<'a>>,
    light: LightEnum,
}

pub trait WorldOps<'a> {
    fn new() -> World<'a>;
    fn set_light(&mut self, light: LightEnum);
    fn get_light(&self) -> &LightEnum;
    fn get_light_mut(&mut self) -> &mut LightEnum;

    fn add_shape(&mut self, shape: Shape<'a>);
    fn get_shapes(&self) -> &Vec<Shape<'a>>;
    fn get_shapes_mut(&mut self) -> &mut Vec<Shape<'a>>;

    fn shade_hit(w: &World, comp: &PrecomputedComponent, remaining: i32) -> Color;

    fn color_at(w: &World, r: &Ray, remaining: i32) -> Color;

    fn reflected_color(w: &World, comp: &PrecomputedComponent, remaining: i32) -> Color;

    fn is_shadowed(w: &World, light_position: &Tuple4D, position: &Tuple4D) -> bool;

    fn intensity_at(light: &LightEnum, point: &Tuple4D, world: &World) -> f64;
    fn refracted_color(w: &World, comp: &PrecomputedComponent, remaining: i32) -> Color;

    fn point_on_light(light: &LightEnum, u: usize, v: usize) -> Tuple4D;

    fn add_floor(&mut self);

    fn add_x_axis(&mut self);
    fn add_y_axis(&mut self);
    fn add_z_axis(&mut self);

    fn intensity_at_point_light(light: &LightEnum, point: &Tuple4D, world: &World) -> f64;

    fn intensity_at_area_light(light: &LightEnum, point: &Tuple4D, world: &World) -> f64;
}

impl<'a> WorldOps<'a> for World<'a> {
    fn new() -> World<'a> {
        // TODO: default light ?!?!?! hmm - where, color why not different solution
        let pl = PointLight::new(Tuple4D::new_point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        World {
            shapes: Vec::new(),
            light: LightEnum::PointLight(pl),
        }
    }

    fn set_light(&mut self, light: LightEnum) {
        self.light = light;
    }

    fn get_light(&self) -> &LightEnum {
        &self.light
    }

    fn get_light_mut(&mut self) -> &mut LightEnum {
        &mut self.light
    }

    fn add_shape(&mut self, shape: Shape<'a>) {
        self.shapes.push(shape);
    }

    fn get_shapes(&self) -> &Vec<Shape<'a>> {
        &self.shapes
    }

    fn get_shapes_mut(&mut self) -> &mut Vec<Shape<'a>> {
        &mut self.shapes
    }

    fn shade_hit(w: &World, comp: &PrecomputedComponent, remaining: i32) -> Color {
        let in_shadow = World::is_shadowed(w, w.get_light().get_position(), comp.get_over_point());
        let intensity = World::intensity_at(w.get_light(), comp.get_over_point(), w);
        let surface = Material::lightning(
            comp.get_object().get_material(),
            comp.get_object(),
            &w.get_light(),
            comp.get_over_point(),
            comp.get_eye_vector(),
            comp.get_normal_vector(),
            intensity,
        );
        let reflected = World::reflected_color(w, comp, remaining);
        let refracted = World::refracted_color(w, comp, remaining);

        let material = comp.get_object().get_material();
        if material.get_reflective() > 0.0 && material.get_transparency() > 0.0 {
            let reflectance = Intersection::schlick(comp);
            return &surface + &(&reflected * reflectance + &refracted * (1.0 - reflectance));
        }
        &surface + &(&reflected + &refracted)
    }

    fn color_at(w: &World, r: &Ray, remaining: i32) -> Color {
        let xs = Intersection::intersect_world(w, r);
        let res = match xs.hit() {
            Some(i) => {
                let comp = Intersection::prepare_computations(&i, &r, &IntersectionList::new());
                World::shade_hit(w, &comp, remaining)
            }
            None => BLACK,
        };
        res
    }

    fn reflected_color(w: &World, comp: &PrecomputedComponent, remaining: i32) -> Color {
        if remaining <= 0 {
            return BLACK;
        }
        if comp.get_object().get_material().get_reflective() == 0.0 {
            return BLACK;
        }
        let reflect_ray = Ray::new(
            Tuple4D::new_point_from(comp.get_over_point()),
            Tuple4D::new_vector_from(comp.get_reflected_vector()),
        );
        let color = World::color_at(w, &reflect_ray, remaining - 1);
        &color * comp.get_object().get_material().get_reflective()
    }

    fn is_shadowed(w: &World, light_position: &Tuple4D, position: &Tuple4D) -> bool {
        let v = light_position - position;

        let distance = Tuple4D::magnitude(&v);
        let mut direction = Tuple4D::normalize(&v);
        direction.w = 0.0;

        let point = Tuple4D::new_point_from(&position);
        let r = Ray::new(point, direction);

        let intersections = Intersection::intersect_world(w, &r);

        let h = intersections.hit();

        if h.is_some() {
            // println!("t = {:?}", h.unwrap().get_t());
            if h.unwrap().get_t() < distance {
                return true;
            }
        }
        false
    }

    fn intensity_at_point_light(light: &LightEnum, point: &Tuple4D, world: &World) -> f64 {
        if Self::is_shadowed(world, light.get_position(), point) {
            return 0.0;
        }
        1.0
    }

    fn intensity_at_area_light(light: &LightEnum, point: &Tuple4D, world: &World) -> f64 {
        let mut total = 1.0;

        for v in 0..light.get_vsteps() - 1 {
            for u in 0..light.get_usteps() - 1 {
                let light_position = World::point_on_light(light, u, v);
                if !World::is_shadowed(world, point, &light_position) {
                    total += 1.0;
                }
            }
        }

        total
    }

    fn intensity_at(light: &LightEnum, point: &Tuple4D, world: &World) -> f64 {
        let res = match light {
            LightEnum::PointLight(ref pl) => World::intensity_at_point_light(light, point, world),
            LightEnum::AreaLight(ref pl) => World::intensity_at_area_light(light, point, world),
        };
        res
    }

    fn point_on_light(light: &LightEnum, u: usize, v: usize) -> Tuple4D {
        light.get_corner() + &(&(light.get_uvec() * (u as f64 + 0.5)) + &(light.get_vvec() * (v as f64 + 0.5)))
    }

    fn refracted_color(w: &World, comp: &PrecomputedComponent, remaining: i32) -> Color {
        if remaining <= 0 {
            return BLACK;
        }
        if comp.get_object().get_material().get_transparency() == 0.0 {
            return BLACK;
        }
        let n_ratio = comp.get_n1() / comp.get_n2();
        let cos_i = comp.get_eye_vector() ^ comp.get_normal_vector();
        let sin2_t = n_ratio.powf(2.0) * (1.0 - cos_i.powf(2.0));

        // total internal reflection -> return black
        if sin2_t > 1.0 {
            return BLACK;
        }
        let cos_t = (1.0 - sin2_t).sqrt();
        let mut direction = comp.get_normal_vector() * (n_ratio * cos_i - cos_t) - comp.get_eye_vector() * n_ratio;
        // fix direction to be a vector and not something in between
        direction.w = 0.0;
        let refracted_ray = Ray::new(Tuple4D::new_point_from(comp.get_under_point()), direction);

        World::color_at(w, &refracted_ray, remaining - 1) * comp.get_object().get_material().get_transparency()
    }

    fn add_floor(&mut self) {
        let m_translate = Matrix::translation(0.0, -1.0, 0.0);
        let m = m_translate;

        let mut floor = Plane::new();
        floor.set_transformation(m);

        floor.get_material_mut().set_ambient(0.1);
        floor.get_material_mut().set_shininess(0.1);
        floor.get_material_mut().set_specular(0.1);

        let mut floor_stripe_pattern = StripePattern::new();
        floor_stripe_pattern.set_color_a(Color::new(1.0, 0.0, 0.0));
        floor_stripe_pattern.set_color_b(Color::new(0.0, 0.0, 1.0));
        let floor_stripe_pattern = Pattern::StripePattern(floor_stripe_pattern);
        floor.get_material_mut().set_pattern(floor_stripe_pattern);

        self.add_shape(Shape::new(ShapeEnum::Plane(floor), "floor"));
    }

    fn add_x_axis(&mut self) {
        // TODO: just show a red cylinder, but no shadows   like Maya does :-)

        let mut x_axis = Cylinder::new();
        x_axis.set_minimum(0.0);
        x_axis.set_maximum(1.0);
        let m_scale = Matrix::scale(1.0, 0.5, 0.5);
        let m_rot = Matrix::new_identity_4x4();
        let m = m_scale * m_rot;
        x_axis.set_transformation(m);

        x_axis.get_material_mut().set_color(Color::new(1.0, 0.0, 0.0));
        x_axis.get_material_mut().set_ambient(0.0);
        x_axis.get_material_mut().set_diffuse(0.0);
        x_axis.get_material_mut().set_specular(0.0);
        x_axis.get_material_mut().set_shininess(0.0);
        x_axis.get_material_mut().set_reflective(0.0);
        x_axis.get_material_mut().set_transparency(0.0);
        x_axis.get_material_mut().set_refractive_index(0.0);
        self.add_shape(Shape::new(ShapeEnum::Cylinder(x_axis), "x axis"));
    }

    fn add_y_axis(&mut self) {
        // TODO: just show a blue cylinder, but no shadows   like Maya does :-)
        unimplemented!()
    }

    fn add_z_axis(&mut self) {
        // TODO: just show a green cylinder, but no shadows   like Maya does :-)
        unimplemented!()
    }
}

pub fn default_world<'a>() -> World<'a> {
    let mut w = World::new();

    let light_pos = Tuple4D::new_point(-10.0, 10., -10.0);
    let light_intensity = Color::new(1.0, 1.0, 1.0);
    let pl = PointLight::new(light_pos, light_intensity);
    let light = LightEnum::PointLight(pl);
    w.set_light(light);

    let mut m = Material::new();
    m.set_color(Color::new(0.8, 1.0, 0.6));
    m.set_diffuse(0.7);
    m.set_specular(0.2);

    let mut s1 = Sphere::new();
    s1.set_material(m);
    let shape1 = Shape::new(ShapeEnum::Sphere(s1), "default_world_ sphere 1");

    let m = Matrix::scale(0.5, 0.5, 0.5);
    let mut s2 = Sphere::new();
    s2.set_transformation(m);
    let shape2 = Shape::new(ShapeEnum::Sphere(s2), "default_world_ sphere 2");

    w.add_shape(shape1);
    w.add_shape(shape2);

    w
}

pub fn default_world_soft_shadows<'a>() -> World<'a> {
    let mut w = World::new();

    // w.get_light_mut().set_position(Tuple4D::new_point(0.0, 0.0, -10.0));
    //w.get_light_mut().set_intensity(Color::new(1.0, 1.0, 1.0));

    let light_pos = Tuple4D::new_point(0.0, 00., -10.0);
    let light_intensity = Color::new(1.0, 1.0, 1.0);
    let pl = PointLight::new(light_pos, light_intensity);
    let light = LightEnum::PointLight(pl);
    w.set_light(light);

    let mut m = Material::new();
    m.set_ambient(0.1);
    m.set_diffuse(0.9);
    m.set_specular(0.0);
    m.set_color(Color::new(1.0, 1.0, 1.0));

    let mut s1 = Sphere::new();
    s1.set_material(m);
    let shape1 = Shape::new(ShapeEnum::Sphere(s1), "default_world_ sphere 1");

    let m = Matrix::scale(0.5, 0.5, 0.5);
    let mut s2 = Sphere::new();
    s2.set_transformation(m);
    let shape2 = Shape::new(ShapeEnum::Sphere(s2), "default_world_ sphere 2");

    w.add_shape(shape1);
    w.add_shape(shape2);

    w
}

pub fn default_world_refracted_color_page_158<'a>() -> World<'a> {
    let mut w = World::new();

    let light_pos = Tuple4D::new_point(-10.0, 10., -10.0);
    let light_intensity = Color::new(1.0, 1.0, 1.0);
    let pl = PointLight::new(light_pos, light_intensity);
    let light = LightEnum::PointLight(pl);
    w.set_light(light);

    let mut material1 = Material::new();
    material1.set_color(Color::new(0.8, 1.0, 0.6));
    material1.set_diffuse(0.7);
    material1.set_specular(0.2);

    material1.set_ambient(1.0);

    let pattern = StripePattern::new();
    let pattern = Pattern::StripePattern(pattern);
    let mut s1 = Sphere::new();
    s1.set_material(material1);
    s1.get_material_mut().set_pattern(pattern);
    // let shape1 = Shape::Sphere(s1);
    let shape1 = Shape::new(ShapeEnum::Sphere(s1), "default_world_refracted_color_page_158 sphere 1");

    let m = Matrix::scale(0.5, 0.5, 0.5);
    let mut s2 = Sphere::new();
    s2.set_transformation(m);
    s2.get_material_mut().set_transparency(1.0);
    s2.get_material_mut().set_refractive_index(1.5);
    let shape2 = Shape::new(ShapeEnum::Sphere(s2), "default_world_refracted_color_page_158 sphere 2");

    w.add_shape(shape1);
    w.add_shape(shape2);

    w
}

pub fn default_world_empty<'a>() -> World<'a> {
    let mut w = World::new();

    let light_pos = Tuple4D::new_point(-10.0, 10., -10.0);
    let light_intensity = Color::new(1.0, 1.0, 1.0);
    let pl = PointLight::new(light_pos, light_intensity);
    let light = LightEnum::PointLight(pl);
    w.set_light(light);

    w
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use crate::light::arealight::AreaLight;
    use crate::math::common::{assert_color, assert_float, assert_tuple, EPSILON};
    use crate::shape::plane::{Plane, PlaneOps};
    use crate::shape::sphere::glass_sphere;

    use super::*;

    // page 92
    #[test]
    fn test_default_world() {
        let w = default_world();

        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);
        let tmp = Intersection::intersect_world(&w, &r);
        let xs = tmp.get_intersections();

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].get_t(), 4.0);
        assert_eq!(xs[1].get_t(), 4.5);
        assert_eq!(xs[2].get_t(), 5.5);
        assert_eq!(xs[3].get_t(), 6.0);
    }

    // page 95
    #[test]
    fn test_shade_hit() {
        let w = default_world();

        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let shapes = w.get_shapes();
        let shape = shapes.get(0).unwrap();
        let i = Intersection::new(4.0, &shape);

        let comps = Intersection::prepare_computations(&i, &r, &IntersectionList::new());
        let c = World::shade_hit(&w, &comps, MAX_REFLECTION_RECURSION_DEPTH);

        let c_expected = Color::new(0.38066125, 0.47583, 0.2855);
        assert_color(&c_expected, &c);
    }

    // page 95
    #[test]
    fn test_shade_hit_inside() {
        let mut w = default_world();

        let p = Tuple4D::new_point(0.0, 0.25, 0.0);
        let c = Color::new(1.0, 1.0, 1.0);
        let pl = PointLight::new(p, c);
        w.set_light(LightEnum::PointLight(pl));

        let origin = Tuple4D::new_point(0.0, 0.0, 0.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let shapes = w.get_shapes();
        let shape = shapes.get(1).unwrap();
        let i = Intersection::new(0.5, &shape);
        let comps = Intersection::prepare_computations(&i, &r, &IntersectionList::new());

        let c = World::shade_hit(&w, &comps, MAX_REFLECTION_RECURSION_DEPTH);
        let c_expected = Color::new(0.90498, 0.90498, 0.90498);

        println!("expected color = {:?}", c_expected);
        println!("actual   color = {:?}", c);
        assert_color(&c_expected, &c);
    }

    // page 96
    #[test]
    fn test_color_at_ray_miss() {
        let w = default_world();

        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(origin, direction);

        let c = World::color_at(&w, &r, MAX_REFLECTION_RECURSION_DEPTH);
        let c_expected = Color::new(0.0, 0.0, 0.0);

        assert_color(&c_expected, &c);
    }

    // page 96
    #[test]
    fn test_color_at_ray_hit() {
        let w = default_world();

        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);
        let c = World::color_at(&w, &r, MAX_REFLECTION_RECURSION_DEPTH);

        let c_expected = Color::new(0.38066125, 0.47583, 0.2855);
        assert_color(&c_expected, &c);
    }

    // page 97
    #[test]
    fn test_color_at_intersection_behind_ray() {
        let mut w = default_world_empty();

        // add the two shapes from "default_word" but set the required propertys
        let mut m = Material::new();
        m.set_color(Color::new(0.8, 1., 0.6));
        m.set_diffuse(0.7);
        m.set_specular(0.2);
        m.set_ambient(1.0);

        let mut s1 = Sphere::new();
        s1.set_material(m);
        let shape1 = Shape::new(ShapeEnum::Sphere(s1), "Sphere");

        let m = Matrix::scale(0.5, 0.5, 0.5);
        let mut s2 = Sphere::new();
        s2.set_transformation(m);
        s2.get_material_mut().set_ambient(1.0);
        let shape2 = Shape::new(ShapeEnum::Sphere(s2), "Sphere");

        w.add_shape(shape1);
        w.add_shape(shape2);

        let origin = Tuple4D::new_point(0.0, 0.0, 0.75);
        let direction = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let r = Ray::new(origin, direction);
        let c = World::color_at(&w, &r, MAX_REFLECTION_RECURSION_DEPTH);

        let c_expected = Color::from_color(w.get_shapes_mut().get(1).unwrap().get_material().get_color());
        assert_color(&c_expected, &c);
    }

    // page 114
    #[test]
    fn test_shade_hit_shadow() {
        let mut w = World::new();

        let point = Tuple4D::new_point(0.0, 0.0, -10.0);
        let color = Color::new(1.0, 1.0, 1.0);
        let pl = PointLight::new(point, color);
        w.set_light(LightEnum::PointLight(pl));

        let s1 = Sphere::new();
        let shape1 = Shape::new(ShapeEnum::Sphere(s1), "Sphere");

        let m = Matrix::translation(0.0, 0.0, 10.0);
        let mut s2 = Sphere::new();
        s2.set_transformation(m);
        let shape2 = Shape::new(ShapeEnum::Sphere(s2), "Sphere");

        w.add_shape(shape1);
        w.add_shape(shape2);

        let origin = Tuple4D::new_point(0.0, 0.0, 5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let shapes = w.get_shapes();
        let shape = shapes.get(1).unwrap();

        let i = Intersection::new(4.0, &shape);

        let comps = Intersection::prepare_computations(&i, &r, &IntersectionList::new());
        let c = World::shade_hit(&w, &comps, MAX_REFLECTION_RECURSION_DEPTH);
        let c_expected = Color::new(0.1, 0.1, 0.1);

        println!("expected color    = {:?}", c_expected);
        println!("actual color      = {:?}", c);

        assert_color(&c_expected, &c);
    }

    // page 115
    #[test]
    fn test_prepare_computations_shadow_offset() {
        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let m = Matrix::translation(0.0, 0.0, 1.0);
        let mut s1 = Sphere::new();
        s1.set_transformation(m);
        let shape1 = Shape::new(ShapeEnum::Sphere(s1), "Sphere");

        let i = Intersection::new(5.0, &shape1);
        let comps = Intersection::prepare_computations(&i, &r, &IntersectionList::new());

        assert!(comps.get_over_point().z < -EPSILON / 2.0);
        assert!(comps.get_point().z > comps.get_over_point().z);
    }

    // page 96
    #[test]
    fn test_color_at_no_hit() {
        let w = default_world();

        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(origin, direction);

        let c = World::color_at(&w, &r, MAX_REFLECTION_RECURSION_DEPTH);

        let c_expected = Color::new(0.0, 0.0, 0.0);
        assert_color(&c_expected, &c);
    }

    // page 96 bottom
    #[test]
    fn test_color_at_single_hit() {
        let w = default_world();

        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let c = World::color_at(&w, &r, MAX_REFLECTION_RECURSION_DEPTH);

        let c_expected = Color::new(0.38066125, 0.47583, 0.2855);
        assert_color(&c_expected, &c);
    }

    // page 97 duplicate, but thats ok
    #[test]
    fn test_color_at_inner_sphere() {
        let mut w = default_world();

        let origin = Tuple4D::new_point(0.0, 0.0, 0.75);
        let direction = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let r = Ray::new(origin, direction);

        let shapes = w.get_shapes_mut();

        let outer_shape = shapes.get_mut(0).unwrap();
        outer_shape.get_material_mut().set_ambient(1.0);

        let inner_shape = shapes.get_mut(1).unwrap();
        inner_shape.get_material_mut().set_ambient(1.0);

        // TODO: using clone() here so the borrow checker is happy. its a test -> so its ok
        let c_expected = inner_shape.get_material_mut().get_color().clone();

        let c = World::color_at(&w, &r, MAX_REFLECTION_RECURSION_DEPTH);

        assert_color(&c_expected, &c);
    }

    // page 111
    #[test]
    fn test_point_in_shadow_collinear() {
        let w = default_world();
        let p = Tuple4D::new_point(0.0, 10.0, 0.0);
        let is_shadowed = World::is_shadowed(&w, w.get_light().get_position(), &p);
        assert_eq!(is_shadowed, false);
    }

    // page 112 top
    #[test]
    fn test_point_in_shadow_object_between_point_and_light() {
        let w = default_world();
        let p = Tuple4D::new_point(10.0, -10.0, 10.0);
        let is_shadowed = World::is_shadowed(&w, w.get_light().get_position(), &p);
        assert_eq!(is_shadowed, true);
    }

    // page 112 center
    #[test]
    fn test_point_in_shadow_object_behind_light() {
        let w = default_world();
        let p = Tuple4D::new_point(-20.0, 20.0, -20.0);
        let is_shadowed = World::is_shadowed(&w, w.get_light().get_position(), &p);
        assert_eq!(is_shadowed, false);
    }

    // page 112 bottom
    #[test]
    fn test_point_in_shadow_object_behind_point() {
        let w = default_world();
        let p = Tuple4D::new_point(-2.0, 2.0, -2.0);
        let is_shadowed = World::is_shadowed(&w, w.get_light().get_position(), &p);
        assert_eq!(is_shadowed, false);
    }

    // page 144 to
    #[test]
    fn test_material_precomputing_reflection_non_reflective_material() {
        let mut w = default_world_empty();

        // add the two shapes from "default_word" but set the required propertys
        let mut m = Material::new();
        m.set_color(Color::new(0.8, 1., 0.6));
        m.set_diffuse(0.7);
        m.set_specular(0.2);

        let mut s1 = Sphere::new();
        s1.set_material(m);
        let shape1 = Shape::new(ShapeEnum::Sphere(s1), "Sphere");

        let m = Matrix::scale(0.5, 0.5, 0.5);
        let mut s2 = Sphere::new();
        s2.set_transformation(m);
        s2.get_material_mut().set_ambient(1.0);
        let shape2 = Shape::new(ShapeEnum::Sphere(s2), "Sphere");

        w.add_shape(shape1);
        w.add_shape(shape2);

        let p = Tuple4D::new_point(0.0, 0.0, 0.0);
        let o = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(p, o);

        let s = &w.get_shapes()[1];
        let i = Intersection::new(1.0, s);

        let comps = Intersection::prepare_computations(&i, &r, &IntersectionList::new());

        let color = World::reflected_color(&w, &comps, MAX_REFLECTION_RECURSION_DEPTH);
        let color_expected = Color::new(0.0, 0.0, 0.0);

        assert_color(&color, &color_expected);
    }

    // page 144  bottom
    #[test]
    fn test_material_precomputing_reflection_reflective_material() {
        let mut w: World = default_world();

        let mut p = Plane::new();
        p.get_material_mut().set_reflective(0.5);
        let m = Matrix::translation(0.0, -1.0, 0.0);
        p.set_transformation(m);
        let plane = Shape::new(ShapeEnum::Plane(p), "Sphere");
        w.add_shape(plane);

        let p = Tuple4D::new_point(0.0, 0.0, -3.0);
        let o = Tuple4D::new_vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0);
        let r = Ray::new(p, o);

        let s = &w.get_shapes()[2];
        let i = Intersection::new(SQRT_2, s);

        let comps = Intersection::prepare_computations(&i, &r, &IntersectionList::new());

        let color = World::reflected_color(&w, &comps, MAX_REFLECTION_RECURSION_DEPTH);
        let color_expected = Color::new(0.190332201495133, 0.23791525186891627, 0.14274915112134975);

        // TODO: this fails - probably/hopefully because the is_shadowed method is broken
        // fix this, when the shadows work

        println!("expected color    = {:?}", color_expected);
        println!("actual color      = {:?}", color);
        assert_color(&color, &color_expected);
    }

    // page 145
    #[test]
    fn test_material_shade_hit_reflective_material() {
        let mut w: World = default_world();

        let mut p = Plane::new();
        p.get_material_mut().set_reflective(0.5);
        let m = Matrix::translation(0.0, -1.0, 0.0);
        p.set_transformation(m);
        let plane = Shape::new(ShapeEnum::Plane(p), "plane");
        w.add_shape(plane);

        let p = Tuple4D::new_point(0.0, 0.0, -3.0);
        let o = Tuple4D::new_vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0);
        let r = Ray::new(p, o);

        let s = &w.get_shapes()[2];
        let i = Intersection::new(SQRT_2, s);

        let comps = Intersection::prepare_computations(&i, &r, &IntersectionList::new());

        let color = World::shade_hit(&w, &comps, MAX_REFLECTION_RECURSION_DEPTH);
        let color_expected = Color::new(0.8767572837020907, 0.924340334075874, 0.8291742333283075);

        println!("expected color    = {:?}", color_expected);
        println!("actual color      = {:?}", color);

        // TODO: this fails - probably/hopefully because the is_shadowed mehtod is borken
        // fix this, when the shadows work
        assert_color(&color, &color_expected);
    }

    // page 146
    #[test]
    fn test_material_shade_hit_handle_recursion() {
        let mut w = World::new();

        let mut l = Plane::new();
        let m_lower = Matrix::translation(0.0, -1.0, 0.0);
        l.set_transformation(m_lower);
        l.get_material_mut().set_reflective(1.0);

        let mut u = Plane::new();
        let m_upper = Matrix::translation(0.0, 1.0, 0.0);
        u.set_transformation(m_upper);
        u.get_material_mut().set_reflective(1.0);

        let upper = Shape::new(ShapeEnum::Plane(u), "plane");
        let lower = Shape::new(ShapeEnum::Plane(l), "plane");

        w.add_shape(lower);
        w.add_shape(upper);

        let p = Tuple4D::new_point(0.0, 0.0, 0.0);
        let o = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(p, o);

        let color = World::color_at(&w, &r, MAX_REFLECTION_RECURSION_DEPTH);
        println!("bla");
        assert!(true, false);
    }

    // page 147
    #[test]
    fn test_material_shade_hit_reflective_material_max_recursive_depth() {
        let mut w: World = default_world();

        let mut p = Plane::new();
        p.get_material_mut().set_reflective(0.5);
        let m = Matrix::translation(0.0, -1.0, 0.0);
        p.set_transformation(m);
        let plane = Shape::new(ShapeEnum::Plane(p), "plane");

        w.add_shape(plane);

        let p = Tuple4D::new_point(0.0, 0.0, -3.0);
        let o = Tuple4D::new_vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0);
        let r = Ray::new(p, o);

        let s = &w.get_shapes()[2];
        let i = Intersection::new(SQRT_2, s);

        let comps = Intersection::prepare_computations(&i, &r, &IntersectionList::new());

        let color = World::reflected_color(&w, &comps, 0);
        let color_expected = Color::new(0., 0., 0.);

        // TODO: this fails - probably/hopefully because the is_shadowed mehtod is borken
        // fix this, when the shadows work
        assert_color(&color, &color_expected);
    }

    // page 154
    #[test]
    fn test_prepare_computations_under_point() {
        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let mut s = glass_sphere();
        let m_trans = Matrix::translation(0.0, 0.0, 1.0);
        s.set_transformation(m_trans);

        let shape1 = Shape::new(ShapeEnum::Sphere(s), "sphere");

        let i = Intersection::new(5.0, &shape1);
        let i_clone = Intersection::new(5.0, &shape1);

        let mut xs = IntersectionList::new();
        xs.add(i_clone);

        let comps = Intersection::prepare_computations(&i, &r, &xs);

        assert!(comps.get_under_point().z > EPSILON / 2.0);
        assert!(comps.get_point().z < comps.get_under_point().z);
    }

    // page 155
    #[test]
    fn test_refracted_color() {
        let w = default_world();

        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let s = &w.get_shapes()[0];

        let i1 = Intersection::new(4.0, s);
        let i2 = Intersection::new(6.0, &s);

        let mut xs = IntersectionList::new();
        xs.add(i1);
        xs.add(i2);

        let comps = Intersection::prepare_computations(&xs.get_intersections()[0], &r, &xs);

        let c = World::refracted_color(&w, &comps, 5);
        let c_expected = Color::new(0.0, 0.0, 0.0);

        println!("expected color    = {:?}", c_expected);
        println!("actual color      = {:?}", c);

        assert_color(&c, &c_expected);
    }

    // page 156
    #[test]
    fn test_refracted_color_max_recursion() {
        let mut w = default_world_empty();

        // add the two shapes from "default_word" but set the required propertys
        let mut m = Material::new();
        m.set_transparency(1.0);
        m.set_refractive_index(1.5);

        let mut s1 = Sphere::new();
        s1.set_material(m);
        let shape1 = Shape::new(ShapeEnum::Sphere(s1), "sphere");

        let m = Matrix::scale(0.5, 0.5, 0.5);
        let mut s2 = Sphere::new();
        s2.set_transformation(m);
        s2.get_material_mut().set_ambient(1.0);
        let shape2 = Shape::new(ShapeEnum::Sphere(s2), "sphere");

        w.add_shape(shape1);
        w.add_shape(shape2);

        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let i1 = Intersection::new(4.0, &w.get_shapes()[0]);
        let i2 = Intersection::new(6.0, &w.get_shapes()[0]);

        let mut xs = IntersectionList::new();
        xs.add(i1);
        xs.add(i2);

        let comps = Intersection::prepare_computations(&xs.get_intersections()[0], &r, &xs);

        let c = World::refracted_color(&w, &comps, 0);
        let c_expected = Color::new(0.0, 0.0, 0.0);

        println!("expected color    = {:?}", c_expected);
        println!("actual color      = {:?}", c);

        assert_color(&c, &c_expected);
    }

    // page 157
    #[test]
    fn test_refracted_color_total_internal_reflection() {
        let mut w = default_world();

        let shapes = w.get_shapes_mut();
        let shape1 = shapes.get_mut(0).unwrap();
        shape1.get_material_mut().set_transparency(1.0);
        shape1.get_material_mut().set_refractive_index(1.5);

        let origin = Tuple4D::new_point(0.0, 0.0, SQRT_2 / 2.0);
        let direction = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(origin, direction);

        let s1 = shape1.clone();
        let i1 = Intersection::new(-SQRT_2 / 2.0, &s1);
        let i2 = Intersection::new(SQRT_2 / 2.0, &s1);

        let mut xs = IntersectionList::new();
        xs.add(i1);
        xs.add(i2);

        let comps = Intersection::prepare_computations(&xs.get_intersections()[1], &r, &xs);
        let c = World::refracted_color(&w, &comps, 5);
        let c_expected = Color::new(0.0, 0.0, 0.0);

        println!("expected color    = {:?}", c_expected);
        println!("actual color      = {:?}", c);

        assert_color(&c, &c_expected);
    }

    // page 158
    #[test]
    fn test_refracted_color_with_refracted_ray() {
        let w = default_world_refracted_color_page_158();

        let origin = Tuple4D::new_point(0.0, 0.0, 0.1);
        let direction = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(origin, direction);

        let i1 = Intersection::new(-0.9899, w.get_shapes().get(0).unwrap());
        let i2 = Intersection::new(-0.4899, w.get_shapes().get(1).unwrap());
        let i3 = Intersection::new(0.4899, w.get_shapes().get(1).unwrap());
        let i4 = Intersection::new(0.9899, w.get_shapes().get(0).unwrap());

        let mut xs = IntersectionList::new();
        xs.add(i1);
        xs.add(i2);
        xs.add(i3);
        xs.add(i4);

        let comps = Intersection::prepare_computations(&xs.get_intersections()[2], &r, &xs);

        let c = World::refracted_color(&w, &comps, 5);
        let c_expected = Color::new(0.0, 0.99888, 0.04725);

        println!("expected color    = {:?}", c_expected);
        println!("actual color      = {:?}", c);

        assert_color(&c, &c_expected);
    }

    // page 159
    #[test]
    fn test_refracted_color_shade_hit() {
        let mut w = default_world();

        let m = Matrix::translation(0.0, -1.0, 0.0);
        let mut plane = Plane::new();
        plane.set_transformation(m);
        plane.get_material_mut().set_transparency(0.5);
        plane.get_material_mut().set_refractive_index(1.5);

        let m = Matrix::translation(0.0, -3.5, -0.5);
        let mut ball = Sphere::new();
        ball.set_transformation(m);
        ball.get_material_mut().set_ambient(0.5);
        ball.get_material_mut().set_color(Color::new(1.0, 0.0, 0.0));

        let plane = Shape::new(ShapeEnum::Plane(plane), "sphere");
        let sphere = Shape::new(ShapeEnum::Sphere(ball), "sphere");
        w.add_shape(plane.clone());
        w.add_shape(sphere);

        let origin = Tuple4D::new_point(0.0, 0.0, -3.0);
        let direction = Tuple4D::new_vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0);
        let r = Ray::new(origin, direction);

        let i1 = Intersection::new(SQRT_2, &plane);
        let mut xs = IntersectionList::new();
        xs.add(i1);

        let comps = Intersection::prepare_computations(&xs.get_intersections()[0], &r, &xs);

        let c = World::shade_hit(&w, &comps, 5);
        let c_expected = Color::new(0.93642, 0.6864253889815014, 0.6864253889815014);

        println!("expected color    = {:?}", c_expected);
        println!("actual color      = {:?}", c);

        assert_color(&c, &c_expected);
    }

    // bonus: soft shadows // area light
    fn test_area_lights_occlusion_between_2_points_helper(point: Tuple4D, expected_result: bool) {
        let w = default_world();
        let light_position = Tuple4D::new_point(-10.0, -10.0, -10.0);

        let result = World::is_shadowed(&w, &light_position, &point);
        assert_eq!(result, expected_result);
    }

    // occlusion between 2 points
    #[test]
    fn test_area_lights_occlusion_between_2_points() {
        let point = Tuple4D::new_point(-10.0, -10.0, 10.0);
        test_area_lights_occlusion_between_2_points_helper(point, false);

        let point = Tuple4D::new_point(10.0, 10.0, 10.0);
        test_area_lights_occlusion_between_2_points_helper(point, true);

        let point = Tuple4D::new_point(-20.0, -20.0, 20.0);
        test_area_lights_occlusion_between_2_points_helper(point, false);

        let point = Tuple4D::new_point(-5.0, -5.0, -5.0);
        test_area_lights_occlusion_between_2_points_helper(point, false);
    }

    // bonus: soft shadows // area light
    fn test_area_lights_point_lights_evaluate_light_intensity_helper(point: Tuple4D, expected_result: f64) {
        let w = default_world();
        let light = w.get_light();

        let result = World::intensity_at(light, &point, &w);
        assert_float(result, expected_result);
    }

    // point lights evaluate light intensity
    #[test]
    fn test_area_lights_point_lights_evaluate_light_intensity() {
        let point = Tuple4D::new_point(0.0, 1.0001, 0.0);
        test_area_lights_point_lights_evaluate_light_intensity_helper(point, 1.0);

        let point = Tuple4D::new_point(-1.0001, 0.0, 0.0);
        test_area_lights_point_lights_evaluate_light_intensity_helper(point, 1.0);

        let point = Tuple4D::new_point(0.0, 0.0, -1.0001);
        test_area_lights_point_lights_evaluate_light_intensity_helper(point, 1.0);

        let point = Tuple4D::new_point(0.0, 0.0, 1.0001);
        test_area_lights_point_lights_evaluate_light_intensity_helper(point, 0.0);

        let point = Tuple4D::new_point(1.0001, 0.0, 0.0);
        test_area_lights_point_lights_evaluate_light_intensity_helper(point, 0.0);

        let point = Tuple4D::new_point(0.0, -1.0001, 0.0);
        test_area_lights_point_lights_evaluate_light_intensity_helper(point, 0.0);

        let point = Tuple4D::new_point(0.0, 0.0, 0.0);
        test_area_lights_point_lights_evaluate_light_intensity_helper(point, 0.0);
    }

    // bonus: soft shadows // area light find single point on light
    fn test_area_lights_single_point_on_area_light_helper(u: f64, v: f64, expected_result: Tuple4D) {
        let corner = Tuple4D::new_point(0.0, 0.0, 0.0);
        let v1 = Tuple4D::new_vector(2.0, 0.0, 0.0);
        let v2 = Tuple4D::new_vector(0.0, 0.0, 1.0);

        let usteps = 4;
        let vsteps = 2;

        let intensity = Color::new(1.0, 1.0, 1.0);

        let arealight = AreaLight::new(corner, v1, usteps, v2, vsteps, intensity);
        let light = LightEnum::AreaLight(arealight);

        let result = World::point_on_light(&light, u as usize, v as usize);
        assert_tuple(&result, &expected_result);
    }

    // Finding a single point on an area light
    #[test]
    fn test_area_lights_single_point_on_area_light() {
        let result = Tuple4D::new_point(0.25, 0.0, 0.25);
        test_area_lights_single_point_on_area_light_helper(0.0, 0.0, result);

        let result = Tuple4D::new_point(0.75, 0.0, 0.25);
        test_area_lights_single_point_on_area_light_helper(1.0, 0.0, result);

        let result = Tuple4D::new_point(0.25, 0.0, 0.75);
        test_area_lights_single_point_on_area_light_helper(0.0, 1.0, result);

        let result = Tuple4D::new_point(1.25, 0.0, 0.25);
        test_area_lights_single_point_on_area_light_helper(2.0, 0.0, result);

        let result = Tuple4D::new_point(1.75, 0.0, 0.75);
        test_area_lights_single_point_on_area_light_helper(3.0, 1.0, result);
    }

    // bonus: soft shadows // area light find single point on light
    fn test_area_lights_intensity_at_helper(point: Tuple4D, expected_result: f64) {
        let w = default_world();

        let corner = Tuple4D::new_point(-0.5, -0.5, -5.0);
        let v1 = Tuple4D::new_vector(1.0, 0.0, 0.0);
        let v2 = Tuple4D::new_vector(0.0, 1.0, 0.0);

        let usteps = 2;
        let vsteps = 2;

        let intensity = Color::new(1.0, 1.0, 1.0);

        let arealight = AreaLight::new(corner, v1, usteps, v2, vsteps, intensity);
        let light = LightEnum::AreaLight(arealight);

        let result = World::intensity_at(&light, &point, &w);
        assert_float(expected_result, result);
    }

    // Finding a single point on an area light
    #[test]
    fn test_area_lights_intensity_at() {
        let point = Tuple4D::new_point(0.0, 0.0, 2.0);
        test_area_lights_intensity_at_helper(point, 0.0);

        let point = Tuple4D::new_point(1.0, -1.0, 2.0);
        test_area_lights_intensity_at_helper(point, 0.25);

        let point = Tuple4D::new_point(1.5, 0.0, 2.0);
        test_area_lights_intensity_at_helper(point, 0.5);

        let point = Tuple4D::new_point(1.25, 1.25, 3.0);
        test_area_lights_intensity_at_helper(point, 0.75);

        let point = Tuple4D::new_point(0.0, 0.0, -2.0);
        test_area_lights_intensity_at_helper(point, 1.0);
    }
}
