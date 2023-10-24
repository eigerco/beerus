POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc":"2.0",
    "method":"eth_getTransactionCount",
    "params":[
        "0xDAFEA492D9c6733ae3d56b7Ed1ADB60692c98Bc5",
        "latest"
    ],
    "id":1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" matches /^0x[A-Fa-f0-9]+$/
