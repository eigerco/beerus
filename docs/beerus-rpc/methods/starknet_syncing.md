# starknet_syncing

## Metadata

- name: starknet_syncing
- prefix: starknet
- state: ⚠️
- [specification]()
- [issue]()

## Specification Description

Returns an object about the sync status, or false if the node is not synching.

### Parameters

- None

### Returns

- None

## Beerus Logic

There is no Beerus logic.

### Starknet methods

- [starknet_syncing](https://github.com/starkware-libs/starknet-specs/blob/e0b76ed0d8d8eba405e182371f9edac8b2bcbc5a/api/starknet_api_openrpc.json#L565)

Example call:

```json
{
    "jsonrpc":"2.0",
  "method":"starknet_syncing",
  "params":[],
  "id":0
}
```

Example responses:

```json
{
  "jsonrpc": "2.0",
  "result": {
    "starting_block_hash": "0x6e3caa841355429168ee5333c69bc42eb7986f12e959942b560a6c7aae89c17",
    "starting_block_num": "0x52cf",
    "current_block_hash": "0x54d3cf25f8db6d0a3568ab03ffd64c739ea6b7e28360c379122532906357de4",
    "current_block_num": "0x54fc",
    "highest_block_hash": "0x54d3cf25f8db6d0a3568ab03ffd64c739ea6b7e28360c379122532906357de4",
    "highest_block_num": "0x54fc"
  },
  "id": 1
}
```