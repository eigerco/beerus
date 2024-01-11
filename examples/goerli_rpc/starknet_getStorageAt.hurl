POST http://0.0.0.0:3030
Content-Type: application/json
{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "starknet_getStorageAt",
    "params": [
        "0x6d8e1f3ed72fc87aa896639a0f50a4b9e59adb24de8a42b477957e1a7996e1b",
        "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60",
        {
            "block_number": 929884
        }
    ]
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" exists
