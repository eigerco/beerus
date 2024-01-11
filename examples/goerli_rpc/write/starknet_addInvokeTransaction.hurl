POST http://0.0.0.0:3030
Content-Type: application/json
{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "starknet_addInvokeTransaction",
    "params": [
        {
            "calldata": [
                "0x1",
                "0x6d8e1f3ed72fc87aa896639a0f50a4b9e59adb24de8a42b477957e1a7996e1b",
                "0x166d775d0cf161f1ce9b90698485f0c7a0e249af1c4b38126bddb37859737ac",
                "0x1",
                "0x737461726b6e6574"
            ],
            "max_fee": "0x24b6e85308e",
            "nonce": "0x2",
            "sender_address": "0x88aede3875549767ad776d7e231240c3ef0eb27e036bf4ceccd4d1d7cdba90",
            "signature": [
                "0x282e04a1949829f8dd4ba4b70e7a915622d426d0478d21f4725521a7a2b193",
                "0x346bf9d55d7f7969b6c88a0b77fe33f28651695aa00a20a8a56c68370dd657a"
            ],
            "type": "INVOKE",
            "version": "0x1"
        }
    ]
}
HTTP 200
[Asserts]
jsonpath "$.error.code" == 59
# "code":59,"message":"A transaction with the same hash already exists in the mempool"
