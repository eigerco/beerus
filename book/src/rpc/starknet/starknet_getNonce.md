## starknet_getNonce
Get the nonce associated with the given address in the given block.

### Parameters
`CONTRACT_ADDRESS` - The address of the contract to read from

### Returns
The last nonce used for the given contract.

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
  "method":"starknet_getNonce",
  "params":["0x049D36570D4e46f48e99674bd3fcc84644DdD6b96F7C741B1562B82f9e004dC7"],
  "id":1
}'
```

### Response
```json
{
  "jsonrpc": "2.0",
  "result": "5",
  "id": 1
}
```