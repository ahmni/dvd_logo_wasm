#!/bin/bash

mkdir -p dist
cp target/asmjs-unknown-emscripten/release/hello-rust-sdl2-wasm.wasm dist
cp target/asmjs-unknown-emscripten/release/hello-rust-sdl2-wasm.js dist
cp index.html dist
