#!/bin/sh
#cd ./lib/libswe-sys/
#./make_c.sh
#cd ../..
cargo build
cargo test
# cargo run --example ferris-astro-example

# Group binary with lipo
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios
# Print NonFat -> Ok
lipo -info ./target/aarch64-apple-ios/debug/libastro.a
lipo -info ./target/x86_64-apple-ios/debug/libastro.a

# Group in one
lipo -create ./target/aarch64-apple-ios/debug/libastro.a ./target/x86_64-apple-ios/debug/libastro.a -output ./target/libastro.a
# Print architecture
lipo -info ./target/libastro.a
