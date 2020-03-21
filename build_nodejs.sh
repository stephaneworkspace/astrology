#!/bin/sh
cargo build
cargo test

# Group binary with lipo
cargo build --target x86_64-apple-darwin
#cargo build --target x86_64-unknown-linux-gnu
#cargo build --target x86_64-pc-windows-gnu
# Print NonFat -> Ok
lipo -info ./target/x86_64-apple-dawrin/debug/libastrology.a
#lipo -info ./target/x86_64-unknown-linux-gnu/debug/libastrology.a
#lipo -info ./target/x86_64-pc-windows-gnu/debug/libastrology.a

# Group in one
#lipo -create ./target/x86_64_apple-darwin/debug/libastrology.a ./target/x86_64-unknown-linux-gnu/debug/libastrology.a ./target/x86_64-pc-windows-gnu/debug/libastrology.a -output ./target/libastrology.a
lipo -create ./target/x86_64_apple-darwin/debug/libastrology.a -output ./target/libastrology.a

# Print architecture
lipo -info ./target/libastrology.a
