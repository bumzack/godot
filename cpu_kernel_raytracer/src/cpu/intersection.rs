use raytracer_lib_no_std::basics::precomputed_component::PrecomputedComponent;
use raytracer_lib_no_std::basics::ray::{Ray, RayOps};
use raytracer_lib_no_std::math::tuple4d::{Tuple, Tuple4D};
use raytracer_lib_no_std::shape::shape::{Shape, ShapeEnum};
use raytracer_lib_no_std::{ShapeOps, EPSILON_OVER_UNDER, MaterialOps};

use crate::cpu::intersection_list::{IntersectionList, IntersectionListOps};

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    t: f32,
    shape_idx: usize,
}

pub trait IntersectionOps {
    fn new(t: f32, shape_idx: usize) -> Intersection;
    fn new_empty() -> Intersection;

    fn intersect(shape_idx: usize, r: &Ray, shapes: &Vec<Shape>) -> IntersectionList;
    fn intersect_world(shapes: &Vec<Shape>, r: &Ray) -> IntersectionList;

    fn prepare_computations(
        intersection: &Intersection,
        r: &Ray,
        list: &IntersectionList,
        shapes: &Vec<Shape>,
    ) -> PrecomputedComponent;

    fn get_t(&self) -> f32;
    fn get_shape(&self) -> usize;
    fn schlick(comp: &PrecomputedComponent) -> f32;
}

impl IntersectionOps for Intersection {
    fn new(t: f32, shape_idx: usize) -> Intersection {
        Intersection {
            t,
            shape_idx: shape_idx,
        }
    }

    fn new_empty() -> Intersection {
        Intersection {
            t: -99999.9,
            shape_idx: 0,
        }
    }

    fn intersect(shape_idx: usize, r: &Ray, shapes: &Vec<Shape>) -> IntersectionList {
        let mut intersection_list = IntersectionList::new();
        let shape = &shapes[shape_idx];
        let r2 = Ray::transform(r, shape.get_inverse_transformation());

        let (res, res_cnt) = match *shape.get_shape() {
            ShapeEnum::Sphere(ref sphere) => sphere.intersect(&r2),
            ShapeEnum::Plane(ref plane) => plane.intersect(&r2),
            ShapeEnum::Cube(ref cube) => cube.intersect(&r2),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.intersect(&r2),
            ShapeEnum::Triangle(ref triangle) => triangle.intersect(&r2),
            // ShapeEnum::Group(ref group) =>
            // let res = Cylinder::intersect(cylinder, &r2);
            //  }
        };
        for i in 0..res_cnt {
            let intersection = Intersection::new(res[i], shape_idx);
            intersection_list.add(intersection);
        }
        intersection_list
    }

    fn intersect_world(shapes: &Vec<Shape>, r: &Ray) -> IntersectionList {
        let mut res = IntersectionList::new();
        for i in 0..shapes.len() {
            let tmp = Intersection::intersect(i, r, shapes);
            for idx in 0..tmp.len() {
                // TODO: something like a drain would be awesome and avoid copying
                // we want to move all intersections from tmp to res ...
                res.add(tmp.at(idx));
            }
        }
        res.sort_intersections();
        res
    }

    fn prepare_computations(
        intersection: &Intersection,
        r: &Ray,
        list: &IntersectionList,
        shapes: &Vec<Shape>,
    ) -> PrecomputedComponent {
        let point = Ray::position(r, intersection.get_t());
        let shape = &shapes[intersection.get_shape()];
        let mut normal_vector = shape.normal_at(&point);
        let eye_vector = r.get_direction() * (-1.0);
        let mut inside = true;
        if (&normal_vector ^ &eye_vector) < 0.0 {
            normal_vector = normal_vector * (-1.0);
        } else {
            inside = false;
        }
        let reflected_vector = Tuple4D::reflect(r.get_direction(), &normal_vector);

        let over_point = &point + &(&normal_vector * EPSILON_OVER_UNDER);
        let under_point = &point - &(&normal_vector * EPSILON_OVER_UNDER);

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

        let mut container: Vec<&Shape> = Vec::new();

        for i in list.get_intersections().iter() {
            if i == intersection {
                // println!("i == intersection");
                if container.is_empty() {
                    comp.set_n1(1.0);
                } else {
                    let last = container.last().unwrap();
                    comp.set_n1(last.get_material().get_refractive_index());
                }
            }

            let shape = &shapes[i.get_shape()];
            if container.contains(&shape) {
                let index = container.iter().position(|&shape| {
                    let s = &shapes[i.get_shape()];
                    shape == s
                }).unwrap();
                container.remove(index);
            } else {
                let shape = &shapes[i.get_shape()];
                container.push(shape);
            }

            if i == intersection {
                if container.is_empty() {
                    comp.set_n2(1.0);
                } else {
                    let last = container.last().unwrap();
                    comp.set_n2(last.get_material().get_refractive_index());
                }
                break;
            }
        }
        comp
    }

    fn get_t(&self) -> f32 {
        self.t
    }

    fn get_shape(&self) -> usize {
        self.shape_idx
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

impl<'a> PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.shape_idx == other.shape_idx && self.t == other.t
    }
}


#[cfg(test)]
mod tests {
    use std::f32::consts::SQRT_2;

    use super::*;
    use raytracer_lib_no_std::Sphere;

    #[test]
    fn test_new_intersection() {
        let s = Sphere::new();
        let t: f32 = 3.5;
        let o = Shape::new(ShapeEnum::Sphere(s));
        let i = Intersection::new(t, &o);
        assert_eq!(i.t, 3.5);
    }

    #[test]
    fn test_new_intersectionlist() {
        let s1 = Sphere::new();
        let t1: f32 = 3.5;
        let o1 = Shape::new(ShapeEnum::Sphere(s1));
        let i1 = Intersection::new(t1, &o1);

        let s2 = Sphere::new();
        let t2: f32 = 4.5;
        let o2 = Shape::new(ShapeEnum::Sphere(s2));
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
        let o = Shape::new(ShapeEnum::Sphere(s));

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
        let o = Shape::new(ShapeEnum::Sphere(s));

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
        let o = Shape::new(ShapeEnum::Sphere(s));

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
        let o = Shape::new(ShapeEnum::Sphere(s));

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
        let o = Shape::new(ShapeEnum::Sphere(s));

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
        let o = Shape::new(ShapeEnum::Sphere(s));

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
        let o = Shape::new(ShapeEnum::Sphere(s));
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
        let o = Shape::new(ShapeEnum::Sphere(s));
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

        let sphere = Shape::new(ShapeEnum::Sphere(sphere));
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

        let sphere = Shape::new(ShapeEnum::Sphere(sphere));
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

        let sphere = Shape::new(ShapeEnum::Sphere(sphere));
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

        let floor = Shape::new(ShapeEnum::Plane(floor));
        let ball = Shape::new(ShapeEnum::Sphere(ball));

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
        let c_expected = Color::new(0.9337956, 0.6963231, 0.69230264);

        println!("expected color    = {:?}", c_expected);
        println!("actual color      = {:?}", c);

        assert_color(&c, &c_expected);
    }
}
