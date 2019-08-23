#!/bin/sh

cd cuda_kernel_raytracer
cargo build --release &&
cd .. &&
cargo run --release

