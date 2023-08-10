POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc": "2.0",
    "method": "starknet_addDeployAccountTransaction",
    "params": {
        "deploy_account_transaction": {
            "type": "DEPLOY_ACCOUNT",
            "max_fee": "0xffffffffffffffff",
            "version": "0x1",
            "signature": [
                "0x7dd3a55d94a0de6f3d6c104d7e6c88ec719a82f4e2bbc12587c8c187584d3d5",
                "0x71456dded17015d1234779889d78f3e7c763ddcfd2662b19e7843c7542614f8"
            ],
            "nonce": "0x0",
            "contract_address_salt": "0x6d44a6aecb4339e23a9619355f101cf3cb9baec289fcd9fd51486655c1bb8a8",
            "constructor_calldata": ["0x677bb1cdc050e8d63855e8743ab6e09179138def390676cc03c484daf112ba1"],
            "class_hash": "0x1fac3074c9d5282f0acc5c69a4781a1c711efea5e73c550c5d9fb253cf7fd3d"
        }
    },
    "id": 0
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
