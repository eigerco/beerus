## starknet_getTransactionReceipt
Get the details of a transaction by a given block number and index.

### Parameters
`TX_HASH` - The hash of the requested transaction.

### Returns
A transaction receipt object.

### Headers
```rust
Content-Type: application/json
```

### Example
```bash
curl -X POST http://localhost:3030 \
-H "Content-Type: application/json" \
-d '{
  "jsonrpc":"2.0",
  "method":"starknet_getTransactionReceipt",
  "params":["0x588dc5eb39eb63c8b8a0e839a32afacdac44926b3ca08607528fdec6d18f856"],
  "id":1
}'
```

### Response
```json
{
   "jsonrpc":"2.0",
   "result":{
      "transaction_hash":"0x588dc5eb39eb63c8b8a0e839a32afacdac44926b3ca08607528fdec6d18f856",
      "actual_fee":"0x72ffea4212bc",
      "status":"ACCEPTED_ON_L1",
      "block_hash":"0x34a3d2f77b3066facad777f80180930ae0106a3f9b4a3bb34fc010179c141a0",
      "block_number":4342,
      "type":"INVOKE",
      "messages_sent":[
         
      ],
      "events":[
         {
            "from_address":"0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
            "keys":[
               "0x99cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9"
            ],
            "data":[
               "0xc103dbe74c95193c5be2e24a9b3866b72ab74d1466fc1b776b8759ca623d5d",
               "0xc103dbe74c95193c5be2e24a9b3866b72ab74d1466fc1b776b8759ca623d5d",
               "0x1550f7dca70000",
               "0x0"
            ]
         },
         {
            "from_address":"0xc103dbe74c95193c5be2e24a9b3866b72ab74d1466fc1b776b8759ca623d5d",
            "keys":[
               "0x5ad857f66a5b55f1301ff1ed7e098ac6d4433148f0b72ebc4a2945ab85ad53"
            ],
            "data":[
               "0x588dc5eb39eb63c8b8a0e839a32afacdac44926b3ca08607528fdec6d18f856",
               "0x1",
               "0x1"
            ]
         }
      ]
   },
   "id":1
}
```