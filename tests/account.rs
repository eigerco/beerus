use std::{thread, time};

use beerus::gen::{
    client::Client, Address, BlockId, BlockTag, BroadcastedDeclareTxn,
    BroadcastedTxn, Felt, Rpc, SimulationFlagForEstimateFee, TxnHash,
};
use common::{
    constants::{
        declare_transaction, deploy_transaction, estimate_fee_transaction,
        invoke_transaction, COMPILED_ACCOUNT_CONTRACT, DECLARE_ACCOUNT,
    },
    katana::Katana,
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
async fn declare_account_katana() {
    let url = "http://127.0.0.1:5050";
    let katana = Katana::init(url).await.unwrap();
    let client = Client::new(url);

    let res_chain_id = client.chainId().await;
    assert!(res_chain_id.is_ok());
    assert_eq!(res_chain_id.unwrap().as_ref(), "0x4b4154414e41");

    let block_id = BlockId::BlockTag(BlockTag::Pending);
    let class_hash = Felt::try_new(
        "0x6b46f84b1bbb779e588a9c5f577907c3dfb66e6b13cf4c4f480d4fb1677c2ba",
    )
    .unwrap();
    let res_class = client.getClass(block_id.clone(), class_hash).await;
    assert!(res_class.is_err());
    assert!(res_class.unwrap_err().message.contains("Class hash not found"));

    let contract_address = Address(
        Felt::try_new(
            "0x6162896d1d7ab204c7ccac6dd5f8e9e7c25ecd5ae4fcb4ad32e57786bb46e03",
        )
        .unwrap(),
    );
    let res_nonce = client.getNonce(block_id, contract_address).await;
    assert!(res_nonce.is_ok());
    assert_eq!(res_nonce.unwrap().as_ref(), "0x0");

    let res_spec_version = client.specVersion().await;
    assert!(res_spec_version.is_ok());
    assert_eq!(res_spec_version.unwrap().as_str(), "0.7.1");

    let contract: Vec<BroadcastedTxn> =
        serde_json::from_str(COMPILED_ACCOUNT_CONTRACT).unwrap();
    let simulation_flags: Vec<SimulationFlagForEstimateFee> = vec![];
    let block_id = BlockId::BlockTag(BlockTag::Pending);
    let res_estimate_fee =
        client.estimateFee(contract, simulation_flags, block_id).await;
    assert!(res_estimate_fee.is_ok());

    let declare_account: BroadcastedDeclareTxn =
        serde_json::from_str(DECLARE_ACCOUNT).unwrap();
    let res_declare_account =
        client.addDeclareTransaction(declare_account).await;
    assert!(res_declare_account.is_ok());

    let block_mining_time = time::Duration::from_millis(1000);
    thread::sleep(block_mining_time);

    let block_id = BlockId::BlockTag(BlockTag::Pending);
    let class_hash = Felt::try_new(
        "0x6b46f84b1bbb779e588a9c5f577907c3dfb66e6b13cf4c4f480d4fb1677c2ba",
    )
    .unwrap();
    let res_class = client.getClass(block_id.clone(), class_hash).await;
    assert!(res_class.is_ok());

    katana.stop().unwrap();
}

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
async fn declare_account_mock() {
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
async fn deploy_account_mock() {
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
