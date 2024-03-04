POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc": "2.0",
    "method": "starknet_estimateFee",
    "params": {
        "request": [{
            "type": "INVOKE",
            "max_fee": "0x28ed6103d0000",
            "version": "0x1",
            "signature": [
                "0x42527ffe9912b338983cbed67e139cfcc26a4d8cf1d1c2a85e4125fdf5f59ed",
                "0x636147d06fefd02ed37984b752556d4b9aefdac1a50b3df0528ec7c201ad84b"
            ],
            "sender_address": "0x13e3ca9a377084c37dc7eacbd1d9f8c3e3733935bcbad887c32a0e213cd6fe0",
            "calldata": [
                "0x2",
                "0x57c4b510d66eb1188a7173f31cccee47b9736d40185da8144377b896d5ff3",
                "0x2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
                "0x0",
                "0x1",
                "0x57c4b510d66eb1188a7173f31cccee47b9736d40185da8144377b896d5ff3",
                "0x2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
                "0x1",
                "0x1",
                "0x2",
                "0x0",
                "0x1"
            ],
            "nonce": "0x1"
        }],
        "simulation_flags": [],
        "block_id": {
            "block_number": 59999
        }
    },
    "id": 0
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result[0].gas_price" exists
jsonpath "$.result[0].overall_fee" exists
