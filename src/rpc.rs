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

use crate::client::{Http, State as ClientState};

use crate::exe::err::Error;

use super::gen::*;
use gen::GetBlockWithTxHashesResult;

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
        client: Arc::new(gen::client::Client::new(url, Http(client))),
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
    client: Arc<gen::client::Client<Http>>,
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

    async fn resolve_block_id(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<(BlockId, Felt), jsonrpc::Error> {
        let state = &self.state.read().await;
        match block_id {
            gen::BlockId::BlockNumber { block_number } => {
                self.resolve_block_by_number(block_number, state).await
            }
            gen::BlockId::BlockHash { block_hash } => {
                self.resolve_block_by_hash(block_hash, state).await
            }
            gen::BlockId::BlockTag(BlockTag::Latest) => {
                let block_number =
                    BlockNumber::try_new(state.block_number as i64)?;
                Ok((BlockId::BlockNumber { block_number }, state.root.clone()))
            }
            gen::BlockId::BlockTag(BlockTag::Pending) => Err(jsonrpc::Error {
                code: -1,
                message: "Pending block is not supported".to_owned(),
            }),
        }
    }

    async fn resolve_block_by_number(
        &self,
        block_number: BlockNumber,
        current_state: &ClientState,
    ) -> Result<(BlockId, Felt), jsonrpc::Error> {
        let req_block_number = *block_number.as_ref() as u64;
        if req_block_number >= current_state.block_number {
            return Ok((
                BlockId::BlockNumber {
                    block_number: BlockNumber::try_new(
                        current_state.block_number as i64,
                    )?,
                },
                current_state.root.clone(),
            ));
        }
        let state = self
            .get_state(BlockId::BlockNumber {
                block_number: block_number.clone(),
            })
            .await?;
        if state.block_number != req_block_number {
            return Err(jsonrpc::Error {
                code: -1,
                message: "Failed to verify requested block by number"
                    .to_string(),
            });
        }
        Ok((BlockId::BlockNumber { block_number }, state.root))
    }

    async fn resolve_block_by_hash(
        &self,
        block_hash: BlockHash,
        current_state: &ClientState,
    ) -> Result<(BlockId, Felt), jsonrpc::Error> {
        if block_hash.0.as_ref() == current_state.block_hash.as_ref() {
            return Ok((
                BlockId::BlockHash { block_hash },
                current_state.root.clone(),
            ));
        }
        let state = self
            .get_state(BlockId::BlockHash { block_hash: block_hash.clone() })
            .await?;
        if block_hash.0.as_ref() != state.block_hash.as_ref()
            || state.block_number >= current_state.block_number
        {
            return Err(jsonrpc::Error {
                code: -1,
                message: "Failed to verify requested block by hash".to_string(),
            });
        }
        Ok((BlockId::BlockHash { block_hash }, state.root))
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
        block_id: BlockId,
    ) -> std::result::Result<Vec<Felt>, jsonrpc::Error> {
        let client = gen::client::blocking::Client::new(&self.url, Http::new());
        let state = self.state.read().await.clone();

        // TODO: address that effectively only the 'latest' block is supported
        tracing::warn!(requested_block=?block_id, current_state=?state, "call");

        let call_info = tokio::task::spawn_blocking(move || {
            crate::exe::call(client.clone(), request, state)
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

    async fn getBlockWithReceipts(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<GetBlockWithReceiptsResult, jsonrpc::Error> {
        self.client.getBlockWithReceipts(block_id).await
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
        let (block_id, state_root) = self.resolve_block_id(block_id).await?;

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

        proof.verify(state_root, contract_address, key, result.clone())?;
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
    ) -> std::result::Result<TxnReceiptWithBlockInfo, jsonrpc::Error> {
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
    use std::sync::Arc;

    use iamgroot::jsonrpc;
    use tokio::sync::RwLock;
    use wiremock::{
        matchers::any, Mock, MockGuard, MockServer, ResponseTemplate,
    };

    use crate::{
        client::Http,
        rpc::{BlockHash, BlockId, BlockNumber, BlockTag, Felt},
    };

    use super::{client::Client, ClientState, Context};

    fn make_state(block_number: u64, block_hash: &str) -> ClientState {
        ClientState {
            block_number,
            block_hash: Felt::try_new(block_hash).unwrap(),
            root: Felt::try_new("0x0").unwrap(),
        }
    }

    fn make_context(
        url_local: &str,
        url_client: &str,
        state: ClientState,
    ) -> Context {
        let client = reqwest::Client::new();
        Context {
            url: url_local.to_string(),
            client: Arc::new(Client::new(url_client, Http(client))),
            state: Arc::new(RwLock::new(state)),
        }
    }

    fn block_from_hash(block_hash: &str) -> BlockId {
        BlockId::BlockHash {
            block_hash: BlockHash(Felt::try_new(block_hash).unwrap()),
        }
    }

    fn block_from_number(block_number: u64) -> BlockId {
        BlockId::BlockNumber {
            block_number: BlockNumber::try_new(block_number as i64).unwrap(),
        }
    }

    fn block_from_tag(tag: &str) -> BlockId {
        match tag {
            "pending" => BlockId::BlockTag(BlockTag::Pending),
            _ => BlockId::BlockTag(BlockTag::Latest),
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

    fn get_block_with_tx_hashes_response(
        block_num: u64,
        block_hash: &str,
    ) -> String {
        serde_json::to_string(&serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id":1,
                "result": {
                    "block_hash": block_hash,
                    "block_number": block_num,
                    "l1_gas_price": {
                        "price_in_fri": "0x2",
                        "price_in_wei": "0x3"
                    },
                    "new_root": "0x4",
                    "parent_hash": "0x5",
                    "sequencer_address": "0x6",
                    "starknet_version": "0.13.1",
                    "status": "ACCEPTED_ON_L1",
                    "timestamp": 1,
                    "transactions" : []
                }
            }
        ))
        .unwrap()
    }

    async fn setup_test_env(
        starknet_server: &MockServer,
        block_num: u64,
        starknet_response_block_num: u64,
        starknet_response_block_hash: &str,
        expect_request: u64,
    ) -> (MockGuard, Context) {
        let mock_guard = Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_body_string(
                get_block_with_tx_hashes_response(
                    starknet_response_block_num,
                    starknet_response_block_hash,
                ),
            ))
            .expect(expect_request)
            .mount_as_scoped(starknet_server)
            .await;
        let state = make_state(block_num, "0x27");
        let context =
            make_context("127.0.0.1:3030", &starknet_server.uri(), state);

        (mock_guard, context)
    }

    async fn resolve_block_by_number_test(
        requested_starknet_block_num: u64,
        block_num: u64,
        starknet_response_block_num: u64,
        expect_request: u64,
    ) -> Result<(BlockId, Felt), jsonrpc::Error> {
        let starknet_server = MockServer::start().await;
        let request_block_num =
            BlockNumber::try_new(requested_starknet_block_num as i64).unwrap();

        let (_mock_guard, context) = setup_test_env(
            &starknet_server,
            block_num,
            starknet_response_block_num,
            "0x3",
            expect_request,
        )
        .await;

        let state = &context.state.read().await;
        context.resolve_block_by_number(request_block_num, state).await
    }

    async fn resolve_block_by_hash_test(
        requested_starknet_block_hash: &str,
        block_num: u64,
        starknet_response_block_hash: &str,
        starknet_response_block_num: u64,
        expect_request: u64,
    ) -> Result<(BlockId, Felt), jsonrpc::Error> {
        let starknet_server = MockServer::start().await;
        let request_block_hash =
            BlockHash(Felt::try_new(requested_starknet_block_hash).unwrap());

        let (_mock_guard, context) = setup_test_env(
            &starknet_server,
            block_num,
            starknet_response_block_num,
            starknet_response_block_hash,
            expect_request,
        )
        .await;

        let state = &context.state.read().await;
        context.resolve_block_by_hash(request_block_hash, state).await
    }

    async fn resolve_block_id_test(
        block: BlockId,
        block_num: u64,
        starknet_response_block_num: u64,
        starknet_response_block_hash: &str,
        expect_request: u64,
    ) -> Result<(BlockId, Felt), jsonrpc::Error> {
        let starknet_server = MockServer::start().await;

        let (_mock_guard, context) = setup_test_env(
            &starknet_server,
            block_num,
            starknet_response_block_num,
            starknet_response_block_hash,
            expect_request,
        )
        .await;

        context.resolve_block_id(block).await
    }

    #[tokio::test]
    async fn resolve_block_by_number_request_lower_success() {
        let requested_starknet_block_num = 3;
        let state_block_num = 27;
        let starknet_response_block_number = 3;
        let expected_num_request = 1;

        let result = resolve_block_by_number_test(
            requested_starknet_block_num,
            state_block_num,
            starknet_response_block_number,
            expected_num_request,
        )
        .await;

        assert!(result.is_ok());
        let (returned_block, _) = result.unwrap();
        assert!(eq(
            &block_from_number(requested_starknet_block_num),
            &returned_block
        ));
    }

    #[tokio::test]
    async fn resolve_block_by_number_request_higher_success() {
        let requested_starknet_block_num = 42;
        let state_block_num = 27;
        let starknet_response_block_number = 42;
        let expected_num_request = 0;

        let result = resolve_block_by_number_test(
            requested_starknet_block_num,
            state_block_num,
            starknet_response_block_number,
            expected_num_request,
        )
        .await;

        assert!(result.is_ok());
        let (returned_block, _) = result.unwrap();
        assert!(eq(
            &block_from_number(state_block_num),
            &returned_block
        ));
    }

    #[tokio::test]
    async fn resolve_block_by_number_same_success() {
        let requested_starknet_block_num = 27;
        let state_block_num = 27;
        let starknet_response_block_number = requested_starknet_block_num;
        let expected_num_request = 0;

        let result = resolve_block_by_number_test(
            requested_starknet_block_num,
            state_block_num,
            starknet_response_block_number,
            expected_num_request,
        )
        .await;

        assert!(result.is_ok());
        let (returned_block, _) = result.unwrap();
        assert!(eq(
            &block_from_number(requested_starknet_block_num),
            &returned_block
        ));
    }

    #[tokio::test]
    async fn resolve_block_by_number_wrong_number_return() {
        let requested_starknet_block_num = 5;
        let state_block_num = 27;
        let starknet_response_block_number = 999;
        let expected_num_request = 1;

        let result = resolve_block_by_number_test(
            requested_starknet_block_num,
            state_block_num,
            starknet_response_block_number,
            expected_num_request,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn resolve_block_by_hash_different_success() {
        let requested_starknet_block_hash = "0x3";
        let state_block_num = 27;
        let starknet_response_block_hash = requested_starknet_block_hash;
        let starknet_response_block_num = 3;
        let expected_num_request = 1;

        let result = resolve_block_by_hash_test(
            requested_starknet_block_hash,
            state_block_num,
            starknet_response_block_hash,
            starknet_response_block_num,
            expected_num_request,
        )
        .await;

        assert!(result.is_ok());
        let (returned_block, _) = result.unwrap();
        assert!(eq(
            &block_from_hash(requested_starknet_block_hash),
            &returned_block
        ));
    }

    #[tokio::test]
    async fn resolve_block_by_hash_same_success() {
        let requested_starknet_block_hash = "0x27";
        let state_block_num = 27;
        let starknet_response_block_hash = requested_starknet_block_hash;
        let starknet_response_block_num = 27;
        let expected_num_request = 0;

        let result = resolve_block_by_hash_test(
            requested_starknet_block_hash,
            state_block_num,
            starknet_response_block_hash,
            starknet_response_block_num,
            expected_num_request,
        )
        .await;

        assert!(result.is_ok());
        let (returned_block, _) = result.unwrap();
        assert!(eq(
            &block_from_hash(requested_starknet_block_hash),
            &returned_block
        ));
    }

    #[tokio::test]
    async fn resolve_block_by_hash_wrong_number_return_error() {
        let requested_starknet_block_hash = "0x99";
        let state_block_num = 27;
        let starknet_response_block_hash = requested_starknet_block_hash;
        let starknet_response_block_num = 27;
        let expected_num_request = 1;

        let result = resolve_block_by_hash_test(
            requested_starknet_block_hash,
            state_block_num,
            starknet_response_block_hash,
            starknet_response_block_num,
            expected_num_request,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn resolve_block_by_hash_wrong_hash_return_error() {
        let requested_starknet_block_hash = "0x99";
        let state_block_num = 27;
        let starknet_response_block_hash = "0xbad";
        let starknet_response_block_num = 3;
        let expected_num_request = 1;

        let result = resolve_block_by_hash_test(
            requested_starknet_block_hash,
            state_block_num,
            starknet_response_block_hash,
            starknet_response_block_num,
            expected_num_request,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn resolve_block_id_number_success() {
        let request_block = block_from_number(3);
        let block_num = 27;
        let starknet_response_block_num = 3;
        let starknet_response_block_hash = "0x3";
        let expected_num_request = 1;

        let result = resolve_block_id_test(
            request_block.clone(),
            block_num,
            starknet_response_block_num,
            starknet_response_block_hash,
            expected_num_request,
        )
        .await;

        assert!(result.is_ok());
        let (returned_block, _) = result.unwrap();
        assert!(eq(&request_block, &returned_block));
    }

    #[tokio::test]
    async fn resolve_block_id_hash_success() {
        let request_block = block_from_hash("0x3");
        let block_num = 27;
        let starknet_response_block_num = 3;
        let starknet_response_block_hash = "0x3";
        let expected_num_request = 1;

        let result = resolve_block_id_test(
            request_block.clone(),
            block_num,
            starknet_response_block_num,
            starknet_response_block_hash,
            expected_num_request,
        )
        .await;

        assert!(result.is_ok());
        let (returned_block, _) = result.unwrap();
        assert!(eq(&request_block, &returned_block));
    }

    #[tokio::test]
    async fn resolve_block_id_tag_latest_success() {
        let request_block = block_from_tag("latest");
        let block_num = 27;
        let starknet_response_block_num = 33;
        let starknet_response_block_hash = "0x33";
        let expected_num_request = 0;

        let result = resolve_block_id_test(
            request_block.clone(),
            block_num,
            starknet_response_block_num,
            starknet_response_block_hash,
            expected_num_request,
        )
        .await;

        assert!(result.is_ok());
        let (returned_block, _) = result.unwrap();
        assert!(eq(&block_from_number(block_num), &returned_block));
    }

    #[tokio::test]
    async fn resolve_block_id_tag_pending_error() {
        let request_block = block_from_tag("pending");
        let block_num = 27;
        let starknet_response_block_num = 33;
        let starknet_response_block_hash = "0x33";
        let expected_num_request = 0;

        let result = resolve_block_id_test(
            request_block.clone(),
            block_num,
            starknet_response_block_num,
            starknet_response_block_hash,
            expected_num_request,
        )
        .await;

        assert!(result.is_err());
    }
}
