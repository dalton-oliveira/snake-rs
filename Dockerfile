FROM messense/rust-musl-cross:x86_64-musl as chef
RUN cargo install cargo-chef wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rustfmt clippy
WORKDIR /snake

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe-wasm.json
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /snake/recipe.json recipe.json
COPY --from=planner /snake/recipe-wasm.json recipe-wasm.json

RUN cargo chef cook --recipe-path recipe-wasm.json -p wasm-render --target wasm32-unknown-unknown --release
RUN cargo chef cook --recipe-path recipe.json -p snake-web --release
COPY . .

RUN bash build.sh
RUN cargo fmt --check \
 && cargo clippy --release -- -D warnings --no-deps

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /snake/target/x86_64-unknown-linux-musl/release/snake-web /snake-web
ENTRYPOINT ["/snake-web"]
EXPOSE 80
