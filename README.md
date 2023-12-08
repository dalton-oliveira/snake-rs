# Overview

This is an attempt to mimic Nokia Snake game while learning Rust, so suggestions are more than welcome! It's using [Salvo](https://github.com/salvo-rs/salvo) as backend which sends game state data via websocket, so yes it's designed to be multiplayer. Front-end is mainly done with wasm.

![snake-web](https://github.com/dalton-oliveira/snake-rust/assets/3465913/23364a6b-3b59-4807-9525-d18470ff2a93)

## Run (Docker)

```bash
docker-compose build && docker-compose up
```

## Run

### Install [Rust](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install [wasm-bindgen-cli](https://github.com/rustwasm/wasm-bindgen)

```bash
cargo install wasm-bindgen-cli
```

### Add WASM as target

```bash
rustup target add wasm32-unknown-unknown
./build.sh
cargo run -p snake-web
```

## Run on terminal

Press `q` to quit

```bash
cargo run -p snake-termion
```

![snake-termion](https://github.com/dalton-oliveira/snake-rust/assets/3465913/95d5d6dd-0027-4286-a562-219c83c2fac4)

## Roadmap

- Trace backend and front-end calls with [Open Telemetry](https://github.com/open-telemetry/opentelemetry-rust)
- Experiment WebRTC in order to reduce latency
- Add unit and integration tests
- Run it on a embedded system with restricted memory and processing power
- Large world where the snake can navigate to stress test chosen data structures
- Other game elements such as walls and wormholes
- Graceful shutdown
