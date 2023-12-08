#! /bin/bash

#@todo build it the rust way

set -e
cargo build -p wasm-render --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir snake-web/www/wasm --no-typescript ./target/wasm32-unknown-unknown/release/wasm_render.wasm
cargo build -p snake-web --release
