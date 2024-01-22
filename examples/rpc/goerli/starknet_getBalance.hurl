POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc": "2.0",
    "method": "starknet_getBalance",
    "params": [
        "latest",
        "0x88aede3875549767ad776d7e231240c3ef0eb27e036bf4ceccd4d1d7cdba90"
    ],
    "id":1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" exists
