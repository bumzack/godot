use raytracer_lib_no_std::basics::color::{BLACK, Color};
use raytracer_lib_no_std::basics::precomputed_component::PrecomputedComponent;
use raytracer_lib_no_std::basics::ray::{Ray, RayOps};
use raytracer_lib_no_std::light::light::{Light, LightOps};
use raytracer_lib_no_std::material::material::{Material, MaterialOps};
use raytracer_lib_no_std::math::tuple4d::{Tuple, Tuple4D};
use raytracer_lib_no_std::shape::shape::Shape;

use crate::cpu::intersection::Intersection;
use crate::cpu::intersection::IntersectionOps;
use crate::cpu::intersection_list::{IntersectionList, IntersectionListOps};

pub struct CpuKernel {}

impl CpuKernel {
    pub fn color_at(shapes: &Vec<Shape>, lights: &Vec<Light>, r: &Ray, remaining: i32) -> Color {
        let mut color = BLACK;

        let xs = Intersection::intersect_world(shapes, r);
        let (intersection, is_hit) = xs.hit();
        if is_hit {
            let comp = Intersection::prepare_computations(
                intersection,
                &r,
                &IntersectionList::new(),
                shapes,
            );
            color = CpuKernel::shade_hit(shapes, lights, &comp, remaining);
        }
        color
    }

    fn shade_hit(
        shapes: &Vec<Shape>,
        lights: &Vec<Light>,
        comp: &PrecomputedComponent,
        remaining: i32,
    ) -> Color {
        // TODO if there is more than 1 light??? pass that to Material::lightning?
        let light = &lights[0];

        let shape = &shapes[comp.get_object()];
        let material =shape.get_material();

        //  let in_shadow = CpuKernel::is_shadowed(w, w.get_light().get_position(), comp.get_over_point());
        let intensity = CpuKernel::intensity_at(shapes, lights, comp.get_over_point());

        let surface = Material::lightning(
            material,
            shape,
            light,
            comp.get_over_point(),
            comp.get_eye_vector(),
            comp.get_normal_vector(),
            intensity,
        );
        let reflected = CpuKernel::reflected_color(shapes, lights, comp, remaining);
        let refracted = CpuKernel::refracted_color(shapes, lights, comp, remaining);

        // let material = comp.get_object().get_material();
        if material.get_reflective() > 0.0 && material.get_transparency() > 0.0 {
            let reflectance = Intersection::schlick(comp);
            return &surface + &(&reflected * reflectance + &refracted * (1.0 - reflectance));
        }
        &surface + &(&reflected + &refracted)
    }

    fn is_shadowed(shapes: &Vec<Shape>, light_position: &Tuple4D, position: &Tuple4D) -> bool {
        let v = light_position - position;

        let distance = Tuple4D::magnitude(&v);
        let mut direction = Tuple4D::normalize(&v);
        direction.w = 0.0;

        let point = Tuple4D::new_point_from(&position);
        let r = Ray::new(point, direction);

        let intersections = Intersection::intersect_world(shapes, &r);

        let (intersection, is_hit) = intersections.hit();

        if is_hit {
            if intersection.get_t() < distance {
                return true;
            }
        }
        false
    }

    fn intensity_at(shapes: &Vec<Shape>, lights: &Vec<Light>, point: &Tuple4D) -> f32 {
        let light = &lights[0];
        let res = match light {
            Light::PointLight(ref _pl) => CpuKernel::intensity_at_point_light(light, point, shapes),
            //  LightEnum::AreaLight(ref pl) => CpuKernel::intensity_at_area_light(light, point, world),
        };
        res
    }

    fn intensity_at_point_light(light: &Light, point: &Tuple4D, shapes: &Vec<Shape>) -> f32 {
        if CpuKernel::is_shadowed(shapes, light.get_position(), point) {
            return 0.0;
        }
        1.0
    }

    fn reflected_color(
        shapes: &Vec<Shape>,
        lights: &Vec<Light>,
        comp: &PrecomputedComponent,
        remaining: i32,
    ) -> Color {
        if remaining <= 0 {
            return BLACK;
        }
        let material = shapes[comp.get_object()].get_material();
        if material.get_reflective() == 0.0 {
            return BLACK;
        }
        let reflect_ray = Ray::new(
            Tuple4D::new_point_from(comp.get_over_point()),
            Tuple4D::new_vector_from(comp.get_reflected_vector()),
        );
        let color = CpuKernel::color_at(shapes, lights, &reflect_ray, remaining - 1);
        &color * material.get_reflective()
    }

    fn refracted_color(
        shapes: &Vec<Shape>,
        lights: &Vec<Light>,
        comp: &PrecomputedComponent,
        remaining: i32,
    ) -> Color {
        if remaining <= 0 {
            return BLACK;
        }
        let material = shapes[comp.get_object()].get_material();
        if material.get_transparency() == 0.0 {
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
        let mut direction =
            comp.get_normal_vector() * (n_ratio * cos_i - cos_t) - comp.get_eye_vector() * n_ratio;
        // fix direction to be a vector and not something in between
        direction.w = 0.0;
        let refracted_ray = Ray::new(Tuple4D::new_point_from(comp.get_under_point()), direction);

        CpuKernel::color_at(shapes, lights, &refracted_ray, remaining - 1)
            * material.get_transparency()
    }
}

impl CpuKernel {
    pub fn new() -> CpuKernel {
        CpuKernel {}
    }
}
