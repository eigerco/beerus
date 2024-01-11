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
                "0x41a78e741e5af2fec34b695679bc6891742439f7afb8484ecd7766661ad02bf",
                "0x1987cbd17808b9a23693d4de7e246a443cfe37e6e7fbaeabd7d7e6532b07c3d",
                "0x4",
                "0x6536a4f29f45a5db24cc0064f87930489f296c32f6b866493f8e0b287172173",
                "0x22528fbf84fcdd9bf0ff6884a6d91dc4abeff9fa536292087380320cd361cd7",
                "0x1",
                "0x0"
            ],
            "max_fee": "0x8205743843ee",
            "nonce": "0x2",
            "sender_address": "0x72a468a0546169acec984da9baad23464df46d8695313180eda9bf7f1a9fbc3",
            "signature": [
                "0x11b9f267426305e6cfaa01c8396084cc93bcb4c5ad4df58e0edec2ea5d9f36a",
                "0xe6e8521ab573cd411849d1f709947a24e28af16552ff294dfb0e67c63f7a77"
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
