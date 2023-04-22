POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc":"2.0","method":"eth_coinbase","params":[],"id":1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" == "0xe688b84b23f322a994a53dbf8e15fa82cdb71127"
