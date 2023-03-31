# starknet_getTransactionByBlockIdAndIndex

## Metadata

- name: starknet_getTransactionByBlockIdAndIndex
- prefix: starknet
- state: ⚠️
- [specification]()
- [issue]()

## Specification Description

Get the details of a transaction by a given block id and index.

### Parameters

- None

### Returns

- None

## Beerus Logic

There is no Beerus logic.

### Starknet methods

- [starknet_getTransactionByBlockIdAndIndex](https://github.com/starkware-libs/starknet-specs/blob/e0b76ed0d8d8eba405e182371f9edac8b2bcbc5a/api/starknet_api_openrpc.json#L184)

Example call:

```json
{
  "jsonrpc":"2.0",
  "method":"starknet_getTransactionByBlockIdAndIndex",
  "params":["tag", "pending", "0"],
  "id":0
}
```

Example responses:

```json
{
  "jsonrpc": "2.0",
  "result": {
    "transaction_hash": "0x7401e10bf3fd61116663ef24bb6c67e3d28c7a43ea90b704487680462e8f4b4",
    "type": "INVOKE",
    "max_fee": "0x699d755fe2000",
    "version": "0x1",
    "signature": [
      "0x49688b862db1ceacfb81b4f9af8966866578ea1e85d05dd8e62ca36cba792f6",
      "0x61ecb3eb82e44b3e0d9db938e11bb4ead73eacd6ae4070130a34337db25157b"
    ],
    "nonce": "0x12",
    "sender_address": "0xd8e0f2eb017035c6f35b8978c46730e867b81dfae44cbeedb443f6617c0687",
    "calldata": [
      "0x1",
      "0x6ac597f8116f886fa1c97a23fa4e08299975ecaf6b598873ca6792b9bbfb678",
      "0x2d01c9f1ed8d814a32aac4171c6cc5a66828d7f97a5da83a6bb6b6f064a0ee2",
      "0x0",
      "0x2",
      "0x2",
      "0x1",
      "0x3723e96"
    ]
  },
  "id": 1
}
```