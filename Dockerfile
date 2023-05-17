# Define the base stage with the required dependencies.
#
# It also installs [cargo-chef](https://github.com/LukeMathWalker/cargo-chef), which helps caching the dependencies of a
# rust project and speed up the docker builds.
FROM rust:1.68.0 AS chef
USER root
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cargo-chef
WORKDIR /beerus

# This stage computes a recipe to be used by cargo chef in the next stage.
#
# Computing a recipe makes changes to the source code (e.g. overriding main.rs files by a dummy one), requiring a
# specific stage.
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# This stage imports the previously computed recipe and builds the beerus binary.
#
# Running a "cargo chef cook" before copying the source code and building the app allows to cache the project
# dependencies and speed up subsequent builds (up to x5).
FROM chef AS builder
COPY --from=planner /beerus/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin beerus-rpc

# The final stage
FROM alpine:latest AS runtime
RUN apk --no-cache add ca-certificates
RUN addgroup -g 1000 beerus && adduser -u 1000 -G beerus -s /bin/sh -D beerus
USER beerus
COPY --chown=beerus:beerus --from=builder /beerus/target/x86_64-unknown-linux-musl/release/beerus-rpc /usr/local/bin/

EXPOSE 3030

LABEL description="StarkNet Light Client"
LABEL authors="Beerus Team"
LABEL source="https://github.com/keep-starknet-strange/beerus"

ENTRYPOINT ["/usr/local/bin/beerus-rpc"]
