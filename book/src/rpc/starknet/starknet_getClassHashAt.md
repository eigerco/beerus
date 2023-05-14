## starknet_getClassHashAt
Get the contract class hash in the given block for the contract deployed at the given address.

### Parameters
`BLOCK_PARAM` - Expected one of `block_number`, `block_hash`, `latest`, `pending`.
`CONTRACT_ADDRESS` - The address of the contract to read from.

### Returns
The class hash of the given contract.

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
  "method":"starknet_getClassHashAt",
  "params":["tag", "latest", "0x073c0847469d786aaf0f09a342c6ac03a3eb84ff10ec2cb7acd2da089ca8ccff"],
  "id":0
}'
```

### Response
```json
// Some code
```