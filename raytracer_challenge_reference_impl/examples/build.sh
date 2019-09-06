#!/bin/sh

rm *.ppm

RUST_BACKTRACE=full  cargo run --release --examples  compare_to_cuda
RUST_BACKTRACE=full  cargo run --release --examples  chapter14_with_aa
RUST_BACKTRACE=full  cargo run --release --examples  shadow_glamour_shot
RUST_BACKTRACE=full  cargo run --release --examples  test_soft_shadow_aka_area_light
