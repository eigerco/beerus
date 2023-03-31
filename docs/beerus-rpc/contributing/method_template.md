# {METHOD_NAME}

## Metadata

- name: {method_name}
- state: {❌ | ⚠️ |⏳ | ✅ |🟡}
- [specification](https://github.com/starkware-libs/starknet-specs)
- [issue](https://github.com/keep-starknet-strange/beerus/pull/{issue_id})

## Specification Description

The method behavior following the specification (copy & paste authorized)

### Parameters

- Name - type - brief description (see [types](types.md))

### Returns

- type - brief description (see [types](types.md))

## Beerus Logic

How is the method working with the ethereum and starknet solution.

### Ethereum methods

The Ethereum api methods needed listed and linked to the
[repo](https://github.com/)
line with permalink

- [ethereum](https://)

### Starknet methods

The Starknet api methods needed listed and linked to the
[starknet api reference file](https://github.com/starkware-libs/starknet-specs/blob/63bdb0fe3e7c0fd21bc47b2301528bff32980bf6/api/starknet_api_openrpc.json)
and line permalink

- [starknet_getBlockWithTxHashes](https://github.com/starkware-libs/starknet-specs/blob/63bdb0fe3e7c0fd21bc47b2301528bff32980bf6/api/starknet_api_openrpc.json#L11)

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