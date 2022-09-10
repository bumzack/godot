use crate::basics::color::Color;
use crate::math::matrix::Matrix;
use crate::math::tuple4d::Tuple4D;
use crate::patterns::checker3d_patterns::Checker3DPattern;
use crate::patterns::gradient_patterns::GradientPattern;
use crate::patterns::ring_patterns::RingPattern;
use crate::patterns::sphere_texture_patterns::SphereTexturePattern;
use crate::patterns::stripe_patterns::StripePattern;
use crate::patterns::test_patterns::TestPattern;
use crate::patterns::PlaneTexturePattern;
use crate::patterns::{AlignCheckTexturePattern, CubeTexturePattern, CylinderTexturePattern};
use crate::shape::shape::Shape;

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    StripePattern(StripePattern),
    GradientPattern(GradientPattern),
    RingPattern(RingPattern),
    Checker3DPattern(Checker3DPattern),
    TestPattern(TestPattern),
    SphereTexturePattern(SphereTexturePattern),
    PlaneTexturePattern(PlaneTexturePattern),
    CylinderTexturePattern(CylinderTexturePattern),
    AlignCheckTexturePattern(AlignCheckTexturePattern),
    CubeTextPattern(CubeTexturePattern),
}

impl Pattern {
    pub fn color_at_object(&self, shape: &Shape, world_point: &Tuple4D) -> Color {
        match self {
            Pattern::StripePattern(ref stripe_pattern) => {
                StripePattern::color_at_object(stripe_pattern, shape, world_point)
            }
            Pattern::GradientPattern(ref gradient_pattern) => {
                GradientPattern::color_at_object(gradient_pattern, shape, world_point)
            }
            Pattern::RingPattern(ref ring_pattern) => RingPattern::color_at_object(ring_pattern, shape, world_point),
            Pattern::Checker3DPattern(ref checker3d_pattern) => {
                Checker3DPattern::color_at_object(checker3d_pattern, shape, world_point)
            }
            Pattern::TestPattern(ref test_pattern) => TestPattern::color_at_object(test_pattern, shape, world_point),
            Pattern::SphereTexturePattern(ref sphere_texture_pattern) => {
                SphereTexturePattern::color_at_object(sphere_texture_pattern, shape, world_point)
            }
            Pattern::PlaneTexturePattern(ref plane_texture_pattern) => {
                PlaneTexturePattern::color_at_object(plane_texture_pattern, shape, world_point)
            }
            Pattern::CylinderTexturePattern(ref cylinder_pattern) => {
                CylinderTexturePattern::color_at_object(cylinder_pattern, shape, world_point)
            }
            Pattern::AlignCheckTexturePattern(ref cube_pattern) => {
                AlignCheckTexturePattern::color_at_object(cube_pattern, shape, world_point)
            }
            Pattern::CubeTextPattern(ref cube_pattern) => {
                CubeTexturePattern::color_at_object(cube_pattern, shape, world_point)
            }
        }
    }

    pub fn set_transformation(&mut self, m: Matrix) {
        match self {
            Pattern::StripePattern(ref mut stripe_pattern) => stripe_pattern.set_transformation(m),
            Pattern::GradientPattern(ref mut gradient_pattern) => gradient_pattern.set_transformation(m),
            Pattern::RingPattern(ref mut ring_pattern) => ring_pattern.set_transformation(m),
            Pattern::Checker3DPattern(ref mut checker3d_pattern) => checker3d_pattern.set_transformation(m),
            Pattern::TestPattern(ref mut test_pattern) => test_pattern.set_transformation(m),
            Pattern::SphereTexturePattern(ref mut sphere_pattern) => sphere_pattern.set_transformation(m),
            Pattern::PlaneTexturePattern(ref mut plane_pattern) => plane_pattern.set_transformation(m),
            Pattern::CylinderTexturePattern(ref mut cylinder_pattern) => cylinder_pattern.set_transformation(m),
            Pattern::AlignCheckTexturePattern(ref mut cube_pattern) => cube_pattern.set_transformation(m),
            Pattern::CubeTextPattern(ref mut cube_pattern) => cube_pattern.set_transformation(m),
        }
    }

    pub fn get_transformation(&self) -> &Matrix {
        match self {
            Pattern::StripePattern(ref stripe_pattern) => stripe_pattern.get_transformation(),
            Pattern::GradientPattern(ref gradient_pattern) => gradient_pattern.get_transformation(),
            Pattern::RingPattern(ref ring_pattern) => ring_pattern.get_transformation(),
            Pattern::Checker3DPattern(ref checker3d_pattern) => checker3d_pattern.get_transformation(),
            Pattern::TestPattern(ref test_pattern) => test_pattern.get_transformation(),
            Pattern::SphereTexturePattern(ref test_pattern) => test_pattern.get_transformation(),
            Pattern::PlaneTexturePattern(ref plane_pattern) => plane_pattern.get_transformation(),
            Pattern::CylinderTexturePattern(ref cylinder_pattern) => cylinder_pattern.get_transformation(),
            Pattern::AlignCheckTexturePattern(ref cube_pattern) => cube_pattern.get_transformation(),
            Pattern::CubeTextPattern(ref cube_pattern) => cube_pattern.get_transformation(),
        }
    }

    pub fn get_inverse_transformation(&self) -> &Matrix {
        match self {
            Pattern::StripePattern(ref stripe_pattern) => stripe_pattern.get_inverse_transformation(),
            Pattern::GradientPattern(ref gradient_pattern) => gradient_pattern.get_inverse_transformation(),
            Pattern::RingPattern(ref ring_pattern) => ring_pattern.get_inverse_transformation(),
            Pattern::Checker3DPattern(ref checker3d_pattern) => checker3d_pattern.get_inverse_transformation(),
            Pattern::TestPattern(ref test_pattern) => test_pattern.get_inverse_transformation(),
            Pattern::SphereTexturePattern(ref test_pattern) => test_pattern.get_inverse_transformation(),
            Pattern::PlaneTexturePattern(ref plane_pattern) => plane_pattern.get_inverse_transformation(),
            Pattern::CylinderTexturePattern(ref cylinder_pattern) => cylinder_pattern.get_inverse_transformation(),
            Pattern::AlignCheckTexturePattern(ref cube_pattern) => cube_pattern.get_inverse_transformation(),
            Pattern::CubeTextPattern(ref cube_pattern) => cube_pattern.get_inverse_transformation(),
        }
    }
}
