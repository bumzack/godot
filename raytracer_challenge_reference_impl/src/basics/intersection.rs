use std::fmt;

use crate::basics::precomputed_component::PrecomputedComponent;
use crate::basics::ray::Ray;
use crate::basics::ray::RayOps;
use crate::material::material::MaterialOps;
use crate::math::common::EPSILON;
use crate::math::tuple4d::{Tuple, Tuple4D};
use crate::shape::cube::{Cube, CubeOps};
use crate::shape::cylinder::{Cylinder, CylinderOps};
use crate::shape::plane::{Plane, PlaneOps};
use crate::shape::shape::{Shape, ShapeEnum};
use crate::shape::sphere::{Sphere, SphereOps};
use crate::shape::triangle::{Triangle, TriangleOps};
use crate::world::world::World;
use crate::world::world::WorldOps;

type IntersectionContainer<'a> = Vec<Intersection<'a>>;

pub struct Intersection<'a> {
    t: f32,
    shape: &'a Shape<'a>,
}

pub trait IntersectionOps<'a> {
    fn new(t: f32, shape: &'a Shape<'a>) -> Intersection<'a>;
    fn intersect(shape: &'a Shape<'a>, r: &Ray) -> IntersectionList<'a>;
    fn intersect_world(w: &'a World, r: &'a Ray) -> IntersectionList<'a>;
    fn get_t(&self) -> f32;
    fn get_shape(&self) -> &'a Shape<'a>;
    fn prepare_computations(
        intersection: &Intersection<'a>,
        r: &Ray,
        list: &IntersectionList<'a>,
    ) -> PrecomputedComponent<'a>;
    fn schlick(comp: &PrecomputedComponent) -> f32;
}

impl<'a> IntersectionOps<'a> for Intersection<'a> {
    fn new(t: f32, shape: &'a Shape<'a>) -> Intersection<'a> {
        Intersection { t, shape }
    }

    fn intersect(shape: &'a Shape<'a>, r: &Ray) -> IntersectionList<'a> {
        let mut intersection_list = IntersectionList::new();
        let r2 = Ray::transform(r, shape.get_inverse_transformation());

        let res = match shape.get_shape() {
            ShapeEnum::Sphere(ref _s) => {
                let res = Sphere::intersect(&r2);
                match res {
                    Some(r) => {
                        let i1 = Intersection::new(r[0], shape);
                        let i2 = Intersection::new(r[1], shape);
                        intersection_list.add(i1);
                        intersection_list.add(i2);
                    }
                    None => {}
                }
                intersection_list
            }

            ShapeEnum::Plane(ref _p) => {
                let res = Plane::intersect(&r2);
                match res {
                    Some(r) => {
                        let i1 = Intersection::new(r[0], shape);
                        intersection_list.add(i1);
                    }
                    None => {}
                }
                intersection_list
            }

            ShapeEnum::Cube(ref _c) => {
                let res = Cube::intersect(&r2);
                match res {
                    Some(r) => {
                        let i1 = Intersection::new(r[0], shape);
                        let i2 = Intersection::new(r[1], shape);
                        intersection_list.add(i1);
                        intersection_list.add(i2);
                    }
                    None => {}
                }
                intersection_list
            }
            ShapeEnum::Cylinder(ref cylinder) => {
                let res = Cylinder::intersect(cylinder, &r2);
                match res {
                    Some(r) => {
                        // can be 1 or 2 intersections
                        for i in r.iter() {
                            let i1 = Intersection::new(*i, shape);
                            intersection_list.add(i1);
                        }
                    }
                    None => {}
                }
                intersection_list
            }
            ShapeEnum::Triangle(ref triangle) => {
                let res = Triangle::intersect(triangle, &r2);
                match res {
                    Some(xs) => {
                        // can be only 1  intersection, but ...
                        for intersection in xs.iter() {
                            let i = Intersection::new(*intersection, shape);
                            intersection_list.add(i);
                        }
                    }
                    None => {}
                }
                intersection_list
            }
        };
        res
    }

    fn intersect_world(w: &'a World, r: &'a Ray) -> IntersectionList<'a> {
        let mut res = IntersectionList::new();
        for shape in w.get_shapes().iter() {
            let mut tmp = Intersection::intersect(shape, r);
            for is in tmp.get_intersections_mut().drain(..) {
                res.add(is);
            }
        }
        res.get_intersections_mut()
            .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));

        // println!("intersect_world   res = {:?}", res);

        res
    }

    fn get_t(&self) -> f32 {
        self.t
    }

    fn get_shape(&self) -> &'a Shape<'a> {
        self.shape
    }

    fn prepare_computations(
        intersection: &Intersection<'a>,
        r: &Ray,
        list: &IntersectionList<'a>,
    ) -> PrecomputedComponent<'a> {
        let point = Ray::position(r, intersection.get_t());
        let mut normal_vector = intersection.get_shape().normal_at(&point);
        let eye_vector = r.get_direction() * (-1.0);
        let mut inside = true;
        if (&normal_vector ^ &eye_vector) < 0.0 {
            normal_vector = normal_vector * (-1.0);
        } else {
            inside = false;
        }
        let reflected_vector = Tuple4D::reflect(r.get_direction(), &normal_vector);

        let over_point = &point + &(&normal_vector * EPSILON);
        let under_point = &point - &(&normal_vector * EPSILON);

        let mut comp = PrecomputedComponent::new(
            intersection.get_t(),
            intersection.get_shape(),
            point,
            over_point,
            under_point,
            eye_vector,
            normal_vector,
            reflected_vector,
            inside,
        );

//        let mut container: Vec<&'a Shape<'a>> = Vec::new();
//
//        for i in list.get_intersections().iter() {
//            if i == intersection {
//                if container.is_empty() {
//                    comp.set_n1(1.0);
//                } else {
//                    let last = container.last().unwrap();
//                    comp.set_n1(last.get_material().get_refractive_index());
//                }
//            }
//
//            if container.contains(&comp.get_object()) {
//                let index = container.iter().position(|&shape| shape == comp.get_object()).unwrap();
//                container.remove(index);
//            } else {
//                container.push(i.get_shape());
//            }
//
//            if i == intersection {
//                if container.is_empty() {
//                    comp.set_n2(1.0);
//                } else {
//                    let last = container.last().unwrap();
//                    comp.set_n2(last.get_material().get_refractive_index());
//                }
//                break;
//            }
//        }
        comp
    }

    fn schlick(comp: &PrecomputedComponent) -> f32 {
        let mut cos = comp.get_eye_vector() ^ comp.get_normal_vector();
        if comp.get_n1() > comp.get_n2() {
            let n = comp.get_n1() / comp.get_n2();
            let sint2_t = n * n * (1.0 - cos * cos);
            if sint2_t > 1.0 {
                return 1.0;
            }
            let cos_t = (1.0 - sint2_t).sqrt();
            cos = cos_t;
        }
        let r0 = ((comp.get_n1() - comp.get_n2()) / (comp.get_n1() + comp.get_n2())).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.shape == other.shape
    }
}

impl<'a> fmt::Debug for Intersection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "is shape t = {}", self.t)
    }
}

pub struct IntersectionList<'a> {
    list_of_intersections: IntersectionContainer<'a>,
}

pub trait IntersectionListOps<'a> {
    fn new() -> IntersectionList<'a>;
    fn add(&mut self, i: Intersection<'a>);

    fn hit(&self) -> Option<&Intersection<'a>>;

    fn get_intersections(&self) -> &IntersectionContainer<'a>;
    fn get_intersections_mut(&mut self) -> &mut IntersectionContainer<'a>;
}

impl<'a> IntersectionListOps<'a> for IntersectionList<'a> {
    fn new() -> IntersectionList<'a> {
        IntersectionList {
            list_of_intersections: Vec::new(),
        }
    }

    fn add(&mut self, i: Intersection<'a>) {
        self.list_of_intersections.push(i);
        self.list_of_intersections
            .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));
        // expect("IntersectionListOps::add : cant unwrap"));
    }

    fn hit(&self) -> Option<&Intersection<'a>> {
        self.list_of_intersections.iter().find(|&i| i.t >= 0.0)
    }

    fn get_intersections(&self) -> &IntersectionContainer<'a> {
        &self.list_of_intersections
    }

    fn get_intersections_mut(&mut self) -> &mut IntersectionContainer<'a> {
        &mut self.list_of_intersections
    }
}

impl<'a> fmt::Debug for IntersectionList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.list_of_intersections.iter() {
            writeln!(f, "isl  {:?}", i)?;
        }
        writeln!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::SQRT_2;

    use crate::basics::color::{Color, ColorOps};
    use crate::math::common::{assert_color, assert_float, assert_tuple};
    use crate::math::matrix::{Matrix, MatrixOps};
    use crate::math::tuple4d::{Tuple, Tuple4D};
    use crate::shape::sphere::glass_sphere;
    use crate::world::world::default_world;

    use super::*;

    #[test]
    fn test_new_intersection() {
        let s = Sphere::new();
        let t: f32 = 3.5;
        let o = Shape::new(ShapeEnum::Sphere(s), "test sphere");
        let i = Intersection::new(t, &o);
        assert_eq!(i.t, 3.5);
    }

    #[test]
    fn test_new_intersectionlist() {
        let s1 = Sphere::new();
        let t1: f32 = 3.5;
        let o1 = Shape::new(ShapeEnum::Sphere(s1), "test sphere");
        let i1 = Intersection::new(t1, &o1);

        let s2 = Sphere::new();
        let t2: f32 = 4.5;
        let o2 = Shape::new(ShapeEnum::Sphere(s2), "test sphere2");
        let i2 = Intersection::new(t2, &o2);

        // let i_list = IntersectionList::new();

        let mut il = IntersectionList::new();
        il.add(i1);
        il.add(i2);

        // TODO: test ???
    }

    // page 65
    #[test]
    fn test_intersection_hit() {
        let s = Sphere::new();
        let o = Shape::new(ShapeEnum::Sphere(s), "test sphere");

        let t1: f32 = 1.0;
        let i1 = Intersection::new(t1, &o);

        let t2: f32 = 2.0;
        let i2 = Intersection::new(t2, &o);

        let mut il = IntersectionList::new();
        il.add(i2);
        il.add(i1);

        let i = il.hit().unwrap();

        assert_eq!(i.t, 1.0);
    }

    // page 65
    #[test]
    fn test_intersection_hit_neg() {
        let s = Sphere::new();
        let o = Shape::new(ShapeEnum::Sphere(s), "test sphere");

        let t1: f32 = -1.0;
        let i1 = Intersection::new(t1, &o);

        let t2: f32 = 1.0;
        let i2 = Intersection::new(t2, &o);

        let mut il = IntersectionList::new();
        il.add(i2);
        il.add(i1);

        let i = il.hit().unwrap();

        assert_eq!(i.t, 1.0);
    }

    // page 65
    #[test]
    fn test_intersection_no_hit() {
        let s = Sphere::new();
        let o = Shape::new(ShapeEnum::Sphere(s), "test sphere");

        let t1: f32 = -1.0;
        let i1 = Intersection::new(t1, &o);

        let t2: f32 = -2.0;
        let i2 = Intersection::new(t2, &o);

        let mut il = IntersectionList::new();
        il.add(i2);
        il.add(i1);

        let i = il.hit();

        // TODO - how to assert????
        assert_eq!(i.is_none(), true);
    }

    // page 66 top
    #[test]
    fn test_intersection_hit_from_list() {
        let s = Sphere::new();
        let o = Shape::new(ShapeEnum::Sphere(s), "test sphere");

        let t1: f32 = 5.0;
        let i1 = Intersection::new(t1, &o);

        let t2: f32 = 7.0;
        let i2 = Intersection::new(t2, &o);

        let t3: f32 = -3.0;
        let i3 = Intersection::new(t3, &o);

        let t4: f32 = 2.0;
        let i4 = Intersection::new(t4, &o);

        let mut il = IntersectionList::new();
        il.add(i1);
        il.add(i2);
        il.add(i3);
        il.add(i4);

        let i = il.hit().unwrap();

        assert_eq!(i.t, 2.0);
        // intersections are sorted, t=2 is second element in list
        assert_eq!(i, &il.get_intersections()[1]);
    }

    #[test]
    fn test_intersect() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();
        let o = Shape::new(ShapeEnum::Sphere(s), "test sphere");

        let i = Intersection::intersect(&o, &r);
        let intersections = i.get_intersections();
        assert_eq!(intersections.len(), 2);
    }

    // page 93
    #[test]
    fn test_prepare_computations() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();
        let o = Shape::new(ShapeEnum::Sphere(s), "test sphere");

        let i = Intersection::new(4.0, &o);

        let c = Intersection::prepare_computations(&i, &r, &IntersectionList::new());

        let point_expected = Tuple4D::new_point(0.0, 0., -1.0);
        let eye_vector_expected = Tuple4D::new_vector(0.0, 0., -1.0);
        let normal_vector_expected = Tuple4D::new_vector(0.0, 0., -1.0);

        assert_tuple(&point_expected, c.get_point());
        assert_tuple(&eye_vector_expected, c.get_eye_vector());
        assert_tuple(&normal_vector_expected, c.get_normal_vector());
        assert_float(i.get_t(), c.get_t());
    }

    // page 94
    #[test]
    fn test_prepare_computations_hit_outside() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();
        let o = Shape::new(ShapeEnum::Sphere(s), "test sphere");
        let i = Intersection::new(4.0, &o);
        let c = Intersection::prepare_computations(&i, &r, &IntersectionList::new());

        assert_eq!(false, c.get_inside());
    }

    // page 95 top
    #[test]
    fn test_precomputations_hit_inside() {
        let o = Tuple4D::new_point(0.0, 0.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();
        let o = Shape::new(ShapeEnum::Sphere(s), "test sphere");
        let i = Intersection::new(1.0, &o);
        let c = Intersection::prepare_computations(&i, &r, &IntersectionList::new());

        let point_expected = Tuple4D::new_point(0.0, 0.0, 1.0);
        let eye_vector_expected = Tuple4D::new_vector(0.0, 0., -1.0);
        let normal_vector_expected = Tuple4D::new_vector(0.0, 0., -1.0);

        assert_tuple(&point_expected, c.get_point());
        assert_tuple(&eye_vector_expected, c.get_eye_vector());
        assert_tuple(&normal_vector_expected, c.get_normal_vector());
        assert_eq!(true, c.get_inside());
    }

    // page 161
    #[test]
    fn test_precomputations_schlick() {
        let sphere = glass_sphere();

        let o = Tuple4D::new_point(0.0, 0.0, SQRT_2 / 2.0);
        let d = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(o, d);

        let sphere = Shape::new(ShapeEnum::Sphere(sphere), "test sphere");
        let i1 = Intersection::new(-SQRT_2 / 2.0, &sphere);
        let i2 = Intersection::new(SQRT_2 / 2.0, &sphere);
        let mut xs = IntersectionList::new();
        xs.add(i1);
        xs.add(i2);
        let c = Intersection::prepare_computations(&xs.get_intersections()[1], &r, &xs);

        let reflectance = Intersection::schlick(&c);

        assert_float(reflectance, 1.0);
    }

    // page 162
    #[test]
    fn test_precomputations_schlick_perpendicular_viewing_angle() {
        let sphere = glass_sphere();

        let o = Tuple4D::new_point(0.0, 0.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(o, d);

        let sphere = Shape::new(ShapeEnum::Sphere(sphere), "test sphere");
        let i1 = Intersection::new(-1.0, &sphere);
        let i2 = Intersection::new(1.0, &sphere);
        let mut xs = IntersectionList::new();
        xs.add(i1);
        xs.add(i2);
        let c = Intersection::prepare_computations(&xs.get_intersections()[1], &r, &xs);

        let reflectance = Intersection::schlick(&c);

        assert_float(reflectance, 0.04);
    }

    // page 163
    #[test]
    fn test_precomputations_schlick_approx_with_small_angle() {
        let sphere = glass_sphere();

        let o = Tuple4D::new_point(0.0, 0.99, -2.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let sphere = Shape::new(ShapeEnum::Sphere(sphere), "test sphere");
        let i1 = Intersection::new(1.8589, &sphere);
        let mut xs = IntersectionList::new();
        xs.add(i1);
        let c = Intersection::prepare_computations(&xs.get_intersections()[0], &r, &xs);

        let reflectance = Intersection::schlick(&c);

        assert_float(reflectance, 0.48873);
    }

    // page 164 - based on test from page 159
    #[test]
    fn test_precomputations_schlick_reflective_transparent_material() {
        let mut w = default_world();

        let m = Matrix::translation(0.0, -1.0, 0.0);
        let mut floor = Plane::new();
        floor.set_transformation(m);
        floor.get_material_mut().set_reflective(0.5);
        floor.get_material_mut().set_transparency(0.5);
        floor.get_material_mut().set_refractive_index(1.5);

        let m = Matrix::translation(0.0, -3.5, -0.5);
        let mut ball = Sphere::new();
        ball.set_transformation(m);
        ball.get_material_mut().set_ambient(0.5);
        ball.get_material_mut().set_color(Color::new(1.0, 0.0, 0.0));

        let floor = Shape::new(ShapeEnum::Plane(floor), "floor");
        let ball = Shape::new(ShapeEnum::Sphere(ball), "ball sphere");

        w.add_shape(floor.clone());
        w.add_shape(ball);

        let origin = Tuple4D::new_point(0.0, 0.0, -3.0);
        let direction = Tuple4D::new_vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0);
        let r = Ray::new(origin, direction);

        let i1 = Intersection::new(SQRT_2, &floor);
        let mut xs = IntersectionList::new();
        xs.add(i1);

        let comps = Intersection::prepare_computations(&xs.get_intersections()[0], &r, &xs);

        let c = World::shade_hit(&w, &comps, 5);
        let c_expected = Color::new(0.93391, 0.69643, 0.69243);

        println!("expected color    = {:?}", c_expected);
        println!("actual color      = {:?}", c);

        assert_color(&c, &c_expected);
    }
}
