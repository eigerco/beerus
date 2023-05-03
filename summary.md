## Hurl Test Execution Summary

This is a summary of the different methods tested.

### Failing Methods

| Group | Method | Status |
| --- | --- | --- |
| eth | eth_getStorageAt | :x: |
| eth | eth_sendRawTransaction | :x: |
| eth | eth_call | :x: |
| eth | eth_getBlockTransactionCountByHash | :x: |
| additional | starknet_addDeployAccountTransaction | :x: |
| additional | starknet_addDeclareTransaction | :x: |
| additional | starknet_getContractStorageProof | :x: |
| additional | starknet_addInvokeTransaction | :x: |
| starknet | starknet_getEvents | :x: |

### Succeeding Methods

| Group | Method | Status |
| --- | --- | --- |
| eth | eth_getTransactionReceipt | :heavy_check_mark: |
| eth | eth_getTransactionCount | :heavy_check_mark: |
| eth | eth_syncing | :heavy_check_mark: |
| eth | eth_getBalance | :heavy_check_mark: |
| eth | eth_getCode | :heavy_check_mark: |
| eth | eth_gasPrice | :heavy_check_mark: |
| eth | eth_getLogs | :heavy_check_mark: |
| eth | eth_getTransactionByBlockHashAndIndex | :heavy_check_mark: |
| eth | eth_coinbase | :heavy_check_mark: |
| eth | eth_getBlockTransactionCountByNumber | :heavy_check_mark: |
| eth | eth_chainId | :heavy_check_mark: |
| eth | eth_blockNumber | :heavy_check_mark: |
| eth | eth_getTransactionByHash | :heavy_check_mark: |
| eth | eth_getBlockByHash | :heavy_check_mark: |
| eth | eth_getBlockByNumber | :heavy_check_mark: |
| eth | eth_maxPriorityFeePerGas | :heavy_check_mark: |
| eth | eth_estimateGas | :heavy_check_mark: |
| additional | starknet_l1_to_l2_message_nonce | :heavy_check_mark: |
| additional | starknet_l1_to_l2_messages | :heavy_check_mark: |
| additional | starknet_l2_to_l1_messages | :heavy_check_mark: |
| additional | starknet_l1_to_l2_message_cancellations | :heavy_check_mark: |
| starknet | starknet_getClassHashAt | :heavy_check_mark: |
| starknet | starknet_getClassAt | :heavy_check_mark: |
| starknet | starknet_chainId | :heavy_check_mark: |
| starknet | starknet_getEstimateFee | :heavy_check_mark: |
| starknet | starknet_getClass | :heavy_check_mark: |
| starknet | starknet_getTransactionByHash | :heavy_check_mark: |
| starknet | starknet_getBlockTransactionCount | :heavy_check_mark: |
| starknet | starknet_getTransactionByBlockIdAndIndex | :heavy_check_mark: |
| starknet | starknet_getStorageAt | :heavy_check_mark: |
| starknet | starknet_blockNumber | :heavy_check_mark: |
| starknet | starknet_getBlockWithTxs | :heavy_check_mark: |
| starknet | starknet_blockHashAndNumber | :heavy_check_mark: |
| starknet | starknet_syncing | :heavy_check_mark: |
| starknet | starknet_getNonce | :heavy_check_mark: |
| starknet | starknet_call | :heavy_check_mark: |
| starknet | starknet_pendingTransactions | :heavy_check_mark: |
| starknet | starknet_getStateUpdate | :heavy_check_mark: |
| starknet | starknet_getBlockWithTxHashes | :heavy_check_mark: |
| starknet | starknet_getTransactionReceipt | :heavy_check_mark: |
