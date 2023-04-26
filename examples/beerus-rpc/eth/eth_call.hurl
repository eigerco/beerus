POST http://0.0.0.0:3030
Content-Type: application/json
{
  "jsonrpc": "2.0",
  "method": "eth_call",
  "params": [
    {
      "to": "0xd46e8dd67c5d32be8058bb8eb970870f07244567",
      "gas": "0x00",
      "gasPrice": "0x09184e72a000",
      "value": "0x00",
      "data": "0x"
    },
    "latest"
  ],
  "id": 1
}

HTTP 200
[Asserts]
# jsonpath "$.error" not exists
# jsonpath "$.result" matches /^0x[A-Fa-f0-9]+$/
