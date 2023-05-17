## starknet_getTransactionByBlockIdAndIndex
Get the details of a transaction by a given block id and index.

### Parameters
`BLOCK_PARAM` - Expected one of `block_number`, `block_hash`, `latest`, `pending`.

`INDEX` - He index in the block to search for the transaction.

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
  "jsonrpc":"2.0",
  "method":"starknet_getTransactionByBlockIdAndIndex",
  "params":["tag", "latest", "10"],
  "id":1
}'
```

### Response
```json
{
   "jsonrpc":"2.0",
   "result":{
      "transaction_hash":"0x56c5223464e08daa45584bc06a5dd2386ce0ba05ef30f522ab6dc994ece62a7",
      "type":"INVOKE",
      "max_fee":"0x2f78a0e056cfe",
      "version":"0x1",
      "signature":[
         "0x3a6657c444639843e5e290b9beaa3fab9408b9b73f9142e79a269890debd3da",
         "0x240b8296e25292fc2e8eb797eb1d0282ccd596ba586663d832bfa390c3fd3e2"
      ],
      "nonce":"0x1e",
      "sender_address":"0x29c23538622e3c63a04a1a42501e173015f7c1b8cdc6f81360b34bedbf3d75e",
      "calldata":[
         "0x2",
         "0xda114221cb83fa859dbdb4c44beeaa0bb37c7537ad5ae66fe5e0efd20e6eb3",
         "0x219209e083275171774dab1df80982e9df2096516f06319c5c6d71ae0a8480c",
         "0x0",
         "0x3",
         "0x10884171baf1914edc28d7afb619b40a4051cfae78a094a55d230f19e944a28",
         "0x15543c3708653cda9d418b4ccd3be11368e40636c10c44b18cfe756b6d88b29",
         "0x3",
         "0x6",
         "0x9",
         "0x10884171baf1914edc28d7afb619b40a4051cfae78a094a55d230f19e944a28",
         "0x183fa1b2dc193222e",
         "0x0",
         "0x6",
         "0xda114221cb83fa859dbdb4c44beeaa0bb37c7537ad5ae66fe5e0efd20e6eb3",
         "0x183fa1b2dc193222e",
         "0x0",
         "0x1a44f99",
         "0x0"
      ]
   },
   "id":1
}
```