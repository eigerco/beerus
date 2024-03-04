POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc": "2.0",
    "method": "starknet_getEvents",
    "params": {
        "filter": {
            "from_block": {
                "block_number": 800
            },
            "to_block": {
                "block_number": 1701
            },
            "keys": [
                [
                    "0x32152e6067d0b8e3ff1aea0749afa5a823e4646d1663a7167c0a92d21d256eb",
                    "0xb83f204dcb21221bd5ef3d70b06988cd4beaa4d48ff498a1a0c9e1e49e2804"
                ],
                [],
                [
                    "0x599573a0023b0ce23bb8537d2c5f3e9b72ebd4f008d033b4c0a1475a68338ac",
                    "0x47a4c36c2932f213e7b054c9573e49a6681d5b710d1b25a96ea6346f5689929"
                ]
            ],
            "chunk_size": 1
        }
    },
    "id": 0
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
jsonpath "$.result.events" exists
