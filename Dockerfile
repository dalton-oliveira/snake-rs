FROM messense/rust-musl-cross:x86_64-musl as chef
RUN cargo install cargo-chef wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown
RUN apt update && apt install wget
RUN wget https://dmej8g5cpdyqd.cloudfront.net/downloads/noip-duc_3.0.0-beta.7.tar.gz && tar xf noip-duc_3.0.0-beta.7.tar.gz
# RUN ls noip-duc_3.0.0-beta.7/binaries -lah
# RUN noip-duc_3.0.0-beta.7/binaries/noip-duc_3.0.0-beta.7_x86_64-musl

WORKDIR /snake
FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /snake/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN bash build.sh

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /snake/target/x86_64-unknown-linux-musl/release/snake-web /snake-web
ENTRYPOINT ["/snake-web"]
EXPOSE 80
