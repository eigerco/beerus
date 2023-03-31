# starknet_getClassHashAt

## Metadata

- name: starknet_getClassHashAt
- prefix: starknet
- state: ⚠️
- [specification]()
- [issue]()

## Specification Description

Get the contract class hash in the given block for the contract deployed at the given address.

### Parameters

- None

### Returns

- None

## Beerus Logic

There is no Beerus logic.

### Starknet methods

- [starknet_getClassHashAt](https://github.com/starkware-libs/starknet-specs/blob/e0b76ed0d8d8eba405e182371f9edac8b2bcbc5a/api/starknet_api_openrpc.json#LL292C21-L292C44)

Example call:

```json
{
  "jsonrpc":"2.0",
  "method":"starknet_getClassAt",
  "params":["tag", "latest", "0x073c0847469d786aaf0f09a342c6ac03a3eb84ff10ec2cb7acd2da089ca8ccff"],
  "id":0
}

```

Example responses:

```json

```