use crate::{Material, MaterialOps, Ray, RayOps, ShapeIntersectionResult, ShapeOps};
use math::prelude::*;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Sphere {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    material: Material,
}

impl ShapeOps for Sphere {
    fn intersect(&self, r: &Ray) -> ShapeIntersectionResult {
        let mut res = [0f32; 4];
        let mut res_cnt = 0;

        let o = Tuple4D::new_point(0.0, 0.0, 0.0);
        let sphere_to_ray = r.get_origin() - &o;
        let a = r.get_direction() ^ r.get_direction();
        let b = 2.0 * (r.get_direction() ^ &sphere_to_ray);
        let c = (&sphere_to_ray ^ &sphere_to_ray) - 1.0;
        let discri = b * b - 4.0 * a * c;

        if discri < 0.0 {
            return (res, res_cnt);
        }

        let sqrt_disc = intri_sqrt(discri);
        res[0] = (-b - sqrt_disc) / (2.0 * a);
        res[1] = (-b + sqrt_disc) / (2.0 * a);
        res_cnt = 2;

        (res, res_cnt)
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix =
            Matrix::invert(&m).expect("Sphere::set_transofrmation:  cant unwrap inverted matrix ");
        self.transformation_matrix = m;
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }
    fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }

    fn normal_at(&self, world_point: &Tuple4D) -> Tuple4D {
        // TODO: its for the tests -remove and fix tests and add unreachable

        let object_point = self.get_inverse_transformation() * world_point;
        let local_normal = self.local_normal_at(&object_point);
        let mut world_normal = &Matrix::transpose(self.get_inverse_transformation()) * &local_normal;
        world_normal.w = 0.0;
        Tuple4D::normalize(&world_normal)
    }

    fn local_normal_at(&self, local_point: &Tuple4D) -> Tuple4D {
        let o = Tuple4D::new_point(0.0, 0.0, 0.0);
        local_point - &o
    }

    fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_material_mut(&mut self) -> &mut Material {
        &mut self.material
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            material: Material::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

    use super::*;

    use crate::basics::ray::RayOps;
    use cpu_kernel_raytracer::{Intersection, IntersectionList, IntersectionListOps, IntersectionOps};

    use crate::{assert_color, assert_float, assert_matrix, assert_tuple, Shape, ShapeEnum};

    fn glass_sphere() -> Sphere {
        let mut s = Sphere::new();
        s.get_material_mut().set_transparency(1.0);
        s.get_material_mut().set_refractive_index(1.5);
        s
    }

    #[test]
    fn test_ray_sphere_intersection() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let (intersections, cnt_hits) = s.intersect(&r);

        assert_eq!(cnt_hits, 2);

        assert_float(intersections[0], 4.0);
        assert_float(intersections[1], 6.0);

        let o = Tuple4D::new_point(0.0, 1.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let (intersections, cnt_hits) = s.intersect(&r);

        assert_eq!(cnt_hits, 2);

        assert_float(intersections[0], 5.0);
        assert_float(intersections[1], 5.0);
    }

    // page 60
    #[test]
    fn test_ray_sphere_intersection_no_hits() {
        let o = Tuple4D::new_point(0.0, 2.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();
        let (intersections, cnt_hits) = s.intersect(&r);

        assert_eq!(cnt_hits, 0);
    }

    // page 61
    #[test]
    fn test_ray_sphere_intersection_origin_inside_sphere() {
        let o = Tuple4D::new_point(0.0, 0.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();
        let (intersections, cnt_hits) = s.intersect(&r);

        assert_eq!(cnt_hits, 2);

        assert_float(intersections[0], -1.0);
        assert_float(intersections[1], 1.0);
    }

    // page 62
    #[test]
    fn test_ray_sphere_intersection_sphere_behind_origin() {
        let o = Tuple4D::new_point(0.0, 0.0, 5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();
        let (intersections, cnt_hits) = s.intersect(&r);

        assert_eq!(cnt_hits, 2);

        assert_float(intersections[0], -6.0);
        assert_float(intersections[1], -4.0);
    }
    //
    // page 69
    #[test]
    fn test_sphere_new_check_transformation_matrix() {
        let s = Sphere::new();

        let m_expected = Matrix::new_identity_4x4();
        let met_m_inv_expected = Matrix::invert(&m_expected).unwrap();

        assert_matrix(&s.get_transformation(), &m_expected);
        assert_matrix(&s.get_inverse_transformation(), &met_m_inv_expected);
    }

    // page 69
    #[test]
    fn test_sphere_transformation() {
        let mut s = Sphere::new();
        let m = Matrix::translation(2.0, 3.0, 4.0);
        s.set_transformation(m);
        let m = Matrix::translation(2.0, 3.0, 4.0);
        let m_inv = Matrix::invert(&m).unwrap();
        assert_matrix(&s.get_transformation(), &m);
        assert_matrix(&s.get_inverse_transformation(), &m_inv);
    }

    // page 69 bottom
    #[test]
    fn test_sphere_scale() {
        use raytracer_lib_no_std::basics::ray::RayOps;
        use raytracer_lib_no_std::math::matrix::MatrixOps;
        use raytracer_lib_no_std::math::tuple4d::Tuple;
        use raytracer_lib_no_std::shape::shape::ShapeOps;

        let o = ::raytracer_lib_no_std::math::tuple4d::Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = ::raytracer_lib_no_std::math::tuple4d::Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = ::raytracer_lib_no_std::basics::ray::Ray::new(o, d);

        let mut s = raytracer_lib_no_std::shape::sphere::Sphere::new();
        let m = raytracer_lib_no_std::math::matrix::Matrix::scale(2.0, 2.0, 2.0);
        s.set_transformation(m);

        let sphere_shape =
            raytracer_lib_no_std::shape::shape::Shape::new(raytracer_lib_no_std::shape::shape::ShapeEnum::Sphere(s));
        let shapes = vec![sphere_shape];

        let is = Intersection::intersect(0, &r, &shapes);

        let intersections = is.get_intersections();

        assert_eq!(is.len(), 2);

        // println!("intersections[0].get_t() {}", intersections[0].get_t());
        // println!("intersections[1].get_t() {}", intersections[1].get_t());
        assert_float(intersections[0].get_t(), 3.0);
        assert_float(intersections[1].get_t(), 7.0);
    }

    // page 70
    #[test]
    fn test_sphere_translated() {
        use raytracer_lib_no_std::basics::ray::RayOps;
        use raytracer_lib_no_std::math::matrix::MatrixOps;
        use raytracer_lib_no_std::math::tuple4d::Tuple;
        use raytracer_lib_no_std::shape::shape::ShapeOps;

        let o = raytracer_lib_no_std::math::tuple4d::Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = raytracer_lib_no_std::math::tuple4d::Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = ::raytracer_lib_no_std::basics::ray::Ray::new(o, d);

        let mut s = raytracer_lib_no_std::shape::sphere::Sphere::new();
        let m = raytracer_lib_no_std::math::matrix::Matrix::translation(5.0, 0.0, 0.0);
        s.set_transformation(m);

        let sphere_shape =
            raytracer_lib_no_std::shape::shape::Shape::new(raytracer_lib_no_std::shape::shape::ShapeEnum::Sphere(s));
        let shapes = vec![sphere_shape];

        let is = Intersection::intersect(0, &r, &shapes);

        let intersections = is.get_intersections();
        assert_eq!(is.len(), 0);
    }

    // page 78
    #[test]
    fn test_sphere_normal_at() {
        let s = Sphere::new();

        let p = Tuple4D::new_point(1.0, 0.0, 0.0);
        let n = s.normal_at(&p);
        let n_expected = Tuple4D::new_vector(1.0, 0.0, 0.0);
        assert_tuple(&n, &n_expected);

        let p = Tuple4D::new_point(0.0, 1.0, 0.0);
        let n = s.normal_at(&p);
        let n_expected = Tuple4D::new_vector(0.0, 1.0, 0.0);
        assert_tuple(&n, &n_expected);

        let p = Tuple4D::new_point(0.0, 0.0, 1.0);
        let n = s.normal_at(&p);
        let n_expected = Tuple4D::new_vector(0.0, 0.0, 1.0);
        assert_tuple(&n, &n_expected);

        let a = 3_f32.sqrt() / 3.0;
        let p = Tuple4D::new_point(a, a, a);
        let n = s.normal_at(&p);
        let n_expected = Tuple4D::new_vector(a, a, a);
        assert_tuple(&n, &n_expected);

        let a = 3_f32.sqrt() / 3.0;
        let p = Tuple4D::new_point(a, a, a);
        let n = Tuple4D::normalize(&s.normal_at(&p));
        let n_expected = Tuple4D::new_vector(a, a, a);
        assert_tuple(&n, &n_expected);
    }

    // page 80
    #[test]
    fn test_sphere_normal_at_transformed() {
        let mut s = Sphere::new();
        s.set_transformation(Matrix::translation(0.0, 1.0, 0.0));

        let p = Tuple4D::new_point(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let n = s.normal_at(&p);
        let n_expected = Tuple4D::new_vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);

        println!("n             = {:?}   ", n);
        println!("n_expected    = {:?}", n_expected);

        assert_tuple(&n, &n_expected);
    }

    // page 80
    #[test]
    fn test_sphere_normal_at_scaled_rotated() {
        let mut s = Sphere::new();
        s.set_transformation(Matrix::scale(1.0, 0.5, 1.0) * Matrix::rotate_z(PI / 5.0));

        let p = Tuple4D::new_point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let n = s.normal_at(&p);

        let n_expected = Tuple4D::new_vector(0.0, 0.97014254, -0.24253564);

        println!("n             = {:?}   ", n);
        println!("n_expected    = {:?}", n_expected);

        assert_tuple(&n, &n_expected);
    }

    fn glass_sphere_module_raytracer_kernel() -> raytracer_lib_no_std::shape::sphere::Sphere {
        use raytracer_lib_no_std::material::material::MaterialOps;
        use raytracer_lib_no_std::shape::shape::ShapeOps;
        let mut s = raytracer_lib_no_std::shape::sphere::Sphere::new();
        s.get_material_mut().set_transparency(1.0);
        s.get_material_mut().set_refractive_index(1.5);
        s
    }

    // page 152
    fn test_helper_n1_n2_calculations(index: usize, n1_expected: f32, n2_expected: f32) {
        use raytracer_lib_no_std::basics::ray::RayOps;
        use raytracer_lib_no_std::material::material::MaterialOps;
        use raytracer_lib_no_std::math::matrix::MatrixOps;
        use raytracer_lib_no_std::math::tuple4d::Tuple;
        use raytracer_lib_no_std::shape::shape::ShapeOps;

        let mut a = glass_sphere_module_raytracer_kernel();
        let m_a = raytracer_lib_no_std::math::matrix::Matrix::scale(2.0, 2.0, 2.0);
        a.set_transformation(m_a);
        a.get_material_mut().set_refractive_index(1.5);

        let mut b = glass_sphere_module_raytracer_kernel();
        let m_b = raytracer_lib_no_std::math::matrix::Matrix::translation(0.0, 0.0, -0.25);
        b.set_transformation(m_b);
        b.get_material_mut().set_refractive_index(2.0);

        let mut c = glass_sphere_module_raytracer_kernel();
        let m_c = raytracer_lib_no_std::math::matrix::Matrix::translation(0.0, 0.0, 0.25);
        c.set_transformation(m_c);
        c.get_material_mut().set_refractive_index(2.5);

        let p = raytracer_lib_no_std::math::tuple4d::Tuple4D::new_point(0.0, 0.0, -4.0);
        let o = raytracer_lib_no_std::math::tuple4d::Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = raytracer_lib_no_std::basics::ray::Ray::new(p, o);

        let a =
            raytracer_lib_no_std::shape::shape::Shape::new(raytracer_lib_no_std::shape::shape::ShapeEnum::Sphere(a));
        let b =
            raytracer_lib_no_std::shape::shape::Shape::new(raytracer_lib_no_std::shape::shape::ShapeEnum::Sphere(b));
        let c =
            raytracer_lib_no_std::shape::shape::Shape::new(raytracer_lib_no_std::shape::shape::ShapeEnum::Sphere(c));

        let shapes = vec![a, b, c];

        let mut xs = IntersectionList::new();

        xs.push(Intersection::new(2.0, 0));
        xs.push(Intersection::new(2.75, 1));
        xs.push(Intersection::new(3.25, 2));
        xs.push(Intersection::new(4.75, 1));
        xs.push(Intersection::new(5.25, 2));
        xs.push(Intersection::new(6.0, 0));

        let comps = Intersection::prepare_computations(&xs.get_intersections()[index], &r, &xs, &shapes);

        println!("n1 = {}   n1_expected = {}", comps.get_n1(), n1_expected);
        println!("n2 = {}   n2_expected = {}", comps.get_n2(), n2_expected);
        println!("");

        assert_float(comps.get_n1(), n1_expected);
        assert_float(comps.get_n2(), n2_expected);
    }

    // page 152
    #[test]
    fn test_n1_n2_calculations() {
        test_helper_n1_n2_calculations(0, 1.0, 1.5);
        test_helper_n1_n2_calculations(1, 1.5, 2.0);
        test_helper_n1_n2_calculations(2, 2.0, 2.5);

        test_helper_n1_n2_calculations(3, 2.5, 2.5);

        test_helper_n1_n2_calculations(4, 2.5, 1.5);
        test_helper_n1_n2_calculations(5, 1.5, 1.0);
    }

    // page 85
    #[test]
    fn test_new_sphere_material() {
        let s = Sphere::new();
        let m = Material::new();

        assert_eq!(s.get_material(), &m);
        assert_color(s.get_material().get_color(), m.get_color());
        assert_float(s.get_material().get_transparency(), m.get_transparency());
        assert_float(s.get_material().get_refractive_index(), m.get_refractive_index());
        assert_float(s.get_material().get_reflective(), m.get_reflective());
        assert_float(s.get_material().get_ambient(), m.get_ambient());
        assert_float(s.get_material().get_diffuse(), m.get_diffuse());
        assert_float(s.get_material().get_specular(), m.get_specular());
        assert_float(s.get_material().get_shininess(), m.get_shininess());
    }
}
