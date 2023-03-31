# starknet_getBlockWithTxs

## Metadata

- name: starknet_getBlockWithTxs
- prefix: starknet
- state: ⚠️
- [specification]()
- [issue]()

## Specification Description

Get block information with full transactions given the block id.

### Parameters

- None

### Returns

- None

## Beerus Logic

There is no Beerus logic.

### Starknet methods

- [starknet_getBlockWithTxs](https://github.com/starkware-libs/starknet-specs/blob/e0b76ed0d8d8eba405e182371f9edac8b2bcbc5a/api/starknet_api_openrpc.json#L44)

Example call:

```json
{
  "jsonrpc":"2.0",
  "method":"starknet_getBlockWithTxs",
  "params":["tag", "pending"],
  "id":1
}
```

Example responses:

```json
{
  "jsonrpc": "2.0",
  "result": {
    "transactions": [
      {
        "transaction_hash": "0x7d833a8c2e2f7edbda36e17ca0c065b99157a273ddc6b555874d501fc502706",
        "type": "INVOKE",
        "max_fee": "0x8ab2ac5408000",
        "version": "0x1",
        "signature": [
          "0x6bf8f38ff805d60561215f8c1d8a668cbc6f467051b0ce314ffcae00b1630d0",
          "0x70b2f4c6afde857249c522ce8bcbc7cb89a9aa0bf105b99e6bb83faa86bab96"
        ],
        "nonce": "0x3",
        "sender_address": "0x325197762aa20aa60d698f2aacd4515b9575ba3e16a51166bcfc6e5bb928289",
        "calldata": [
          "0x2",
          "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
          "0x219209e083275171774dab1df80982e9df2096516f06319c5c6d71ae0a8480c",
          "0x0",
          "0x3",
          "0x79b882cb8200c1c1d20e849a2ef19124b0b8985358c1313ea6af588cfe4fec8",
          "0x3e2bd66aeb9284521dbba619e698cd99508e215c152bf788b608349e67bba61",
          "0x3",
          "0x19",
          "0x1c",
          "0x79b882cb8200c1c1d20e849a2ef19124b0b8985358c1313ea6af588cfe4fec8",
          "0x6d23ad5f8000",
          "0x0",
          "0x0",
          "0x325197762aa20aa60d698f2aacd4515b9575ba3e16a51166bcfc6e5bb928289",
          "0x6d23ad5f8000",
          "0xa655",
          "0x0",
          "0x2134",
          "0x0",
          "0x1",
          "0x5a9d4a6e2238f08c01b73f4933afe4175379d64bef88878d3cf6f0a5fea94ca",
          "0x4a3621276a83251b557a8140e915599ae8e7b6207b067ea701635c0d509801e",
          "0x6d23ad5f8000",
          "0xa655",
          "0x0",
          "0x1",
          "0x4218d4cf9879f6a7ed53338b121e5b7021cb9632501bd3b880c59c1dc7b5256",
          "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
          "0x1",
          "0x0",
          "0x66276cc7",
          "0x2134",
          "0x0",
          "0x2",
          "0x7644d6b17692cba2b2d0ea75d9a2335b3fe5b47214884cab0787d99c951ec9a",
          "0x515274c68f149022027f879b7343595fcfd9128dd60e4ac47ed4123cadbb873",
          "0x0"
        ]
      },
      ....
```