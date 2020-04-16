#!/bin/sh
#cd ./lib/libswe-sys/
#./make_c.sh
#cd ../..
cargo build
cargo test

# Group binary with lipo
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios
# Print NonFat -> Ok
lipo -info ./target/aarch64-apple-ios/debug/libastrology.a
lipo -info ./target/x86_64-apple-ios/debug/libastrology.a

# Group in one
lipo -create ./target/aarch64-apple-ios/debug/libastrology.a ./target/x86_64-apple-ios/debug/libastrology.a -output ./target/libastrology.a
# Print architecture
lipo -info ./target/libastrology.a

#cbindgen . -o target/libastrology.h

# -> conflict with docs.rs

#extern crate cbindgen;

#use std::env;

#fn main() {
#    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
#    let mut config: cbindgen::Config = Default::default();
#    config.language = cbindgen::Language::C;
#    cbindgen::generate_with_config(&crate_dir, config)
#        .unwrap()
#        .write_to_file("target/libastrology.h");
#}
