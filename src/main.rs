use crate::math::canvas::Canvas;
use crate::math::canvas::CanvasOps;
use crate::math::color::Color;
use crate::math::color::ColorOps;
use crate::math::common::float_equal;
use crate::math::commonshape::CommonShape;
use crate::math::intersection::{Intersection, IntersectionListOps};
use crate::math::intersection::IntersectionOps;
use crate::math::matrix::Matrix;
use crate::math::matrix::MatrixOps;
use crate::math::ray::Ray;
use crate::math::ray::RayOps;
use crate::math::scene::{Scene, SceneOps};
use crate::math::sphere::Sphere;
use crate::math::sphere::SphereOps;
use crate::math::tuple4d::{Tuple, Tuple4D};

mod math;

fn main() {
    let mut s=Scene::new(500,500);
    s.render_scene();
    s.write_ppm("blupp.ppm");
}
