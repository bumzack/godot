use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    material: Material,
    p1: Tuple4D,
    p2: Tuple4D,
    p3: Tuple4D,
    e1: Tuple4D,
    e2: Tuple4D,
    normal: Tuple4D,
}

impl ShapeOps for Triangle {
    fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix =
            Matrix::invert(&m).expect("plane::set_transofrmation: cant unwrap inverse matrix");
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
        self.normal.clone()
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

impl Triangle {
    pub fn new(p1: Tuple4D, p2: Tuple4D, p3: Tuple4D) -> Triangle {
        let e1 = &p2 - &p1;
        let e2 = &p3 - &p1;
        let normal = Tuple4D::normalize(&(&e2 * &e1));
        Triangle {
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            material: Material::new(),
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
        }
    }

    pub(crate) fn intersect(t: &Triangle, r: &Ray) -> Option<Vec<f32>> {
        let mut res = Vec::new();

        let dir_cross_e2 = r.get_direction() * t.get_e2();
        let det = t.get_e1() ^ &dir_cross_e2;
        if det.abs() < EPSILON {
            return Some(res);
        }

        let f = 1.0 / det;
        let p1_to_origin = r.get_origin() - t.get_p1();
        let u = f * (&p1_to_origin ^ &dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return Some(res);
        }

        let origin_cross_e1 = &p1_to_origin * t.get_e1();
        let v = f * (r.get_direction() ^ &origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return Some(res);
        }
        let t = f * (t.get_e2() ^ &origin_cross_e1);
        res.push(t);
        Some(res)
    }

    fn get_p1(&self) -> &Tuple4D {
        &self.p1
    }

    fn get_p2(&self) -> &Tuple4D {
        &self.p2
    }

    fn get_p3(&self) -> &Tuple4D {
        &self.p3
    }

    fn get_e1(&self) -> &Tuple4D {
        &self.e1
    }

    fn get_e2(&self) -> &Tuple4D {
        &self.e2
    }

    fn get_normal(&self) -> &Tuple4D {
        &self.normal
    }
}

#[cfg(test)]
mod tests {
    use crate::basics::ray::RayOps;
    use crate::math::common::{assert_float, assert_tuple};

    use super::*;

    // page 208
    #[test]
    fn test_triangle_new() {
        let (t, p1_clone, p2_clone, p3_clone) = setup_triangle();

        let e1_expected = Tuple4D::new_vector(-1.0, -1.0, 0.0);
        let e2_expected = Tuple4D::new_vector(1.0, -1.0, 0.0);

        assert_tuple(t.get_p1(), &p1_clone);
        assert_tuple(t.get_p2(), &p2_clone);
        assert_tuple(t.get_p3(), &p3_clone);

        assert_tuple(t.get_e1(), &e1_expected);
        assert_tuple(t.get_e2(), &e2_expected);
    }

    // page 209
    #[test]
    fn test_triangle_normal_at() {
        let (t, _, _, _) = setup_triangle();

        let point1 = Tuple4D::new_point(0.0, 0.5, 0.0);
        let point2 = Tuple4D::new_point(-0.5, 0.75, 0.0);
        let point3 = Tuple4D::new_point(0.5, 0.25, 0.0);

        let n1 = Triangle::normal_at(&t, &point1);
        let n2 = Triangle::normal_at(&t, &point2);
        let n3 = Triangle::normal_at(&t, &point3);

        assert_tuple(&n1, t.get_normal());
        assert_tuple(&n2, t.get_normal());
        assert_tuple(&n3, t.get_normal());
    }

    // page 210 top
    #[test]
    fn test_triangle_ray_intersection_miss_parallel() {
        let (t, _, _, _) = setup_triangle();

        let origin = Tuple4D::new_point(0.0, -1.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(origin, direction);

        let xs = Triangle::intersect(&t, &r);

        assert_eq!(xs.is_some(), true);
        assert_eq!(xs.unwrap().len(), 0);
    }

    // page 210 bottom
    #[test]
    fn test_triangle_ray_intersection_miss_p1_p3_edge() {
        let (t, _, _, _) = setup_triangle();

        let origin = Tuple4D::new_point(1.0, 1.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let xs = Triangle::intersect(&t, &r);

        assert_eq!(xs.is_some(), true);
        assert_eq!(xs.unwrap().len(), 0);
    }

    // page 211 top
    #[test]
    fn test_triangle_ray_intersection_miss_p1_p2_edge() {
        let (t, _, _, _) = setup_triangle();

        let origin = Tuple4D::new_point(-1.0, 1.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let xs = Triangle::intersect(&t, &r);

        assert_eq!(xs.is_some(), true);
        assert_eq!(xs.unwrap().len(), 0);
    }

    // page 211 top part 2
    #[test]
    fn test_triangle_ray_intersection_miss_p2_p3_edge() {
        let (t, _, _, _) = setup_triangle();

        let origin = Tuple4D::new_point(0.0, -1.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let xs = Triangle::intersect(&t, &r);

        assert_eq!(xs.is_some(), true);
        assert_eq!(xs.unwrap().len(), 0);
    }

    // page 212 center
    #[test]
    fn test_triangle_ray_intersection_hit() {
        let (t, _, _, _) = setup_triangle();

        let origin = Tuple4D::new_point(0.0, 0.5, -2.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let xs = Triangle::intersect(&t, &r);

        assert_eq!(xs.is_some(), true);
        let xs = xs.unwrap();
        assert_eq!(xs.len(), 1);
        assert_float(xs[0], 2.0);
    }

    fn setup_triangle() -> (Triangle, Tuple4D, Tuple4D, Tuple4D) {
        let p1 = Tuple4D::new_point(0.0, 1.0, 0.0);
        let p2 = Tuple4D::new_point(-1.0, 0.0, 0.0);
        let p3 = Tuple4D::new_point(1.0, 0.0, 0.0);

        let p1_clone = p1.clone();
        let p2_clone = p2.clone();
        let p3_clone = p3.clone();

        let t = Triangle::new(p1, p2, p3);

        (t, p1_clone, p2_clone, p3_clone)
    }
}
