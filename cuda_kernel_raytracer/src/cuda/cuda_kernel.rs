use raytracer_lib_no_std::basics::color::{Color, ColorOps, BLACK};
use raytracer_lib_no_std::basics::ray::{Ray, RayOps};
use raytracer_lib_no_std::light::light::{Light, LightOps};
use raytracer_lib_no_std::shape::shape::{Shape, ShapeEnum};

use crate::cuda::intersection::Intersection;
use crate::cuda::intersection::IntersectionOps;

use crate::cuda::intersection_list::{IntersectionList, IntersectionListOps};
use core::f32::INFINITY;
use raytracer_lib_no_std::basics::precomputed_component::PrecomputedComponent;
use raytracer_lib_no_std::material::material::{Material, MaterialOps};
use raytracer_lib_no_std::math::math::{intri_powf, intri_powi, intri_sqrt};
use raytracer_lib_no_std::math::tuple4d::{Tuple, Tuple4D};
use raytracer_lib_no_std::shape::sphere::SphereOps;

pub struct CudaKernel {}

// TODO: pass Shape insted of ShapeENum ??!?!?? will this work? why does this work when in the World there are Shapes?! and not ENums?

impl CudaKernel {
    pub fn color_at(
        shapes: *mut Shape,
        cnt_shapes: usize,
        lights: *const Light,
        cnt_lights: usize,
        r: &Ray,
        remaining: i32,
    ) -> Color {
        let mut color = BLACK;

        let xs = Intersection::intersect_world(shapes, cnt_shapes, r);
        let (intersection, is_hit) = xs.hit();
        if is_hit {
            let comp = Intersection::prepare_computations(
                intersection,
                &r,
                &IntersectionList::new(),
                shapes,
                cnt_shapes,
            );
            color = CudaKernel::shade_hit(shapes, cnt_shapes, lights, cnt_lights, &comp, remaining);
        }
        color
    }
    fn shade_hit(
        shapes: *mut Shape,
        cnt_shapes: usize,
        lights: *const Light,
        cnt_lights: usize,
        comp: &PrecomputedComponent,
        remaining: i32,
    ) -> Color {
        // TODO if there is more than 1 light??? pass that to Material::lightning?
        let light = unsafe { lights.offset(0).as_ref().unwrap() };

        let shape = unsafe { shapes.offset(comp.get_object() as isize).as_ref().unwrap() };
        let material = shape.get_material();

        //  let in_shadow = CudaKernel::is_shadowed(w, w.get_light().get_position(), comp.get_over_point());
        let intensity = CudaKernel::intensity_at(
            shapes,
            cnt_shapes,
            lights,
            cnt_lights,
            comp.get_over_point(),
        );

        // TODO: move lightning back to material if mehtod signatures are the same
        let surface = Material::lightning(
            material,
            shape,
            light,
            comp.get_over_point(),
            comp.get_eye_vector(),
            comp.get_normal_vector(),
            intensity,
        );
        let reflected =
            CudaKernel::reflected_color(shapes, cnt_shapes, lights, cnt_lights, comp, remaining);
        let refracted =
            CudaKernel::refracted_color(shapes, cnt_shapes, lights, cnt_lights, comp, remaining);

        // let material = comp.get_object().get_material();
        if material.get_reflective() > 0.0 && material.get_transparency() > 0.0 {
            let reflectance = Intersection::schlick(comp);
            return &surface + &(&reflected * reflectance + &refracted * (1.0 - reflectance));
        }
        &surface + &(&reflected + &refracted)
    }

    fn is_shadowed(
        shapes: *mut Shape,
        cnt_shapes: usize,
        light_position: &Tuple4D,
        position: &Tuple4D,
    ) -> bool {
        let v = light_position - position;

        let distance = Tuple4D::magnitude(&v);
        let mut direction = Tuple4D::normalize(&v);
        direction.w = 0.0;

        let point = Tuple4D::new_point_from(&position);
        let r = Ray::new(point, direction);

        let intersections = Intersection::intersect_world(shapes, cnt_shapes, &r);

        let (intersection, is_hit) = intersections.hit();

        if is_hit {
            if intersection.get_t() < distance {
                return true;
            }
        }
        false
    }

    fn intensity_at(
        shapes: *mut Shape,
        cnt_shapes: usize,
        lights: *const Light,
        cnt_lights: usize,
        point: &Tuple4D,
    ) -> f32 {
        let light = unsafe { lights.offset(0).as_ref().unwrap() };
        let res = match light {
            Light::PointLight(ref _pl) => {
                CudaKernel::intensity_at_point_light(light, point, shapes, cnt_shapes)
            } //  LightEnum::AreaLight(ref pl) => CudaKernel::intensity_at_area_light(light, point, world),
        };
        res
    }

    fn intensity_at_point_light(
        light: &Light,
        point: &Tuple4D,
        shapes: *mut Shape,
        cnt_shapes: usize,
    ) -> f32 {
        if CudaKernel::is_shadowed(shapes, cnt_shapes, light.get_position(), point) {
            return 0.0;
        }
        1.0
    }

    fn reflected_color(
        shapes: *mut Shape,
        cnt_shapes: usize,
        lights: *const Light,
        cnt_lights: usize,
        comp: &PrecomputedComponent,
        remaining: i32,
    ) -> Color {
        if remaining <= 0 {
            return BLACK;
        }

        let shape = unsafe { shapes.offset(comp.get_object() as isize).as_ref().unwrap() };
        let material = shape.get_material();

        if material.get_reflective() == 0.0 {
            return BLACK;
        }
        let reflect_ray = Ray::new(
            Tuple4D::new_point_from(comp.get_over_point()),
            Tuple4D::new_vector_from(comp.get_reflected_vector()),
        );
        let color = CudaKernel::color_at(
            shapes,
            cnt_shapes,
            lights,
            cnt_lights,
            &reflect_ray,
            remaining - 1,
        );
        &color * material.get_reflective()
    }

    fn refracted_color(
        shapes: *mut Shape,
        cnt_shapes: usize,
        lights: *const Light,
        cnt_lights: usize,
        comp: &PrecomputedComponent,
        remaining: i32,
    ) -> Color {
        if remaining <= 0 {
            return BLACK;
        }
        let shape = unsafe { shapes.offset(comp.get_object() as isize).as_ref().unwrap() };
        let material = shape.get_material();
        if material.get_transparency() == 0.0 {
            return BLACK;
        }
        let n_ratio = comp.get_n1() / comp.get_n2();
        let cos_i = comp.get_eye_vector() ^ comp.get_normal_vector();
        let sin2_t = intri_powi(n_ratio, 2) * (1.0 - intri_powi(cos_i, 2));

        // total internal reflection -> return black
        if sin2_t > 1.0 {
            return BLACK;
        }
        let cos_t = intri_sqrt(1.0 - sin2_t);
        let mut direction =
            comp.get_normal_vector() * (n_ratio * cos_i - cos_t) - comp.get_eye_vector() * n_ratio;
        // fix direction to be a vector and not something in between
        direction.w = 0.0;
        let refracted_ray = Ray::new(Tuple4D::new_point_from(comp.get_under_point()), direction);

        CudaKernel::color_at(
            shapes,
            cnt_shapes,
            lights,
            cnt_lights,
            &refracted_ray,
            remaining - 1,
        ) * material.get_transparency()
    }
}
