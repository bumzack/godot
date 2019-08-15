use crate::basics::color::BLACK;
use crate::basics::color::Color;
use crate::basics::color::ColorOps;
use crate::basics::intersection::{Intersection, IntersectionListOps, IntersectionOps};
use crate::basics::precomputed_component::PrecomputedComponent;
use crate::basics::ray::Ray;
use crate::basics::ray::RayOps;
use crate::light::light::{Light, LightOps};
use crate::light::pointlight::PointLight;
use crate::material::material::{Material, MaterialOps};
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;
use crate::shape::shape::Shape;
use crate::shape::sphere::{Sphere, SphereOps};

pub struct World {
    shapes: Vec<Shape>,
    light: Light,
}

pub trait WorldOps<'a> {
    fn new() -> World;
    fn set_light(&mut self, light: Light);
    fn add_shape(&mut self, shape: Shape);
    fn get_shapes(&self) -> &Vec<Shape>;
    fn get_shapes_mut(&mut self) -> &mut Vec<Shape>;

    fn shade_hit(&self, comp: &PrecomputedComponent) -> Color;

    fn color_at(w: &World, r: &Ray) -> Color;

    fn is_shadowed(&self, p: &Tuple4D) -> bool;
}

impl<'a> WorldOps<'a> for World {
    fn new() -> World {
        // TODO: default light ?!?!?! hmm - where, color why not different solution
        let pl = PointLight::new(Tuple4D::new_point(0.0, 0.0, 0.0), Color::new(0.0, 0.0, 0.0));
        World {
            shapes: Vec::new(),
            light: Light::PointLight(pl),
        }
    }

    fn set_light(&mut self, light: Light) {
        self.light = light;
    }

    fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    fn get_shapes(&self) -> &Vec<Shape> {
        &self.shapes
    }

    fn get_shapes_mut(&mut self) -> &mut Vec<Shape> {
        &mut self.shapes
    }

    fn shade_hit(&self, comp: &PrecomputedComponent) -> Color {
        let in_shadow = self.is_shadowed(comp.get_over_point());
        Material::lightning(
            comp.get_shape().get_material(),
            &self.light,
            comp.get_point(),
            comp.get_eye_vector(),
            comp.get_normal_vector(),
            in_shadow,
        )
    }

    fn color_at(w: &World, r: &Ray) -> Color {
        let xs = Intersection::intersect_world(w, r);
        let res = match xs.hit() {
            Some(i) => {
                let comp = Intersection::prepare_computations(&i, &r);
                w.shade_hit(&comp)
            }
            None => BLACK,
        };
        res
    }

    fn is_shadowed(&self, p: &Tuple4D) -> bool {
        let v = self.light.get_position() - p;
        //        println!("light pos = {:?}" ,self.light.get_position());
        //        for s in  self.shapes.iter() {
        //            println!("shape pos = {:?}", s.get_transformation());
        //        }

        let distance = Tuple4D::magnitude(&v);
        let direction = Tuple4D::normalize(&v);

        let point = Tuple4D::new_point_from(&v);
        let r = Ray::new(point, direction);

        let intersections = Intersection::intersect_world(self, &r);

        // println!("intersections = {:?}", intersections);

        let h = intersections.hit();
        //println!("distance= {:?}", distance);
        // println!("t = {:?}", h.unwrap().get_t());

        if h.is_some() {
            // println!("t = {:?}", h.unwrap().get_t());
            if h.unwrap().get_t() < distance {
                return true;
            }
        }
        false
    }
}

pub fn default_world() -> World {
    let mut w = World::new();

    let light_pos = Tuple4D::new_point(-10.0, 10., -10.0);
    let light_intensity = Color::new(1.0, 1.0, 1.0);
    let pl = PointLight::new(light_pos, light_intensity);
    let light = Light::PointLight(pl);
    w.set_light(light);

    let mut m = Material::new();
    m.set_color(Color::new(0.8, 1., 0.6));
    m.set_diffuse(0.7);
    m.set_specular(0.2);

    let mut s1 = Sphere::new();
    s1.set_material(m);
    let shape1 = Shape::Sphere(s1);

    let m = Matrix::scale(0.5, 0.5, 0.5);
    let mut s2 = Sphere::new();
    s2.set_transformation(m);
    let shape2 = Shape::Sphere(s2);

    w.add_shape(shape1);
    w.add_shape(shape2);

    w
}

#[cfg(test)]
mod tests {
    use crate::math::common::{assert_color, EPSILON};

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

        let comps = Intersection::prepare_computations(&i, &r);
        let c = World::shade_hit(&w, &comps);

        let c_expected = Color::new(0.38066125, 0.47583, 0.2855);
        assert_color(&c_expected, &c);
    }

    // page 95
    #[test]
    fn test_shade_hit_inside() {
        let mut w = default_world();

        let pl = PointLight::new(Tuple4D::new_point(0.0, 0.25, 0.0), Color::new(1.0, 1., 1.0));
        w.set_light(Light::PointLight(pl));

        let origin = Tuple4D::new_point(0.0, 0.0, 0.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let shapes = w.get_shapes();
        let shape = shapes.get(1).unwrap();

        let i = Intersection::new(0.5, &shape);

        let comps = Intersection::prepare_computations(&i, &r);
        let c = World::shade_hit(&w, &comps);

        let c_expected = Color::new(0.90498, 0.90498, 0.90498);
        assert_color(&c_expected, &c);
    }

    // page 114
    #[test]
    fn test_shade_hit_shadow() {
        let mut w = World::new();

        let pl = PointLight::new(Tuple4D::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        w.set_light(Light::PointLight(pl));

        let mut s1 = Sphere::new();
        let shape1 = Shape::Sphere(s1);

        let m = Matrix::translation(0.0, 0.0, 10.0);
        let mut s2 = Sphere::new();
        s2.set_transformation(m);
        let shape2 = Shape::Sphere(s2);

        w.add_shape(shape1);
        w.add_shape(shape2);

        let origin = Tuple4D::new_point(0.0, 0.0, 5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let shapes = w.get_shapes();
        let shape = shapes.get(1).unwrap();

        let i = Intersection::new(4.0, &shape);

        let comps = Intersection::prepare_computations(&i, &r);
        let c = World::shade_hit(&w, &comps);
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
        let shape1 = Shape::Sphere(s1);

        let i = Intersection::new(5.0, &shape1);
        let comps = Intersection::prepare_computations(&i, &r);

        assert!(comps.get_over_point().z < -EPSILON / 2.0);
        assert!(comps.get_point().z > comps.get_over_point().z);
    }

    // page 96
    #[test]
    fn test_color_at_no_hit() {
        let mut w = default_world();

        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(origin, direction);

        let c = World::color_at(&w, &r);

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

        let c = World::color_at(&w, &r);

        let c_expected = Color::new(0.38066125, 0.47583, 0.2855);
        assert_color(&c_expected, &c);
    }

    // page 97
    #[test]
    fn test_color_at_inner_sphere() {
        let mut w = default_world();

        let origin = Tuple4D::new_point(0.0, 0.0, 0.75);
        let direction = Tuple4D::new_vector(0.0, 0.0, -1.0);
        let r = Ray::new(origin, direction);

        let mut shapes = w.get_shapes_mut();

        let outer_shape = shapes.get_mut(0).unwrap();
        outer_shape.get_material_mut().set_ambient(1.0);

        let inner_shape = shapes.get_mut(1).unwrap();
        inner_shape.get_material_mut().set_ambient(1.0);

        // TODO: using clone() here so the borrow checker is happy. its a test -> so its ok
        let mut c_expected = inner_shape.get_material_mut().get_color().clone();

        let c = World::color_at(&w, &r);

        assert_color(&c_expected, &c);
    }

    // page 111
    #[test]
    fn test_point_in_shadow_collinear() {
        let mut w = default_world();
        let p = Tuple4D::new_point(0.0, 10.0, 0.0);
        let is_shadowed = w.is_shadowed(&p);
        assert_eq!(is_shadowed, false);
    }

    // page 112 top
    #[test]
    fn test_point_in_shadow_object_between_point_and_light() {
        let mut w = default_world();
        let p = Tuple4D::new_point(10.0, -10.0, 10.0);
        let is_shadowed = w.is_shadowed(&p);
        assert_eq!(is_shadowed, true);
    }

    // page 112 center
    #[test]
    fn test_point_in_shadow_object_behind_light() {
        let mut w = default_world();
        let p = Tuple4D::new_point(-20.0, 20.0, -20.0);
        let is_shadowed = w.is_shadowed(&p);
        assert_eq!(is_shadowed, false);
    }

    // page 112 bottom
    #[test]
    fn test_point_in_shadow_object_behind_point() {
        let mut w = default_world();
        let p = Tuple4D::new_point(-2.0, 2.0, -2.0);
        let is_shadowed = w.is_shadowed(&p);
        assert_eq!(is_shadowed, false);
    }
}
