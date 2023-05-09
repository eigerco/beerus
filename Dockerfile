FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /beerus

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /beerus/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --all --locked --release

FROM ubuntu:22.04 AS runtime
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /beerus/target/release/beerus-rpc /usr/local/bin/
LABEL description="Docker image for Beerus, light client for Starknet." \
      image.authors="Keep Starknet Strange team." \
      image.description="Docker image for Beerus, light client for Starknet." \
      image.source="https://github.com/keep-starknet-strange/beerus" \
      image.documentation="https://github.com/keep-starknet-strange/beerus"
EXPOSE 3030
EXPOSE 3031
ENTRYPOINT ["/usr/local/bin/beerus-rpc"]

FROM runtime