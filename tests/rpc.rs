use beerus::gen::{
    Address, BlockHash, BlockId, BlockNumber, BlockTag, BroadcastedInvokeTxn,
    BroadcastedTxn, Felt, FunctionCall, GetBlockWithTxHashesResult,
    GetBlockWithTxsResult, GetClassAtResult, GetClassResult,
    GetTransactionByBlockIdAndIndexIndex, GetTransactionReceiptResult,
    InvokeTxn, InvokeTxnV1, InvokeTxnV1Version, PriceUnit, Rpc, StorageKey,
    SyncingResult, Txn, TxnExecutionStatus, TxnHash, TxnReceipt, TxnStatus,
};

mod common;

use common::error::Error;

#[tokio::test]
#[allow(non_snake_case)]
async fn test_specVersion() -> Result<(), Error> {
    let ctx = setup!();

    let ret = ctx.client.specVersion().await?;
    assert_eq!(ret, "0.6.0");
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_chainId() -> Result<(), Error> {
    let ctx = setup!();

    let ret = ctx.client.chainId().await?;
    assert_eq!(ret.as_ref(), "0x534e5f4d41494e");
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_blockHashAndNumber() -> Result<(), Error> {
    let ctx = setup!();

    let ret = ctx.client.blockHashAndNumber().await?;
    assert!(*ret.block_number.as_ref() > 600612);
    assert!(!ret.block_hash.0.as_ref().is_empty());
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_blockNumber() -> Result<(), Error> {
    let ctx = setup!();

    let ret = ctx.client.blockNumber().await?;
    assert!(*ret.as_ref() > 600612);
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_call() -> Result<(), Error> {
    let ctx = setup!();

    let request = FunctionCall {
        calldata: Vec::default(),
        contract_address: Address(Felt::try_new(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )?),
        entry_point_selector: Felt::try_new(
            "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60",
        )?,
    };

    let block_id =
        BlockId::BlockNumber { block_number: BlockNumber::try_new(33482)? };

    let ret = ctx.client.call(request, block_id).await?;
    assert_eq!(ret.len(), 1);
    assert_eq!(ret[0].as_ref(), "0x4574686572");
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_estimateFee() -> Result<(), Error> {
    let ctx = setup!();

    let calldata = vec![
        "0x2",
        "0x57c4b510d66eb1188a7173f31cccee47b9736d40185da8144377b896d5ff3",
        "0x2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
        "0x0",
        "0x1",
        "0x57c4b510d66eb1188a7173f31cccee47b9736d40185da8144377b896d5ff3",
        "0x2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
        "0x1",
        "0x1",
        "0x2",
        "0x0",
        "0x1",
    ];
    let calldata: Result<Vec<Felt>, _> =
        calldata.into_iter().map(Felt::try_new).collect();

    let signature = vec![
        "0x42527ffe9912b338983cbed67e139cfcc26a4d8cf1d1c2a85e4125fdf5f59ed",
        "0x636147d06fefd02ed37984b752556d4b9aefdac1a50b3df0528ec7c201ad84b",
    ];
    let signature: Result<Vec<Felt>, _> =
        signature.into_iter().map(Felt::try_new).collect();

    let request = vec![
        BroadcastedTxn::BroadcastedInvokeTxn(
            BroadcastedInvokeTxn(
                InvokeTxn::InvokeTxnV1(
                    InvokeTxnV1 {
                        calldata: calldata?,
                        signature: signature?,
                        sender_address: Address(Felt::try_new("0x13e3ca9a377084c37dc7eacbd1d9f8c3e3733935bcbad887c32a0e213cd6fe0")?), 
                        max_fee: Felt::try_new("0x28ed6103d0000")?, 
                        nonce: Felt::try_new("0x1")?,
                        version: InvokeTxnV1Version::V0x1,
                        r#type: beerus::gen::InvokeTxnV1Type::Invoke, 
                    }
                )
            )
        )
    ];

    let simulation_flags = vec![];

    let block_id =
        BlockId::BlockNumber { block_number: BlockNumber::try_new(59999)? };

    let ret =
        ctx.client.estimateFee(request, simulation_flags, block_id).await?;
    assert_eq!(ret.len(), 1);
    assert_eq!(ret[0].overall_fee.as_ref(), "0x1abd7b153e472");
    assert_eq!(ret[0].gas_price.as_ref(), "0x67edb4f57");
    assert_eq!(ret[0].gas_consumed.as_ref(), "0x41de");
    assert!(matches!(ret[0].unit, PriceUnit::Wei));
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getBlockTransactionCount() -> Result<(), Error> {
    let ctx = setup!();

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let ret = ctx.client.getBlockTransactionCount(block_id).await?;
    assert!(*ret.as_ref() > 0);
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getBlockWithTxHashes() -> Result<(), Error> {
    let ctx = setup!();

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let ret = ctx.client.getBlockWithTxHashes(block_id).await?;
    assert!(matches!(ret, GetBlockWithTxHashesResult::BlockWithTxHashes(_)));
    let GetBlockWithTxHashesResult::BlockWithTxHashes(ret) = ret else {
        panic!("unexpected pending block");
    };
    assert!(!ret.block_body_with_tx_hashes.transactions.is_empty());
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getBlockWithTxs() -> Result<(), Error> {
    let ctx = setup!();

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let ret = ctx.client.getBlockWithTxs(block_id).await?;
    assert!(matches!(ret, GetBlockWithTxsResult::BlockWithTxs(_)));
    let GetBlockWithTxsResult::BlockWithTxs(ret) = ret else {
        panic!("unexpected pending block");
    };
    assert!(!ret.block_body_with_txs.transactions.is_empty());
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_syncing() -> Result<(), Error> {
    let ctx = setup!();

    let ret = ctx.client.syncing().await?;
    assert!(matches!(ret, SyncingResult::SyncStatus(_)));
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getNonce() -> Result<(), Error> {
    let ctx = setup!();

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let address = Address(Felt::try_new(
        "0x10b6c96d364cf182964fbd4a3438a5ae84cab990770c07994f9cb99fd26f6dc",
    )?);

    let ret = ctx.client.getNonce(block_id, address).await?;
    assert!(!ret.as_ref().is_empty());
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getTransactionByHash() -> Result<(), Error> {
    let ctx = setup!();

    let hash =
        "0x2e2a98c1731ece2691edfbb4ed9b057182cec569735bd89825f17e3b342583a";

    let transaction_hash = TxnHash(Felt::try_new(hash)?);

    let ret = ctx.client.getTransactionByHash(transaction_hash).await?;
    assert!(matches!(ret.txn, Txn::InvokeTxn(InvokeTxn::InvokeTxnV1(_))));
    assert_eq!(ret.transaction_hash.0.as_ref(), hash);
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getTransactionByBlockIdAndIndex() -> Result<(), Error> {
    let ctx = setup!();

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let index = GetTransactionByBlockIdAndIndexIndex::try_new(0)?;

    let ret =
        ctx.client.getTransactionByBlockIdAndIndex(block_id, index).await?;
    assert!(!ret.transaction_hash.0.as_ref().is_empty());
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getStorageAt() -> Result<(), Error> {
    let ctx = setup!();

    let contract_address = Address(Felt::try_new(
        "0x6a05844a03bb9e744479e3298f54705a35966ab04140d3d8dd797c1f6dc49d0",
    )?);

    let key = StorageKey::try_new(
        "0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1",
    )?;

    let block_id_number =
        BlockId::BlockNumber { block_number: BlockNumber::try_new(600612)? };
    let ret = ctx
        .client
        .getStorageAt(contract_address.clone(), key.clone(), block_id_number)
        .await?;
    assert_eq!(ret.as_ref(), "0x47616d65206f66204c69666520546f6b656e");

    let block_id_hash = BlockId::BlockHash {
        block_hash: BlockHash(Felt::try_new(
            "0x1cbed30c5f1eb355f13e69562eda81b3f3edd5b46d5ef261ce5f24de55f0bdb",
        )?),
    };
    let ret = ctx
        .client
        .getStorageAt(contract_address.clone(), key.clone(), block_id_hash)
        .await?;
    assert_eq!(ret.as_ref(), "0x47616d65206f66204c69666520546f6b656e");

    let block_id_tag = BlockId::BlockTag(BlockTag::Pending);
    let ret =
        ctx.client.getStorageAt(contract_address, key, block_id_tag).await;
    assert!(ret.is_err());
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getProof() -> Result<(), Error> {
    let ctx = setup!();

    let contract_address = Address(Felt::try_new(
        "0x49D36570D4e46f48e99674bd3fcc84644DdD6b96F7C741B1562B82f9e004dC7",
    )?);

    let key = StorageKey::try_new(
        "0x02c401056f9582175d3219f1ac8f974b7960f2edfc8bc03197718dc8967ba1ab",
    )?;

    let block_id =
        BlockId::BlockNumber { block_number: BlockNumber::try_new(354824)? };

    let ret =
        ctx.client.getProof(block_id, contract_address, vec![key]).await?;
    assert_eq!(
        ret.class_commitment.unwrap().as_ref(),
        "0x4570dad16b85ea5076806bfb74c85bbb2b38485e6f3bd1bf163ab5f9ce1de53"
    );
    assert_eq!(
        ret.state_commitment.unwrap().as_ref(),
        "0xd9b8e8d51f3f284e62eb8c1fd7278c20bd4c0cd3033c4cce32c513e93ed663"
    );
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getTransactionStatus() -> Result<(), Error> {
    let ctx = setup!();

    let transaction_hash = TxnHash(Felt::try_new(
        "0x2e2a98c1731ece2691edfbb4ed9b057182cec569735bd89825f17e3b342583a",
    )?);

    let ret = ctx.client.getTransactionStatus(transaction_hash).await?;
    assert!(matches!(
        ret.execution_status,
        Some(TxnExecutionStatus::Succeeded)
    ));
    assert!(matches!(ret.finality_status, TxnStatus::AcceptedOnL1));
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getTransactionReceipt() -> Result<(), Error> {
    let ctx = setup!();

    let hash =
        "0x4c1672e824b5cd7477fca31ee3ab5a1058534ed1820bb27abc976c2e6095151";

    let transaction_hash = TxnHash(Felt::try_new(hash)?);

    let ret = ctx.client.getTransactionReceipt(transaction_hash).await?;
    let GetTransactionReceiptResult::TxnReceipt(TxnReceipt::InvokeTxnReceipt(
        ret,
    )) = ret
    else {
        panic!("unexpected pending block");
    };
    assert_eq!(ret.common_receipt_properties.transaction_hash.0.as_ref(), hash);
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getClass() -> Result<(), Error> {
    let ctx = setup!();

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let class_hash = Felt::try_new(
        "0xd0e183745e9dae3e4e78a8ffedcce0903fc4900beace4e0abf192d4c202da3",
    )?;

    let ret = ctx.client.getClass(block_id, class_hash).await?;
    let GetClassResult::DeprecatedContractClass(ret) = ret else {
        panic!("unexpected contract class type");
    };

    assert!(!ret.program.as_ref().is_empty());

    assert!(matches!(ret.abi, Some(vec) if !vec.is_empty()));

    assert!(
        matches!(ret.entry_points_by_type.constructor, Some(vec) if !vec.is_empty())
    );
    assert!(
        matches!(ret.entry_points_by_type.l1_handler, Some(vec) if !vec.is_empty())
    );
    assert!(
        matches!(ret.entry_points_by_type.external, Some(vec) if !vec.is_empty())
    );
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getClassAt() -> Result<(), Error> {
    let ctx = setup!();

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let contract_address = Address(Felt::try_new(
        "0x40688250Ef0074B4c9e1057B19F9b62139ac28179c7d35e2daE5abAD909d558",
    )?);

    let ret = ctx.client.getClassAt(block_id, contract_address).await?;
    let GetClassAtResult::ContractClass(ret) = ret else {
        panic!("unexpected contract class type");
    };

    assert!(!ret.abi.unwrap_or_default().is_empty());

    assert_eq!(ret.contract_class_version, "0.1.0");

    assert!(!ret.entry_points_by_type.constructor.is_empty());
    assert!(!ret.entry_points_by_type.external.is_empty());
    assert!(ret.entry_points_by_type.l1_handler.is_empty());

    assert!(!ret.sierra_program.is_empty());

    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getClassHashAt() -> Result<(), Error> {
    let ctx = setup!();

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let contract_address = Address(Felt::try_new(
        "0x7f38ab7537dbb5f8dc2d049d441f2b250c2186a13d943b8467cfa86b8dba12b",
    )?);

    let ret = ctx.client.getClassHashAt(block_id, contract_address).await?;
    assert_eq!(
        ret.as_ref(),
        "0x1a736d6ed154502257f02b1ccdf4d9d1089f80811cd6acad48e6b6a9d1f2003"
    );
    Ok(())
}
