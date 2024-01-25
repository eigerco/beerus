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
                "0x1cacce8218959a6d1bc3a18d64cc4ce1571e51c6feaa4348b268a7fcb76f173"
            ],
            "contract_address_salt": "0x7814c588af860ff66762baa0c2086f5be50959bba6183ee4793255ad6aa8e7f",
            "max_fee": "0xf527e62b4e50",
            "nonce": "0x0",
            "signature": [
                "0xaf69c7148948ca0e1554e211a6541438e5ceda50215066ca3f531ec09b223",
                "0x6903e1b7550ba3a181b8b1c3a8050514defb70c85b4fa970b6c7f57bc20e092"
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
