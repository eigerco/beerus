## starknet_blockHashAndNumber
Get the most recent accepted block hash and number.
### Parameters
None

### Returns
The latest block hash and number.

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
  "method":"starknet_blockHashAndNumber",
  "params":[],
  "id":1
}'
```

### Response
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