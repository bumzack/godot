use core::fmt;

use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Sphere {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    material: Material,
}

impl<'a> ShapeOps<'a> for Sphere {
    fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix =
            Matrix::invert(&m).expect("Sphere::set_transformation:  cant unwrap inverted matrix ");
        self.transformation_matrix = m;
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }
    fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }

    fn normal_at(&self, world_point: &Tuple4D, _shapes: &ShapeArr, i: &Intersection<'a>) -> Tuple4D {
        let object_point = self.get_inverse_transformation() * world_point;
        let local_normal = self.local_normal_at(&object_point, i);
        let mut world_normal = &Matrix::transpose(self.get_inverse_transformation()) * &local_normal;
        world_normal.w = 0.0;
        Tuple4D::normalize(&world_normal)
    }

    fn local_normal_at(&self, local_point: &Tuple4D, _i: &Intersection<'a>) -> Tuple4D {
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

    fn get_parent(&self) -> &Option<ShapeIdx> {
        unreachable!("this should never be called");
    }

    fn get_children(&self) -> &Vec<ShapeIdx> {
        unreachable!("this should never be called");
    }

    fn get_children_mut(&mut self) -> &mut Vec<ShapeIdx> {
        unreachable!("this should never be called");
    }
}

impl<'a> ShapeIntersectOps<'a> for Sphere {
    fn intersect_local(shape: &'a Shape, r: Ray, _shapes: &'a ShapeArr) -> IntersectionList<'a> {
        let mut intersection_list = IntersectionList::new();
        let o = Tuple4D::new_point(0.0, 0.0, 0.0);
        let sphere_to_ray = &r.origin - &o;
        let a = &r.direction ^ &r.direction;
        let b = 2.0 * (&r.direction ^ &sphere_to_ray);
        let c = (&sphere_to_ray ^ &sphere_to_ray) - 1.0;
        let disc = b * b - 4.0 * a * c;

        if disc < 0.0 {
            return intersection_list;
        }
        let mut res = vec![0.0; 2];
        res[0] = (-b - disc.sqrt()) / (2.0 * a);
        res[1] = (-b + disc.sqrt()) / (2.0 * a);

        let i1 = Intersection::new(res[0], shape);
        let i2 = Intersection::new(res[1], shape);
        intersection_list.add(i1);
        intersection_list.add(i2);

        intersection_list
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
    pub(crate) fn get_bounds_of(&self) -> BoundingBox {
        println!("get_bounds_of sphere");
        let min = Tuple4D::new_point(-1.0, -1.0, -1.0);
        let max = Tuple4D::new_point(1.0, 1.0, 1.0);
        BoundingBox::new_from_min_max(min, max)
    }
}

impl fmt::Debug for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i am a sphere  ")
    }
}

// helper
// page 151
pub fn glass_sphere() -> Sphere {
    let mut s = Sphere::new();
    s.get_material_mut().set_transparency(1.0);
    s.get_material_mut().set_refractive_index(1.5);
    s
}

#[cfg(test)]
mod tests {
    use crate::basics::{Intersection, IntersectionList, IntersectionListOps, IntersectionOps, Ray, RayOps};
    use crate::material::{Material, MaterialOps};
    use crate::math::{assert_color, assert_float, assert_matrix, assert_tuple, Matrix, MatrixOps, Tuple, Tuple4D};
    use crate::prelude::{glass_sphere, Shape, ShapeEnum, ShapeIntersectOps, ShapeOps, Sphere};
    use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

    #[test]
    fn test_ray_sphere_intersection() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        let shapes = vec![];
        let is = Shape::intersect_local(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), 2);

        assert_float(is.get_intersections().get(0).unwrap().get_t(), 4.0);
        assert_float(is.get_intersections().get(1).unwrap().get_t(), 6.0);

        let o = Tuple4D::new_point(0.0, 1.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let is = Shape::intersects(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), 2);

        assert_float(is.get_intersections().get(0).unwrap().get_t(), 5.0);
        assert_float(is.get_intersections().get(1).unwrap().get_t(), 5.0);
    }

    // page 60
    #[test]
    fn test_ray_sphere_intersection_no_hits() {
        let o = Tuple4D::new_point(0.0, 2.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        let shapes = vec![];
        let is = Shape::intersects(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), 0);
    }

    // page 61
    #[test]
    fn test_ray_sphere_intersection_origin_inside_sphere() {
        let o = Tuple4D::new_point(0.0, 0.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        let shapes = vec![];
        let is = Shape::intersects(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), 2);
        assert_float(is.get_intersections().get(0).unwrap().get_t(), -1.0);
        assert_float(is.get_intersections().get(1).unwrap().get_t(), 1.0);
    }

    // page 62
    #[test]
    fn test_ray_sphere_intersection_sphere_behind_origin() {
        let o = Tuple4D::new_point(0.0, 0.0, 5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        let shapes = vec![];
        let is = Shape::intersects(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), 2);
        assert_float(is.get_intersections().get(0).unwrap().get_t(), -6.0);
        assert_float(is.get_intersections().get(1).unwrap().get_t(), -4.0);
    }

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
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let mut s = Sphere::new();
        let m = Matrix::scale(2.0, 2.0, 2.0);
        s.set_transformation(m);

        let shape = Shape::new(ShapeEnum::SphereEnum(s));
        let shapes = vec![];
        let is = Shape::intersects(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), 2);
        assert_float(is.get_intersections().get(0).unwrap().get_t(), 3.0);
        assert_float(is.get_intersections().get(1).unwrap().get_t(), 7.0);
    }

    // page 70
    #[test]
    fn test_sphere_translated() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let mut s = Sphere::new();
        let m = Matrix::translation(5.0, 0.0, 0.0);
        s.set_transformation(m);

        let shape = Shape::new(ShapeEnum::SphereEnum(s));
        let shapes = vec![];
        let is = Shape::intersects(&shape, r, &shapes);

        let intersections = is.get_intersections();
        assert_eq!(intersections.len(), 0);
    }

    // page 78
    #[test]
    fn test_sphere_normal_at() {
        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        let intersection = Intersection::new(1.0, &shape);
        let shapes = vec![];
        let s = Sphere::new();

        let p = Tuple4D::new_point(1.0, 0.0, 0.0);
        let n = s.normal_at(&p, &shapes, &intersection);
        let n_expected = Tuple4D::new_vector(1.0, 0.0, 0.0);
        assert_tuple(&n, &n_expected);

        let p = Tuple4D::new_point(0.0, 1.0, 0.0);
        let n = s.normal_at(&p, &shapes, &intersection);
        let n_expected = Tuple4D::new_vector(0.0, 1.0, 0.0);
        assert_tuple(&n, &n_expected);

        let p = Tuple4D::new_point(0.0, 0.0, 1.0);
        let n = s.normal_at(&p, &shapes, &intersection);
        let n_expected = Tuple4D::new_vector(0.0, 0.0, 1.0);
        assert_tuple(&n, &n_expected);

        let a = 3_f64.sqrt() / 3.0;
        let p = Tuple4D::new_point(a, a, a);
        let n = s.normal_at(&p, &shapes, &intersection);
        let n_expected = Tuple4D::new_vector(a, a, a);
        assert_tuple(&n, &n_expected);

        let a = 3_f64.sqrt() / 3.0;
        let p = Tuple4D::new_point(a, a, a);
        let n = Tuple4D::normalize(&s.normal_at(&p, &shapes, &intersection));
        let n_expected = Tuple4D::new_vector(a, a, a);
        assert_tuple(&n, &n_expected);
    }

    // page 80
    #[test]
    fn test_sphere_normal_at_transformed() {
        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        let intersection = Intersection::new(1.0, &shape);
        let shapes = vec![];
        let mut s = Sphere::new();
        s.set_transformation(Matrix::translation(0.0, 1.0, 0.0));

        let p = Tuple4D::new_point(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let n = s.normal_at(&p, &shapes, &intersection);
        let n_expected = Tuple4D::new_vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);

        println!("n             = {:?}   ", n);
        println!("n_expected    = {:?}", n_expected);

        assert_tuple(&n, &n_expected);
    }

    // page 80
    #[test]
    fn test_sphere_normal_at_scaled_rotated() {
        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        let intersection = Intersection::new(1.0, &shape);
        let shapes = vec![];
        let mut s = Sphere::new();
        s.set_transformation(Matrix::scale(1.0, 0.5, 1.0) * Matrix::rotate_z(PI / 5.0));

        let p = Tuple4D::new_point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let n = s.normal_at(&p, &shapes, &intersection);

        let n_expected = Tuple4D::new_vector(0.0, 0.97014254, -0.24253564);

        println!("n             = {:?}   ", n);
        println!("n_expected    = {:?}", n_expected);

        assert_tuple(&n, &n_expected);
    }

    // page 152
    fn test_helper_n1_n2_calculations(index: usize, n1_expected: f64, n2_expected: f64) {
        let mut a = glass_sphere();
        let m_a = Matrix::scale(2.0, 2.0, 2.0);
        a.set_transformation(m_a);
        a.get_material_mut().set_refractive_index(1.5);

        let mut b = glass_sphere();
        let m_b = Matrix::translation(0.0, 0.0, -0.25);
        b.set_transformation(m_b);
        b.get_material_mut().set_refractive_index(2.0);

        let mut c = glass_sphere();
        let m_c = Matrix::translation(0.0, 0.0, 0.25);
        c.set_transformation(m_c);
        c.get_material_mut().set_refractive_index(2.5);

        let p = Tuple4D::new_point(0.0, 0.0, -4.0);
        let o = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(p, o);

        let a = Shape::new(ShapeEnum::SphereEnum(a));
        let b = Shape::new(ShapeEnum::SphereEnum(b));
        let c = Shape::new(ShapeEnum::SphereEnum(c));

        let mut xs = IntersectionList::new();

        xs.add(Intersection::new(2.0, &a));
        xs.add(Intersection::new(2.75, &b));
        xs.add(Intersection::new(3.25, &c));
        xs.add(Intersection::new(4.75, &b));
        xs.add(Intersection::new(5.25, &c));
        xs.add(Intersection::new(6.0, &a));

        let shapes = vec![];
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
