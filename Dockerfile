FROM rust:bullseye as builder
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y libssl-dev
WORKDIR /beerus
COPY . .
RUN cargo build --release --bin beerus
RUN strip target/release/beerus

FROM debian:bullseye-slim
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y ca-certificates
COPY --chown=beerus:beerus --from=builder /beerus/target/release/beerus /usr/local/bin/

EXPOSE 3030

LABEL description="Starknet Light Client"
LABEL authors="Beerus Team @ Eiger"
LABEL source="https://github.com/eigerco/beerus"

ENTRYPOINT ["/usr/local/bin/beerus"]
