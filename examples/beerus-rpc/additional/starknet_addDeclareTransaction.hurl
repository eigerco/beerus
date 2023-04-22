POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc":"2.0","method":"starknet_addDeclareTransaction","params":["0x0001"],"id":1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
