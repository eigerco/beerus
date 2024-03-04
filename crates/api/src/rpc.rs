use std::sync::Arc;
use axum::{
    extract::State, response::IntoResponse, routing::post, Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use iamgroot::jsonrpc;

use super::gen::*;

pub async fn serve(url: &str, bind: &str) {
    let ctx = Context { client: Arc::new(gen::client::Client::new(&url)) };

    let app = Router::new().route("/rpc", post(handle_request)).with_state(ctx);

    let listener = TcpListener::bind(bind).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum Request {
    Single(jsonrpc::Request),
    Batch(Vec<jsonrpc::Request>),
}

#[derive(Default, Deserialize, Serialize)]
#[serde(untagged)]
enum Response {
    #[default]
    Empty,
    Single(jsonrpc::Response),
    Batch(Vec<jsonrpc::Response>),
}

struct RpcError(jsonrpc::Error);

impl IntoResponse for RpcError {
    fn into_response(self) -> axum::response::Response {
        let code = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
        (code, self.0.message).into_response()
    }
}

#[derive(Clone)]
struct Context {
    client: Arc<gen::client::Client>,
    // TODO: add Helios client
    // TODO: add executor impl (based on blockifier)
    // TODO: add mappings from DTO to blockifier
}

async fn handle_request(
    State(ctx): State<Context>,
    Json(req): Json<Request>,
) -> Result<impl IntoResponse, RpcError> {
    match req {
        Request::Single(req) => {
            let res = gen::handle(&ctx, &req).await;
            if req.id.is_some() {
                Ok(Json(Response::Single(res)))
            } else {
                Ok(Json::default()) // no response for notifications
            }
        }
        Request::Batch(reqs) => {
            let mut ret = Vec::with_capacity(reqs.len());
            for req in reqs {
                let ctx = ctx.clone();
                let res = gen::handle(&ctx, &req).await;
                if req.id.is_some() {
                    ret.push(res);
                }
            }
            Ok(Json(Response::Batch(ret)))
        }
    }
}

#[allow(unused_variables)]
#[async_trait::async_trait]
impl gen::Rpc for Context {
    async fn addDeclareTransaction(
        &self,
        declare_transaction: BroadcastedDeclareTxn,
    ) -> std::result::Result<AddDeclareTransactionResult, jsonrpc::Error> {
        self.client.addDeclareTransaction(declare_transaction).await
    }

    async fn addDeployAccountTransaction(
        &self,
        deploy_account_transaction: BroadcastedDeployAccountTxn,
    ) -> std::result::Result<AddDeployAccountTransactionResult, jsonrpc::Error>
    {
        self.client
            .addDeployAccountTransaction(deploy_account_transaction)
            .await
    }

    async fn addInvokeTransaction(
        &self,
        invoke_transaction: BroadcastedInvokeTxn,
    ) -> std::result::Result<AddInvokeTransactionResult, jsonrpc::Error> {
        self.client.addInvokeTransaction(invoke_transaction).await
    }

    async fn blockHashAndNumber(
        &self,
    ) -> std::result::Result<BlockHashAndNumberResult, jsonrpc::Error> {
        self.client.blockHashAndNumber().await
    }

    async fn blockNumber(
        &self,
    ) -> std::result::Result<BlockNumber, jsonrpc::Error> {
        self.client.blockNumber().await
    }

    async fn call(
        &self,
        request: FunctionCall,
        block_id: BlockId,
    ) -> std::result::Result<Vec<Felt>, jsonrpc::Error> {
        self.client.call(request, block_id).await
    }

    async fn chainId(&self) -> std::result::Result<ChainId, jsonrpc::Error> {
        self.client.chainId().await
    }

    async fn estimateFee(
        &self,
        request: Vec<BroadcastedTxn>,
        simulation_flags: Vec<SimulationFlagForEstimateFee>,
        block_id: BlockId,
    ) -> std::result::Result<Vec<FeeEstimate>, jsonrpc::Error> {
        self.client.estimateFee(request, simulation_flags, block_id).await
    }

    async fn estimateMessageFee(
        &self,
        message: MsgFromL1,
        block_id: BlockId,
    ) -> std::result::Result<FeeEstimate, jsonrpc::Error> {
        self.client.estimateMessageFee(message, block_id).await
    }

    async fn getBlockTransactionCount(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<GetBlockTransactionCountResult, jsonrpc::Error>
    {
        self.client.getBlockTransactionCount(block_id).await
    }

    async fn getBlockWithTxHashes(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<GetBlockWithTxHashesResult, jsonrpc::Error> {
        self.client.getBlockWithTxHashes(block_id).await
    }

    async fn getBlockWithTxs(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<GetBlockWithTxsResult, jsonrpc::Error> {
        self.client.getBlockWithTxs(block_id).await
    }

    async fn getClass(
        &self,
        block_id: BlockId,
        class_hash: Felt,
    ) -> std::result::Result<GetClassResult, jsonrpc::Error> {
        self.client.getClass(block_id, class_hash).await
    }

    async fn getClassAt(
        &self,
        block_id: BlockId,
        contract_address: Address,
    ) -> std::result::Result<GetClassAtResult, jsonrpc::Error> {
        self.client.getClassAt(block_id, contract_address).await
    }

    async fn getClassHashAt(
        &self,
        block_id: BlockId,
        contract_address: Address,
    ) -> std::result::Result<Felt, jsonrpc::Error> {
        self.client.getClassHashAt(block_id, contract_address).await
    }

    async fn getEvents(
        &self,
        filter: GetEventsFilter,
    ) -> std::result::Result<EventsChunk, jsonrpc::Error> {
        self.client.getEvents(filter).await
    }

    async fn getNonce(
        &self,
        block_id: BlockId,
        contract_address: Address,
    ) -> std::result::Result<Felt, jsonrpc::Error> {
        self.client.getNonce(block_id, contract_address).await
    }

    async fn getStateUpdate(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<GetStateUpdateResult, jsonrpc::Error> {
        self.client.getStateUpdate(block_id).await
    }

    async fn getStorageAt(
        &self,
        contract_address: Address,
        key: StorageKey,
        block_id: BlockId,
    ) -> std::result::Result<Felt, jsonrpc::Error> {
        self.client.getStorageAt(contract_address, key, block_id).await
    }

    async fn getTransactionByBlockIdAndIndex(
        &self,
        block_id: BlockId,
        index: GetTransactionByBlockIdAndIndexIndex,
    ) -> std::result::Result<
        GetTransactionByBlockIdAndIndexResult,
        jsonrpc::Error,
    > {
        self.client.getTransactionByBlockIdAndIndex(block_id, index).await
    }

    async fn getTransactionByHash(
        &self,
        transaction_hash: TxnHash,
    ) -> std::result::Result<GetTransactionByHashResult, jsonrpc::Error> {
        self.client.getTransactionByHash(transaction_hash).await
    }

    async fn getTransactionReceipt(
        &self,
        transaction_hash: TxnHash,
    ) -> std::result::Result<GetTransactionReceiptResult, jsonrpc::Error> {
        self.client.getTransactionReceipt(transaction_hash).await
    }

    async fn getTransactionStatus(
        &self,
        transaction_hash: TxnHash,
    ) -> std::result::Result<GetTransactionStatusResult, jsonrpc::Error> {
        self.client.getTransactionStatus(transaction_hash).await
    }

    async fn simulateTransactions(
        &self,
        block_id: BlockId,
        transactions: Vec<BroadcastedTxn>,
        simulation_flags: Vec<SimulationFlag>,
    ) -> std::result::Result<Vec<SimulatedTransaction>, jsonrpc::Error> {
        self.client
            .simulateTransactions(block_id, transactions, simulation_flags)
            .await
    }

    async fn specVersion(&self) -> std::result::Result<String, jsonrpc::Error> {
        self.client.specVersion().await
    }

    async fn syncing(
        &self,
    ) -> std::result::Result<SyncingResult, jsonrpc::Error> {
        self.client.syncing().await
    }

    async fn traceBlockTransactions(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<Vec<BlockTransactionTrace>, jsonrpc::Error> {
        self.client.traceBlockTransactions(block_id).await
    }

    async fn traceTransaction(
        &self,
        transaction_hash: TxnHash,
    ) -> std::result::Result<TransactionTrace, jsonrpc::Error> {
        self.client.traceTransaction(transaction_hash).await
    }

    async fn getProof(
        &self,
        block_id: gen::BlockId,
        contract_address: gen::Address,
        keys: Vec<gen::Address>,
    ) -> std::result::Result<gen::GetProofResult, jsonrpc::Error> {
        self.client.getProof(block_id, contract_address, keys).await
    }

    async fn getTxStatus(
        &self,
        transaction_hash: gen::TxnHash,
    ) -> std::result::Result<gen::TxGatewayStatus, jsonrpc::Error> {
        self.client.getTxStatus(transaction_hash).await
    }

    async fn version(&self) -> std::result::Result<String, jsonrpc::Error> {
        self.client.version().await
    }
}
