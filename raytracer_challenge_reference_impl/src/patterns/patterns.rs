use crate::basics::color::Color;
use crate::math::matrix::Matrix;
use crate::math::tuple4d::Tuple4D;
use crate::patterns::checker3d_patterns::Checker3DPattern;
use crate::patterns::gradient_patterns::GradientPattern;
use crate::patterns::ring_patterns::RingPattern;
use crate::patterns::stripe_patterns::StripePattern;
use crate::patterns::test_patterns::TestPattern;
use crate::shape::shape::Shape;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    StripePattern(StripePattern),
    GradientPattern(GradientPattern),
    RingPattern(RingPattern),
    Checker3DPattern(Checker3DPattern),
    TestPattern(TestPattern),
}

impl Pattern {
    pub fn color_at_object(&self, shape: &Shape, world_point: &Tuple4D) -> Color {
        let res = match self {
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
            Pattern::TestPattern(ref test_pattern) => {
                TestPattern::color_at_object(test_pattern, shape, world_point)
            }
        };
        res
    }

    pub fn set_transformation(&mut self, m: Matrix) {
        match self {
            Pattern::StripePattern(ref mut stripe_pattern) => stripe_pattern.set_transformation(m),
            Pattern::GradientPattern(ref mut gradient_pattern) => gradient_pattern.set_transformation(m),
            Pattern::RingPattern(ref mut ring_pattern) => ring_pattern.set_transformation(m),
            Pattern::Checker3DPattern(ref mut checker3d_pattern) => checker3d_pattern.set_transformation(m),
            Pattern::TestPattern(ref mut test_pattern) => test_pattern.set_transformation(m),
        }
    }

    pub fn get_transformation(&self) -> &Matrix {
        let res = match self {
            Pattern::StripePattern(ref stripe_pattern) => stripe_pattern.get_transformation(),
            Pattern::GradientPattern(ref gradient_pattern) => gradient_pattern.get_transformation(),
            Pattern::RingPattern(ref ring_pattern) => ring_pattern.get_transformation(),
            Pattern::Checker3DPattern(ref checker3d_pattern) => checker3d_pattern.get_transformation(),
            Pattern::TestPattern(ref test_pattern) => test_pattern.get_transformation(),
        };
        res
    }

    pub fn get_inverse_transformation(&self) -> &Matrix {
        let res = match self {
            Pattern::StripePattern(ref stripe_pattern) => stripe_pattern.get_inverse_transformation(),
            Pattern::GradientPattern(ref gradient_pattern) => gradient_pattern.get_inverse_transformation(),
            Pattern::RingPattern(ref ring_pattern) => ring_pattern.get_inverse_transformation(),
            Pattern::Checker3DPattern(ref checker3d_pattern) => checker3d_pattern.get_inverse_transformation(),
            Pattern::TestPattern(ref test_pattern) => test_pattern.get_inverse_transformation(),
        };
        res
    }
}
