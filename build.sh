#! /bin/bash

set -e

cargo build -p snake-web --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir app/web/www/wasm --no-typescript ./target/wasm32-unknown-unknown/release/snake_web.wasm
