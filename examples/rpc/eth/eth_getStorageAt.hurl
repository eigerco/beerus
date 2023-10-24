POST http://0.0.0.0:3030
Content-Type: application/json
{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "eth_getStorageAt",
    "params": [
        "0xC18360217D8F7Ab5e7c516566761Ea12Ce7F9D72",
        "0x0000000000000000000000000000000000000000000000000000000000000000",
        "latest"
    ]
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
