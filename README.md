# snake-rust

I'm learning Rust by implementing a multiplayer Snake game, so feedback/suggestions is more than welcome! The idea is to have the core as a separate crate and distinct apps based on the rendering system. Also looked into rendering the same way as the old Nokia phones.

## Web App

### Install [Rust](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install [wasm-bindgen-cli](https://github.com/rustwasm/wasm-bindgen)

```bash
cargo install wasm-bindgen-cli
```

### Web version (multi-player)

```bash
rustup target add wasm32-unknown-unknown
./build.sh
cargo run -p snake-multiplayer
```

### Running on browser

https://github.com/dalton-oliveira/snake-rust/assets/3465913/9c18d0d3-a21b-4da9-bc58-d93de935021c

## Terminal version

To run the game on terminal just:

```bash
cargo run -p snake-multiplayer
```

### Running on termion

https://github.com/dalton-oliveira/snake-rust/assets/3465913/f8d9febc-8eb0-4f54-8604-49749a308b27
