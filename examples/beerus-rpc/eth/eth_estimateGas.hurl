POST http://0.0.0.0:3030
Content-Type: application/json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_estimateGas",
  "params": [
    {
      "to": "0xd46e8dd67c5d32be8058bb8eb970870f07244567",
      "gas": "0x10000000",
      "gasPrice": "0x09184e72a000",
      "value": "0x00",
      "data": "0x"
    }
  ]
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
# jsonpath "$.result" == "0x5a3c"
jsonpath "$.result" matches /^[0-9]+$/
