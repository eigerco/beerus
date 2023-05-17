## starknet_getStateUpdate
Get the information about the result of executing the requested block.

### Parameters
The information about the state update of the requested block.

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
  "method": "starknet_getStateUpdate",
  "params": ["tag", "latest"],
  "id":1
}'
```

### Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "block_hash": "0x6fadc205742da06cf36d7ff9c89201423e538f7722425b3a095ba8ba9c58c7c",
    "new_root": "0x49b868883d8fe4a83ff7b2dee636a10f2f6ca6f8b2920f03057c331b9494f3a",
    "old_root": "0x7999b08b5292c450c149031655ca7b48072d79bb2c623c8ac22c8bf238c94a0",
    "state_diff": {
      "storage_diffs": [
        {
          "address": "0x480258f58d43fb73936f803780047a0f6d0a563697d80bd3f95b603f9c8b1c8",
          "storage_entries": [
            {
              "key": "0x5e902bd23ef21d327d31628321c5c482279a9d3f1646280ff1290f8dd81be79",
              "value": "0x1"
            },
            {
              "key": "0x74386bd0bfdede1f3838b1201b89a277c1170d8bd07772279daeb5e74ea1ae6",
              "value": "0x1"
            },
            {
              "key": "0x196e1fdd019993b7cc8ec7054e421b349b5becc1a8bddec778d4e02f6b0c7ca",
              "value": "0x1"
            },
            {
              "key": "0x68fbfc0640fa293d4ae4cd5e6eb170e38709a1916f4ea566c7e56055cc80a2f",
              "value": "0x1"
            }
          ]
        }
      ],
      "declared_contract_hashes": [],
      "deployed_contracts": [
        {
          "address": "0x4d47a7762ad159f472ee7c0c2d0d566b99f71c7fa2ee1682b0a88587a87d5dc",
          "class_hash": "0x2a5de1b145e18dfeb31c7cd7ff403714ededf5f3fdf75f8b0ac96f2017541bc"
        },
        {
          "address": "0xbee6f0a4cb9b1ff4da5ce71262db9b9e461b98677642e87257c40b1cd8fdd8",
          "class_hash": "0x2a5de1b145e18dfeb31c7cd7ff403714ededf5f3fdf75f8b0ac96f2017541bc"
        }
      ],
      "nonces": [
        {
          "contract_address": "0x308e9c7b51e9d3f648bcd413c5717cb3d3783e74da49b9a0e7b90353bfc2e5b",
          "nonce": "0x17"
        }
      ]
    }
  },
  "id": 1
}
```