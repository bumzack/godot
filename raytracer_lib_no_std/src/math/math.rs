// #![feature(core_intrinsics)]

use core::intrinsics;

use crate::math::libm_striped_to_pow::powf::powf;

#[inline]
pub fn intri_abs(x: f32) -> f32 {
    unsafe { intrinsics::fabsf32(x) }
}

#[inline]
pub fn intri_powi(x: f32, i: i32) -> f32 {
    unsafe { intrinsics::powif32(x, i) }
}

#[inline]
pub fn intri_powf(x: f32, i: f32) -> f32 {
    // ¯\_(ツ)_/¯ use libm implementation - implementation for powf and exp are missing - there are some funny error message from the LLVM linker
    powf(x, i)
}

#[inline]
pub fn intri_sqrt(x: f32) -> f32 {
    unsafe { intrinsics::sqrtf32(x) }
}

#[inline]
pub fn intri_sin(x: f32) -> f32 {
    unsafe { intrinsics::sinf32(x) }
}

#[inline]
pub fn intri_cos(x: f32) -> f32 {
    unsafe { intrinsics::cosf32(x) }
}

#[inline]
pub fn intri_tan(x: f32) -> f32 {
    intri_sin(x) / intri_cos(x)
}

#[inline]
pub fn intri_min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
pub fn intri_max(a: f32, b: f32) -> f32 {
    if a < b {
        b
    } else {
        a
    }
}

#[inline]
pub fn intri_floor(f: f32) -> f32 {
    unsafe { intrinsics::floorf32(f) }
}

#[cfg(test)]
mod tests {
    use core::f32::*;

    use crate::math::libm_striped_to_pow::fabsf::fabsf;

    use super::*;

    #[test]
    fn test_powf() {
        assert!(fabsf(NAN).is_nan());

        assert_eq!(intri_powf(1.0, 1.0), 1.0);
        assert_eq!(intri_powf(1.0, 2.0), 1.0);
        assert_eq!(intri_powf(2.0, 2.0), 4.0);

        assert_eq!(intri_powf(4.0, 0.5), 2.0);
        assert_eq!(intri_powf(1048576.0, 1.0 / 10.), 4.0);
    }
}
