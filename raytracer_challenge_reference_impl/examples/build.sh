#!/bin/sh

rm *.ppm

RUST_BACKTRACE=full  cargo run --release --example  compare_to_cuda
RUST_BACKTRACE=full  cargo run --release --example  chapter14_with_aa
RUST_BACKTRACE=full  cargo run --release --example  shadow_glamour_shot
RUST_BACKTRACE=full  cargo run --release --example  test_soft_shadow_aka_area_light
