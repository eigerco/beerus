POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc":"2.0",
    "method":"starknet_l2_to_l1_messages",
    "params": [
        "0x28164f78e696225e725b47da74019a17ccf6ff6b84107282b7eaaa6f3cfee4a4"
    ],
    "id":1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" matches /^0x[A-Fa-f0-9]+$/
