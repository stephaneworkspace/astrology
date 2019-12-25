#!/bin/sh
cd ./src/swisseph/2.08/src/
rm -rf build
mkdir build
cd ./build
cmake .. -G Xcode -DCMAKE_TOOLCHAIN_FILE=/Users/stephanebressani/Code/Rust/astro_compute_swisseph/lib/libswe-sys/src/swisseph/2.08/src/ios-cmake/ios.toolchain.cmake -DPLATFORM=OS64
cmake --build .
