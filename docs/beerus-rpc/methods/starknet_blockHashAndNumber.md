# starknet_blockHashAndNumber

## Metadata

- name: starknet_blockHashAndNumber
- prefix: starknet
- state: ⚠️
- [specification]()
- [issue]()

## Specification Description

Get the most recent accepted block hash and number.

### Parameters

- None

### Returns

- None

## Beerus Logic

There is no Beerus logic.

### Starknet methods

- [starknet_blockHashAndNumber](https://github.com/starkware-libs/starknet-specs/blob/e0b76ed0d8d8eba405e182371f9edac8b2bcbc5a/api/starknet_api_openrpc.json#L513)

Example call:

```json
{
  "jsonrpc":"2.0",
  "method":"starknet_blockHashAndNumber",
  "params":[],
  "id":0
}
```

Example responses:

```json
{
  "jsonrpc": "2.0",
  "result": {
    "block_hash": "0x7c2f5fc9c3945932141d8e54a000cdab41bdcaf67b744893b2482db4ca496db",
    "block_number": 21705
  },
  "id": 1
}
```