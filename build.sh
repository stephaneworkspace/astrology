#!/bin/sh
cd ./lib/libswe-sys/
./make_c.sh
cd ../..
cargo build
#cargo test
cargo run --example ferris-astro-example

# Group binary with lipo
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios
# Print NonFat -> Ok
lipo -info ./target/aarch64-apple-ios/debug/libastro_compute_swisseph.a
lipo -info ./target/x86_64-apple-ios/debug/libastro_compute_swisseph.a

# Group in one
lipo -create ./target/aarch64-apple-ios/debug/libastro_compute_swisseph.a ./target/x86_64-apple-ios/debug/libastro_compute_swisseph.a -output ./target/libastro_compute_swisseph.a
# Print architecture
lipo -info ./target/libastro_compute_swisseph.a
