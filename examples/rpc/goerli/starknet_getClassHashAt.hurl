POST http://0.0.0.0:3030
Content-Type: application/json
{
  "jsonrpc": "2.0",
  "method": "starknet_getClassHashAt",
  "params": [
    {
      "block_number": 930000
    },
    "0x040688250Ef0074B4c9e1057B19F9b62139ac28179c7d35e2daE5abAD909d558"
  ],
  "id": 1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" matches /^0x[A-Fa-f0-9]+$/
