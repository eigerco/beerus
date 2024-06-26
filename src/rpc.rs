use axum::{
    extract::State, response::IntoResponse, routing::post, Json, Router,
};
use iamgroot::jsonrpc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::{oneshot, RwLock},
    task::JoinHandle,
};

use crate::client::State as ClientState;

use crate::exe::err::Error;

use super::gen::*;

pub struct Server(oneshot::Sender<()>, JoinHandle<()>, u16);

impl Server {
    /// Send signal for terminating the server and await until it is terminated
    pub async fn stop(self) {
        let _ = self.0.send(());
        let _ = self.1.await;
    }

    /// Wait until the server is terminated (without initiating the termination)
    pub async fn done(self) {
        let _ = self.1.await;
    }

    /// Return server's listening port (convenience method for testing)
    pub fn port(&self) -> u16 {
        self.2
    }
}

pub async fn serve<A: ToSocketAddrs>(
    url: &str,
    addr: A,
    state: Arc<RwLock<ClientState>>,
) -> Result<Server, Error> {
    let listener = TcpListener::bind(addr).await?;
    let server = serve_on(url, listener, state)?;
    Ok(server)
}

fn serve_on(
    url: &str,
    listener: TcpListener,
    state: Arc<RwLock<ClientState>>,
) -> Result<Server, Error> {
    const DEFAULT_TIMEOUT: std::time::Duration =
        std::time::Duration::from_secs(30);
    let client = reqwest::ClientBuilder::new()
        .connect_timeout(DEFAULT_TIMEOUT)
        .timeout(DEFAULT_TIMEOUT)
        .build()?;

    let ctx = Context {
        url: url.to_owned(),
        client: Arc::new(gen::client::Client::with_client(url, client)),
        state,
    };

    let app = Router::new().route("/rpc", post(handle_request)).with_state(ctx);

    let (tx, rx) = oneshot::channel::<()>();
    let port = listener.local_addr()?.port();
    let jh = tokio::spawn(async move {
        let ret = axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(async move {
                let _ = rx.await;
            })
            .await;
        if let Err(e) = ret {
            tracing::error!("server shut down with error: {e:?}");
        }
    });

    Ok(Server(tx, jh, port))
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
    url: String,
    client: Arc<gen::client::Client>,
    state: Arc<RwLock<ClientState>>,
}

impl Context {
    async fn get_state(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<ClientState, jsonrpc::Error> {
        let block = self.getBlockWithTxHashes(block_id).await?;
        let gen::GetBlockWithTxHashesResult::BlockWithTxHashes(block) = block
        else {
            return Err(jsonrpc::Error {
                code: -1,
                message: "Pending block received".to_owned(),
            });
        };
        Ok(ClientState {
            block_number: *block.block_header.block_number.as_ref() as u64,
            block_hash: block.block_header.block_hash.0,
            root: block.block_header.new_root,
        })
    }

    async fn get_latest_state(
        &self,
    ) -> std::result::Result<ClientState, jsonrpc::Error> {
        let block_id = gen::BlockId::BlockTag(gen::BlockTag::Latest);
        self.get_state(block_id).await
    }
}

fn resolve_block_id(
    block_id: BlockId,
    state: &ClientState,
) -> std::result::Result<BlockId, jsonrpc::Error> {
    let state_block_number = state.block_number as i64;
    match block_id {
        gen::BlockId::BlockNumber { ref block_number }
            if block_number.as_ref() <= &state_block_number =>
        {
            Ok(block_id)
        }
        gen::BlockId::BlockNumber { .. } => {
            let block_number = BlockNumber::try_new(state_block_number)?;
            Ok(BlockId::BlockNumber { block_number })
        }
        gen::BlockId::BlockHash { ref block_hash }
            if block_hash.0.as_ref() == state.block_hash.as_ref() =>
        {
            Ok(block_id)
        }
        gen::BlockId::BlockHash { .. } => {
            // TODO Find matching block number and resolve it properly
            Ok(block_id)
        }
        gen::BlockId::BlockTag(BlockTag::Latest) => {
            let block_number = BlockNumber::try_new(state.block_number as i64)?;
            Ok(BlockId::BlockNumber { block_number })
        }
        gen::BlockId::BlockTag(BlockTag::Pending) => Err(jsonrpc::Error {
            code: -1,
            message: "Pending block is not supported".to_owned(),
        }),
    }
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
        _block_id: BlockId,
    ) -> std::result::Result<Vec<Felt>, jsonrpc::Error> {
        let client = gen::client::blocking::Client::new(&self.url);

        // TODO: address that effectively only the 'latest' block is supported
        let state_root = self.get_latest_state().await?.root;
        let call_info = tokio::task::spawn_blocking(move || {
            crate::exe::call(&client, request, state_root)
        })
        .await
        .map_err(|e| {
            iamgroot::jsonrpc::Error::new(500, format!("join error: {e}"))
        })??;

        let ret: Result<Vec<Felt>, Error> = call_info
            .execution
            .retdata
            .0
            .into_iter()
            .map(|e| e.try_into())
            .collect();

        Ok(ret?)
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
        let state = self.state.read().await.clone();
        let block_id = resolve_block_id(block_id, &state)?;

        let result = self
            .client
            .getStorageAt(
                contract_address.clone(),
                key.clone(),
                block_id.clone(),
            )
            .await?;
        tracing::info!(
            ?contract_address,
            ?key,
            ?block_id,
            ?result,
            "getStorageAt"
        );

        let proof = self
            .client
            .getProof(block_id, contract_address.clone(), vec![key.clone()])
            .await?;
        tracing::info!(?proof, "getProof");

        proof.verify(state.root, contract_address, key, result.clone())?;
        tracing::info!("getProof: verified");

        Ok(result)
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
        keys: Vec<gen::StorageKey>,
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

#[cfg(test)]
mod tests {
    use crate::rpc::{BlockHash, BlockId, BlockNumber, BlockTag, Felt};

    use super::{resolve_block_id, ClientState};

    fn make_state(block_number: i64) -> ClientState {
        ClientState {
            block_number: block_number as u64,
            block_hash: Felt::try_new("0x0").unwrap(),
            root: Felt::try_new("0x0").unwrap(),
        }
    }

    fn number(block_number: i64) -> BlockId {
        BlockId::BlockNumber {
            block_number: BlockNumber::try_new(block_number).unwrap(),
        }
    }

    fn hash(block_hash: &str) -> BlockId {
        BlockId::BlockHash {
            block_hash: BlockHash(Felt::try_new(block_hash).unwrap()),
        }
    }

    fn eq(lhs: &BlockId, rhs: &BlockId) -> bool {
        match (lhs, rhs) {
            (
                BlockId::BlockNumber { block_number: lhs },
                BlockId::BlockNumber { block_number: rhs },
            ) => lhs.as_ref() == rhs.as_ref(),
            (
                BlockId::BlockHash { block_hash: lhs },
                BlockId::BlockHash { block_hash: rhs },
            ) => lhs.0.as_ref() == rhs.0.as_ref(),
            (
                BlockId::BlockTag(BlockTag::Latest),
                BlockId::BlockTag(BlockTag::Latest),
            ) => true,
            (
                BlockId::BlockTag(BlockTag::Pending),
                BlockId::BlockTag(BlockTag::Pending),
            ) => true,
            _ => false,
        }
    }

    fn assert_resolved_block(current: i64, requested: i64, expected: i64) {
        let state = make_state(current);
        let resolved = resolve_block_id(number(requested), &state).unwrap();

        if let BlockId::BlockNumber { block_number } = resolved {
            assert_eq!(
                block_number.as_ref(),
                &expected,
                "Block number must match"
            );
        } else {
            unreachable!("Unexpected BlockId variant: {resolved:#?}")
        }
    }

    #[test]
    fn resolve_to_lowest() {
        assert_resolved_block(27, 3, 3);
    }

    #[test]
    fn resolve_to_current() {
        assert_resolved_block(27, 42, 27);
    }

    #[test]
    fn resolve_by_hash() {
        const HASH: &str = "0xCAFEBABE";

        let state = make_state(42);
        let resolved = resolve_block_id(hash(HASH), &state).unwrap();

        assert!(eq(&resolved, &hash(HASH)));
    }
}
