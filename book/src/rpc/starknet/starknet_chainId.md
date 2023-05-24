## starknet_chainId
Return the currently configured Starknet chain id.

### Parameters
None

### Returns
The chain id the node is connected to.

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
  "method":"starknet_chainId",
  "params":[],
  "id":0
}'
```

### Response
```json
{
  "jsonrpc": "2.0",
  "result": "23448594291968334",
  "id": 1
}
```