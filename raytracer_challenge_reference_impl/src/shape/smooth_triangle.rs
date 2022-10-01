use std::fmt;

use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SmoothTriangle {
    p1: Tuple4D,
    p2: Tuple4D,
    p3: Tuple4D,
    n1: Tuple4D,
    n2: Tuple4D,
    n3: Tuple4D,
    e1: Tuple4D,
    e2: Tuple4D,
    normal: Tuple4D,
}

impl<'a> ShapeIntersectOps<'a> for SmoothTriangle {
    fn intersect_local(shape: &'a Shape, r: Ray, _shapes: &'a ShapeArr) -> IntersectionList<'a> {
        let triangle = match shape.get_shape() {
            ShapeEnum::SmoothTriangleEnum(triangle) => Some(triangle),
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
        let u = f * (p1_to_origin ^ dir_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            //  u < 0.0 || u > 1.0 {
            return intersection_list;
        }

        let origin_cross_e1 = &p1_to_origin * triangle.get_e1();
        let v = f * (r.get_direction() ^ &origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return intersection_list;
        }
        let t = f * (triangle.get_e2() ^ &origin_cross_e1);
        intersection_list.add(Intersection::new_u_v(t, shape, u, v));

        intersection_list
    }

    fn local_normal_at(&self, _local_point: &Tuple4D, i: &Intersection<'a>) -> Tuple4D {
        self.get_n2() * i.get_u() + self.get_n3() * i.get_v() + self.get_n1() * (1.0 - i.get_u() - i.get_v())
    }
}

impl SmoothTriangle {
    pub fn new(p1: Tuple4D, p2: Tuple4D, p3: Tuple4D, n1: Tuple4D, n2: Tuple4D, n3: Tuple4D) -> SmoothTriangle {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = Tuple4D::normalize(&(e2 * e1));

        SmoothTriangle {
            p1,
            p2,
            p3,
            n1,
            n2,
            n3,
            e1,
            e2,
            normal,
        }
    }

    pub fn get_p1(&self) -> &Tuple4D {
        &self.p1
    }

    pub fn get_p2(&self) -> &Tuple4D {
        &self.p2
    }

    pub fn get_p3(&self) -> &Tuple4D {
        &self.p3
    }

    fn get_e1(&self) -> &Tuple4D {
        &self.e1
    }

    fn get_e2(&self) -> &Tuple4D {
        &self.e2
    }

    pub fn get_n1(&self) -> &Tuple4D {
        &self.n1
    }

    pub fn get_n2(&self) -> &Tuple4D {
        &self.n2
    }

    pub fn get_n3(&self) -> &Tuple4D {
        &self.n3
    }

    pub fn get_normal(&self) -> &Tuple4D {
        &self.n1
    }

    pub fn get_bounds_of(&self) -> BoundingBox {
        println!("get_bounds_of smooth_triangle");
        let mut bb = BoundingBox::new();
        bb.add_point(self.get_p1());
        bb.add_point(self.get_p2());
        bb.add_point(self.get_p3());
        bb
    }
}

impl fmt::Debug for SmoothTriangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "triangle   p1  = {:?}   //  p2 = {:?}  // p3 = {:?}  // n1= {:?}  //   n2 = {:?}  // n3 = {:?}   ",
            &self.p1, &self.p2, &self.p3, &self.n1, &self.n2, &self.n3
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::basics::ray::RayOps;
    use crate::math::common::{assert_float, assert_tuple};

    use super::*;

    // page 221
    // Constructing a smooth triangle
    #[test]
    fn test_triangle_new() {
        let (t, _, _, _, _, _, _) = setup_smooth_triangle();

        let p1_expected = Tuple4D::new_point(0.0, 1.0, 0.0);
        let p2_expected = Tuple4D::new_point(-1.0, 0.0, 0.0);
        let p3_expected = Tuple4D::new_point(1.0, 0.0, 0.0);

        let n1_expected = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let n2_expected = Tuple4D::new_vector(-1.0, 0.0, 0.0);
        let n3_expected = Tuple4D::new_vector(1.0, 0.0, 0.0);

        assert_tuple(t.get_p1(), &p1_expected);
        assert_tuple(t.get_p2(), &p2_expected);
        assert_tuple(t.get_p3(), &p3_expected);

        assert_tuple(t.get_n1(), &n1_expected);
        assert_tuple(t.get_n2(), &n2_expected);
        assert_tuple(t.get_n3(), &n3_expected);
    }

    // page 221
    // An intersection can encapsulate u and v
    #[test]
    fn test_smooth_triangle_intersection_encapsulates_u_and_v() {
        let (t, _, _, _, _, _, _) = setup_smooth_triangle();
        let s = Shape::new(ShapeEnum::SmoothTriangleEnum(t));
        let is = Intersection::new_u_v(3.5, &s, 0.2, 0.4);

        assert_eq!(0.2, is.get_u());
        assert_eq!(0.4, is.get_v());
    }

    // page 222
    // An intersection with a smooth triangle stores u/v
    #[test]
    fn test_intersection_with_smooth_triangle_intersection_stores_u_and_v() {
        let shapes = vec![];
        let (t, _, _, _, _, _, _) = setup_smooth_triangle();
        let s = Shape::new(ShapeEnum::SmoothTriangleEnum(t));
        let r = Ray::new(Tuple4D::new_point(-0.2, 0.3, -2.0), Tuple4D::new_vector(0.0, 0.0, 1.0));

        let xs = SmoothTriangle::intersect_local(&s, r, &shapes);

        assert_float(xs.get_intersections()[0].get_u(), 0.45);
        assert_float(xs.get_intersections()[0].get_v(), 0.25);
    }

    // page 222
    // A smooth triangle uses u/v to interpolate the normal
    #[test]
    fn test_smooth_triangle_interpolates_normal() {
        let (t, _, _, _, _, _, _) = setup_smooth_triangle();
        let s = Shape::new(ShapeEnum::SmoothTriangleEnum(t));
        let i = Intersection::new_u_v(1.0, &s, 0.45, 0.25);

        let point = Tuple4D::new_point(0.0, 0.0, 0.0);
        let n = s.normal_at(&point, &i);
        let n_expected = Tuple4D::new_vector(-0.5547, 0.83205, 0.0);

        assert_tuple(&n, &n_expected);
    }

    // page 223
    // Preparing the normal on a smooth triangle
    #[test]
    fn test_smooth_triangle_prepare_normal() {
        let (t, _, _, _, _, _, _) = setup_smooth_triangle();
        let s = Shape::new(ShapeEnum::SmoothTriangleEnum(t));
        let i = Intersection::new_u_v(1.0, &s, 0.45, 0.25);
        let i2 = Intersection::new_u_v(1.0, &s, 0.45, 0.25);
        let r = Ray::new(Tuple4D::new_point(-0.2, 0.3, -2.0), Tuple4D::new_vector(0.0, 0.0, 1.0));

        let mut xs = IntersectionList::new();
        xs.add(i2);

        let comps = Intersection::prepare_computations(&i, &r, &xs);

        let n = comps.get_normal_vector();

        let n_expected = Tuple4D::new_vector(-0.5547, 0.83205, 0.0);

        assert_tuple(&n, &n_expected);
    }

    fn setup_smooth_triangle() -> (SmoothTriangle, Tuple4D, Tuple4D, Tuple4D, Tuple4D, Tuple4D, Tuple4D) {
        let p1 = Tuple4D::new_point(0.0, 1.0, 0.0);
        let p2 = Tuple4D::new_point(-1.0, 0.0, 0.0);
        let p3 = Tuple4D::new_point(1.0, 0.0, 0.0);

        let n1 = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let n2 = Tuple4D::new_vector(-1.0, 0.0, 0.0);
        let n3 = Tuple4D::new_vector(1.0, 0.0, 0.0);

        let p1_clone = p1.clone();
        let p2_clone = p2.clone();
        let p3_clone = p3.clone();

        let n1_clone = n1.clone();
        let n2_clone = n2.clone();
        let n3_clone = n3.clone();

        let t = SmoothTriangle::new(p1, p2, p3, n1, n2, n3);

        (t, p1_clone, p2_clone, p3_clone, n1_clone, n2_clone, n3_clone)
    }
}
