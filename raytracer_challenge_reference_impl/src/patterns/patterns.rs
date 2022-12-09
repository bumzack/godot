use crate::basics::color::Color;
use crate::math::matrix::Matrix;
use crate::math::tuple4d::Tuple4D;
use crate::math::MatrixOps;
use crate::patterns::checker3d_patterns::Checker3DPattern;
use crate::patterns::gradient_patterns::GradientPattern;
use crate::patterns::ring_patterns::RingPattern;
use crate::patterns::sphere_texture_patterns::SphereTexturePattern;
use crate::patterns::stripe_patterns::StripePattern;
use crate::patterns::test_patterns::TestPattern;
use crate::patterns::{AlignCheckTexturePattern, CubeTexturePattern, CylinderTexturePattern};
use crate::patterns::{ImageTexturePattern, PlaneTexturePattern};
use crate::prelude::ShapeOps;
use crate::shape::shape::Shape;

#[derive(Debug, PartialEq, Clone)]
pub enum PatternEnum {
    StripePatternEnum(StripePattern),
    GradientPatternEnum(GradientPattern),
    RingPatternEnum(RingPattern),
    Checker3DPatternEnum(Checker3DPattern),
    TestPatternEnum(TestPattern),
    SphereTexturePatternEnum(SphereTexturePattern),
    PlaneTexturePatternEnum(PlaneTexturePattern),
    CylinderTexturePatternEnum(CylinderTexturePattern),
    AlignCheckTexturePatternEnum(AlignCheckTexturePattern),
    CubeTextPatternEnum(CubeTexturePattern),
    ImageTexturePatternEnum(ImageTexturePattern),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Pattern {
    transformation_matrix: Matrix,
    inverse_transformation_matrix: Matrix,
    pattern: PatternEnum,
}

impl Pattern {
    pub fn new(pattern: PatternEnum) -> Pattern {
        Pattern {
            pattern,
            transformation_matrix: Matrix::new_identity_4x4(),
            inverse_transformation_matrix: Matrix::new_identity_4x4(),
        }
    }

    pub fn pattern_at_shape(&self, shape: &Shape, world_point: &Tuple4D) -> Color {
        let object_point = shape.get_inverse_transformation() * world_point;
        let pattern_point = self.get_inverse_transformation() * &object_point;

        match self.pattern {
            PatternEnum::StripePatternEnum(ref stripe_pattern) => stripe_pattern.pattern_at(&pattern_point),
            PatternEnum::GradientPatternEnum(ref gradient_pattern) => gradient_pattern.pattern_at(&pattern_point),
            PatternEnum::RingPatternEnum(ref ring_pattern) => ring_pattern.pattern_at(&pattern_point),
            PatternEnum::Checker3DPatternEnum(ref checker3d_pattern) => checker3d_pattern.pattern_at(&pattern_point),
            PatternEnum::TestPatternEnum(ref test_pattern) => test_pattern.pattern_at(&pattern_point),
            PatternEnum::SphereTexturePatternEnum(ref sphere_texture_pattern) => {
                sphere_texture_pattern.pattern_at(&pattern_point)
            }
            PatternEnum::PlaneTexturePatternEnum(ref plane_texture_pattern) => {
                plane_texture_pattern.pattern_at(&pattern_point)
            }
            PatternEnum::CylinderTexturePatternEnum(ref cylinder_pattern) => {
                cylinder_pattern.pattern_at(&pattern_point)
            }
            PatternEnum::AlignCheckTexturePatternEnum(ref align_check_texture_pattern) => {
                align_check_texture_pattern.pattern_at(&pattern_point)
            }
            PatternEnum::CubeTextPatternEnum(ref cube_pattern) => cube_pattern.pattern_at(&pattern_point),
            PatternEnum::ImageTexturePatternEnum(ref image_texture_pattern) => {
                image_texture_pattern.pattern_at(&pattern_point)
            }
        }
    }

    pub fn set_transformation(&mut self, m: Matrix) {
        self.inverse_transformation_matrix = Matrix::invert(&m).unwrap();
        self.transformation_matrix = m;
    }

    pub fn get_transformation(&self) -> &Matrix {
        &self.transformation_matrix
    }

    pub fn get_inverse_transformation(&self) -> &Matrix {
        &self.inverse_transformation_matrix
    }
}
