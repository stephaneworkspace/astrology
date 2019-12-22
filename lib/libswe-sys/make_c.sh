#!/bin/sh
cd ./src/swisseph/2.08/src/
rm -rf build
mkdir build
cd ./build
cmake ..
cmake --build .
