POST http://0.0.0.0:3030
Content-Type: application/json
{
  "jsonrpc": "2.0",
  "method": "starknet_getEvents",
  "params": {
    "custom_filter": {
      "filter": {
      "from_block": {"block_number": 800},
      "to_block": {"block_number": 1701},
        "address": "0x049D36570D4e46f48e99674bd3fcc84644DdD6b96F7C741B1562B82f9e004dC7"
      },
      "page": {
        "chunk_size": 10
      }
    }
  },
  "id": 1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" exists
