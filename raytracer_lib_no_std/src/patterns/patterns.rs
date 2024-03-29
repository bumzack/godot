#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

use math::prelude::*;

use crate::{Checker3DPattern, Color, GradientPattern, RingPattern, Shape, StripePattern, TestPattern};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "cuda", derive(DeviceCopy))]
pub enum Pattern {
    StripePattern(StripePattern),
    GradientPattern(GradientPattern),
    RingPattern(RingPattern),
    Checker3DPattern(Checker3DPattern),
    TestPattern(TestPattern),
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
        }
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
        match self {
            Pattern::StripePattern(ref stripe_pattern) => stripe_pattern.get_transformation(),
            Pattern::GradientPattern(ref gradient_pattern) => gradient_pattern.get_transformation(),
            Pattern::RingPattern(ref ring_pattern) => ring_pattern.get_transformation(),
            Pattern::Checker3DPattern(ref checker3d_pattern) => checker3d_pattern.get_transformation(),
            Pattern::TestPattern(ref test_pattern) => test_pattern.get_transformation(),
        }
    }

    pub fn get_inverse_transformation(&self) -> &Matrix {
        match self {
            Pattern::StripePattern(ref stripe_pattern) => stripe_pattern.get_inverse_transformation(),
            Pattern::GradientPattern(ref gradient_pattern) => gradient_pattern.get_inverse_transformation(),
            Pattern::RingPattern(ref ring_pattern) => ring_pattern.get_inverse_transformation(),
            Pattern::Checker3DPattern(ref checker3d_pattern) => checker3d_pattern.get_inverse_transformation(),
            Pattern::TestPattern(ref test_pattern) => test_pattern.get_inverse_transformation(),
        }
    }
}
