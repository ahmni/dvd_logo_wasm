#!/bin/bash

set -eu

cargo build --target=wasm32-unknown-emscripten --release

mkdir -p dist
#cp target/asmjs-unknown-emscripten/release/hello-rust-sdl2-wasm.js dist

cp target/wasm32-unknown-emscripten/release/hello_rust_sdl2_wasm.wasm dist
cp target/wasm32-unknown-emscripten/release/hello-rust-sdl2-wasm.js dist
cp index.html dist
