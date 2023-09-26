# snake-rust

I'm learning Rust by implementing Snake game, so feedback/suggestions is more than welcome! The idea is to have the core as a separate crate and distinct apps based on the rendering system.

## Web App

### Install [wasm-bindgen-cli](https://github.com/rustwasm/wasm-bindgen)

```bash
cargo install wasm-bindgen-cli
```

Then you can run `./build.sh`. You'll need a way to serve the folder `app/web/www`

#### Using [basic-http-server](https://github.com/brson/basic-http-server)

```bash
cargo install basic-http-server
cd app/web/wwww
basic-http-server
```

### Running on browser

Press `space` or key `p` to unpause. It starts paused.



https://github.com/dalton-oliveira/snake-rust/assets/3465913/2266c36e-3695-4536-9726-e49bdcef91f2



As you can see, it's a work in progress.

## Termion App

To run the game on terminal just:

```bash
cd app/termion
cargo run
```

### Running on termion

https://github.com/dalton-oliveira/snake-rust/assets/3465913/f8d9febc-8eb0-4f54-8604-49749a308b27
