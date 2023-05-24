## starknet_getBlockWithTxHashes
Get block information with transaction hashes given the block id.

### Parameters
`BLOCK_PARAM` - Expected one of `block_number`, `block_hash`, `latest`, `pending`.

### Returns
The resulting block information with transaction hashes.

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
  "method": "starknet_getBlockWithTxHashes",
  "params": ["number", "145"],
  "id": 1
}'
```

### Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "status": "ACCEPTED_ON_L2",
    "block_hash": "0x75b424fb7b66aa6177a524a7505ce183ee9af3459d386a9ad1cb512e7ff8669",
    "parent_hash": "0x44216e3ac878c0a3d582f43e63d29bc3b6667a680755d1e04108e6a88f06768",
    "block_number": 21746,
    "new_root": "0x4b6cab72d0d6350d618d939077435aa32536db628f85667c4ad7efdc08192b2",
    "timestamp": 1678442047,
    "sequencer_address": "0x5dcd266a80b8a5f29f04d779c6b166b80150c24f2180a75e82427242dab20a9",
    "transactions": [
      "0x5d75cf9c871f98265fc033830f36565c08c06d3b9aa527434d26177f6dcdd66",
      "0x65e52f89dbdbedebdf875a131c315bdf22dca936c15cff9c5b14ff4613f7893",
      "0x4b126ef8f3eee62cbaf3df22221187cb4ff98247df5d042166fca7cae575d5c",
      "0x10fb59c0b6e040928dfd69557d05cbbf287d10a6a6560fc9651fbae58649561",
      "0x1e8909f416faacd4a08135f0b084d2e0e474d6885cde76bc3f8313cb5455ce7"
    ]
  },
  "id": 1
}
```