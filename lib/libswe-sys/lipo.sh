#!/bin/bash
# 01.2020 : DEPREACED
lipo -info ./build/libswe_ios.a
lipo -info ./build/libswe_sim.a

# Group in one
lipo -create ./build/libswe_ios.a ./build/libswe_sim.a -output ./build/libswe.a
# Print architecture
lipo -info ./build/libswe.a
