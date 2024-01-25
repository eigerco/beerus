POST http://0.0.0.0:3030
Content-Type: application/json
{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "starknet_call",
    "params": [
        {
            "calldata": [],
            "contract_address": "0x6d8e1f3ed72fc87aa896639a0f50a4b9e59adb24de8a42b477957e1a7996e1b",
            "entry_point_selector": "0x293c9b0657d7591853c62ddc495b09ff833e04ad61f066dd7c8cc3a5b6b303d"
        },
        "pending"
    ]
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result" exists
