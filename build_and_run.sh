#!/bin/sh

cd cuda_kernel_raytracer
cargo build &&
cd .. &&
cargo run

