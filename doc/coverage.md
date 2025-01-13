How to make a code coverage report

1. Install [tarpaulin](https://github.com/xd009642/tarpaulin)

```
cargo install cargo-tarpaulin
```

1. Run the tests


```
## Exclude ./web from coverage
## WARNING: Commit any changes made to ./web/* first, or they will be lost
rm -rf web/*

export STARKNET_MAINNET_URL="https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0_7/${ALCHEMY_KEY}"
export STARKNET_SEPOLIA_URL="https://starknet-sepolia.g.alchemy.com/starknet/version/rpc/v0_7/${ALCHEMY_KEY}"
BEERUS_TEST_RUN=1 cargo tarpaulin --out html

## Restore content of ./web
git restore web
```

```
2024-10-23T09:38:36.285998Z  INFO cargo_tarpaulin::report: Coverage Results:
|| Tested/Total Lines:
|| src/client.rs: 9/63
|| src/config.rs: 21/59
|| src/eth.rs: 0/78
|| src/exe/err.rs: 0/6
|| src/exe/map.rs: 48/50
|| src/exe/mod.rs: 75/127
|| src/proof.rs: 86/157
|| src/rpc.rs: 114/154
|| src/util.rs: 13/14
||
51.69% coverage, 366/708 lines covered
```

1. Check out the report

Open `tarpaulin-report.html` in a browser.

1. Update report (optional)

```
mv tarpaulin-report.html coverage.html
rm doc/coverage.html
mv coverage.html doc/

git add doc/coverage.html
git commit -m 'docs(cov): update coverage report'
```

1. Alternative coverage (optional)

Use [llvm-cov](https://github.com/taiki-e/cargo-llvm-cov):

```
cargo +stable install cargo-llvm-cov --locked

export STARKNET_MAINNET_URL="https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0_7/${ALCHEMY_KEY}"
export STARKNET_SEPOLIA_URL="https://starknet-sepolia.g.alchemy.com/starknet/version/rpc/v0_7/${ALCHEMY_KEY}"
BEERUS_TEST_RUN=1 cargo llvm-cov --html
```
