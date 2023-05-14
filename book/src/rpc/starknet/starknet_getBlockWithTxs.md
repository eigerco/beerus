## starknet_getBlockWithTxs
Get block information with full transactions given the block id.

### Parameters
`BLOCK_PARAM` - Expected one of `block_number`, `block_hash`, `latest`, `pending`.

### Returns
The resulting block information with full transactions.

### Headers
```rust
Content-Type: application/json
```

### Example
```bash
curl -X POST http://localhost:3030 \
-H "Content-Type: application/json" \
-d '{
  "jsonrpc": "2.0",
  "method": "starknet_getBlockWithTxs",
  "params": ["tag", "latest"],
  "id": 1
}'
```

### Response
```json
{
   "jsonrpc":"2.0",
   "result":{
      "status":"ACCEPTED_ON_L2",
      "block_hash":"0x4316d196c29b6e3d7da351e5016111ce616be280d8b0f5bcd6468fea2a88a9a",
      "parent_hash":"0x789d5f5ebd812b84cdcbd026e12e77390e99ebab4ca5b39f7ad16a19114f169",
      "block_number":37384,
      "new_root":"0x68bbdfc44482576ef65b951a25a168f0c63303b93721059bd0363223e5458f6",
      "timestamp":1681307821,
      "sequencer_address":"0x1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8",
      "transactions":[
         {
            "transaction_hash":"0x215f21f6f3576ca87f406d1b9567bcaaf679a1a0a3b4cc293c08341818ccebd",
            "type":"INVOKE",
            "max_fee":"0x1eec3dec20000",
            "version":"0x1",
            "signature":[
               "0x2b37a53a3ac35da626cbed9d127feaea6e415146d345d5994b493c246d18486",
               "0x602629d79826cbadd3f1ab23f73c837ba9c85e13dca8e435619794fb9444a0d"
            ],
            "nonce":"0x6",
            "sender_address":"0x38ca57110d6636258a000e5e8fd4de7dda47c507af851c934899f7904c7829b",
            "calldata":[
               "0x3",
               "0xda114221cb83fa859dbdb4c44beeaa0bb37c7537ad5ae66fe5e0efd20e6eb3",
               "0x219209e083275171774dab1df80982e9df2096516f06319c5c6d71ae0a8480c",
               "0x0",
               "0x3",
               "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
               "0x219209e083275171774dab1df80982e9df2096516f06319c5c6d71ae0a8480c",
               "0x3",
               "0x3",
               "0x28c858a586fa12123a1ccb337a0a3b369281f91ea00544d0c086524b759f627",
               "0x3f35dbce7a07ce455b128890d383c554afbc1b07cf7390a13e2d602a38c1a0a",
               "0x6",
               "0xd",
               "0x13",
               "0x28c858a586fa12123a1ccb337a0a3b369281f91ea00544d0c086524b759f627",
               "0x56bc75e2d63100000",
               "0x0",
               "0x28c858a586fa12123a1ccb337a0a3b369281f91ea00544d0c086524b759f627",
               "0xbd438133a24865",
               "0x0",
               "0xda114221cb83fa859dbdb4c44beeaa0bb37c7537ad5ae66fe5e0efd20e6eb3",
               "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
               "0x0",
               "0x56bc75e2d63100000",
               "0x0",
               "0xbd438133a24865",
               "0x0",
               "0x55005f0c614480000",
               "0x0",
               "0xb97a797f66b796",
               "0x0",
               "0x38ca57110d6636258a000e5e8fd4de7dda47c507af851c934899f7904c7829b",
               "0x6436c5ba"
            ]
         },
         {
            "transaction_hash":"0x61567eac6f9019b15304ef39d40027d73a67f30494c1f76ef1e0059288b7634",
            "type":"INVOKE",
            "max_fee":"0x337a8ecd5ea78",
            "version":"0x1",
            "signature":[
               "0x7c216890b0ebb49b41abc1a9c52f3cde0f588076690d8f6515296606ef8a848",
               "0x62135924763938bb94122e8a794a1d9c3ff17f1b93d77192c1bc3bb40ad2693"
            ],
            "nonce":"0x14",
            "sender_address":"0xb66196036c2e4568f01ec4d65a32a5f2254d06b65483c97de47ca8bb11b587",
            "calldata":[
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
               "0x18de76816d8000",
               "0x0",
               "0x18de76816d8000",
               "0x0",
               "0xc7a2e4",
               "0x0",
               "0x2",
               "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
               "0x53c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8",
               "0xb66196036c2e4568f01ec4d65a32a5f2254d06b65483c97de47ca8bb11b587",
               "0x6439599e"
            ]
         }
      ]
   },
   "id":1
}
```