use crate::math::matrix::Matrix;
use crate::math::tuple4d::Tuple;
use crate::math::tuple4d::Tuple4D;

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Tuple4D,
    pub direction: Tuple4D,
}

pub trait RayOps {
    fn new(origin: Tuple4D, direction: Tuple4D) -> Self;
    fn position(r: &Ray, t: f64) -> Tuple4D;
    fn transform(r: &Ray, m: &Matrix) -> Ray;

    fn get_direction(&self) -> &Tuple4D;
    fn get_origin(&self) -> &Tuple4D;
}

impl RayOps for Ray {
    fn new(origin: Tuple4D, direction: Tuple4D) -> Self {
        assert!(Tuple4D::is_point(&origin));
        assert!(Tuple4D::is_vector(&direction));
        Ray { origin, direction }
    }

    fn position(r: &Ray, t: f64) -> Tuple4D {
        r.origin + (r.direction * t)
    }

    fn transform(r: &Ray, m: &Matrix) -> Ray {
        let mut o_transformed = m * &r.origin;
        let mut d_transformed = m * &r.direction;
        o_transformed.w = 1.0;
        d_transformed.w = 0.0;
        Ray::new(o_transformed, d_transformed)
    }

    fn get_direction(&self) -> &Tuple4D {
        &self.direction
    }

    fn get_origin(&self) -> &Tuple4D {
        &self.origin
    }
}

#[cfg(test)]
mod tests {
    use crate::math::common::{assert_float, assert_tuple};
    use crate::math::matrix::MatrixOps;

    use super::*;

    #[test]
    fn test_ray_new() {
        let o = Tuple4D::new_point(1.0, 2.0, 3.0);
        let d = Tuple4D::new_vector(4.0, 5.0, 6.0);

        let r = Ray::new(o, d);

        assert!(Tuple4D::is_point(&r.origin));
        assert!(Tuple4D::is_vector(&r.direction));

        assert_float(r.origin.x, 1.0);
        assert_float(r.origin.y, 2.0);
        assert_float(r.origin.z, 3.0);

        assert_float(r.direction.x, 4.0);
        assert_float(r.direction.y, 5.0);
        assert_float(r.direction.z, 6.0);
    }

    #[test]
    fn test_ray_position() {
        let o = Tuple4D::new_point(2.0, 3.0, 4.0);
        let d = Tuple4D::new_vector(1.0, 0.0, 0.0);

        let r = Ray::new(o, d);

        let p1 = Ray::position(&r, 0.0);
        let p2 = Ray::position(&r, 1.0);
        let p3 = Ray::position(&r, -1.0);
        let p4 = Ray::position(&r, 2.5);

        assert!(Tuple4D::is_point(&p1));
        assert!(Tuple4D::is_point(&p2));
        assert!(Tuple4D::is_point(&p3));
        assert!(Tuple4D::is_point(&p4));

        assert_float(p1.x, 2.0);
        assert_float(p1.y, 3.0);
        assert_float(p1.z, 4.0);

        assert_float(p2.x, 3.0);
        assert_float(p2.y, 3.0);
        assert_float(p2.z, 4.0);

        assert_float(p3.x, 1.0);
        assert_float(p3.y, 3.0);
        assert_float(p3.z, 4.0);

        assert_float(p4.x, 4.5);
        assert_float(p4.y, 3.0);
        assert_float(p4.z, 4.0);
    }

    // page 69
    #[test]
    fn test_ray_translation() {
        let o = Tuple4D::new_point(1.0, 2.0, 3.0);
        let d = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(o, d);

        let m = Matrix::translation(3.0, 4.0, 5.0);

        let r2 = Ray::transform(&r, &m);

        let o_expected = Tuple4D::new_point(4.0, 6.0, 8.0);
        let d_expected = Tuple4D::new_vector(0.0, 1.0, 0.0);

        assert_tuple(&r2.origin, &o_expected);
        assert_tuple(&r2.direction, &d_expected);
    }

    // page 69
    #[test]
    fn test_ray_scaling() {
        let o = Tuple4D::new_point(1.0, 2.0, 3.0);
        let d = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(o, d);

        let m = Matrix::scale(2.0, 3.0, 4.0);

        let r2 = Ray::transform(&r, &m);

        let o_expected = Tuple4D::new_point(2.0, 6.0, 12.0);
        let d_expected = Tuple4D::new_vector(0.0, 3.0, 0.0);

        assert_tuple(&r2.origin, &o_expected);
        assert_tuple(&r2.direction, &d_expected);
    }

    #[test]
    fn test_ray_rotation() {
        let o = Tuple4D::new_point(1.0, 2.0, 3.0);
        let d = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let r = Ray::new(o, d);

        let m = Matrix::scale(2.0, 3.0, 4.0);

        let r2 = Ray::transform(&r, &m);

        let o_expected = Tuple4D::new_point(2.0, 6.0, 12.0);
        let d_expected = Tuple4D::new_vector(0.0, 3.0, 0.0);

        assert_tuple(&r2.origin, &o_expected);
        assert_tuple(&r2.direction, &d_expected);
    }
}
