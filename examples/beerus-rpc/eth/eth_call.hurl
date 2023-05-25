POST http://0.0.0.0:3030
Content-Type: application/json
{
  "jsonrpc": "2.0",
  "method": "eth_call",
  "params": [
    {
      "to":"0xc2edad668740f1aa35e4d8f227fb8e17dca888cd",
      "data":"0x1526fe270000000000000000000000000000000000000000000000000000000000000001"
    },
    "latest"
  ],
  "id": 1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" matches /^0x[0-9a-f]*$/



POST http://0.0.0.0:3030
Content-Type: application/json
{
  "jsonrpc": "2.0",
  "method": "eth_call",
  "params": [
    {
      "to": "0xd46e8dd67c5d32be8058bb8eb970870f07244567",
      "gas": "0x76c0",
      "data": "0x"
    },
    "latest"
  ],
  "id": 1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" matches /^0x[0-9a-f]*$/
