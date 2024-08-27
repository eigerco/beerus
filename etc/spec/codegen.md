USAGE:

```
cd /path/to/beerus

## Clone & build iamgroot
cd ..
git clone https://github.com/sergey-melnychuk/iamgroot.git --branch v0.2.7
cd iamgroot && cargo build --release
cp ./target/release/iamgroot ../beerus/tmp
cd ../beerus

## Generate the code
RUST_LOG=off ./tmp/iamgroot CODE \
etc/spec/starknet/0.7.0/starknet_query_api_openrpc.json \
etc/spec/starknet/0.7.0/starknet_write_api_openrpc.json \
etc/spec/starknet/0.7.0/starknet_trace_api_openrpc.json \
etc/spec/pathfinder_api_openrpc.json \
--async --blocking --client --reexport > ./src/gen.rs

## Auto-format and check the generated code
cargo fmt && cargo check

# if previons line succeeded, iamgroot is no longer necessary
rm ./tmp/iamgroot
```
