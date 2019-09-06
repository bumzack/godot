use crate::{intri_sqrt, Material, MaterialOps, Matrix, MatrixOps, Ray, RayOps, ShapeEnum, ShapeOps, Tuple, Tuple4D};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub struct Cylinder {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    material: Material,
    minimum: f32,
    maximum: f32,
    closed: bool,
}

impl ShapeOps for Cylinder {
    fn intersect(r: &Ray) -> Option<Vec<f32>> {
        if r.direction.y.abs() < EPSILON {
            return None;
        }
        let t = -r.origin.y / r.direction.y;
        let mut res = vec![0.0; 1];

        res[0] = t;
        Some(res)
    }
    fn intersect(cylinder: &Cylinder, r: &Ray) -> ([f32; 2], usize) {
        let mut res = [0f32; 2];
        let mut res_cnt = 0;

        let a = r.get_direction().x.powi(2) + r.get_direction().z.powi(2);
        if !(a.abs() < EPSILON) {
            let b = 2.0 * r.get_origin().x * r.get_direction().x + 2.0 * r.get_origin().z * r.get_direction().z;
            let c = r.get_origin().x.powi(2) + r.get_origin().z.powi(2) - 1.0;

            let disc = b * b - 4.0 * a * c;
            if disc < 0.0 {
                return (res, res_cnt);
            }
            let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
            let mut t1 = (-b + disc.sqrt()) / (2.0 * a);

            if t0 > t1 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }
            let y0 = r.get_origin().y + t0 * r.get_direction().y;
            if cylinder.get_minimum() < y0 && y0 < cylinder.get_maximum() {
                res[res_cnt] = t0;
                res_cnt += 1;
            }

            let y1 = r.get_origin().y + t1 * r.get_direction().y;
            if cylinder.get_minimum() < y1 && y1 < cylinder.get_maximum() {
                res[res_cnt] = t0;
                res_cnt += 1;
            }
        }
        Self::intersect_caps(cylinder, r, &mut res, &mut res_cnt);
        (res, res_cnt)
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix =
            Matrix::invert(&m).expect("Cube::set_transofrmation: cant unwrap inverse matrix");
        self.transformation_matrix = m;
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }

    fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }

    fn normal_at(c: &Cylinder, world_point: &Tuple4D) -> Tuple4D {
        let dist = world_point.x.powi(2) + world_point.z.powi(2);
        if dist < 1.0 && world_point.y >= c.get_maximum() - EPSILON {
            return Tuple4D::new_vector(0.0, 1.0, 0.0);
        } else if dist < 1.0 && world_point.y <= c.get_maximum() + EPSILON {
            return Tuple4D::new_vector(0.0, -1.0, 0.0);
        }
        Tuple4D::new_vector(world_point.x, 0.0, world_point.z)
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

impl Cylinder {
    fn new() -> Cylinder {
        Cylinder {
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            material: Material::new(),
            minimum: -INFINITY,
            maximum: INFINITY,
            closed: false,
        }
    }

    fn get_minimum(&self) -> f32 {
        self.minimum
    }

    fn get_maximum(&self) -> f32 {
        self.maximum
    }

    fn set_minimum(&mut self, min: f32) {
        self.minimum = min;
    }

    fn set_maximum(&mut self, max: f32) {
        self.maximum = max;
    }

    fn get_closed(&self) -> bool {
        self.closed
    }

    fn set_closed(&mut self, closed: bool) {
        self.closed = closed;
    }

    fn check_cap(r: &Ray, t: f32) -> bool {
        let x = r.get_origin().x + t * r.get_direction().x;
        let z = r.get_origin().z + t * r.get_direction().z;
        (x.powi(2) + z.powi(2)) - 1.0 < EPSILON
    }

    fn intersect_caps(c: &Cylinder, r: &Ray, res: &mut [f32], res_cnt: usize) {
        if !c.get_closed() || r.get_direction().y.abs() < EPSILON {
            return;
        }
        let t = (c.get_minimum() - r.get_origin().y) / r.get_direction().y;
        if Self::check_cap(r, t) {
            res[res_cnt] = t;
            res_cnt += 1;
        }
        let t = (c.get_maximum() - r.get_origin().y) / r.get_direction().y;
        if Self::check_cap(r, t) {
            res[res_cnt] = t;
            res_cnt += 1;
        }
    }
}

//// helper
//// page 151
//pub fn glass_sphere() -> Sphere {
//    let mut s = Sphere::new();
//    s.get_material_mut().set_transparency(1.0);
//    s.get_material_mut().set_refractive_index(1.5);
//    s
//}
//
//#[cfg(test)]
//mod tests {
//    use core::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};
//
//    use crate::basics::intersection::{Intersection, IntersectionList, IntersectionListOps, IntersectionOps};
//    use crate::basics::ray::RayOps;
//    use crate::math::raytracer_lib_no_std::{assert_color, assert_float, assert_matrix, assert_tuple};
//    use crate::shape::shape::{Shape, ShapeEnum};
//
//    use super::*;
//
//    #[test]
//    fn test_ray_sphere_intersection() {
//        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
//        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
//        let r = Ray::new(o, d);
//
//        let intersections = Sphere::intersect(&r).unwrap();
//
//        assert_eq!(intersections.len(), 2);
//
//        assert_float(intersections[0], 4.0);
//        assert_float(intersections[1], 6.0);
//
//        let o = Tuple4D::new_point(0.0, 1.0, -5.0);
//        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
//        let r = Ray::new(o, d);
//
//        let intersections = Sphere::intersect(&r).unwrap();
//
//        assert_eq!(intersections.len(), 2);
//
//        assert_float(intersections[0], 5.0);
//        assert_float(intersections[1], 5.0);
//    }
//
//    // page 60
//    #[test]
//    fn test_ray_sphere_intersection_no_hits() {
//        let o = Tuple4D::new_point(0.0, 2.0, -5.0);
//        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
//        let r = Ray::new(o, d);
//
//        let intersections = Sphere::intersect(&r);
//
//        assert_eq!(intersections, None);
//    }
//
//    // page 61
//    #[test]
//    fn test_ray_sphere_intersection_origin_inside_sphere() {
//        let o = Tuple4D::new_point(0.0, 0.0, 0.0);
//        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
//        let r = Ray::new(o, d);
//
//        let intersections = Sphere::intersect(&r).unwrap();
//
//        assert_eq!(intersections.len(), 2);
//
//        assert_float(intersections[0], -1.0);
//        assert_float(intersections[1], 1.0);
//    }
//
//    // page 62
//    #[test]
//    fn test_ray_sphere_intersection_sphere_behind_origin() {
//        let o = Tuple4D::new_point(0.0, 0.0, 5.0);
//        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
//        let r = Ray::new(o, d);
//
//        let intersections = Sphere::intersect(&r).unwrap();
//
//        assert_eq!(intersections.len(), 2);
//
//        assert_float(intersections[0], -6.0);
//        assert_float(intersections[1], -4.0);
//    }
//
//    // page 69
//    #[test]
//    fn test_sphere_new_check_transformation_matrix() {
//        let s = Sphere::new();
//
//        let m_expected = Matrix::new_identity_4x4();
//        let met_m_inv_expected = Matrix::invert(&m_expected).unwrap();
//
//        assert_matrix(&s.get_transformation(), &m_expected);
//        assert_matrix(&s.get_inverse_transformation(), &met_m_inv_expected);
//    }
//
//    // page 69
//    #[test]
//    fn test_sphere_transformation() {
//        let mut s = Sphere::new();
//        let m = Matrix::translation(2.0, 3.0, 4.0);
//        s.set_transformation(m);
//        let m = Matrix::translation(2.0, 3.0, 4.0);
//        let m_inv = Matrix::invert(&m).unwrap();
//        assert_matrix(&s.get_transformation(), &m);
//        assert_matrix(&s.get_inverse_transformation(), &m_inv);
//    }
//
//    // page 69 bottom
//    #[test]
//    fn test_sphere_scale() {
//        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
//        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
//        let r = Ray::new(o, d);
//
//        let mut s = Sphere::new();
//        let m = Matrix::scale(2.0, 2.0, 2.0);
//        s.set_transformation(m);
//
//        let sphere_shape = Shape::new(ShapeEnum::Sphere(s), "Sphere");
//
//        let is = Intersection::intersect(&sphere_shape, &r);
//
//        let intersections = is.get_intersections();
//
//        assert_eq!(intersections.len(), 2);
//
//        // println!("intersections[0].get_t() {}", intersections[0].get_t());
//        // println!("intersections[1].get_t() {}", intersections[1].get_t());
//        assert_float(intersections[0].get_t(), 3.0);
//        assert_float(intersections[1].get_t(), 7.0);
//    }
//
//    // page 70
//    #[test]
//    fn test_sphere_translated() {
//        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
//        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
//        let r = Ray::new(o, d);
//
//        let mut s = Sphere::new();
//        let m = Matrix::translation(5.0, 0.0, 0.0);
//        s.set_transformation(m);
//
//        let sphere_shape = Shape::new(ShapeEnum::Sphere(s), "Sphere");
//        let is = Intersection::intersect(&sphere_shape, &r);
//
//        let intersections = is.get_intersections();
//        assert_eq!(intersections.len(), 0);
//    }
//
//    // page 78
//    #[test]
//    fn test_sphere_normal_at() {
//        let s = Sphere::new();
//
//        let p = Tuple4D::new_point(1.0, 0.0, 0.0);
//        let n = s.normal_at(&p);
//        let n_expected = Tuple4D::new_vector(1.0, 0.0, 0.0);
//        assert_tuple(&n, &n_expected);
//
//        let p = Tuple4D::new_point(0.0, 1.0, 0.0);
//        let n = s.normal_at(&p);
//        let n_expected = Tuple4D::new_vector(0.0, 1.0, 0.0);
//        assert_tuple(&n, &n_expected);
//
//        let p = Tuple4D::new_point(0.0, 0.0, 1.0);
//        let n = s.normal_at(&p);
//        let n_expected = Tuple4D::new_vector(0.0, 0.0, 1.0);
//        assert_tuple(&n, &n_expected);
//
//        let a = 3_f64.sqrt() / 3.0;
//        let p = Tuple4D::new_point(a, a, a);
//        let n = s.normal_at(&p);
//        let n_expected = Tuple4D::new_vector(a, a, a);
//        assert_tuple(&n, &n_expected);
//
//        let a = 3_f64.sqrt() / 3.0;
//        let p = Tuple4D::new_point(a, a, a);
//        let n = Tuple4D::normalize(&s.normal_at(&p));
//        let n_expected = Tuple4D::new_vector(a, a, a);
//        assert_tuple(&n, &n_expected);
//    }
//
//    // page 80
//    #[test]
//    fn test_sphere_normal_at_transformed() {
//        let mut s = Sphere::new();
//        s.set_transformation(Matrix::translation(0.0, 1.0, 0.0));
//
//        let p = Tuple4D::new_point(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
//        let n = s.normal_at(&p);
//        let n_expected = Tuple4D::new_vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
//        assert_tuple(&n, &n_expected);
//    }
//
//    // page 80
//    #[test]
//    fn test_sphere_normal_at_scaled_rotated() {
//        let mut s = Sphere::new();
//        s.set_transformation(Matrix::scale(1.0, 0.5, 1.0) * Matrix::rotate_z(PI / 5.0));
//
//        let p = Tuple4D::new_point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
//        let n = s.normal_at(&p);
//        let n_expected = Tuple4D::new_vector(0.0, 0.97014, -0.24254);
//
//        assert_tuple(&n, &n_expected);
//    }
//
//    // page 152
//    fn test_helper_n1_n2_calculations(index: usize, n1_expected: f64, n2_expected: f64) {
//        let mut a = glass_sphere();
//        let m_a = Matrix::scale(2.0, 2.0, 2.0);
//        a.set_transformation(m_a);
//        a.get_material_mut().set_refractive_index(1.5);
//
//        let mut b = glass_sphere();
//        let m_b = Matrix::translation(0.0, 0.0, -0.25);
//        b.set_transformation(m_b);
//        b.get_material_mut().set_refractive_index(2.0);
//
//        let mut c = glass_sphere();
//        let m_c = Matrix::translation(0.0, 0.0, 0.25);
//        c.set_transformation(m_c);
//        c.get_material_mut().set_refractive_index(2.5);
//
//        let p = Tuple4D::new_point(0.0, 0.0, -4.0);
//        let o = Tuple4D::new_vector(0.0, 0.0, 1.0);
//        let r = Ray::new(p, o);
//
//        let a = Shape::new(ShapeEnum::Sphere(a), "Sphere");
//        let b = Shape::new(ShapeEnum::Sphere(b), "Sphere");
//        let c = Shape::new(ShapeEnum::Sphere(c), "Sphere");
//
//        let mut xs = IntersectionList::new();
//
//        xs.add(Intersection::new(2.0, &a));
//        xs.add(Intersection::new(2.75, &b));
//        xs.add(Intersection::new(3.25, &c));
//        xs.add(Intersection::new(4.75, &b));
//        xs.add(Intersection::new(5.25, &c));
//        xs.add(Intersection::new(6.0, &a));
//
//        let comps = Intersection::prepare_computations(&xs.get_intersections()[index], &r, &xs);
//
//        println!("n1 = {}   n1_expected = {}", comps.get_n1(), n1_expected);
//        println!("n2 = {}   n2_expected = {}", comps.get_n2(), n2_expected);
//        println!("");
//
//        assert_float(comps.get_n1(), n1_expected);
//        assert_float(comps.get_n2(), n2_expected);
//    }
//
//    // page 152
//    #[test]
//    fn test_n1_n2_calculations() {
//        test_helper_n1_n2_calculations(0, 1.0, 1.5);
//        test_helper_n1_n2_calculations(1, 1.5, 2.0);
//        test_helper_n1_n2_calculations(2, 2.0, 2.5);
//
//        test_helper_n1_n2_calculations(3, 2.5, 2.5);
//
//        test_helper_n1_n2_calculations(4, 2.5, 1.5);
//        test_helper_n1_n2_calculations(5, 1.5, 1.0);
//    }
//
//    // page 85
//    #[test]
//    fn test_new_sphere_material() {
//        let s = Sphere::new();
//        let m = Material::new();
//
//        assert_eq!(s.get_material(), &m);
//        assert_color(s.get_material().get_color(), m.get_color());
//        assert_float(s.get_material().get_transparency(), m.get_transparency());
//        assert_float(s.get_material().get_refractive_index(), m.get_refractive_index());
//        assert_float(s.get_material().get_reflective(), m.get_reflective());
//        assert_float(s.get_material().get_ambient(), m.get_ambient());
//        assert_float(s.get_material().get_diffuse(), m.get_diffuse());
//        assert_float(s.get_material().get_specular(), m.get_specular());
//        assert_float(s.get_material().get_shininess(), m.get_shininess());
//    }
//}
