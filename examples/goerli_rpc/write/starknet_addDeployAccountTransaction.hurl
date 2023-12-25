POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc": "2.0",
    "method": "starknet_addDeployAccountTransaction",
    "params": {
         "deploy_account_transaction": {
             "type": "DEPLOY_ACCOUNT",
             "version": "0x1",
             "max_fee": "0xffffffffffffffff",
             "signature": [],
             "contract_address_salt": "0x1234",
             "constructor_calldata": [],
             "nonce": "0x0",
             "class_hash": "0x711c0c3e56863e29d3158804aac47f424241eda64db33e2cc2999d60ee5105"
        }
    },
    "id": 0
}

HTTP 200
[Asserts]
jsonpath "$.error" not exists
