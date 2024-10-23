use std::{thread, time};

use beerus::{
    client::Http,
    gen::{
        client::Client, Address, BlockId, BlockTag, BroadcastedDeclareTxn,
        BroadcastedDeployAccountTxn, BroadcastedInvokeTxn, BroadcastedTxn,
        DeployAccountTxn, DeployAccountTxnV1, DeployAccountTxnV1Type,
        DeployAccountTxnV1Version, Felt, InvokeTxn, InvokeTxnV1,
        InvokeTxnV1Type, InvokeTxnV1Version, Rpc, SimulationFlagForEstimateFee,
        TxnHash,
    },
};
use common::err::Error;
use starknet::katana::Katana;
use starknet::scarb::Compiler;
use starknet::{
    constants::{
        CLASS_HASH, COMPILED_ACCOUNT_CONTRACT_V2, COMPILED_ACCOUNT_CONTRACT_V3,
        CONTRACT_ADDRESS, DECLARE_ACCOUNT_V2, DECLARE_ACCOUNT_V3,
        SENDER_ADDRESS,
    },
    coordinator::{Coordinator, TestMode},
};

mod common;
mod starknet;

async fn setup() -> (Katana, Client<Http>) {
    let katana = Katana::init("http://127.0.0.1:0").await.unwrap();
    let url = format!("http://127.0.0.1:{}", katana.port());
    let client = Client::new(&url, Http::new());
    (katana, client)
}

#[tokio::test]
async fn declare_account_v3() {
    let (_katana, client) = setup().await;
    declare(&client, COMPILED_ACCOUNT_CONTRACT_V3, DECLARE_ACCOUNT_V3).await;
}

#[tokio::test]
async fn declare_deploy_account_v2() {
    let (_katana, client) = setup().await;
    declare(&client, COMPILED_ACCOUNT_CONTRACT_V2, DECLARE_ACCOUNT_V2).await;
    estimate_deploy(&client).await;
    transfer_eth(&client).await;
    deploy(client).await;
}

#[tokio::test]
async fn deploy_new_account_on_katana() -> Result<(), Error> {
    let _katana = Katana::init("http://127.0.0.1:0").await?;
    let coordinator = Coordinator::new(TestMode::Katana);
    coordinator.copy_template_to_target()?;
    coordinator.update_account()?;
    let compiler = Compiler::new(&coordinator.target_scarb())?;
    compiler.compile().await?;
    // TODO
    // #804 starkli signer keystore new key.json - Storing somewhere or deleting?
    // #804 starkli account oz init account.json - Storing somewhere or deleting?
    // #804 declare accounts
    // #804 #805 fund accounts from pre-funded account
    // #804 deploy accounts
    // #806 iterate through class hashes and call getClass to see if they are verified
    Ok(())
}

async fn declare(
    client: &Client<Http>,
    compiled_contract: &str,
    declare_account: &str,
) {
    let block_id = BlockId::BlockTag(BlockTag::Pending);
    let class_hash = Felt::try_new(CLASS_HASH).unwrap();
    let contract_address = Address(Felt::try_new(SENDER_ADDRESS).unwrap());

    let res_chain_id = client.chainId().await;
    assert!(res_chain_id.is_ok());
    assert_eq!(res_chain_id.unwrap().as_ref(), "0x4b4154414e41");

    let res_class = client.getClass(block_id.clone(), class_hash.clone()).await;
    assert!(res_class.is_err());
    assert!(res_class.unwrap_err().message.contains("Class hash not found"));

    let res_nonce = client.getNonce(block_id.clone(), contract_address).await;
    assert!(res_nonce.is_ok());
    assert_eq!(res_nonce.unwrap().as_ref(), "0x0");

    let res_spec_version = client.specVersion().await;
    assert!(res_spec_version.is_ok());
    assert_eq!(res_spec_version.unwrap().as_str(), "0.7.1");

    let contract: Vec<BroadcastedTxn> =
        serde_json::from_str(compiled_contract).unwrap();
    let simulation_flags: Vec<SimulationFlagForEstimateFee> = vec![];
    let res_estimate_fee =
        client.estimateFee(contract, simulation_flags, block_id.clone()).await;
    assert!(res_estimate_fee.is_ok());

    let declare_account: BroadcastedDeclareTxn =
        serde_json::from_str(declare_account).unwrap();
    let res_declare_account =
        client.addDeclareTransaction(declare_account).await;
    assert!(res_declare_account.is_ok());

    let block_mining_time = time::Duration::from_millis(1000);
    thread::sleep(block_mining_time);

    let res_class = client.getClass(block_id, class_hash).await;
    assert!(res_class.is_ok());
}

async fn estimate_deploy(client: &Client<Http>) {
    let block_id = BlockId::BlockTag(BlockTag::Pending);
    let contract_address = Address(Felt::try_new(CONTRACT_ADDRESS).unwrap());

    let res_chain_id = client.chainId().await;
    assert!(res_chain_id.is_ok());
    assert_eq!(res_chain_id.unwrap().as_ref(), "0x4b4154414e41");

    let res_nonce = client.getNonce(block_id.clone(), contract_address).await;
    assert!(res_nonce.is_err());

    let res_spec_version = client.specVersion().await;
    assert!(res_spec_version.is_ok());
    assert_eq!(res_spec_version.unwrap().as_str(), "0.7.1");

    let max_fee = Felt::try_new("0x0").unwrap();
    let signature = vec![
        Felt::try_new(
            "0x4695bbb6bc179a263b534f6083ca3dc45ffe3935ea86f3cc54a55c6de34eaa6",
        )
        .unwrap(),
        Felt::try_new(
            "0x2bd23130483dc20cbca45bd8f9f2c67e3393c2dd6763c7d08f99821a12e3ac5",
        )
        .unwrap(),
    ];
    let version =
        DeployAccountTxnV1Version::V0x100000000000000000000000000000001;
    let deploy_account: Vec<BroadcastedTxn> =
        vec![BroadcastedTxn::BroadcastedDeployAccountTxn(deploy_tx(
            max_fee, signature, version,
        ))];
    let simulation_flags: Vec<SimulationFlagForEstimateFee> = vec![];
    let res_estimate_fee =
        client.estimateFee(deploy_account, simulation_flags, block_id).await;
    assert!(res_estimate_fee.is_ok());
}

async fn transfer_eth(client: &Client<Http>) {
    let block_id = BlockId::BlockTag(BlockTag::Pending);
    let sender_address = Address(Felt::try_new(SENDER_ADDRESS).unwrap());

    let res_chain_id = client.chainId().await;
    assert!(res_chain_id.is_ok());
    assert_eq!(res_chain_id.unwrap().as_ref(), "0x4b4154414e41");

    let res_chain_id = client.chainId().await;
    assert!(res_chain_id.is_ok());
    assert_eq!(res_chain_id.unwrap().as_ref(), "0x4b4154414e41");

    let res_nonce =
        client.getNonce(block_id.clone(), sender_address.clone()).await;
    assert!(res_nonce.is_ok());
    assert_eq!(res_nonce.unwrap().as_ref(), "0x1");

    let res_spec_version = client.specVersion().await;
    assert!(res_spec_version.is_ok());
    assert_eq!(res_spec_version.unwrap().as_str(), "0.7.1");

    let max_fee_estimate = Felt::try_new("0x0").unwrap();
    let signature_estimate = vec![
        Felt::try_new(
            "0x1bc1b911315a5d7b5b2201b49e9622d755dbb383cf156523a369db0e742e266",
        )
        .unwrap(),
        Felt::try_new(
            "0x104e9068ad119cd9284e5c7822530bf224460c53dd99bd24e00ccd85dd15e6e",
        )
        .unwrap(),
    ];
    let version_estimate_fee =
        InvokeTxnV1Version::V0x100000000000000000000000000000001;
    let estimate_fee_tx =
        vec![BroadcastedTxn::BroadcastedInvokeTxn(invoke_tx(
            sender_address.clone(),
            max_fee_estimate,
            signature_estimate,
            version_estimate_fee,
        ))];
    let simulation_flags: Vec<SimulationFlagForEstimateFee> = vec![];
    let res_estimate_fee = client
        .estimateFee(estimate_fee_tx, simulation_flags, block_id.clone())
        .await;
    assert!(res_estimate_fee.is_ok());

    let max_fee_invoke = Felt::try_new("0x17a70d4b0e800").unwrap();
    let signature_invoke = vec![
        Felt::try_new(
            "0x5d4ef80a4e4217d8de52475799b4f3df3c82acaede506d90d34f44eedc0506b",
        )
        .unwrap(),
        Felt::try_new(
            "0x6786f06333100af063a9fce3388f5b733f0e2e7d247738a909594b92b90d8b9",
        )
        .unwrap(),
    ];
    let version_invoke = InvokeTxnV1Version::V0x1;
    let transfer_eth_tx = invoke_tx(
        sender_address,
        max_fee_invoke,
        signature_invoke,
        version_invoke,
    );
    let res_invoke_tx = client.addInvokeTransaction(transfer_eth_tx).await;
    assert!(res_invoke_tx.is_ok());
}

async fn deploy(client: Client<Http>) {
    let block_id = BlockId::BlockTag(BlockTag::Pending);
    let contract_address = Address(Felt::try_new(CONTRACT_ADDRESS).unwrap());

    let res_nonce = client.getNonce(block_id, contract_address).await;
    assert!(res_nonce.is_err());

    let max_fee = Felt::try_new("0x1c484e3020c00").unwrap();
    let signature = vec![
        Felt::try_new(
            "0x1892f4e8d35a0f73f4eab5fd085c25ec576c9887da17af82f13b8caecb3cb9c",
        )
        .unwrap(),
        Felt::try_new(
            "0x11408b2e3e95e3c5d9dbdb354cf9c06dd66d1f99e86ac31e5820c6db4647a64",
        )
        .unwrap(),
    ];
    let version = DeployAccountTxnV1Version::V0x1;
    let deploy_account_tx = deploy_tx(max_fee, signature, version);
    let res_deploy =
        client.addDeployAccountTransaction(deploy_account_tx).await;
    assert!(res_deploy.is_ok());

    let block_mining_time = time::Duration::from_millis(1000);
    thread::sleep(block_mining_time);

    let tx_hash = TxnHash(
        Felt::try_new(
            "0x1eb7ad201058042d681bb3159068978046b1d992561fdefa0bffcd4bc187572",
        )
        .unwrap(),
    );
    let res_get_receipt = client.getTransactionReceipt(tx_hash).await;
    assert!(res_get_receipt.is_ok());
}

fn deploy_tx(
    max_fee: Felt,
    signature: Vec<Felt>,
    version: DeployAccountTxnV1Version,
) -> BroadcastedDeployAccountTxn {
    BroadcastedDeployAccountTxn(DeployAccountTxn::DeployAccountTxnV1(DeployAccountTxnV1{
        class_hash: Felt::try_new(CLASS_HASH).unwrap(),
        constructor_calldata: vec![
            Felt::try_new("0x44c65058267bdcca53dbc4323fe64e547942389abe448d19daae570d99b3c0a").unwrap(),
        ],
        contract_address_salt: Felt::try_new("0x608eca42681e117e32199488d442377b68ced54f71b8af6b0aa5b8163caaf8f").unwrap(),
        max_fee,
        nonce: Felt::try_new("0x0").unwrap(),
        r#type: DeployAccountTxnV1Type::DeployAccount,
        signature,
        version,
    }))
}

fn invoke_tx(
    sender_address: Address,
    max_fee: Felt,
    signature: Vec<Felt>,
    version: InvokeTxnV1Version,
) -> BroadcastedInvokeTxn {
    BroadcastedInvokeTxn(InvokeTxn::InvokeTxnV1(InvokeTxnV1{
            calldata: vec![
                Felt::try_new("0x1").unwrap(),
                Felt::try_new("0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7").unwrap(),
                Felt::try_new("0x83afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e").unwrap(),
                Felt::try_new("0x3").unwrap(),
                Felt::try_new(CONTRACT_ADDRESS).unwrap(),
                Felt::try_new("0x4563918244f40000").unwrap(),
                Felt::try_new("0x0").unwrap(),
            ],
            max_fee,
            nonce: Felt::try_new("0x1").unwrap(),
            r#type: InvokeTxnV1Type::Invoke,
            sender_address,
            signature,
            version,
    }))
}
