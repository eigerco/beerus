## starknet_pendingTransactions
Returns the transactions in the transaction pool, recognized by this sequencer.

### Parameters

### Returns

### Headers
```rust
Content-Type: application/json
```

### Example 
```bash 
curl -X POST http://localhost:3030 \
-H "Content-Type: application/json" \
-d '{
  "jsonrpc":"2.0",
  "method":"starknet_pendingTransactions",
  "params":[],
  "id":0
}'
```

### Response
```json
{
  "jsonrpc": "2.0",
  "result": [
    {
      "transaction_hash": "0x77bc31ba5f3060794c4a87fb1ff91a9228730eb7e3f3d7631a63c96590904ac",
      "type": "INVOKE",
      "max_fee": "0x4e8e1d65fa860",
      "version": "0x1",
      "signature": [
        "0x265a0100df301936361d3a23d962bc99aa10640eb33378d92ffd53026d751e2",
        "0x3f8b8b2ba97233dc8b1ece9ef6078ed433a7496f5b60d958bd0499be8385e5b"
      ],
      "nonce": "0x1f",
      "sender_address": "0x3000dcaa29950849dff602bff06751ebf41760debfba97d85e2cc86be6f23ed",
      "calldata": [
        "0x1",
        "0x1435498bf393da86b4733b9264a86b58a42b31f8d8b8ba309593e5c17847672",
        "0x2f2e26c65fb52f0e637c698caccdefaa2a146b9ec39f18899efe271f0ed83d3",
        "0x0",
        "0x10",
        "0x10",
        "0x3000dcaa29950849dff602bff06751ebf41760debfba97d85e2cc86be6f23ed",
        "0xde3ddb848f7f40bab6fb1c8de68c7a68",
        "0x1",
        "0x76696e65636879",
        "0x2",
        "0x4120736574206d616465206f662062",
        "0x726971732076696e65636879",
        "0x1",
        "0x1",
        "0x1",
        "0x0",
        "0x1",
        "0x233339343138330000000000000000000000000000000001",
        "0x7ffffffffffffffe80000000000000008000000000000000",
        "0x1",
        "0x2"
      ]
    },
    {
      "transaction_hash": "0x71d357487b3f2e6b5521d5bca772a49a7a3a0e30c3d7034002e35a10f757fa9",
      "type": "DEPLOY_ACCOUNT",
      "max_fee": "0x191f13c554e56",
      "version": "0x1",
      "signature": [
        "0x35f2b4ea197a5e5563caa63e1a6028ccf31dd62fdba6b0751b76f683d62c89d",
        "0x308f58fd082ba8f032e9fc772b76405a7ee84a953fba539c570e3c459ba6ce6"
      ],
      "nonce": "0x0",
      "contract_address_salt": "0x7b42265eb896ce998af4f4f6885f34e1970fc151a79092f9ef713236192b57c",
      "constructor_calldata": [
        "0x33434ad846cdd5f23eb73ff09fe6fddd568284a0fb7d1be20ee482f044dabe2",
        "0x79dc0da7c54b95f10aa182ad0a46400db63156920adb65eca2654c0945a463",
        "0x2",
        "0x7b42265eb896ce998af4f4f6885f34e1970fc151a79092f9ef713236192b57c",
        "0x0"
      ],
      "class_hash": "0x25ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918"
    },
    {
      "transaction_hash": "0x2baaac81c63b9300818852931b7281be8665ec9263b6cdae997ad21234b3b5b",
      "type": "INVOKE",
      "max_fee": "0x70dec93d06970",
      "version": "0x1",
      "signature": [
        "0x6bdf30bd89accd3cc19a5d1ff509d827d457e822a945046ad83ad2f487a80ab",
        "0x2f88ba6d6a47fd3687a639926ff1fd194011baacb5183f12f8f448910d1019d"
      ],
      "nonce": "0x1",
      "sender_address": "0x3618571270431f216e9c2171e1c663fcf7aef366fb86a78bb3ba1e043b29e62",
      "calldata": [
        "0x2",
        "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        "0x219209e083275171774dab1df80982e9df2096516f06319c5c6d71ae0a8480c",
        "0x0",
        "0x3",
        "0x7a6f98c03379b9513ca84cca1373ff452a7462a3b61598f0af5bb27ad7f76d1",
        "0x2c0f7bf2d6cf5304c29171bf493feb222fef84bdaf17805a6574b0c2e8bcc87",
        "0x3",
        "0x9",
        "0xc",
        "0x7a6f98c03379b9513ca84cca1373ff452a7462a3b61598f0af5bb27ad7f76d1",
        "0xf5232269808000",
        "0x0",
        "0xf5232269808000",
        "0x0",
        "0x7a7ac30",
        "0x0",
        "0x2",
        "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        "0x68f5c6a61780768455de69077e07e89787839bf8166decfbf92b645209c0fb8",
        "0x3618571270431f216e9c2171e1c663fcf7aef366fb86a78bb3ba1e043b29e62",
        "0x6439a05f"
      ]
    }
  ],
  "id": 1
}
```