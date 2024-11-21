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

#[derive(Debug)]
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
    use std::{collections::HashMap, sync::Arc};

    use axum::response::IntoResponse;
    use iamgroot::jsonrpc;
    use tokio::sync::RwLock;
    use wiremock::{
        matchers::any, Mock, MockGuard, MockServer, Request, ResponseTemplate,
    };


    use crate::{
        client::Http,
        rpc::{BlockHash, BlockId, BlockNumber, BlockTag, Felt},
        gen::*,
    };

    use super::{client::Client, ClientState, Context, handle_request, Json, RpcError, serve, gen::*, State};

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
        helios_block_num: u64,
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
        let state = make_state(helios_block_num, "0x27");
        let context =
            make_context("127.0.0.1:3030", &starknet_server.uri(), state);

        (mock_guard, context)
    }

    async fn resolve_block_by_number_test(
        requested_starknet_block_num: u64,
        helios_block_num: u64,
        starknet_response_block_num: u64,
        expect_request: u64,
    ) -> Result<(BlockId, Felt), jsonrpc::Error> {
        let starknet_server = MockServer::start().await;
        let request_block_num =
            BlockNumber::try_new(requested_starknet_block_num as i64).unwrap();

        let (_mock_guard, context) = setup_test_env(
            &starknet_server,
            helios_block_num,
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
        helios_block_num: u64,
        starknet_response_block_hash: &str,
        starknet_response_block_num: u64,
        expect_request: u64,
    ) -> Result<(BlockId, Felt), jsonrpc::Error> {
        let starknet_server = MockServer::start().await;
        let request_block_hash =
            BlockHash(Felt::try_new(requested_starknet_block_hash).unwrap());

        let (_mock_guard, context) = setup_test_env(
            &starknet_server,
            helios_block_num,
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
        helios_block_num: u64,
        starknet_response_block_num: u64,
        starknet_response_block_hash: &str,
        expect_request: u64,
    ) -> Result<(BlockId, Felt), jsonrpc::Error> {
        let starknet_server = MockServer::start().await;

        let (_mock_guard, context) = setup_test_env(
            &starknet_server,
            helios_block_num,
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
        let helios_state_block_num = 27;
        let starknet_response_block_number = 3;
        let expected_num_request = 1;

        let result = resolve_block_by_number_test(
            requested_starknet_block_num,
            helios_state_block_num,
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
        let helios_state_block_num = 27;
        let starknet_response_block_number = 42;
        let expected_num_request = 0;

        let result = resolve_block_by_number_test(
            requested_starknet_block_num,
            helios_state_block_num,
            starknet_response_block_number,
            expected_num_request,
        )
        .await;

        assert!(result.is_ok());
        let (returned_block, _) = result.unwrap();
        assert!(eq(
            &block_from_number(helios_state_block_num),
            &returned_block
        ));
    }

    #[tokio::test]
    async fn resolve_block_by_number_same_success() {
        let requested_starknet_block_num = 27;
        let helios_state_block_num = 27;
        let starknet_response_block_number = requested_starknet_block_num;
        let expected_num_request = 0;

        let result = resolve_block_by_number_test(
            requested_starknet_block_num,
            helios_state_block_num,
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
        let helios_state_block_num = 27;
        let starknet_response_block_number = 999;
        let expected_num_request = 1;

        let result = resolve_block_by_number_test(
            requested_starknet_block_num,
            helios_state_block_num,
            starknet_response_block_number,
            expected_num_request,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn resolve_block_by_hash_different_success() {
        let requested_starknet_block_hash = "0x3";
        let helios_state_block_num = 27;
        let starknet_response_block_hash = requested_starknet_block_hash;
        let starknet_response_block_num = 3;
        let expected_num_request = 1;

        let result = resolve_block_by_hash_test(
            requested_starknet_block_hash,
            helios_state_block_num,
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
        let helios_state_block_num = 27;
        let starknet_response_block_hash = requested_starknet_block_hash;
        let starknet_response_block_num = 27;
        let expected_num_request = 0;

        let result = resolve_block_by_hash_test(
            requested_starknet_block_hash,
            helios_state_block_num,
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
        let helios_state_block_num = 27;
        let starknet_response_block_hash = requested_starknet_block_hash;
        let starknet_response_block_num = 27;
        let expected_num_request = 1;

        let result = resolve_block_by_hash_test(
            requested_starknet_block_hash,
            helios_state_block_num,
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
        let helios_state_block_num = 27;
        let starknet_response_block_hash = "0xbad";
        let starknet_response_block_num = 3;
        let expected_num_request = 1;

        let result = resolve_block_by_hash_test(
            requested_starknet_block_hash,
            helios_state_block_num,
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
        let helios_block_num = 27;
        let starknet_response_block_num = 3;
        let starknet_response_block_hash = "0x3";
        let expected_num_request = 1;

        let result = resolve_block_id_test(
            request_block.clone(),
            helios_block_num,
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
        let helios_block_num = 27;
        let starknet_response_block_num = 3;
        let starknet_response_block_hash = "0x3";
        let expected_num_request = 1;

        let result = resolve_block_id_test(
            request_block.clone(),
            helios_block_num,
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
        let helios_block_num = 27;
        let starknet_response_block_num = 33;
        let starknet_response_block_hash = "0x33";
        let expected_num_request = 0;

        let result = resolve_block_id_test(
            request_block.clone(),
            helios_block_num,
            starknet_response_block_num,
            starknet_response_block_hash,
            expected_num_request,
        )
        .await;

        assert!(result.is_ok());
        let (returned_block, _) = result.unwrap();
        assert!(eq(&block_from_number(helios_block_num), &returned_block));
    }

    #[tokio::test]
    async fn resolve_block_id_tag_pending_error() {
        let request_block = block_from_tag("pending");
        let helios_block_num = 27;
        let starknet_response_block_num = 33;
        let starknet_response_block_hash = "0x33";
        let expected_num_request = 0;

        let result = resolve_block_id_test(
            request_block.clone(),
            helios_block_num,
            starknet_response_block_num,
            starknet_response_block_hash,
            expected_num_request,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_request() {

        let url = "url";

        let result = handle_request(
            State(Context {
                url: url.to_owned(),
                client: Arc::new(
                    client::Client::new(url, Http(reqwest::ClientBuilder::new().build().unwrap()))
                ),
                state: Arc::new(RwLock::new(ClientState {
                    block_number: 0,
                    block_hash: Felt::try_new("0x0").unwrap(),
                    root: Felt::try_new("0x0").unwrap(),
                }))}
            ),
            Json(super::Request::Single(
                iamgroot::jsonrpc::Request::new(
                    "starknet_getNonce".to_string(),
                    serde_json::json!("param"),
                )
            )),
        ).await;
        assert!(result.is_ok());

        let result = handle_request(
            State(Context {
                url: url.to_owned(),
                client: Arc::new(
                    client::Client::new(url, Http(reqwest::ClientBuilder::new().build().unwrap()))
                ),
                state: Arc::new(RwLock::new(ClientState {
                    block_number: 0,
                    block_hash: Felt::try_new("0x0").unwrap(),
                    root: Felt::try_new("0x0").unwrap(),
                }))}
            ),
            Json(super::Request::Single(
                iamgroot::jsonrpc::Request::new(
                    "starknet_getNonce".to_string(),
                    serde_json::json!("param"),
                ).with_id(iamgroot::jsonrpc::Id::Number(1))
            )),
        ).await;
        assert!(result.is_ok());



        let result = handle_request(
            State(Context {
                url: url.to_owned(),
                client: Arc::new(
                    client::Client::new(url, Http(reqwest::ClientBuilder::new().build().unwrap()))
                ),
                state: Arc::new(RwLock::new(ClientState {
                    block_number: 0,
                    block_hash: Felt::try_new("0x0").unwrap(),
                    root: Felt::try_new("0x0").unwrap(),
                }))}
            ),
            Json(super::Request::Batch(vec![
                iamgroot::jsonrpc::Request::new(
                    "starknet_getNonce".to_string(),
                    serde_json::json!("param"),
                ).with_id(iamgroot::jsonrpc::Id::Number(1))
            ])),
        ).await;
        assert!(result.is_ok());


    }

    #[tokio::test]
    async fn test_into_response() {
        let rpc_error = RpcError(jsonrpc::Error{code: 1, message: "test".to_string()});
        let response: axum::response::Response = rpc_error.into_response();

        assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);

    }

    #[tokio::test]
    async fn test_serve() {

        let client_state = Arc::new(RwLock::new(ClientState {
            block_number: 0,
            block_hash: Felt::try_new("0x0").unwrap(),
            root: Felt::try_new("0x0").unwrap(),
        }));

        assert!(matches!(
            serve("test", "invalid_addr", client_state.clone()).await.unwrap_err(),
            crate::exe::err::Error::Io(..),
        ));

        let result = serve("test", "127.0.0.1:5000", client_state.clone()).await.unwrap();
        assert_eq!(result.port(), 5000);
        result.1.abort();
        result.done().await;

        let result = serve("test", "127.0.0.1:5000", client_state.clone()).await.unwrap();
        result.1.abort();
        result.stop().await;

    }

    #[tokio::test]
    async fn test_get_nonce() {
        let mock_server = MockServer::start().await;
        Mock::given(any())
        .respond_with(|req: &Request| {
            let data: iamgroot::jsonrpc::Request = serde_json::from_slice(&req.body).unwrap();
            let body_string = match data.method.as_str() {
                "starknet_getNonce" => {
                    r#"{
                        "jsonrpc": "2.0",
                        "result": "0x52",
                        "id": 1
                    }"#
                },
                _ => "",
            };

            ResponseTemplate::new(200).set_body_string(body_string)
        })
        .mount(&mock_server)
        .await;

        let state = make_state(0, "0x1");
        let context = make_context("127.0.0.1:3030", &mock_server.uri(), state);

        let result = context.getNonce(
            crate::gen::BlockId::BlockTag(crate::gen::BlockTag::Latest),
            crate::gen::Address(crate::gen::Felt::try_new("0x0").unwrap()),
        ).await;
        assert_eq!(result.unwrap().as_ref(), "0x52");


        mock_server.reset().await;
        Mock::given(any())
        .respond_with(|req: &Request| {
            let data: iamgroot::jsonrpc::Request = serde_json::from_slice(&req.body).unwrap();
            let body_string = match data.method.as_str() {
                "starknet_getNonce" => {
                    r#"{
                        "jsonrpc": "2.0",
                        "error": {
                            "code": 1,
                            "message": "test"
                        },
                        "id": 1
                    }"#
                },
                _ => "",
            };

            ResponseTemplate::new(200).set_body_string(body_string)
        })
        .mount(&mock_server)
        .await;

        let error = context.getNonce(
            crate::gen::BlockId::BlockTag(crate::gen::BlockTag::Latest),
            crate::gen::Address(crate::gen::Felt::try_new("0x0").unwrap()),
        ).await.unwrap_err();

        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());

    }

    async fn add_responses_to_mock_server(
        mock_server: &MockServer,
        responses: std::collections::HashMap<&'static str, &'static str>
    ) {
        Mock::given(any())
        .respond_with(move |req: &Request| {
            let data: iamgroot::jsonrpc::Request = serde_json::from_slice(&req.body).unwrap();
            let method = data.method.as_str();

            let body_string = match responses.get(method) {
                Some(response) => response,
                None => "",
            };
            ResponseTemplate::new(200).set_body_string(body_string)
        })
        .mount(&mock_server)
        .await;
    }
    async fn add_error_response_to_mock_server(
        mock_server: &MockServer,
        method: &'static str,
    ) {
        mock_server.reset().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                method,
                r#"{
                    "jsonrpc": "2.0",
                    "error": {
                        "code": 1,
                        "message": "test"
                    },
                    "id": 1
                }"#,
            )
        ])).await;
    }

    fn verify_testing_rpc_error(error: jsonrpc::Error) {
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());
    }

    #[tokio::test]
    async fn test_version() {
        let mock_server = MockServer::start().await;
        Mock::given(any())
        .respond_with(|req: &Request| {
            let data: iamgroot::jsonrpc::Request = serde_json::from_slice(&req.body).unwrap();
            let body_string = match data.method.as_str() {
                "pathfinder_version" => {
                    r#"{
                        "jsonrpc": "2.0",
                        "result": "v1.0.0",
                        "id": 1
                    }"#
                },
                _ => "",
            };

            ResponseTemplate::new(200).set_body_string(body_string)
        })
        .mount(&mock_server)
        .await;

        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );
        assert_eq!(context.version().await.unwrap(), "v1.0.0".to_string());

        mock_server.reset().await;
        Mock::given(any())
        .respond_with(|req: &Request| {
            let data: iamgroot::jsonrpc::Request = serde_json::from_slice(&req.body).unwrap();
            let body_string = match data.method.as_str() {
                "pathfinder_version" => {
                    r#"{
                        "jsonrpc": "2.0",
                        "error": {
                            "code": 1,
                            "message": "test"
                        },
                        "id": 1
                    }"#
                },
                _ => "",
            };

            ResponseTemplate::new(200).set_body_string(body_string)
        })
        .mount(&mock_server)
        .await;
        let error = context.version().await.unwrap_err();
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());

    }

    #[tokio::test]
    async fn test_get_tx_status() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "pathfinder_getTxStatus",
                r#"{
                    "jsonrpc": "2.0",
                    "result": "RECEIVED",
                    "id": 1
                }"#,
            )
        ])).await;

        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );

        let result = context.getTxStatus(gen::TxnHash(gen::Felt::try_new("0x0").unwrap())).await.unwrap();
        assert_eq!(format!("{:?}", result), "Received");

        mock_server.reset().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "pathfinder_getTxStatus",
                r#"{
                    "jsonrpc": "2.0",
                    "error": {
                        "code": 1,
                        "message": "test"
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let error = context.getTxStatus(gen::TxnHash(gen::Felt::try_new("0x0").unwrap())).await.unwrap_err();
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());

    }
    #[tokio::test]
    async fn test_get_proof() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "pathfinder_getProof",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "class_commitment": "0x2",
                        "state_commitment": "0x157598a5ab5c9f01da1a279e2fba356e3f7d0ee9977c80e32922f2ca5cd5d56",
                        "contract_data": {
                            "class_hash": "0x0",
                            "contract_state_hash_version": "0x0",
                            "nonce": "0x0",
                            "root": "0x1e224db31dfb3e1b8c95670a12f1903d4a32ac7bb83f4b209029e14155bbca9",
                            "storage_proofs": [[
                                {
                                    "edge": {
                                        "child": "0x47616d65206f66204c69666520546f6b656e",
                                        "path": {
                                            "len": 231,
                                            "value": "0x3dfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                                        }
                                    }
                                }
                            ]]
                        },
                        "contract_proof": []
                    },
                    "id": 1
                }"#,
            )
        ])).await;

        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );

        let result = context.getProof(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
            gen::Address(gen::Felt::try_new("0x0").unwrap()),
            vec![],
        ).await.unwrap();

        assert_eq!(
            result.class_commitment.unwrap().as_ref(),
            gen::Felt::try_new("0x2").unwrap().as_ref(),
        );

        mock_server.reset().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "pathfinder_getProof",
                r#"{
                    "jsonrpc": "2.0",
                    "error": {
                        "code": 1,
                        "message": "test"
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let error = context.getProof(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
            gen::Address(gen::Felt::try_new("0x0").unwrap()),
            vec![],
        ).await.unwrap_err();

        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());
    }
    #[tokio::test]
    async fn test_trace_transaction() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_traceTransaction",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "execute_invocation": {
                            "revert_reason": "test"
                        },
                        "execution_resources": {
                            "steps": 1
                        }, 
                        "type": "INVOKE"
                    },
                    "id": 1
                }"#,
            )
        ])).await;

        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );
        let result = context.traceTransaction(
            gen::TxnHash(gen::Felt::try_new("0x0").unwrap()),
        ).await.unwrap();
        assert!(matches!(result, gen::TransactionTrace::InvokeTxnTrace(..)));

        mock_server.reset().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_traceTransaction",
                r#"{
                    "jsonrpc": "2.0",
                    "error": {
                        "code": 1,
                        "message": "test"
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let error = context.traceTransaction(
            gen::TxnHash(gen::Felt::try_new("0x0").unwrap()),
        ).await.unwrap_err();

        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());
    }

    #[tokio::test]
    async fn test_trace_block_transactions(){
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_traceBlockTransactions",
                r#"{
                    "jsonrpc": "2.0",
                    "result": [
                    ],
                    "id": 1
                }"#,
            )
        ])).await;

        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );

        context.traceBlockTransactions(gen::BlockId::BlockTag(gen::BlockTag::Latest)).await.unwrap();
        mock_server.reset().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_traceBlockTransactions",
                r#"{
                    "jsonrpc": "2.0",
                    "error": {
                        "code": 1,
                        "message": "test"
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let error = context.traceBlockTransactions(gen::BlockId::BlockTag(gen::BlockTag::Latest)).await.unwrap_err();
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());
    }

    #[tokio::test]
    async fn test_syncing() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_syncing",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "current_block_hash": "0x3edaeda7f911f9753348bc71b01e661e22b7cc8bc2d1dbae394ab98f75556a1",
                        "current_block_num": 938534,
                        "highest_block_hash": "0x3edaeda7f911f9753348bc71b01e661e22b7cc8bc2d1dbae394ab98f75556a1",
                        "highest_block_num": 938534,
                        "starting_block_hash": "0x5bff350a33dd2a725106697e6cd5c9ef08cf7a16129a6445f2592f9346be97c",
                        "starting_block_num": 854876
                    },
                    "id": 1
                }"#,
            )
        ])).await;

        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );
        let result = context.syncing().await.unwrap();
        assert!(matches!(result, gen::SyncingResult::SyncStatus(..)));

        add_error_response_to_mock_server(&mock_server, "starknet_syncing").await;
        let error = context.syncing().await.unwrap_err();
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());
    }

    #[tokio::test]
    async fn test_spec_version() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_specVersion",
                r#"{
                    "jsonrpc": "2.0",
                    "result": "v0.7",
                    "id": 1
                }"#,
            )
        ])).await;

        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );

        let result = context.specVersion().await;
        assert_eq!(result.unwrap(), "v0.7");

        add_error_response_to_mock_server(&mock_server, "starknet_specVersion").await;
        let error = context.specVersion().await.unwrap_err();
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());
    }

    #[tokio::test]
    async fn test_simulate_transactions() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_simulateTransactions",
                r#"{
                    "jsonrpc": "2.0",
                    "result": [],
                    "id": 1
                }"#,
            )
        ])).await;

        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );

        let _result = context.simulateTransactions(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
            vec![],
            vec![],
        ).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_simulateTransactions").await;
        let error = context.simulateTransactions(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
            vec![],
            vec![],
        ).await.unwrap_err();
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());
    }

    #[tokio::test]
    async fn test_get_transaction_status() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getTransactionStatus",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "finality_status": "RECEIVED"
                    },
                    "id": 1
                }"#,
            )
        ])).await;

        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );
        let result = context.getTransactionStatus(gen::TxnHash(gen::Felt::try_new("0x0").unwrap())).await.unwrap();
        assert_eq!(format!("{:?}", result.finality_status), "Received");

        add_error_response_to_mock_server(&mock_server, "starknet_getTransactionStatus").await;
        let error = context.getTransactionStatus(gen::TxnHash(gen::Felt::try_new("0x0").unwrap())).await.unwrap_err();
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());
    }

    #[tokio::test]
    async fn test_get_transaction_receipt() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getTransactionReceipt",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "actual_fee": {
                            "amount": "0x12435cd0d61f0440",
                            "unit": "FRI"
                        },
                        "block_hash": "0x1c73712cf9df2f668f6901dfc7c0dd9699297ceb42365bc45e50dcd7c0a7c4c",
                        "block_number": 589335,
                        "events": [],
                        "execution_resources": {
                        "data_availability": {
                            "l1_data_gas": 0,
                            "l1_gas": 0
                        },
                        "memory_holes": 2726,
                        "pedersen_builtin_applications": 28,
                        "range_check_builtin_applications": 523,
                        "steps": 13602
                        },
                        "execution_status": "SUCCEEDED",
                        "finality_status": "ACCEPTED_ON_L1",
                        "messages_sent": [],
                        "transaction_hash": "0x778bed983dc662706c623db1b339e2674ebb35da917897738a1a6360186df25",
                        "type": "INVOKE"
                    },
                    "id": 1
                }"#,
            )
        ])).await;

        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );
        let _result = context.getTransactionReceipt(gen::TxnHash(gen::Felt::try_new("0x0").unwrap())).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_getTransactionReceipt").await;
        let error = context.getTransactionReceipt(gen::TxnHash(gen::Felt::try_new("0x0").unwrap())).await.unwrap_err();
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());
    }

    #[tokio::test]
    async fn test_get_transaction_by_hash() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getTransactionByHash",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "calldata": [
                            "0x1",
                            "0x63d69ae657bd2f40337c39bf35a870ac27ddf91e6623c2f52529db4c1619a51",
                            "0x2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
                            "0x3",
                            "0x70bb25d9764abbcc56b1da1fda8ffd3c51366dd4c823b8199ffc1540c31a31a",
                            "0xee6b280",
                            "0x0"
                        ],
                        "max_fee": "0x48b175f80d56e",
                        "nonce": "0x4b",
                        "sender_address": "0x70bb25d9764abbcc56b1da1fda8ffd3c51366dd4c823b8199ffc1540c31a31a",
                        "signature": [
                            "0x5c3bfbf73dc20a87c67c0801572343774a6320a05ebbaf6d98cbb52325116ac",
                            "0x4bd26d4f10d4bf70b5585d33877825d28f048d7e8f9e2c28877be2cb6e0efde"
                        ],
                        "transaction_hash": "0x64f7c084d9cba540cba343f3ec1b69a06bd9169c9016e518d06d418003a31fd",
                        "type": "INVOKE",
                        "version": "0x1"
                    },
                    "id": 1
                }"#,
            )
        ])).await;

        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );
        let _result = context.getTransactionByHash(gen::TxnHash(gen::Felt::try_new("0x0").unwrap())).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_getTransactionByHash").await;
        let error = context.getTransactionByHash(gen::TxnHash(gen::Felt::try_new("0x0").unwrap())).await.unwrap_err();
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());

    }

    #[tokio::test]
    async fn test_get_transaction_by_block_id_and_index() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getTransactionByBlockIdAndIndex",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "account_deployment_data": [],
                        "calldata": [],
                        "fee_data_availability_mode": "L1",
                        "nonce": "0x84",
                        "nonce_data_availability_mode": "L1",
                        "paymaster_data": [],
                        "resource_bounds": {
                            "l1_gas": {
                                "max_amount": "0x1f9d",
                                "max_price_per_unit": "0x5c728233ce27"
                            },
                            "l2_gas": {
                                "max_amount": "0x0",
                                "max_price_per_unit": "0x0"
                            }
                        },
                        "sender_address": "0x53b37904c8c85d47eec549f0c3e98ffe7ee26cc3671f25b2a62ab9af95a3961",
                        "signature": [
                            "0x1",
                            "0x0",
                            "0x46395696e7f5ceb1ac96287cb92a6867a87fd5dd776de9b14363abdd7d1a36b",
                            "0x70b16f77486c963232a8c64c554b9fb4d6d3cee5efb7ff826e622405e7523c5",
                            "0xf455078cc11df48c9a53790b5b06ca2dad17e5afdd9d8e9a5862e74fc1832b"
                        ],
                        "tip": "0x0",
                        "transaction_hash": "0x3ffabde8beb43001a6cf2fa760357be2754051ba0458c607acdee7e212c3d41",
                        "type": "INVOKE",
                        "version": "0x3"
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );

        let _result = context.getTransactionByBlockIdAndIndex(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
            gen::GetTransactionByBlockIdAndIndexIndex::try_new(1).unwrap(),
        ).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_getTransactionByBlockIdAndIndex").await;
        let error = context.getTransactionByBlockIdAndIndex(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
            gen::GetTransactionByBlockIdAndIndexIndex::try_new(1).unwrap(),
        ).await.unwrap_err();
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());
    }

    #[tokio::test]
    async fn test_get_state_update() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getStateUpdate",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "block_hash": "0x7e806bc9c27a38fa9eca4724cd54619f850354985dcc12fd68165b43dbeea68",
                        "new_root": "0xe12d4d041c79a2711dd3ecf27a2228054101839c22f1bc17b06e57c840b8a8",
                        "old_root": "0x79435b4219750c284eff3e270d35d79cf20b27e1173311a52bac7e4ea41c67b",
                        "state_diff": {
                            "declared_classes": [],
                            "deployed_contracts": [
                                {
                                "address": "0x665a9e6b25e9def04f5310938f67af9e1e97f36009e5f38a4bc7b7eda28038c",
                                "class_hash": "0x2c8c7e6fbcfb3e8e15a46648e8914c6aa1fc506fc1e7fb3d1e19630716174bc"
                                }
                            ],
                            "deprecated_declared_classes": [],
                            "nonces": [
                                {
                                    "contract_address": "0x47304606d2ffeddcc0bf4943f32f9af46ff8659355ca37721d8d17db2157882",
                                    "nonce": "0x81"
                                }
                            ],
                            "replaced_classes": [],
                            "storage_diffs": []
                        }
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        ); 

        let _result = context.getStateUpdate(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_getStateUpdate").await;
        let error = context.getStateUpdate(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap_err();
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());
    }

    #[tokio::test]
    async fn test_get_storage_at() {
         let mock_server = MockServer::start().await;
         add_responses_to_mock_server(&mock_server, HashMap::from([
             (
                 "starknet_getStorageAt",
                 r#"{
                     "jsonrpc": "2.0",
                     "result": "0x0",
                     "id": 1
                 }"#,
             ),
             (
                "pathfinder_getProof",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "class_commitment": "0x0",
                        "state_commitment": "0x0",
                        "contract_data": {
                            "class_hash": "0x0",
                            "contract_state_hash_version": "0x0",
                            "nonce": "0x0",
                            "root": "0x0",
                            "storage_proofs": [[]]
                        },
                        "contract_proof": []
                    },
                    "id": 1
                }"#,
             )
         ])).await;
         let context = make_context(
             "127.0.0.1:3030",
             &mock_server.uri(),
             ClientState {
                block_number: 0,
                block_hash: Felt::try_new("0x0").unwrap(),
                root: Felt::try_new("0x0").unwrap(),
            }
         );
         let _result = context.getStorageAt(
            gen::Address(gen::Felt::try_new("0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1").unwrap()),
            gen::StorageKey::try_new("0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1").unwrap(),
            gen::BlockId::BlockHash { block_hash: gen::BlockHash(gen::Felt::try_new("0x0").unwrap()) }
        ).await.unwrap_err();
    }

    #[tokio::test]
    async fn test_get_events() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getEvents",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "events": []
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        ); 
        let events_filter = gen::GetEventsFilter{
            event_filter: gen::EventFilter{
                address: None,
                from_block: Some(gen::BlockId::BlockTag(gen::BlockTag::Latest)),
                to_block: Some(gen::BlockId::BlockTag(gen::BlockTag::Latest)),
                keys: None,
            },
            result_page_request: gen::ResultPageRequest{
                chunk_size: gen::ResultPageRequestChunkSize::try_new(1).unwrap(),
                continuation_token: None,
            }
        };

        let _result = context.getEvents(events_filter.clone()).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_getEvents").await;
        let error = context.getEvents(events_filter).await.unwrap_err();
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "test".to_string());
    }

    #[tokio::test]
    async fn test_get_class_hash_at() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getClassHashAt",
                r#"{
                    "jsonrpc": "2.0",
                    "result": "0x0",
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );

        let _result = context.getClassHashAt(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
            gen::Address(gen::Felt::try_new("0x0").unwrap()),
        ).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_getClassHashAt").await;
        let error = context.getClassHashAt(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
            gen::Address(gen::Felt::try_new("0x0").unwrap()),
        ).await.unwrap_err();
        verify_testing_rpc_error(error);
    }

    #[tokio::test]
    async fn test_get_class_at() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getClassAt",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "abi": "",
                        "contract_class_version": "0.1.0",
                        "entry_points_by_type": {
                            "CONSTRUCTOR": [],
                            "EXTERNAL": [],
                            "L1_HANDLER": []
                        },
                        "sierra_program": []
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        ); 

        let _result = context.getClassAt(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
            gen::Address(gen::Felt::try_new("0x0").unwrap()),
        ).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_getClassAt").await;
        let error = context.getClassAt(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
            gen::Address(gen::Felt::try_new("0x0").unwrap()),
        ).await.unwrap_err();
        verify_testing_rpc_error(error);
    }

    #[tokio::test]
    async fn test_get_class() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getClass",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "abi": "",
                        "contract_class_version": "0.1.0",
                        "entry_points_by_type": {
                            "CONSTRUCTOR": [],
                            "EXTERNAL": [],
                            "L1_HANDLER": []
                        },
                        "sierra_program": []
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        ); 
        let _result = context.getClass(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
            gen::Felt::try_new("0x0").unwrap(),
        ).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_getClass").await;
        let error = context.getClass(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
            gen::Felt::try_new("0x0").unwrap(),
        ).await.unwrap_err();
        verify_testing_rpc_error(error);
    }

    #[tokio::test]
    async fn test_get_block_with_txs() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getBlockWithTxs",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "block_hash": "0x630a2b189fd443a4964ad00961f63c12f4c5b39ebd2ba297a868dbf61a8dd26",
                        "block_number": 938782,
                        "l1_da_mode": "BLOB",
                        "l1_data_gas_price": {
                        "price_in_fri": "0x2287b917d1e8",
                        "price_in_wei": "0x189e94d87"
                        },
                        "l1_gas_price": {
                        "price_in_fri": "0x2d10f388b88d",
                        "price_in_wei": "0x2021a9826"
                        },
                        "new_root": "0x1b25c42ee26ff16ca5066508336fdf5cf66f994df21d89b88363e9e94611ad8",
                        "parent_hash": "0x627cc3b661a0f3687cdc40851e760fa4b5a6925e2ccf7fa89b56acfb25aa13d",
                        "sequencer_address": "0x1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8",
                        "starknet_version": "0.13.3",
                        "status": "ACCEPTED_ON_L2",
                        "timestamp": 1732829253,
                        "transactions": []
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );

        let _result = context.getBlockWithTxs(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_getBlockWithTxs").await;
        let error = context.getBlockWithTxs(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap_err();
        verify_testing_rpc_error(error);
    }

    #[tokio::test]
    async fn test_get_block_with_tx_hashes() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getBlockWithTxHashes",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "block_hash": "0x693354bfe79d7201c2f1bbcb5fbb0a9d079d4cf41351e33eb6cc45a2d21b906",
                        "block_number": 938826,
                        "l1_da_mode": "BLOB",
                        "l1_data_gas_price": {
                            "price_in_fri": "0x16b604d8204e",
                            "price_in_wei": "0x103aecfbf"
                        },
                        "l1_gas_price": {
                            "price_in_fri": "0x2c85f68ce60c",
                            "price_in_wei": "0x1fd16e561"
                        },
                        "new_root": "0x19468069c1a1be8fc680db7c24516c3f2170739d42d2ff37a48e9b44dae7408",
                        "parent_hash": "0x58a2df3f2647a4506b3d3f98b93c8ddfee3a0bb349dc99ac3878486966d9ca5",
                        "sequencer_address": "0x1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8",
                        "starknet_version": "0.13.3",
                        "status": "ACCEPTED_ON_L2",
                        "timestamp": 1732830628,
                        "transactions": [
                            "0x70b32e6fb2eeb80992ebe34c4c0f04d04ee4b056735370cca36c3420ed9891b"
                        ]
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );
        let _result = context.getBlockWithTxHashes(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_getBlockWithTxHashes").await;
        let error = context.getBlockWithTxHashes(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap_err();
        verify_testing_rpc_error(error);
    }

    #[tokio::test]
    async fn test_get_block_with_receipts() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getBlockWithReceipts",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "block_hash": "0x6212497a914ba848ce94c1796bf53e16b98d34ccc637261060bacad095d0ed0",
                        "block_number": 938853,
                        "l1_da_mode": "BLOB",
                        "l1_data_gas_price": {
                        "price_in_fri": "0x11aef0af6505",
                        "price_in_wei": "0xcd688798"
                        },
                        "l1_gas_price": {
                        "price_in_fri": "0x2e4789ca9758",
                        "price_in_wei": "0x21993f020"
                        },
                        "new_root": "0x116f62f385f4145ea628bf8a391c00e05fc9e929eaf784ffda1c407f07d65e4",
                        "parent_hash": "0x7b4b4d40ffa618bd87d983c0fac56a57fac9c74ff05988e2c5ea40b05089d25",
                        "sequencer_address": "0x1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8",
                        "starknet_version": "0.13.3",
                        "status": "ACCEPTED_ON_L2",
                        "timestamp": 1732831464,
                        "transactions": [
                        ]
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        ); 
        let _result = context.getBlockWithReceipts(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_getBlockWithReceipts").await;
        let error = context.getBlockWithReceipts(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap_err();
        verify_testing_rpc_error(error);
    }

    #[tokio::test]
    async fn test_get_block_transaction_count() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getBlockTransactionCount",
                r#"{
                    "jsonrpc": "2.0",
                    "result": 100,
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        ); 
        let _result = context.getBlockTransactionCount(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_getBlockTransactionCount").await;
        let error = context.getBlockTransactionCount(
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap_err();
        verify_testing_rpc_error(error);
    }

    #[tokio::test]
    async fn test_estimate_message_fee(){
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_estimateMessageFee",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "data_gas_consumed": "0x0",
                        "data_gas_price": "0x1",
                        "gas_consumed": "0x41d1",
                        "gas_price": "0x67edb4f57",
                        "overall_fee": "0x1ab834030dd07",
                        "unit": "WEI"
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        ); 

        let _result = context.estimateMessageFee(
            gen::MsgFromL1{
                entry_point_selector: Felt::try_new("0x0").unwrap(),
                from_address: gen::EthAddress::try_new("0x0000000000000000000000000000000000000001").unwrap(),
                payload: vec![],
                to_address: gen::Address(gen::Felt::try_new("0x0").unwrap()),

            },
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap();
    }

    #[tokio::test]
    async fn test_estimate_fee() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_estimateFee",
                r#"{
                    "jsonrpc": "2.0",
                    "result": [
                        {
                            "data_gas_consumed": "0x0",
                            "data_gas_price": "0x1",
                            "gas_consumed": "0x41d1",
                            "gas_price": "0x67edb4f57",
                            "overall_fee": "0x1ab834030dd07",
                            "unit": "WEI"
                        }
                    ],
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        ); 
        let _result = context.estimateFee(
            vec![],
            vec![],
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap();
        add_error_response_to_mock_server(&mock_server, "starknet_estimateFee").await;
        let error = context.estimateFee(
            vec![],
            vec![],
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap_err();
        verify_testing_rpc_error(error);
    }

    #[tokio::test]
    async fn test_chain_id() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_chainId",
                r#"{
                    "jsonrpc": "2.0",
                    "result": "0xffff",
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        ); 

        let _result = context.chainId().await.unwrap();
        add_error_response_to_mock_server(&mock_server, "starknet_chainId").await;
        let error = context.chainId().await.unwrap_err();
        verify_testing_rpc_error(error);
    }

    #[tokio::test]
    async fn test_call() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getClassHashAt",
                r#"{
                    "jsonrpc": "2.0",
                    "result": "0x1",
                    "id": 1
                }"#,
            ),
            (
                "starknet_getClass",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "abi": "",
                        "contract_class_version": "0.1.0",
                        "entry_points_by_type": {
                            "CONSTRUCTOR": [],
                            "EXTERNAL": [],
                            "L1_HANDLER": []
                        },
                        "sierra_program": []
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "http://127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );

        context.call(
            gen::FunctionCall{
                calldata: vec![],
                contract_address: gen::Address(gen::Felt::try_new("0x0").unwrap()),
                entry_point_selector: gen::Felt::try_new("0x0").unwrap(),
            },
            gen::BlockId::BlockTag(gen::BlockTag::Latest),
        ).await.unwrap_err();

    }

    #[tokio::test]
    async fn test_block_number() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_blockNumber",
                r#"{
                    "jsonrpc": "2.0",
                    "result": 1,
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );
        let _result = context.blockNumber().await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_blockNumber").await;
        let error = context.blockNumber().await.unwrap_err();
        verify_testing_rpc_error(error);
    }

    #[tokio::test]
    async fn test_block_hash_and_number() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_blockHashAndNumber",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "block_hash": "0x33f95632ef39564f11e2d0e3ea2fca3e8cc9d62803454493e004273df676d67",
                        "block_number": 1 
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );
        let _result = context.blockHashAndNumber().await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_blockHashAndNumber").await;
        let error = context.blockHashAndNumber().await.unwrap_err();
        verify_testing_rpc_error(error);
    }
    #[tokio::test]
    async fn test_add_invoke_transaction() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_addInvokeTransaction",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "transaction_hash": "0x33f95632ef39564f11e2d0e3ea2fca3e8cc9d62803454493e004273df676d67"
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );
        let invoke_transaction = gen::BroadcastedInvokeTxn(
                gen::InvokeTxn::InvokeTxnV1(InvokeTxnV1{
                    calldata: vec![],
                    sender_address: gen::Address(gen::Felt::try_new("0x0").unwrap()),
                    max_fee: gen::Felt::try_new("0x0").unwrap(),
                    nonce: Felt::try_new("0x1").unwrap(),
                    r#type: gen::InvokeTxnV1Type::Invoke,
                    signature: vec![],
                    version: gen::InvokeTxnV1Version::V0x1,
                }
        ));
        let _result = context.addInvokeTransaction(invoke_transaction.clone()).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_addInvokeTransaction").await;
        let error = context.addInvokeTransaction(invoke_transaction.clone()).await.unwrap_err();
        verify_testing_rpc_error(error);
    }

    #[tokio::test]
    async fn test_add_deploy_account_transaction() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_addDeployAccountTransaction",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "transaction_hash": "0x33f95632ef39564f11e2d0e3ea2fca3e8cc9d62803454493e004273df676d67",
                        "contract_address": "0x33f95632ef39564f11e2d0e3ea2fca3e8cc9d62803454493e004273df676d67"
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );
        let deploy_account_transaction = gen::BroadcastedDeployAccountTxn(
                gen::DeployAccountTxn::DeployAccountTxnV1(DeployAccountTxnV1{
                    class_hash: gen::Felt::try_new("0x0").unwrap(),
                    constructor_calldata: vec![],
                    contract_address_salt: gen::Felt::try_new("0x0").unwrap(),
                    max_fee: gen::Felt::try_new("0x0").unwrap(),
                    nonce: Felt::try_new("0x1").unwrap(),
                    r#type: gen::DeployAccountTxnV1Type::DeployAccount,
                    signature: vec![],
                    version: gen::DeployAccountTxnV1Version::V0x1,
                }
        ));
        let _result = context.addDeployAccountTransaction(deploy_account_transaction.clone()).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_addDeployAccountTransaction").await;
        let error = context.addDeployAccountTransaction(deploy_account_transaction.clone()).await.unwrap_err();
        verify_testing_rpc_error(error);
    }

    #[tokio::test]
    async fn test_add_declare_transaction() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_addDeclareTransaction",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "transaction_hash": "0x33f95632ef39564f11e2d0e3ea2fca3e8cc9d62803454493e004273df676d67",
                        "class_hash": "0x33f95632ef39564f11e2d0e3ea2fca3e8cc9d62803454493e004273df676d67"
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );

        let declare_transaction = gen::BroadcastedDeclareTxn::BroadcastedDeclareTxnV1(gen::BroadcastedDeclareTxnV1{
            contract_class: gen::DeprecatedContractClass{
                abi: None,
                entry_points_by_type: gen::DeprecatedContractClassEntryPointsByType{
                    constructor: None,
                    external: None,
                    l1_handler: None,
                },
                program: gen::DeprecatedContractClassProgram::try_new("aaa=").unwrap(),
            },
            max_fee: gen::Felt::try_new("0x0").unwrap(),
            nonce: Felt::try_new("0x1").unwrap(),
            r#type: gen::BroadcastedDeclareTxnV1Type::Declare,
            sender_address: gen::Address(gen::Felt::try_new("0x0").unwrap()),
            signature: vec![],
            version: gen::BroadcastedDeclareTxnV1Version::V0x1,
        });
        let _result = context.addDeclareTransaction(declare_transaction.clone()).await.unwrap();

        add_error_response_to_mock_server(&mock_server, "starknet_addDeclareTransaction").await;
        let error = context.addDeclareTransaction(declare_transaction.clone()).await.unwrap_err();
        verify_testing_rpc_error(error);
    }

    #[tokio::test]
    async fn test_context_get_state_for_pending_block() {
        let mock_server = MockServer::start().await;
        add_responses_to_mock_server(&mock_server, HashMap::from([
            (
                "starknet_getBlockWithTxHashes",
                r#"{
                    "jsonrpc": "2.0",
                    "result": {
                        "l1_da_mode": "BLOB",
                        "l1_data_gas_price": {
                            "price_in_fri": "0x16b604d8204e",
                            "price_in_wei": "0x103aecfbf"
                        },
                        "l1_gas_price": {
                            "price_in_fri": "0x2c85f68ce60c",
                            "price_in_wei": "0x1fd16e561"
                        },
                        "parent_hash": "0x58a2df3f2647a4506b3d3f98b93c8ddfee3a0bb349dc99ac3878486966d9ca5",
                        "sequencer_address": "0x1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8",
                        "starknet_version": "0.13.3",
                        "timestamp": 1732830628,
                        "transactions": [
                            "0x70b32e6fb2eeb80992ebe34c4c0f04d04ee4b056735370cca36c3420ed9891b"
                        ]
                    },
                    "id": 1
                }"#,
            )
        ])).await;
        let context = make_context(
            "127.0.0.1:3030",
            &mock_server.uri(),
            make_state(0, "0x1"),
        );

        context.get_state(gen::BlockId::BlockTag(gen::BlockTag::Latest)).await.unwrap_err();
    }
}
