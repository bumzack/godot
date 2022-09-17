#!/bin/sh

time RUST_BACKTRACE=full cargo run --release --example chapter15_non_smoothed_teapot
time RUST_BACKTRACE=full cargo run --release --example chapter15_smoothed_suzanne
time RUST_BACKTRACE=full cargo run --release --example chapter15_smoothed_teapot
