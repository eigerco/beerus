use std::{thread, time};

use beerus::gen::BroadcastedDeclareTxn;
use beerus::gen::{
    client::Client, Address, BlockId, BlockTag, BroadcastedTxn, Felt, Rpc,
    SimulationFlagForEstimateFee,
};
use common::constants::{COMPILED_ACCOUNT_CONTRACT, DECLARE_ACCOUNT};
use common::katana::Katana;

mod common;

#[tokio::test]
async fn declare_account() {
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
