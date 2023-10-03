#! /bin/bash

#@todo build it the rust way

set -e
cargo build --release
cargo build -p wasm-render --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir wasm-render/www/wasm --no-typescript ./target/wasm32-unknown-unknown/release/wasm_render.wasm
