POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc":"2.0","method":"starknet_addInvokeTransaction","params":[],"id":1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
