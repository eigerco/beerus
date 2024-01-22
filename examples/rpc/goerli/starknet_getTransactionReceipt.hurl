POST http://0.0.0.0:3030
Content-Type: application/json
{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "starknet_getTransactionReceipt",
    "params": [
        "0x71f6f2272d068e6d73f92fcddb53555ffb923fdef395b0d2716348145f70a45"
    ]
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result.transaction_hash" matches /^0x[A-Fa-f0-9]+$/
