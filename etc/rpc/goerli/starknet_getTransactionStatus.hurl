POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc": "2.0",
    "method": "starknet_getTransactionStatus",
    "params": [
        "0x71f6f2272d068e6d73f92fcddb53555ffb923fdef395b0d2716348145f70a45"
    ],
    "id":1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists

