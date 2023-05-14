## starknet_getBlockTransactionCount
Get the number of transactions in a block given a block id.

### Parameters
`BLOCK_PARAM` - Expected one of `block_number`, `block_hash`, `latest`, `pending`.

### Returns
The number of transactions in the designated block.

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
  "method":"starknet_getBlockTransactionCount",
  "params":["tag", "latest"],
  "id":0
}'
```

### Response
```json
{
  "jsonrpc": "2.0",
  "result": 201,
  "id": 0
}
```