# Beerus Usage

Beerus CLI provides a variety of endpoints to interact with via the
StarkNet and Ethereum Networks

## Reference

```bash
# beerus <ENDPOINT> <FUNCTION> <FLAG> <EXAMPLE>
beerus ethereum query-balance --address 0x00000000219ab540356cBB839Cbe05303d7705Fa
```

## CLI Invocation

| Endpoint | Function | Flag | Example(mainnet) |
| -------- | -------- | ---- | ------- |
| `ethereum` | `query-balance` | `--address` | `0x00000000219ab540356cBB839Cbe05303d7705Fa` |

### Examples

```bash
beerus starknet query-state-root
# Should output something like:
# 3018024614248707887030376849268409283849258987090518998455999582305744756580
```

```bash
beerus ethereum query-balance --address 0x00000000219ab540356cBB839Cbe05303d7705Fa
# 2011.286832686010020640 ETH
```

```bash
beerus starknet query-contract --address 0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7 --selector 0x1e888a1026b19c8c0b57c72d63ed1737106aa10034105b980ba117bd0c29fe1 --calldata 0x00,0x01
[FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000000 }, FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000000 }]
```

```bash
beerus starknet query-get-storage-at --address 0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7 --key 0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1
298305742194
```