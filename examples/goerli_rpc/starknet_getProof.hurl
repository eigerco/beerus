POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc":"2.0",
    "method":"starknet_getProof",
    "params":[
        {
            "block_number": 354824
        },
        "0x049D36570D4e46f48e99674bd3fcc84644DdD6b96F7C741B1562B82f9e004dC7",
        [
            "0x2c401056f9582175d3219f1ac8f974b7960f2edfc8bc03197718dc8967ba1ab"
        ]
    ],
    "id":1
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result.state_commitment" exists
jsonpath "$.result.class_commitment" exists
jsonpath "$.result.contract_proof" exists
