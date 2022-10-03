use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Plane {}

impl<'a> ShapeIntersectOps<'a> for Plane {
    fn intersect_local(shape: &'a Shape, r: Ray, _shapes: &'a ShapeArr) -> IntersectionList<'a> {
        let mut intersection_list = IntersectionList::new();

        if r.direction.y.abs() < EPSILON {
            return intersection_list;
        }
        let t = -r.origin.y / r.direction.y;
        let mut res = vec![0.0; 1];

        res[0] = t;
        let i1 = Intersection::new(res[0], shape);

        intersection_list.add(i1);
        intersection_list
    }

    fn local_normal_at(&self, _local_point: &Tuple4D, _i: &Intersection<'a>) -> Tuple4D {
        Tuple4D::new_vector(0.0, 1.0, 0.0)
    }
}

impl Plane {
    pub fn new() -> Plane {
        Plane {}
    }

    pub fn get_bounds_of(&self) -> BoundingBox {
        println!("get_bounds_of plane");
        let min = Tuple4D::new_point(-f64::INFINITY, 0.0, -f64::INFINITY);
        let max = Tuple4D::new_point(f64::INFINITY, 0.0, f64::INFINITY);
        BoundingBox::new_from_min_max(min, max)
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

    use crate::basics::intersection::{IntersectionListOps, IntersectionOps};
    use crate::basics::ray::RayOps;
    use crate::math::common::assert_float;
    use crate::math::common::assert_matrix;
    use crate::shape::shape::{Shape, ShapeEnum};
    use crate::shape::sphere::Sphere;

    use super::*;

    // page 123 top
    // Intersect with a ray parallel to the plane
    #[test]
    fn test_ray_plane_intersection_parallel() {
        let p = Shape::new_plane(Plane::new(), "plane".to_string());
        let o = Tuple4D::new_point(0.0, 10.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let shapes = vec![];
        let intersections = Shape::intersect_local(&p, r, &shapes);

        assert_eq!(intersections.get_intersections().len(), 0);
    }

    // page 123
    // A ray intersecting a plane from above
    #[test]
    fn test_ray_plane_intersection_above_and_below() {
        // above
        let p = Shape::new_plane(Plane::new(), "plane".to_string());
        let o = Tuple4D::new_point(0.0, 1.0, 0.0);
        let d = Tuple4D::new_vector(0.0, -1.0, 0.0);
        let r = Ray::new(o, d);

        let shapes = vec![];
        let intersections = Shape::intersect_local(&p, r, &shapes);

        assert_eq!(intersections.get_intersections().len() > 0, true);
        assert_float(intersections.get_intersections().get(0).unwrap().get_t(), 1.0);

        // below
        let o = Tuple4D::new_point(0.0, -1.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(o, d);

        let shapes = vec![];
        let intersections = Shape::intersect_local(&p, r, &shapes);

        assert_eq!(intersections.get_intersections().len() > 0, true);
        assert_float(intersections.get_intersections().get(0).unwrap().get_t(), 1.0);
    }

    #[test]
    fn test_ray_sphere_intersection_no_hits() {
        let p = Shape::new_sphere(Sphere::new(), "sphere".to_string());
        let o = Tuple4D::new_point(0.0, 2.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let shapes = vec![];
        let intersections = Shape::intersect_local(&p, r, &shapes);

        assert_eq!(intersections.get_intersections().len(), 0);
    }

    #[test]
    fn test_ray_sphere_intersection_origin_inside_sphere() {
        let p = Shape::new_sphere(Sphere::new(), "sphere".to_string());
        let o = Tuple4D::new_point(0.0, 0.0, 0.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let shapes = vec![];
        let intersections = Shape::intersect_local(&p, r, &shapes);

        assert_eq!(intersections.get_intersections().len(), 2);

        assert_float(intersections.get_intersections().get(0).unwrap().get_t(), -1.0);
        assert_float(intersections.get_intersections().get(1).unwrap().get_t(), 1.0);
    }

    #[test]
    fn test_ray_sphere_intersection_sphere_behind_origin() {
        let p = Shape::new_sphere(Sphere::new(), "sphere".to_string());
        let o = Tuple4D::new_point(0.0, 0.0, 5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let shapes = vec![];
        let intersections = Shape::intersect_local(&p, r, &shapes);

        assert_eq!(intersections.get_intersections().len(), 2);

        assert_float(intersections.get_intersections().get(0).unwrap().get_t(), -6.0);
        assert_float(intersections.get_intersections().get(1).unwrap().get_t(), -4.0);
    }

    #[test]
    fn test_sphere_transformation() {
        let mut s = Shape::new_sphere(Sphere::new(), "sphere".to_string());
        let m = Matrix::translation(2.0, 3.0, 4.0);

        s.set_transformation(m);

        let m = Matrix::translation(2.0, 3.0, 4.0);

        assert_matrix(&s.get_transformation(), &m);
    }

    #[test]
    fn test_sphere_scale() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let mut sphere = Shape::new_sphere(Sphere::new(), "sphere".to_string());
        let m = Matrix::scale(2.0, 2.0, 2.0);
        sphere.set_transformation(m);

        let shapes = vec![];
        let is = Shape::intersects(&sphere, r, &shapes);

        let intersections = is.get_intersections();

        assert_eq!(intersections.len(), 2);

        println!("is[0] {}", intersections.get(0).unwrap().get_t());
        println!("is[1] {}", intersections.get(1).unwrap().get_t());

        assert_float(intersections.get(0).unwrap().get_t(), 3.0);
        assert_float(intersections.get(1).unwrap().get_t(), 7.0);
    }

    #[test]
    fn test_sphere_translated() {
        let o = Tuple4D::new_point(0.0, 0.0, -5.0);
        let d = Tuple4D::new_vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let mut sphere_shape = Shape::new_sphere(Sphere::new(), "sphere".to_string());
        let m = Matrix::translation(5.0, 0.0, 0.0);
        sphere_shape.set_transformation(m);

        let shapes = vec![];
        let is = Shape::intersects(&sphere_shape, r, &shapes);

        let intersections = is.get_intersections();
        assert_eq!(intersections.len(), 0);
    }

    // page 122
    #[test]
    fn test_sphere_normal_at() {
        let shape = Shape::new_sphere(Sphere::new(), "sphere".to_string());
        let intersection = Intersection::new(1.0, &shape);

        let shape = Shape::new_plane(Plane::new(), "plane".to_string());
        let n_expected = Tuple4D::new_vector(0.0, 1.0, 0.0);

        let point = Tuple4D::new_point(0.0, 0.0, 0.0);
        let n = shape.normal_at(&point, &intersection);
        assert_tuple(&n, &n_expected);

        let point = Tuple4D::new_point(10.0, 0.0, -10.0);
        let n = shape.normal_at(&point, &intersection);
        assert_tuple(&n, &n_expected);

        let point = Tuple4D::new_point(-5.0, 0.0, 150.0);
        let n = shape.normal_at(&point, &intersection);
        assert_tuple(&n, &n_expected);
    }

    #[test]
    fn test_sphere_normal_at_transformed() {
        let shape = Shape::new_sphere(Sphere::new(), "sphere".to_string());
        let intersection = Intersection::new(1.0, &shape);
        let mut shape = Shape::new_sphere(Sphere::new(), "sphere".to_string());
        shape.set_transformation(Matrix::translation(0.0, 1.0, 0.0));

        let p = Tuple4D::new_point(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let n = shape.normal_at(&p, &intersection);
        let n_expected = Tuple4D::new_vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        println!(
            "test_sphere_normal_at_transformed    n = {:#?}, n_expected = {:#?}",
            n, n_expected
        );
        assert_tuple(&n, &n_expected);
    }

    #[test]
    fn test_sphere_normal_at_scaled_rotated() {
        let mut shape = Shape::new_sphere(Sphere::new(), "sphere".to_string());
        shape.set_transformation(Matrix::scale(1.0, 0.5, 1.0) * Matrix::rotate_z(PI / 5.0));
        let intersection = Intersection::new(1.0, &shape);

        let p = Tuple4D::new_point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let n = shape.normal_at(&p, &intersection);
        let n_expected = Tuple4D::new_vector(0.0, 0.97014, -0.24254);
        println!(
            "test_sphere_normal_at_scaled_rotated    n = {:#?}, n_expected = {:#?}",
            n, n_expected
        );
        assert_tuple(&n, &n_expected);
    }
}
