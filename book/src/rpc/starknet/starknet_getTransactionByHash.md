## starknet_getTransactionByHash
Get the details and status of a submitted transaction.

### Parameters
`TX_HASH` - The hash of the requested transaction.

### Returns
A transaction object.

### Headers
```rust
Content-Type: application/json
```

### Example
```bash
curl -X POST http://localhost:3030 \
-H "Content-Type: application/json" \
-d '{
  "jsonrpc": "2.0",
  "method": "starknet_getTransactionByHash",
  "params": ["0x588dc5eb39eb63c8b8a0e839a32afacdac44926b3ca08607528fdec6d18f856"],
  "id":1
}'
```

### Response
```json
{
   "jsonrpc":"2.0",
   "result":{
      "transaction_hash":"0x588dc5eb39eb63c8b8a0e839a32afacdac44926b3ca08607528fdec6d18f856",
      "type":"INVOKE",
      "max_fee":"0xac7fdf631c1a",
      "version":"0x0",
      "signature":[
         "0x448486257149f24f409df891957636cb8f3abe4ff3422991e6787ec54f6f28b",
         "0x3dff6a10f81b6a45968bee850177d01e72e95b71396b71ef787f793184c240d"
      ],
      "nonce":"0x0",
      "contract_address":"0xc103dbe74c95193c5be2e24a9b3866b72ab74d1466fc1b776b8759ca623d5d",
      "entry_point_selector":"0x15d40a3d6ca2ac30f4031e42be28da9b056fef9bb7357ac5e85627ee876e5ad",
      "calldata":[
         "0x1",
         "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
         "0x83afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e",
         "0x0",
         "0x3",
         "0x3",
         "0xc103dbe74c95193c5be2e24a9b3866b72ab74d1466fc1b776b8759ca623d5d",
         "0x1550f7dca70000",
         "0x0",
         "0x4"
      ]
   },
   "id":1
}
```