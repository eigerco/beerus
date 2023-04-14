POST http://0.0.0.0:3031
Content-Type: application/json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getLogs",
  "params": [
    {
      "address": [
        "0xb59f67a8bff5d8cd03f6ac17265c550ed8f33907"
      ],
      "fromBlock": "0x103b659",
      "toBlock": "latest"
    }
  ]
}