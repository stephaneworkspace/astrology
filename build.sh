#!/bin/sh
cd ./lib/libswe-sys/
./make_c.sh
cd ../..
cargo build
#cargo test
cargo run --example ferris-astro-example
# --verbose
