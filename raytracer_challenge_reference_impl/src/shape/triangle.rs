use crate::prelude::*;
use std::fmt;

#[derive(Clone, PartialEq)]
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

impl<'a> ShapeOps<'a> for Triangle {
    fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix =
            Matrix::invert(&m).expect("plane::set_transformation: cant unwrap inverse matrix");
        self.transformation_matrix = m;
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }

    fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }

    fn normal_at(&self, world_point: &Tuple4D, _shapes: &ShapeArr, i: &Intersection<'a>) -> Tuple4D {
        // TODO: its for the tests -remove and fix tests and add unreachable
        let object_point = self.get_inverse_transformation() * world_point;
        let local_normal = self.local_normal_at(&object_point, i);
        let mut world_normal = &Matrix::transpose(self.get_inverse_transformation()) * &local_normal;
        world_normal.w = 0.0;
        Tuple4D::normalize(&world_normal)
    }

    fn local_normal_at(&self, _local_point: &Tuple4D, _i: &Intersection<'a>) -> Tuple4D {
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

impl<'a> ShapeIntersectOps<'a> for Triangle {
    fn intersect_local(shape: &'a Shape, r: Ray, _shapes: &'a ShapeArr) -> IntersectionList<'a> {
        let triangle = match shape.get_shape() {
            ShapeEnum::TriangleEnum(triangle) => Some(triangle),
            _ => None,
        };
        if triangle.is_none() {
            return IntersectionList::new();
        }
        let triangle = triangle.unwrap();

        let mut intersection_list = IntersectionList::new();

        let dir_cross_e2 = r.get_direction() * triangle.get_e2();
        let det = triangle.get_e1() ^ &dir_cross_e2;
        if det.abs() < EPSILON {
            return intersection_list;
        }

        let f = 1.0 / det;
        let p1_to_origin = r.get_origin() - triangle.get_p1();
        let u = f * (&p1_to_origin ^ &dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return intersection_list;
        }

        let origin_cross_e1 = &p1_to_origin * triangle.get_e1();
        let v = f * (r.get_direction() ^ &origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return intersection_list;
        }
        let t = f * (triangle.get_e2() ^ &origin_cross_e1);
        intersection_list.add(Intersection::new(t, shape));

        intersection_list
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

    pub(crate) fn get_p1(&self) -> &Tuple4D {
        &self.p1
    }

    pub(crate) fn get_p2(&self) -> &Tuple4D {
        &self.p2
    }

    pub(crate) fn get_p3(&self) -> &Tuple4D {
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
    pub(crate) fn get_bounds_of(&self) -> BoundingBox {
        println!("get_bounds_of triangle");
        let mut bb = BoundingBox::new();
        bb.add_point(self.get_p1());
        bb.add_point(self.get_p2());
        bb.add_point(self.get_p3());

        bb
    }
}

impl<'a> fmt::Debug for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "triangle   p1  = {:?}   //  p2 = {:?}  // p3 = {:?} ",
            &self.p1, &self.p2, &self.p3
        )
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
        let shape = Shape::new(ShapeEnum::SphereEnum(Sphere::new()));
        let intersection = Intersection::new(1.0, &shape);
        let (t, _, _, _) = setup_triangle();

        let point1 = Tuple4D::new_point(0.0, 0.5, 0.0);
        let point2 = Tuple4D::new_point(-0.5, 0.75, 0.0);
        let point3 = Tuple4D::new_point(0.5, 0.25, 0.0);

        let shapes = vec![];
        let n1 = Triangle::normal_at(&t, &point1, &shapes, &intersection);
        let n2 = Triangle::normal_at(&t, &point2, &shapes, &intersection);
        let n3 = Triangle::normal_at(&t, &point3, &shapes, &intersection);

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

        let shape = Shape::new(ShapeEnum::TriangleEnum(t));
        let shapes = vec![];
        let is = Shape::intersect_local(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), 0);
    }

    // page 210 bottom
    #[test]
    fn test_triangle_ray_intersection_miss_p1_p3_edge() {
        let (t, _, _, _) = setup_triangle();

        let origin = Tuple4D::new_point(1.0, 1.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let shape = Shape::new(ShapeEnum::TriangleEnum(t));
        let shapes = vec![];
        let is = Shape::intersect_local(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), 0);
    }

    // page 211 top
    #[test]
    fn test_triangle_ray_intersection_miss_p1_p2_edge() {
        let (t, _, _, _) = setup_triangle();

        let origin = Tuple4D::new_point(-1.0, 1.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let shape = Shape::new(ShapeEnum::TriangleEnum(t));
        let shapes = vec![];
        let is = Shape::intersect_local(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), 0);
    }

    // page 211 top part 2
    #[test]
    fn test_triangle_ray_intersection_miss_p2_p3_edge() {
        let (t, _, _, _) = setup_triangle();

        let origin = Tuple4D::new_point(0.0, -1.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let shape = Shape::new(ShapeEnum::TriangleEnum(t));
        let shapes = vec![];
        let is = Shape::intersect_local(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), 0);
    }

    // page 212 center
    #[test]
    fn test_triangle_ray_intersection_hit() {
        let (t, _, _, _) = setup_triangle();

        let origin = Tuple4D::new_point(0.0, 0.5, -2.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);

        let shape = Shape::new(ShapeEnum::TriangleEnum(t));
        let shapes = vec![];
        let is = Shape::intersect_local(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), 1);

        assert_float(is.get_intersections().get(0).unwrap().get_t(), 2.0);
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
