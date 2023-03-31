# starknet_getBlockTransactionCount

## Metadata

- name: starknet_getBlockTransactionCount
- prefix: starknet
- state: ⚠️
- [specification]()
- [issue]()

## Specification Description

Get the number of transactions in a block given a block id.

### Parameters

- None

### Returns

- None

## Beerus Logic

There is no Beerus logic.

### Starknet methods

- [starknet_getBlockWithTxs](https://github.com/starkware-libs/starknet-specs/blob/master/api/starknet_api_openrpc.json#L44)

Example call:

```json
{
  "jsonrpc":"2.0",
  "method":"starknet_getBlockTransactionCount",
  "params":[
    "tag",
    "latest"
  ],
  "id":0
}
```

Example responses:

```json
{
  "jsonrpc": "2.0",
  "result": "0x24",
  "id": 0
}
```