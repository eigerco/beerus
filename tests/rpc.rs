use beerus::{
    config::MAINNET_STARKNET_CHAINID,
    gen::{
        Address, BlockHash, BlockId, BlockNumber, BlockTag,
        BroadcastedInvokeTxn, BroadcastedTxn, Felt, FunctionCall,
        GetBlockWithTxHashesResult, GetBlockWithTxsResult, GetClassAtResult,
        GetClassResult, GetTransactionByBlockIdAndIndexIndex, InvokeTxn,
        InvokeTxnV1, InvokeTxnV1Version, PriceUnit, Rpc, StorageKey,
        SyncingResult, Txn, TxnExecutionStatus, TxnHash, TxnReceipt,
        TxnReceiptWithBlockInfo, TxnStatus,
    },
};

mod common;

use common::err::Error;

#[tokio::test]
#[allow(non_snake_case)]
async fn test_specVersion() -> Result<(), Error> {
    let ctx = setup!();

    let ret = ctx.client.specVersion().await?;
    assert_eq!(ret, "0.7.1");
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_chainId() -> Result<(), Error> {
    let ctx = setup!();

    let ret = ctx.client.chainId().await?;
    assert_eq!(ret.as_ref(), MAINNET_STARKNET_CHAINID);
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

    const EXPECTED: &str = "0x47616d65206f66204c69666520546f6b656e";

    let block_id_number =
        BlockId::BlockNumber { block_number: BlockNumber::try_new(600612)? };
    let ret = ctx
        .client
        .getStorageAt(contract_address.clone(), key.clone(), block_id_number)
        .await?;
    assert_eq!(ret.as_ref(), EXPECTED);

    let block_id_hash = BlockId::BlockHash {
        block_hash: BlockHash(Felt::try_new(
            "0x1cbed30c5f1eb355f13e69562eda81b3f3edd5b46d5ef261ce5f24de55f0bdb",
        )?),
    };
    let ret = ctx
        .client
        .getStorageAt(contract_address.clone(), key.clone(), block_id_hash)
        .await?;
    assert_eq!(ret.as_ref(), EXPECTED);

    let block_id_tag = BlockId::BlockTag(BlockTag::Latest);
    let ret = ctx
        .client
        .getStorageAt(contract_address.clone(), key.clone(), block_id_tag)
        .await?;
    assert_eq!(ret.as_ref(), EXPECTED);

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
    let TxnReceiptWithBlockInfo {
        txn_receipt: TxnReceipt::InvokeTxnReceipt(ret),
        ..
    } = ret
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

#[tokio::test]
async fn erc20_call() -> Result<(), Error> {
    let ctx = setup!();
    let block_id = BlockId::BlockTag(BlockTag::Pending);
    let erc20_address = Address(Felt::try_new(
        "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    )?);

    let felt_name = Felt::try_new(
        "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60",
    )?;
    let request_name = FunctionCall {
        calldata: vec![],
        contract_address: erc20_address.clone(),
        entry_point_selector: felt_name,
    };
    let res_call_name = ctx.client.call(request_name, block_id.clone()).await?;
    assert_eq!(res_call_name.len(), 1);
    let ether = "0x4574686572";
    assert_eq!(res_call_name[0].as_ref(), ether);

    let felt_decimals = Felt::try_new(
        "0x4c4fb1ab068f6039d5780c68dd0fa2f8742cceb3426d19667778ca7f3518a9",
    )?;
    let request_decimals = FunctionCall {
        calldata: vec![],
        contract_address: erc20_address.clone(),
        entry_point_selector: felt_decimals,
    };
    let res_call_decimals =
        ctx.client.call(request_decimals, block_id.clone()).await?;
    assert_eq!(res_call_decimals.len(), 1);
    let twelve = "0x12";
    assert_eq!(res_call_decimals[0].as_ref(), twelve);

    let felt_symbol = Felt::try_new(
        "0x216b05c387bab9ac31918a3e61672f4618601f3c598a2f3f2710f37053e1ea4",
    )?;
    let request_symbol = FunctionCall {
        calldata: vec![],
        contract_address: erc20_address,
        entry_point_selector: felt_symbol,
    };
    let res_call_symbol = ctx.client.call(request_symbol, block_id).await?;
    assert_eq!(res_call_symbol.len(), 1);
    let eth = "0x455448";
    assert_eq!(res_call_symbol[0].as_ref(), eth);

    Ok(())
}

#[tokio::test]
async fn account_call() -> Result<(), Error> {
    let ctx = setup!("sepolia");
    let block_id = BlockId::BlockTag(BlockTag::Pending);
    let account_address = Address(Felt::try_new(
        "0x61ce2b8e048c19ee48af79a95e984769366611bb3f46c45cf70460b82efff8e",
    )?);

    let felt_public_key = Felt::try_new(
        "0x3b28019ccfdbd30ffc65951d94bb85c9e2b8434111a000b5afd533ce65f57a4",
    )?;
    let request_name = FunctionCall {
        calldata: vec![],
        contract_address: account_address.clone(),
        entry_point_selector: felt_public_key,
    };
    let res_call_public_key =
        ctx.client.call(request_name, block_id.clone()).await?;
    assert_eq!(res_call_public_key.len(), 1);
    let public_key =
        "0x145b000feec4f33c8622e91311922950d813ff8514b6a6552fc662eeb61cdf9";
    assert_eq!(res_call_public_key[0].as_ref(), public_key);

    let interface = Felt::try_new(
        "0x2ceccef7f994940b3962a6c67e0ba4fcd37df7d131417c604f91e03caecc1cd",
    )?;
    let felt_supports_interface = Felt::try_new(
        "0xfe80f537b66d12a00b6d3c072b44afbb716e78dde5c3f0ef116ee93d3e3283",
    )?;
    let request_supports_interface = FunctionCall {
        calldata: vec![interface],
        contract_address: account_address.clone(),
        entry_point_selector: felt_supports_interface,
    };
    let res_call_supports_interface =
        ctx.client.call(request_supports_interface, block_id.clone()).await?;
    assert_eq!(res_call_supports_interface.len(), 1);
    assert_eq!(res_call_supports_interface[0].as_ref(), "0x1");

    let hash = Felt::try_new(
        "0x259cbf64e5b2beb31cfac3b444b8dd20650e841581b25be14d5e08947e81cf2",
    )?;
    let array_size = Felt::try_new("0x2")?;
    let signature_r = Felt::try_new(
        "0x58f1ad9bb6331bb460c80260eee0c65980b4c1a659a5d84e7e51418afdf7311",
    )?;
    let signature_s = Felt::try_new(
        "0x74d9f54825e422fc5004533c33813ab2772057f87652e75a24e10ffac14726d",
    )?;
    let felt_is_valid_signature = Felt::try_new(
        "0x28420862938116cb3bbdbedee07451ccc54d4e9412dbef71142ad1980a30941",
    )?;
    let request_is_valid_signature = FunctionCall {
        calldata: vec![hash, array_size, signature_r, signature_s],
        contract_address: account_address,
        entry_point_selector: felt_is_valid_signature,
    };
    let res_call_is_valid_signature =
        ctx.client.call(request_is_valid_signature, block_id).await?;
    assert_eq!(res_call_is_valid_signature.len(), 1);
    let valid = "0x56414c4944";
    assert_eq!(res_call_is_valid_signature[0].as_ref(), valid);

    Ok(())
}
