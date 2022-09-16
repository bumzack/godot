use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Cylinder {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    material: Material,
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl ShapeOps for Cylinder {
    fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix =
            Matrix::invert(&m).expect("Cube::set_transformation: cant unwrap inverse matrix");
        self.transformation_matrix = m;
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }

    fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }

    fn normal_at(&self, world_point: &Tuple4D, _shapes: &ShapeArr) -> Tuple4D {
        // TODO: its for the tests -remove and fix tests and add unreachable
        let object_point = self.get_inverse_transformation() * world_point;
        let local_normal = self.local_normal_at(&object_point);
        let mut world_normal = &Matrix::transpose(self.get_inverse_transformation()) * &local_normal;
        world_normal.w = 0.0;
        Tuple4D::normalize(&world_normal)
    }

    fn local_normal_at(&self, local_point: &Tuple4D) -> Tuple4D {
        let dist = local_point.x.powi(2) + local_point.z.powi(2);
        if dist < 1.0 && local_point.y >= self.get_maximum() - EPSILON {
            return Tuple4D::new_vector(0.0, 1.0, 0.0);
        } else if dist < 1.0 && local_point.y <= self.get_minimum() + EPSILON {
            return Tuple4D::new_vector(0.0, -1.0, 0.0);
        }
        Tuple4D::new_vector(local_point.x, 0.0, local_point.z)

        //        let dist = intri_powi(local_point.x, 2) + intri_powi(local_point.z, 2);
        //        if dist < 1.0 && local_point.y >= self.get_maximum() - EPSILON {
        //            return Tuple4D::new_vector(0.0, 1.0, 0.0);
        //        } else if dist < 1.0 && local_point.y <= self.get_minimum() + EPSILON {
        //            return Tuple4D::new_vector(0.0, -1.0, 0.0);
        //        }
        //        Tuple4D::new_vector(local_point.x, 0.0, local_point.z)
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

impl<'a> ShapeIntersectOps<'a> for Cylinder {
    fn intersect_local(shape: &'a Shape, r: Ray, _shapes: &'a ShapeArr) -> IntersectionList<'a> {
        let cylinder = match shape.get_shape() {
            ShapeEnum::Cylinder(cylinder) => Some(cylinder),
            _ => None,
        };
        if cylinder.is_none() {
            return IntersectionList::new();
        }
        let cylinder = cylinder.unwrap();
        let mut ts = Vec::new();

        let a = r.get_direction().x.powi(2) + r.get_direction().z.powi(2);
        if !(a.abs() < EPSILON_OVER_UNDER) {
            let b = 2.0 * r.get_origin().x * r.get_direction().x + 2.0 * r.get_origin().z * r.get_direction().z;
            let c = r.get_origin().x.powi(2) + r.get_origin().z.powi(2) - 1.0;

            let disc = b * b - 4.0 * a * c;
            if disc < 0.0 {
                return IntersectionList::new();
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
                ts.push(t0);
            }

            let y1 = r.get_origin().y + t1 * r.get_direction().y;
            if cylinder.get_minimum() < y1 && y1 < cylinder.get_maximum() {
                ts.push(t1);
            }
        }
        Self::intersect_caps(&cylinder, &r, &mut ts);
        let mut intersection_list = IntersectionList::new();
        ts.into_iter()
            .for_each(|t| intersection_list.add(Intersection::new(t, shape)));
        intersection_list
    }
}

impl Cylinder {
    pub fn new() -> Cylinder {
        Cylinder {
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
            material: Material::new(),
            minimum: -f64::INFINITY,
            maximum: f64::INFINITY,
            closed: false,
        }
    }

    pub fn get_minimum(&self) -> f64 {
        self.minimum
    }

    pub fn get_maximum(&self) -> f64 {
        self.maximum
    }

    pub fn set_minimum(&mut self, min: f64) {
        self.minimum = min;
    }

    pub fn set_maximum(&mut self, max: f64) {
        self.maximum = max;
    }

    pub fn get_closed(&self) -> bool {
        self.closed
    }

    pub fn set_closed(&mut self, closed: bool) {
        self.closed = closed;
    }

    fn check_cap(r: &Ray, t: f64) -> bool {
        let x = r.get_origin().x + t * r.get_direction().x;
        let z = r.get_origin().z + t * r.get_direction().z;
        (x.powi(2) + z.powi(2)) - 1.0 < EPSILON_OVER_UNDER
    }

    fn intersect_caps(c: &Cylinder, r: &Ray, xs: &mut Vec<f64>) {
        if !c.get_closed() || r.get_direction().y.abs() < EPSILON {
            return;
        }
        let t = (c.get_minimum() - r.get_origin().y) / r.get_direction().y;
        if Self::check_cap(r, t) {
            xs.push(t);
        }
        let t = (c.get_maximum() - r.get_origin().y) / r.get_direction().y;
        if Self::check_cap(r, t) {
            xs.push(t);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::basics::ray::RayOps;
    use crate::math::common::{assert_float, assert_tuple};

    use super::*;

    // page 178
    fn test_ray_cylinder_intersection_miss_helper(origin: Tuple4D, mut direction: Tuple4D) {
        direction = Tuple4D::normalize(&direction);
        let r = Ray::new(origin, direction);

        let shape = Shape::new(ShapeEnum::Cylinder(Cylinder::new()));
        let shapes = vec![];
        let is = Shape::intersect_local(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), 0);
    }

    // page 178
    #[test]
    fn test_ray_cylinder_intersection_miss() {
        // 1
        let origin = Tuple4D::new_point(1.0, 0.0, 0.0);
        let direction = Tuple4D::new_vector(0.0, 1.0, 0.0);
        test_ray_cylinder_intersection_miss_helper(origin, direction);

        // 2
        let origin = Tuple4D::new_point(0.0, 0.0, 0.0);
        let direction = Tuple4D::new_vector(0.0, 1.0, 0.0);
        test_ray_cylinder_intersection_miss_helper(origin, direction);

        // 3
        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(1.0, 1.0, 1.0);
        test_ray_cylinder_intersection_miss_helper(origin, direction);
    }

    // page 180
    fn test_ray_cylinder_intersection_intersection_helper(origin: Tuple4D, mut direction: Tuple4D, t1: f64, t2: f64) {
        direction = Tuple4D::normalize(&direction);
        let r = Ray::new(origin.clone(), direction.clone());

        let shape = Shape::new(ShapeEnum::Cylinder(Cylinder::new()));
        let shapes = vec![];
        let is = Shape::intersect_local(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), 2);

        println!("origin        = {:?} ", origin);
        println!("direction n   = {:?} ", direction);
        println!(
            "expected  t1   = {:?}       actual t1 = {:?}",
            t1,
            is.get_intersections()
        );
        println!(
            "expected  t2   = {:?}       actual t1 = {:?}",
            t2,
            is.get_intersections()
        );

        assert_float(is.get_intersections().get(0).unwrap().get_t(), t1);
        assert_float(is.get_intersections().get(1).unwrap().get_t(), t2);
    }

    // page 180
    #[test]
    fn test_ray_cylinder_intersection_intersection() {
        // 1
        let origin = Tuple4D::new_point(1.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_intersection_intersection_helper(origin, direction, 5.0, 5.0);

        // 2
        let origin = Tuple4D::new_point(0.0, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_intersection_intersection_helper(origin, direction, 4.0, 6.0);

        // 3
        let origin = Tuple4D::new_point(0.5, 0.0, -5.0);
        let direction = Tuple4D::new_vector(0.1, 1.0, 1.0);
        test_ray_cylinder_intersection_intersection_helper(origin, direction, 6.80798191702732, 7.088723439378861);
    }

    // page 181
    fn test_ray_cylinder_normal_at_helper(point: Tuple4D, n_expected: Tuple4D) {
        let shapes = vec![];
        let cyl = Cylinder::new();

        let n = Cylinder::normal_at(&cyl, &point, &shapes);

        println!("point        = {:?} ", point);
        println!("expected  n  = {:?} ", n_expected);
        println!("actual    n  = {:?} ", n);

        assert_tuple(&n, &n_expected);
    }

    // page 181
    #[test]
    fn test_ray_cylinder_normal_at() {
        // 1
        let point = Tuple4D::new_point(1.0, 0.0, 0.0);
        let normal = Tuple4D::new_vector(1.0, 0.0, 0.0);
        test_ray_cylinder_normal_at_helper(point, normal);

        // 2
        let point = Tuple4D::new_point(0.0, 5.0, -1.0);
        let normal = Tuple4D::new_vector(0.0, 0.0, -1.0);
        test_ray_cylinder_normal_at_helper(point, normal);

        // 3
        let point = Tuple4D::new_point(0.0, -2.0, 1.0);
        let normal = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_normal_at_helper(point, normal);

        // 4
        let point = Tuple4D::new_point(-1.0, 1.0, 0.0);
        let normal = Tuple4D::new_vector(-1.0, 0.0, 0.0);
        test_ray_cylinder_normal_at_helper(point, normal);
    }

    // page 182
    #[test]
    fn test_ray_cylinder_new() {
        let c = Cylinder::new();

        println!(
            "c.getminimum() = {},    -INFINITY = {}",
            c.get_minimum(),
            -f64::INFINITY
        );
        println!("c.get_maximum() = {},    INFINITY = {}", c.get_maximum(), f64::INFINITY);
        assert_eq!(c.get_minimum(), -f64::INFINITY);
        assert_eq!(c.get_maximum(), f64::INFINITY);
    }

    // page 182
    fn test_ray_cylinder_truncate_helper(point: Tuple4D, mut direction: Tuple4D, count: usize) {
        let mut cyl = Cylinder::new();
        cyl.set_minimum(1.0);
        cyl.set_maximum(2.0);
        direction = Tuple4D::normalize(&direction);

        let r = Ray::new(point.clone(), direction.clone());

        let shape = Shape::new(ShapeEnum::Cylinder(cyl));
        let shapes = vec![];
        let is = Shape::intersect_local(&shape, r, &shapes);

        assert_eq!(is.get_intersections().len(), count);
    }

    // page 182
    #[test]
    fn test_ray_cylinder_truncate() {
        // 1
        let point = Tuple4D::new_point(0.0, 1.5, 0.0);
        let direction = Tuple4D::new_vector(0.1, 1.0, 0.0);
        test_ray_cylinder_truncate_helper(point, direction, 0);

        // 2
        let point = Tuple4D::new_point(0.0, 3., -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_truncate_helper(point, direction, 0);

        // 3
        let point = Tuple4D::new_point(0.0, 0., -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_truncate_helper(point, direction, 0);

        // 4
        let point = Tuple4D::new_point(0.0, 2., -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_truncate_helper(point, direction, 0);

        // 5
        let point = Tuple4D::new_point(0.0, 1., -5.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_truncate_helper(point, direction, 0);

        // 6
        let point = Tuple4D::new_point(0.0, 1.5, -2.0);
        let direction = Tuple4D::new_vector(0.0, 0.0, 1.0);
        test_ray_cylinder_truncate_helper(point, direction, 2);
    }

    // page 185
    #[test]
    fn test_ray_cylinder_closed_cylinder() {
        let c = Cylinder::new();

        assert!(c.get_closed() == false);
    }

    // page 185
    fn test_ray_cylinder_capped_helper(point: Tuple4D, mut direction: Tuple4D, count: usize) {
        let mut cyl = Cylinder::new();
        cyl.set_minimum(1.0);
        cyl.set_maximum(2.0);
        cyl.set_closed(true);
        direction = Tuple4D::normalize(&direction);

        let r = Ray::new(point.clone(), direction.clone());

        let shape = Shape::new(ShapeEnum::Cylinder(cyl));
        let shapes = vec![];
        let is = Shape::intersect_local(&shape, r, &shapes);

        println!("point        = {:?} ", point);
        println!("direction     = {:?} ", direction);
        println!("expected  count  = {:?} ", count);

        assert_eq!(is.get_intersections().len(), count);
    }

    // page 185
    #[test]
    fn test_ray_cylinder_capped() {
        // 1
        let point = Tuple4D::new_point(0.0, 3.0, 0.0);
        let direction = Tuple4D::new_vector(0.0, -1.0, 0.0);
        test_ray_cylinder_capped_helper(point, direction, 2);

        // 2
        let point = Tuple4D::new_point(0.0, 3.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, -1.0, 2.0);
        test_ray_cylinder_capped_helper(point, direction, 2);

        // 3
        let point = Tuple4D::new_point(0.0, 4.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, -1.0, 1.0);
        test_ray_cylinder_capped_helper(point, direction, 2);

        // 4
        let point = Tuple4D::new_point(0.0, 0.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, 1.0, 2.0);
        test_ray_cylinder_capped_helper(point, direction, 2);

        // 5
        let point = Tuple4D::new_point(0.0, -1.0, -2.0);
        let direction = Tuple4D::new_vector(0.0, 1.0, 1.0);
        test_ray_cylinder_capped_helper(point, direction, 2);
    }

    // page 186
    fn test_ray_cylinder_capped_normal_at_helper(point: Tuple4D, normal: Tuple4D) {
        let shapes = vec![];
        let mut cyl = Cylinder::new();
        cyl.set_minimum(1.0);
        cyl.set_maximum(2.0);
        cyl.set_closed(true);
        let n = Cylinder::normal_at(&cyl, &point, &shapes);

        println!("point        = {:?} ", point);
        println!("direction     = {:?} ", normal);

        assert_tuple(&n, &normal);
    }

    // page 185
    #[test]
    fn test_ray_cylinder_capped_normal_at() {
        // 1
        let point = Tuple4D::new_point(0.0, 1.0, 0.0);
        let normal = Tuple4D::new_vector(0.0, -1.0, 0.0);
        test_ray_cylinder_capped_normal_at_helper(point, normal);

        // 2
        let point = Tuple4D::new_point(0.5, 1.0, 0.0);
        let normal = Tuple4D::new_vector(0.0, -1.0, 0.0);
        test_ray_cylinder_capped_normal_at_helper(point, normal);

        // 3
        let point = Tuple4D::new_point(0.0, 1.0, 0.5);
        let normal = Tuple4D::new_vector(0.0, -1.0, 0.0);
        test_ray_cylinder_capped_normal_at_helper(point, normal);

        // 4
        let point = Tuple4D::new_point(0.0, 2.0, 0.0);
        let normal = Tuple4D::new_vector(0.0, 1.0, 0.0);
        test_ray_cylinder_capped_normal_at_helper(point, normal);

        // 5
        let point = Tuple4D::new_point(0.5, 2.0, 0.0);
        let normal = Tuple4D::new_vector(0.0, 1.0, 0.0);
        test_ray_cylinder_capped_normal_at_helper(point, normal);

        // 6
        let point = Tuple4D::new_point(0.0, 2.0, 0.5);
        let normal = Tuple4D::new_vector(0.0, 1.0, 0.0);
        test_ray_cylinder_capped_normal_at_helper(point, normal);
    }
}
