use beerus::gen::{Address, BlockId, BlockTag, Felt, Rpc, TxnHash};

use common::{
    dummies::{
        declare_transaction, deploy_transaction, estimate_fee_transaction,
        invoke_transaction,
    },
    matchers::StarknetMatcher::{
        self, AddDeclareTransaction, AddDeclareTransactionMalicious,
        AddDeployAccountTransaction, AddInvokeTransaction, ChainId,
        ChainIdMalicious, ClassError, ClassMalicious, ClassSuccess,
        EstimateFee, EstimateFeeMalicious, GetTransactionReceipt, Nonce,
        NonceMalicious, SpecVersion, SpecVersionMalicious,
    },
    node::setup_client_with_mock_starknet_node,
};

mod common;

#[tokio::test]
async fn chain_id_test() {
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(vec![ChainId]).await;
    let result = client.chainId().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn chain_id_nonce() {
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(vec![ChainId, Nonce]).await;
    assert!(client.chainId().await.is_ok());
    assert!(client
        .getNonce(
            BlockId::BlockTag(BlockTag::Latest),
            Address(Felt::try_new("0x0").unwrap())
        )
        .await
        .is_ok())
}

#[tokio::test]
async fn chain_id_called_twice() {
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(vec![ChainId, ChainId]).await;
    assert!(client.chainId().await.is_ok());
    assert!(client.chainId().await.is_ok());
}

#[tokio::test]
async fn get_class_error() {
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(vec![ClassError]).await;
    assert!(client
        .getClass(
            BlockId::BlockTag(BlockTag::Latest),
            Felt::try_new("0x0").unwrap()
        )
        .await
        .is_err());
}

#[tokio::test]
async fn get_class_success() {
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(vec![ClassSuccess]).await;
    assert!(client
        .getClass(
            BlockId::BlockTag(BlockTag::Latest),
            Felt::try_new("0x0").unwrap()
        )
        .await
        .is_ok());
}

#[tokio::test]
async fn spec_version_estimate_fee() {
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(vec![SpecVersion, EstimateFee])
            .await;
    let tx = estimate_fee_transaction();
    assert!(client.specVersion().await.is_ok());
    let res = client
        .estimateFee(vec![tx], vec![], BlockId::BlockTag(BlockTag::Latest))
        .await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn add_declare_transaction() {
    let tx = declare_transaction();
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(vec![AddDeclareTransaction]).await;
    assert!(client.addDeclareTransaction(tx).await.is_ok());
}

#[tokio::test]
async fn declare_account() {
    let (client, _starknet_node) = setup_client_with_mock_starknet_node(vec![
        ChainId,
        ClassError,
        ChainId,
        Nonce,
        SpecVersion,
        EstimateFee,
        AddDeclareTransaction,
    ])
    .await;
    let block_id = BlockId::BlockTag(BlockTag::Latest);
    let class_hash = Felt::try_new("0x0").unwrap();
    let contract_address = Address(class_hash.clone());
    let estimate_tx = estimate_fee_transaction();
    let declare_tx = declare_transaction();

    assert!(client.chainId().await.is_ok());
    assert!(client.getClass(block_id.clone(), class_hash).await.is_err());
    assert!(client.chainId().await.is_ok());
    assert!(client.getNonce(block_id.clone(), contract_address).await.is_ok());
    assert!(client.specVersion().await.is_ok());
    assert!(client
        .estimateFee(vec![estimate_tx], vec![], block_id)
        .await
        .is_ok());
    assert!(client.addDeclareTransaction(declare_tx).await.is_ok());
}

#[tokio::test]
async fn malicious_data_results_in_err() {
    let (client, _starknet_node) = setup_client_with_mock_starknet_node(vec![
        AddDeclareTransactionMalicious,
        ChainIdMalicious,
        ClassMalicious,
        EstimateFeeMalicious,
        NonceMalicious,
        SpecVersionMalicious,
    ])
    .await;
    let block_id = BlockId::BlockTag(BlockTag::Latest);
    let class_hash = Felt::try_new("0x0").unwrap();
    let contract_address = Address(class_hash.clone());
    let declare_tx = declare_transaction();
    let estimate_tx = estimate_fee_transaction();

    assert!(client.addDeclareTransaction(declare_tx).await.is_err());
    assert!(client.chainId().await.is_err());
    assert!(client
        .estimateFee(vec![estimate_tx], vec![], block_id.clone())
        .await
        .is_err());
    assert!(client.getClass(block_id.clone(), class_hash).await.is_err());
    assert!(client.getNonce(block_id, contract_address).await.is_err());
    assert!(client.specVersion().await.is_err());
}

#[tokio::test]
async fn deploy_account() {
    let mut account_deploy: Vec<StarknetMatcher> =
        vec![ChainId, Nonce, SpecVersion, EstimateFee];
    let mut invoke_eth_transfer: Vec<StarknetMatcher> = vec![
        ChainId,
        ChainId,
        Nonce,
        SpecVersion,
        EstimateFee,
        AddInvokeTransaction,
    ];
    let mut account_deploy_last: Vec<StarknetMatcher> =
        vec![Nonce, AddDeployAccountTransaction, GetTransactionReceipt];
    account_deploy.append(&mut invoke_eth_transfer);
    account_deploy.append(&mut account_deploy_last);
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(account_deploy).await;
    let block_id = BlockId::BlockTag(BlockTag::Latest);
    let class_hash = Felt::try_new("0x0").unwrap();
    let contract_address = Address(class_hash.clone());
    let estimate_tx = estimate_fee_transaction();
    let invoke_tx = invoke_transaction();
    let deploy_tx = deploy_transaction();
    let tx_hash = TxnHash(class_hash);

    assert!(client.chainId().await.is_ok());
    assert!(client
        .getNonce(block_id.clone(), contract_address.clone())
        .await
        .is_ok());
    assert!(client.specVersion().await.is_ok());
    assert!(client
        .estimateFee(vec![estimate_tx.clone()], vec![], block_id.clone())
        .await
        .is_ok());

    assert!(client.chainId().await.is_ok());
    assert!(client.chainId().await.is_ok());
    assert!(client
        .getNonce(block_id.clone(), contract_address.clone())
        .await
        .is_ok());
    assert!(client.specVersion().await.is_ok());
    assert!(client
        .estimateFee(vec![estimate_tx], vec![], block_id.clone())
        .await
        .is_ok());
    assert!(client.addInvokeTransaction(invoke_tx).await.is_ok());
    assert!(client.getNonce(block_id, contract_address).await.is_ok());
    assert!(client.addDeployAccountTransaction(deploy_tx).await.is_ok());
    assert!(client.getTransactionReceipt(tx_hash).await.is_ok());
}
