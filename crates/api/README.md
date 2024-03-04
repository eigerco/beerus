beerus-api
==========

**This is highly experimental. Don't try this at home.**

SCOPE:

- iamgroot: take spec upgrade from O(N) to O(1 + e)
  - (N == number of updates to the spec)
  - (e == DTO mapping if necessary)
  - [X] auto-generated `Rpc` trait
  - [X] auto-generated RPC Client
- stateless execution/tracing on transactions
  - see [example](https://github.com/sergey-melnychuk/beerthem/blob/main/examples/exec.rs)
  - [ ] blockifier: chain/block/etc context
  - [ ] blockifier: implement `BlockifierState`
  - [ ] blockifier: implement `StateReader`
- [ ] helios: standalone lightweight client
  - `reqwest` + `keccak` + `primitive_types` should be enough
  - see [example](https://github.com/eqlabs/pathfinder/blob/v0.11.0/crates/ethereum/src/lib.rs#L108)
- [ ] axum: `wasm32-unknown-unknown` is not supported
  - any server that supports JSON over HTTP can do
    - the generated code is not bound to any specific framework or library
    - thus allows seamless migration practically anywhere
  - what exact HTTP server library to use - subject for research


USAGE:

```
cd /path/to/beerus

cd ..
git clone https://github.com/sergey-melnychuk/iamgroot.git
cd iamgroot && cargo build --release
cp ./target/release/iamgroot ../beerus/
cd ../beerus

./iamgroot CODE \
etc/spec/starknet/0.6.0/starknet_query_api_openrpc.json \
etc/spec/starknet/0.6.0/starknet_write_api_openrpc.json \
etc/spec/starknet/0.6.0/starknet_trace_api_openrpc.json \
etc/spec/pathfinder_api_openrpc.json \
--async --client --reexport > crates/api/src/gen.rs

cargo fmt && cargo check --features experimental

# if previons line succeeded, iamgroot is no longer necessary
rm ./iamgroot
```
