#!/bin/sh

rm *.ppm

#RUST_BACKTRACE=full  cargo run --release --examples  compare_to_cuda
#RUST_BACKTRACE=full  cargo run --release --examples  chapter14_with_aa
#RUST_BACKTRACE=full  cargo run --release --examples  shadow_glamour_shot
#RUST_BACKTRACE=full  cargo run --release --examples  test_soft_shadow_aka_area_light


for f in *.rs
do
  filename="$(basename ${f} .rs)"
  echo "building file ${f}   -> filename ${filename}"
  RUST_BACKTRACE=full  cargo build --release --example  $filename
done


for f in *.rs
do
  filename="$(basename ${f} .rs)"
  echo "running file ${f}   -> filename ${filename}"
  RUST_BACKTRACE=full  cargo run --release --example  $filename
done
