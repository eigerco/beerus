POST http://0.0.0.0:3031
Content-Type: application/json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getLogs",
  "params": [
    {
      "address": [
        "0x000000000000Ad05Ccc4F10045630fb830B95127"
      ],
      "fromBlock": "17044003",
      "toBlock": "latest"
    }
  ]
}