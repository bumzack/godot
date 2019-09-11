use raytracer_lib_no_std::{assert_valid_color, ColorOps, DEBUG, EPSILON_OVER_UNDER, Light, ShapeOps};
use raytracer_lib_no_std::basics::color::{BLACK, Color};
use raytracer_lib_no_std::basics::precomputed_component::PrecomputedComponent;
use raytracer_lib_no_std::basics::ray::{Ray, RayOps};
use raytracer_lib_no_std::light::light::LightOps;
use raytracer_lib_no_std::material::material::{Material, MaterialOps};
use raytracer_lib_no_std::math::tuple4d::{Tuple, Tuple4D};
use raytracer_lib_no_std::shape::shape::Shape;

use crate::cpu::intersection::Intersection;
use crate::cpu::intersection::IntersectionOps;
use crate::cpu::intersection_list::{IntersectionList, IntersectionListOps};

pub struct CpuKernel {}

impl CpuKernel {
    pub fn color_at(
        shapes: &Vec<Shape>,
        lights: &Vec<Light>,
        r: &Ray,
        remaining: i32,
        calc_reflection: bool,
        calc_refraction: bool,
        calc_shadows: bool,
        is_debug_render: bool,
    ) -> Color {
        let mut color = BLACK;

        let xs = Intersection::intersect_world(shapes, r);

        let (intersection, is_hit) = xs.hit();

        if is_debug_render {
            println!("'color_at'   intersection_list   = {:?}", xs);
            println!("'color_at'   intersection        = {:?}", intersection);
            println!("'color_at'   is_hit              = {:?}", is_hit);
        }

        if is_hit {
            let comp = Intersection::prepare_computations(intersection, &r, &IntersectionList::new(), shapes);
            if is_debug_render {
                println!("'color_at'   comp   t                        = {:?}", comp.get_t());
                println!(
                    "'color_at'   comp  get_eye_vector            = {:?}",
                    comp.get_eye_vector()
                );
                println!(
                    "'color_at'   comp  get_normal_vector         = {:?}",
                    comp.get_normal_vector()
                );
                println!(
                    "'color_at'   comp  get_reflected_vector      = {:?}",
                    comp.get_reflected_vector()
                );
                println!("'color_at'   comp  get_point                  = {:?}", comp.get_point());
                println!(
                    "'color_at'   comp  get_over_point             = {:?}",
                    comp.get_over_point()
                );
                println!(
                    "'color_at'   comp  get_under_point            = {:?}",
                    comp.get_under_point()
                );
            }
            color = CpuKernel::shade_hit(
                shapes,
                lights,
                &comp,
                remaining,
                calc_reflection,
                calc_refraction,
                calc_shadows,
                is_debug_render,
            );
        }
        color
    }

    fn shade_hit(
        shapes: &Vec<Shape>,
        lights: &Vec<Light>,
        comp: &PrecomputedComponent,
        remaining: i32,
        calc_reflection: bool,
        calc_refraction: bool,
        calc_shadows: bool,
        is_debug_render: bool,
    ) -> Color {
        // TODO if there is more than 1 light??? pass that to Material::lightning?
        let light = &lights[0];

        let shape = &shapes[comp.get_object()];
        let material = shape.get_material();

        //  let in_shadow = CpuKernel::is_shadowed(w, w.get_light().get_position(), comp.get_over_point());
        let intensity = CpuKernel::intensity_at(shapes, lights, comp.get_over_point());

        let surface = CpuKernel::lightning(
            material,
            shape,
            light,
            comp.get_over_point(),
            comp.get_eye_vector(),
            comp.get_normal_vector(),
            intensity,
            calc_shadows,
            is_debug_render,
        );

        assert_valid_color(&surface);

        let mut reflected = BLACK;
        if calc_reflection {
            reflected = CpuKernel::reflected_color(
                shapes,
                lights,
                comp,
                remaining,
                calc_reflection,
                calc_refraction,
                calc_shadows,
                is_debug_render,
            );
        }
        let mut refracted = BLACK;
        if calc_refraction {
            refracted = CpuKernel::refracted_color(
                shapes,
                lights,
                comp,
                remaining,
                calc_reflection,
                calc_refraction,
                calc_shadows,
                is_debug_render,
            );
        }

        if is_debug_render {
            println!("'shade_hit'   intensity   = {:?}", intensity);
            println!("'shade_hit'   surface        = {:?}", surface);
            println!("'shade_hit'   reflected        = {:?}", reflected);
            println!("'shade_hit'   refracted        = {:?}", refracted);
        }

        assert_valid_color(&reflected);
        assert_valid_color(&refracted);

        // let material = comp.get_object().get_material();
        if calc_reflection && material.get_reflective() > 0.0 && material.get_transparency() > 0.0 {
            let reflectance = Intersection::schlick(comp);
            if is_debug_render {
                println!(
                    "'shade_hit'   with schlick    return    = {:?}",
                    &surface + &(&reflected * reflectance + &refracted * (1.0 - reflectance))
                );
            }

            return &surface + &(&reflected * reflectance + &refracted * (1.0 - reflectance));
        }
        if is_debug_render {
            println!(
                "'shade_hit'   no schlick    return    = {:?}",
                &surface + &(&reflected + &refracted)
            );
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
            let s_idx = intersection.get_shape();
            let shape = &shapes[s_idx];
            if intersection.get_t() - distance < EPSILON_OVER_UNDER && shape.get_casts_shadow() {
                return true;
            }
        }
        false
    }

    fn intensity_at(shapes: &Vec<Shape>, lights: &Vec<Light>, point: &Tuple4D) -> f32 {
        let light = &lights[0];
        let res = match light {
            Light::PointLight(ref _pl) => CpuKernel::intensity_at_point_light(light, point, shapes),
            Light::AreaLight(ref _al) => CpuKernel::intensity_at_area_light(light, point, shapes),
        };
        res
    }

    fn intensity_at_area_light(light: &Light, point: &Tuple4D, shapes: &Vec<Shape>) -> f32 {
        let mut total = 0.0;

        if DEBUG {
            println!("light.get_usteps()  = {:?}", light.get_usteps());
            println!("light.get_vsteps()  = {:?}", light.get_vsteps());
        }
        for v in 0..light.get_vsteps() {
            for u in 0..light.get_usteps() {
                let light_position = light.point_on_light(u, v);
                if !CpuKernel::is_shadowed(shapes, &light_position, point) {
                    total += 1.0;
                }
            }
        }

        total / light.get_samples() as f32
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
        calc_reflection: bool,
        calc_refraction: bool,
        calc_shadows: bool,
        is_debug_render: bool,
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
        let color = CpuKernel::color_at(
            shapes,
            lights,
            &reflect_ray,
            remaining - 1,
            calc_reflection,
            calc_refraction,
            calc_shadows,
            is_debug_render,
        );
        &color * material.get_reflective()
    }

    fn refracted_color(
        shapes: &Vec<Shape>,
        lights: &Vec<Light>,
        comp: &PrecomputedComponent,
        remaining: i32,
        calc_reflection: bool,
        calc_refraction: bool,
        calc_shadows: bool,
        is_debug_render: bool,
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
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

        // total internal reflection -> return black
        if sin2_t > 1.0 {
            return BLACK;
        }
        let cos_t = (1.0 - sin2_t).sqrt();
        let mut direction = comp.get_normal_vector() * (n_ratio * cos_i - cos_t) - comp.get_eye_vector() * n_ratio;
        // fix direction to be a vector and not something in between
        direction.w = 0.0;
        let refracted_ray = Ray::new(Tuple4D::new_point_from(comp.get_under_point()), direction);

        CpuKernel::color_at(
            shapes,
            lights,
            &refracted_ray,
            remaining - 1,
            calc_reflection,
            calc_refraction,
            calc_shadows,
            is_debug_render,
        ) * material.get_transparency()
    }

    fn lightning(
        material: &Material,
        shape: &Shape,
        light: &Light,
        point: &Tuple4D,
        eye: &Tuple4D,
        n: &Tuple4D,
        intensity: f32,
        calc_shadows: bool,
        is_debug_render: bool,
    ) -> Color {
        let c: Color;
        // TODO: a lot of color copying here ...
        if material.get_pattern().is_some() {
            c = material.get_pattern().as_ref().unwrap().color_at_object(shape, point);
        } else {
            c = Color::from_color(&material.get_color());
        }

        // ambient
        let effective_color = &c * light.get_intensity();
        let ambient = &effective_color * material.get_ambient();

        if is_debug_render {
            println!(
                "'lightning'           light.get_intensity(     = {:?} ",
                light.get_intensity()
            );
            println!("'lightning'           effective_color     = {:?} ", effective_color);
            println!("'lightning'           ambient             = {:?} ", ambient);
        }

        if !calc_shadows {
            return ambient;
        }

        let mut sum = BLACK;

        // create the sample points for the different lights
        let mut samples = Vec::new();

        for v in 0..light.get_vsteps() {
            for u in 0..light.get_usteps() {
                samples.push(light.point_on_light(u, v));
            }
        }

        for sample in samples.iter() {
            let mut specular;
            let diffuse;

            let light_v = Tuple4D::normalize(&(sample - point));
            let light_dot_normal = &light_v ^ &n;

            if is_debug_render {
                println!("");
                println!("'lightning'           point                    = {:?} ", point);
                println!("'lightning'           sample  (light_position)    = {:?} ", sample);

                println!("'lightning'           light_v              = {:?} ", light_v);
                println!("'lightning'           light_dot_normal     = {:?} ", light_dot_normal);
            }

            if light_dot_normal < 0.0 || intensity == 0.0 {
                specular = BLACK;
                diffuse = BLACK;
            } else {
                diffuse = &effective_color * material.get_diffuse() * light_dot_normal;
                assert_valid_color(&diffuse);
                // diffuse.fix_nan();
                let reflect_v = Tuple4D::reflect(&(light_v * (-1.0)), &n);
                let reflect_dot_eye = &reflect_v ^ eye;

                specular = BLACK;
                if reflect_dot_eye > 0.0 {
                    let factor = reflect_dot_eye.powf(material.get_shininess());
                    specular = light.get_intensity() * material.get_specular() * factor;

                    assert_valid_color(&specular);
                    //specular.fix_nan();
                }
            }
            assert_valid_color(&diffuse);
            assert_valid_color(&specular);
            if is_debug_render {
                println!("'lightning'           diffuse     = {:?} ", diffuse);
                println!("'lightning'           specular     = {:?} ", specular);
            }

            sum = &sum + &diffuse;
            sum = &sum + &specular;
        }

        assert_valid_color(&ambient);
        assert_valid_color(&sum);
        // sum.replace_inf_with_max();

        if intensity == 1.0 {
            ambient + sum / light.get_samples() as f32 * intensity
        } else {
            ambient
        }
    }
}

impl CpuKernel {
    pub fn new() -> CpuKernel {
        CpuKernel {}
    }
}
