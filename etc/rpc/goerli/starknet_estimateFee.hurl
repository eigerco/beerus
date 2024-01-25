POST http://0.0.0.0:3030
Content-Type: application/json
{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "starknet_estimateFee",
    "params": [
        [
            {
                "calldata": [
                    "0x1",
                    "0x6d8e1f3ed72fc87aa896639a0f50a4b9e59adb24de8a42b477957e1a7996e1b",
                    "0x166d775d0cf161f1ce9b90698485f0c7a0e249af1c4b38126bddb37859737ac",
                    "0x0",
                    "0x1",
                    "0x1",
                    "0x737461726b6e6574"
                ],
                "max_fee": "0x0",
                "nonce": "0x5",
                "sender_address": "0x179eaac35ef5370f1c997ed1ff088b0e52f7fec7ef38c444e9171f895e41207",
                "signature": [
                    "0x2d1ab2a39b7356d2e71efdf7a4a1a47446de9ad7f23ceef00f7df00ca01df69",
                    "0x7d60831c0e4fc24fccedd2b6a0e729416c83d15accbda3f8db76f374a40b70c"
                ],
                "type": "INVOKE",
                "version": "0x100000000000000000000000000000001"
            }
        ],
        [],
        {
            "block_number": 930199
        }
    ]
}
HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result[0].gas_price" exists
jsonpath "$.result[0].overall_fee" exists
