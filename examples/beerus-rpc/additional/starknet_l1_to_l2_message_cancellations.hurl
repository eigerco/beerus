POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc":"2.0",
    "method":"starknet_l1_to_l2_message_cancellations",
    "params":[
        "0x174b734e268c567806bc9b275e7076941f90ec7df001ca7b997c0c84c83f89d5"
    ],
    "id":1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" matches /^0x[A-Fa-f0-9]+$/
