POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc": "2.0",
    "method": "eth_getTransactionByHash",
    "params": [
        "0x9d595806f98aaa70a04497079ccec9019d614148137174cde11480de3a8bf229"
    ],
    "id": 1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" exists
