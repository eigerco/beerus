## JSON-RPC API Methods

Based on this specification:
[starkware-libs/starknet-specs](https://github.com/starkware-libs/starknet-specs)

### Method Implementation State

- ❌ -> TODO
- ⚠️ -> Logic created, to be verified
- ⏳ -> Logic verified, being implemented
- ✅ -> Implemented
- 🟡 -> Not respecting the specification
- ❎ -> Unsupported method (e.g. PoW specific methods, deprecated methods, etc.)

### Contribute

The template for the method file can be found
[here](./contributing/method_template.md) copy it to the new method file and
edit it corresponding to the method you're implementing. All methods should be
documented in `docs/beerus-rpc/methods/{method}.md`

| Name                                                                                            | Description                                                                                    | State |
|-------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------|-------|
| [starknet_getBlockWithTxHashes](methods/starknet_getBlockWithTxHashes.md)                       | Get block information with transaction hashes given the block id.                              | ✅     |
| [starknet_getBlockWithTxs](methods/starknet_getBlockWithTxs.md)                                 | Get block information with full transactions given the block id.                               | ✅     |
| [starknet_getStateUpdate](methods/starknet_getStateUpdate.md)                                   | Get the information about the result of executing the requested block.                         | ✅     |
| [starknet_getStorageAt](methods/starknet_getStorageAt.md)                                       | Get the value of the storage at the given address and key.                                     | ❌     |
| [starknet_getTransactionByHash](methods/starknet_getTransactionByHash.md)                       | Get the details and status of a submitted transaction.                                         | ❌     |
| [starknet_getTransactionByBlockIdAndIndex](methods/starknet_getTransactionByBlockIdAndIndex.md) | Get the details of a transaction by a given block id and index.                                | ✅     |
| [starknet_getTransactionReceipt](methods/starknet_getTransactionReceipt.md)                     | Get the details of a transaction by a given block number and index.                            | ✅     |
| [starknet_getClass](methods/starknet_getClass.md)                                               | Get the contract class definition in the given block associated with the given hash.           | ✅     |
| [starknet_getClassHashAt](methods/starknet_getClassHashAt.md)                                   | Get the contract class hash in the given block for the contract deployed at the given address. | ✅     |
| [starknet_getClassAt](methods/starknet_getClassAt.md)                                           | Get the contract class definition in the given block at the given address.                     | ✅     |
| [starknet_getBlockTransactionCount](methods/starknet_getBlockTransactionCount.md)               | Get the number of transactions in a block given a block id.                                    | ✅     |
| [starknet_call](methods/starknet_call.md)                                                       | Call a starknet function without creating a Starknet transaction.                              | ❌     |
| [starknet_estimateFee](methods/starknet_estimateFee.md)                                         | Estimate the fee for a given Starknet transaction.                                             | ❌     |
| [starknet_blockNumber](methods/starknet_blockNumber.md)                                         | Get the most recent accepted block number.                                                     | ✅     |
| [starknet_blockHashAndNumber](methods/starknet_blockHashAndNumber.md)                           | Get the most recent accepted block hash and number.                                            | ✅     |
| [starknet_chainId](methods/starknet_chainId.md)                                                 | Return the currently configured Starknet chain id.                                             | ✅     |
| [starknet_pendingTransactions](methods/starknet_pendingTransactions.md)                         | Returns the transactions in the transaction pool, recognized by this sequencer.                | ✅     |
| [starknet_syncing](methods/starknet_syncing.md)                                                 | Returns an object about the sync status, or false if the node is not synching.                 | ✅     |
| [starknet_getEvents](methods/starknet_getEvents.md)                                             | Returns all events matching the given filter.                                                  | ✅     |
| [starknet_getNonce](methods/starknet_getNonce.md)                                               | Get the nonce associated with the given address in the given block.                            | ✅     |
| [starknet_addInvokeTransaction](methods/starknet_addInvokeTransaction.md)                       | Submit a new transaction to be added to the chain.                                             | ❌     |
| [starknet_addDeclareTransaction](methods/starknet_addDeclareTransaction.md)                     | Submit a new transaction to be added to the chain.                                             | ✅     |
| [starknet_addDeployTransaction](methods/starknet_addDeployTransaction.md)                       | Submit a new deploy contract transaction.                                                      | ✅     |
| [starknet_addDeployAccountTransaction](methods/starknet_addDeployAccountTransaction.md)         | Submit a new deploy account transaction.                                                       | ❌     |