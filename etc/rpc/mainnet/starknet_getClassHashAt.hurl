POST http://0.0.0.0:3030
Content-Type: application/json
{
  "jsonrpc": "2.0",
  "method": "starknet_getClassHashAt",
  "params": [
    {
      "block_number": 22050
    },
    "0x07f38ab7537dbb5f8dc2d049d441f2b250c2186a13d943b8467cfa86b8dba12b"
  ],
  "id": 1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" matches /^0x[A-Fa-f0-9]+$/
