# starknet_getTransactionReceipt

## Metadata

- name: starknet_getTransactionReceipt
- prefix: starknet
- state: ⚠️
- [specification]()
- [issue]()

## Specification Description

Get the details of a transaction by a given block number and index.

### Parameters

- None

### Returns

- None

## Beerus Logic

There is no Beerus logic.

### Starknet methods

- [starknet_getTransactionReceipt](https://github.com/starkware-libs/starknet-specs/blob/e0b76ed0d8d8eba405e182371f9edac8b2bcbc5a/api/starknet_api_openrpc.json#L222)

Example call:

```json
{
  "jsonrpc":"2.0",
  "method":"starknet_getTransactionReceipt",
  "params":["0x4c1672e824b5cd7477fca31ee3ab5a1058534ed1820bb27abc976c2e6095151"],
  "id":0
}
```

Example responses:

```json

```