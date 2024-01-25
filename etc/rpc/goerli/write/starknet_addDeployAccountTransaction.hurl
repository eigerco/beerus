POST http://0.0.0.0:3030
Content-Type: application/json
{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "starknet_addDeployAccountTransaction",
    "params": [
        {
            "class_hash": "0x4c6d6cf894f8bc96bb9c525e6853e5483177841f7388f74a46cfda6f028c755",
            "constructor_calldata": [
                "0x2702d892c77d9ff2c5aaeede5e01c4d9679fc6e6f11da5e17e96aa7163ba034"
            ],
            "contract_address_salt": "0x11aaa08540d92acb434c5b7769e9aab1433d2677f538ccdf1046bdda7750375",
            "max_fee": "0x60b824d6a1b",
            "nonce": "0x0",
            "signature": [
                "0x7c497e30256ba415efdc5bf60a65a7f04eec8a58e135c6461232b66e4f00b58",
                "0x3daff44c5a2ab084778d314d1a6b724ab4bb70852b85b2b6ba2a98fcacdb4fa"
            ],
            "type": "DEPLOY_ACCOUNT",
            "version": "0x1"
        }
    ]
}
HTTP 200
[Asserts]
jsonpath "$.error.code" == 59
# "code":59,"message":"A transaction with the same hash already exists in the mempool"
