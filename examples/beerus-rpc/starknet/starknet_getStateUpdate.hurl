POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc": "2.0",
    "method": "starknet_getStateUpdate",
    "params": [
        "latest"
    ],
    "id": 0
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" exists
