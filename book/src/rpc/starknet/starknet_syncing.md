## starknet_syncing
Returns an object about the sync status, or false if the node is not synching.

### Parameters
None

### Returns
The status of the node, if it is currently synchronizing state. FALSE otherwise.

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
  "method":"starknet_syncing",
  "params":[],
  "id":1
}'
```

### Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "starting_block_hash": "0x6e3caa841355429168ee5333c69bc42eb7986f12e959942b560a6c7aae89c17",
    "starting_block_num": "0x52cf",
    "current_block_hash": "0x54d3cf25f8db6d0a3568ab03ffd64c739ea6b7e28360c379122532906357de4",
    "current_block_num": "0x54fc",
    "highest_block_hash": "0x54d3cf25f8db6d0a3568ab03ffd64c739ea6b7e28360c379122532906357de4",
    "highest_block_num": "0x54fc"
  },
  "id": 1
}
```