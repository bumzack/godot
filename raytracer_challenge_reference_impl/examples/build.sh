#!/bin/sh

rm *.ppm

RUST_BACKTRACE=full cargo run --release --example  test_soft_shadow_aka_area_light

cargo run --release --example  compare_to_cuda
cargo run --release --example  chapter14_with_aa
cargo run --release --example  shadow_glamour_shot
