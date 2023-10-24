POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc": "2.0",
    "method": "eth_getTransactionByBlockHashAndIndex",
    "params": [
        "0x9f2946a7816d37831accdadba832c34688c98ef9335cbb5f7a2bed4905a0e6bd",
        0
    ],
    "id": 1
}

HTTP 200
[Asserts]
jsonpath "$.error" exists
