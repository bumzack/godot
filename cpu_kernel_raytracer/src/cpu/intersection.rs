use raytracer_lib_no_std::basics::precomputed_component::PrecomputedComponent;
use raytracer_lib_no_std::basics::ray::{Ray, RayOps};
use raytracer_lib_no_std::EPSILON_OVER_UNDER;
use raytracer_lib_no_std::math::common::EPSILON;
use raytracer_lib_no_std::math::tuple4d::{Tuple, Tuple4D};
use raytracer_lib_no_std::shape::shape::{Shape, ShapeEnum};
use raytracer_lib_no_std::shape::sphere::{Sphere, SphereOps};

use crate::cpu::intersection_list::{IntersectionList, IntersectionListOps};

#[derive(Clone, Copy, Debug, PartialEq)]
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

       let (res, res_cnt) =  match *shape.get_shape() {
            ShapeEnum::Sphere(ref sphere) => Sphere::intersect(&r2),
            ShapeEnum::Plane(ref _p) => Plane::intersect(&r2) ,
            ShapeEnum::Cube(ref _c) =>  Cube::intersect(&r2),
            ShapeEnum::Cylinder(ref cylinder) =>   Cylinder::intersect(&r2),
            ShapeEnum::Triangle(ref triangle) =>Triangle::intersect(triangle, &r2),
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
        _list: &IntersectionList,
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

        let comp = PrecomputedComponent::new(
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
//        //println!("all intersections:  {:?}", list.get_intersections());
//        //println!("intersection :  {:?}", intersection);
//
//        for i in list.get_intersections().iter() {
//            //            println!("NEXT ITERATION");
//            //            println!(" i = {:?}", i);
//            //            println!("container  begin for    {:?}",container);
//            //
//            if i == intersection {
//                // println!("i == intersection");
//                if container.is_empty() {
//                    comp.set_n1(1.0);
//                } else {
//                    let last = container.last().unwrap();
//
//                    comp.set_n1(last.get_material().get_refractive_index());
//                }
//            }
//
//            if container.contains(&i.get_shape()) {
//                let index = container.iter().position(|&shape| shape == i.get_shape()).unwrap();
//                // println!("remove index     {:}",index);
//                container.remove(index);
//                // println!("container   AFTER      {:?}",container);
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
            let cos_t = (1.0 - sint2_t).sqrt();
            cos = cos_t;
        }
        let r0 = ((comp.get_n1() - comp.get_n2()) / (comp.get_n1() + comp.get_n2())).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl<'a> PartialEq for Intersection  {
    fn eq(&self, other: &Self) -> bool {
        self.shape_idx == other.shape_idx && self.t == other.t
    }
}