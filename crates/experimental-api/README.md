beerus-experimental-api
==========

**This is highly experimental. Don't try this at home.**

SCOPE:

- iamgroot: enforced RPC spec consistency
  - owned DTO and RPC code
    - no 3rd-party dependencies besides `serde_json`
    - seamless migration between any web frameworks
    - space for customisations & optimisations
  - seamless support for multiple spec versions
  - no more "spec inconsistensy" fixes
  - auto-generated `Rpc` trait (both blocking and async)
  - auto-generated RPC Client (both blocking and async)
- blockifier: stateless execution/tracing of transactions
  - necessary step towards trustless-ness
    - verify state consumed/produced by a transaction
  - follows existing Helios approach 
    - Helios uses `revm` for execution
  - blockifier: chain/block/tx context
  - blockifier: `BlockifierState` impl
  - blockifier: `StateReader` impl
  - see [example](https://github.com/sergey-melnychuk/beerthem/blob/main/examples/exec.rs)
- helios: standalone lightweight client
  - `reqwest` + `keccak` + `primitive_types` should be enough
  - see [example](https://github.com/eqlabs/pathfinder/blob/v0.11.0/crates/ethereum/src/lib.rs#L108)
- axum: `wasm32-unknown-unknown` is not supported
  - any server that supports JSON over HTTP can do
    - the generated code is not bound to any specific framework or library
    - thus allows seamless migration practically anywhere
  - what exact HTTP server library to use - subject for research


USAGE:

```
cd /path/to/beerus

cd ..
git clone https://github.com/sergey-melnychuk/iamgroot.git --branch v0.2.6
cd iamgroot && cargo build --release
cp ./target/release/iamgroot ../beerus/tmp
cd ../beerus

RUST_LOG=off ./tmp/iamgroot CODE \
etc/spec/starknet/0.6.0/starknet_query_api_openrpc.json \
etc/spec/starknet/0.6.0/starknet_write_api_openrpc.json \
etc/spec/starknet/0.6.0/starknet_trace_api_openrpc.json \
etc/spec/pathfinder_api_openrpc.json \
--async --blocking --client --reexport > crates/experimental-api/src/gen.rs

cargo fmt && cargo check

# if previons line succeeded, iamgroot is no longer necessary
rm ./tmp/iamgroot
```
