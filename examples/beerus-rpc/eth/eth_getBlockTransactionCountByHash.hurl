POST http://0.0.0.0:3030
Content-Type: application/json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getBlockByNumber",
  "params": ["latest", true]
}

HTTP 200
[Captures]
block_hash: jsonpath "$.result.hash"

POST http://0.0.0.0:3031
Content-Type: application/json
{
    "jsonrpc":"2.0",
    "method":"eth_getBlockTransactionCountByHash",
    "params":["{{block_hash}}"],
    "id":1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" matches /^0x[A-Fa-f0-9]+$/
