## starknet_blockNumber
Get the most recent accepted block number.

### Parameters
None

### Returns
The latest block number.

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
  "method":"starknet_blockNumber",
  "params":[],
  "id":1
}'
```

### Response
```json
{
  "jsonrpc":"2.0",
  "result":35970,
  "id":1
}
```