use core::f32::INFINITY;
use raytracer_lib_no_std::basics::precomputed_component::PrecomputedComponent;
use raytracer_lib_no_std::basics::ray::{Ray, RayOps};
use raytracer_lib_no_std::math::common::{EPSILON, EPSILON_OVER_UNDER};
use raytracer_lib_no_std::math::math::{intri_powi, intri_sqrt};
use raytracer_lib_no_std::math::tuple4d::{Tuple, Tuple4D};
use raytracer_lib_no_std::shape::shape::{Shape, ShapeEnum};
use raytracer_lib_no_std::shape::sphere::{Sphere, SphereOps};

use crate::cuda::intersection_list::IntersectionList;
use crate::cuda::intersection_list::IntersectionListOps;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intersection {
    t: f32,
    shape_idx: usize,
}

pub trait IntersectionOps {
    fn new(t: f32, shape_idx: usize) -> Intersection;
    fn new_empty() -> Intersection;
    fn intersect(
        shape_idx: usize,
        r: &Ray,
        shapes: *mut Shape,
        cnt_shapes: usize,
    ) -> IntersectionList;
    fn intersect_world(shapes: *mut Shape, cnt_shapes: usize, r: &Ray) -> IntersectionList;

    fn prepare_computations(
        intersection: &Intersection,
        r: &Ray,
        list: &IntersectionList,
        shapes: *mut Shape,
        cnt_shapes: usize,
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

    fn intersect(
        shape_idx: usize,
        r: &Ray,
        shapes: *mut Shape,
        cnt_shapes: usize,
    ) -> IntersectionList {
        let shape = unsafe { shapes.offset(shape_idx as isize).as_ref().unwrap() };
        let mut res = IntersectionList::new();
        let res = match *shape.get_shape() {
            ShapeEnum::Sphere(ref sphere) => {
                let mut intersection_list = IntersectionList::new();
                let r2 = Ray::transform(r, sphere.get_inverse_transformation());
                let (res, res_cnt) = Sphere::intersect(&r2);
                for i in 0..res_cnt {
                    let intersection = Intersection::new(res[i], shape_idx);
                    intersection_list.add(intersection);
                }
                intersection_list
            }
        };
        res
    }

    fn intersect_world(shapes: *mut Shape, cnt_shapes: usize, r: &Ray) -> IntersectionList {
        let mut res = IntersectionList::new();
        for i in 0..cnt_shapes {
            let tmp = Intersection::intersect(i, r, shapes, cnt_shapes);
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
        shapes: *mut Shape,
        cnt_shapes: usize,
    ) -> PrecomputedComponent {
        let point = Ray::position(r, intersection.get_t());
        let shape = unsafe {
            shapes
                .offset(intersection.get_shape() as isize)
                .as_ref()
                .unwrap()
        };
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

        // TODO Implement this funny thing foir reflection
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
            let cos_t = intri_sqrt(1.0 - sint2_t);
            cos = cos_t;
        }
        let r0 = intri_powi(
            ((comp.get_n1() - comp.get_n2()) / (comp.get_n1() + comp.get_n2())),
            2,
        );
        r0 + (1.0 - r0) * intri_powi(1.0 - cos, 5)
    }
}
