use axum::{
    extract::State, response::IntoResponse, routing::post, Json, Router,
};
use beerus_core::client::NodeData;
use iamgroot::jsonrpc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::{oneshot, RwLock},
    task::JoinHandle,
};

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
    bind: A,
    node: Arc<RwLock<NodeData>>,
) -> Result<Server, Error> {
    let listener = TcpListener::bind(bind).await?;
    let server = serve_on(url, listener, node)?;
    Ok(server)
}

fn serve_on(
    url: &str,
    listener: TcpListener,
    node: Arc<RwLock<NodeData>>,
) -> Result<Server, Error> {
    const DEFAULT_TIMEOUT: std::time::Duration =
        std::time::Duration::from_secs(30);
    let client = reqwest::ClientBuilder::new()
        .connect_timeout(DEFAULT_TIMEOUT)
        .timeout(DEFAULT_TIMEOUT)
        .build()?;

    let ctx = Context {
        client: Arc::new(gen::client::Client::with_client(url, client)),
        url: url.to_owned(),
        node,
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
    client: Arc<gen::client::Client>,
    url: String,
    node: Arc<RwLock<NodeData>>,
}

impl Context {
    pub async fn get_proper_block_id(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<BlockId, jsonrpc::Error> {
        let l1_block_num = self.node.read().await.l1_block_number as i64;
        match block_id {
            gen::BlockId::BlockNumber { block_number } => {
                let mut block_num = l1_block_num;
                let user_request_block_num = *block_number.as_ref();
                if user_request_block_num < block_num {
                    block_num = user_request_block_num;
                }
                Ok(BlockId::BlockNumber {
                    block_number: BlockNumber::try_new(block_num)?,
                })
            }
            gen::BlockId::BlockHash { block_hash } => {
                Ok(gen::BlockId::BlockHash { block_hash })
            }
            _ => Ok(BlockId::BlockNumber {
                block_number: BlockNumber::try_new(l1_block_num)?,
            }),
        }
    }

    pub async fn get_l1_root(
        &self,
    ) -> std::result::Result<Felt, jsonrpc::Error> {
        Felt::try_new(&format!("0x{:x}", self.node.read().await.l1_state_root))
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

        let call_info = tokio::task::spawn_blocking(move || {
            crate::exe::call(&client, request)
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
        let block_id = self.get_proper_block_id(block_id).await?;
        let l1_root = self.get_l1_root().await?;

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

        let mut proof = self
            .client
            .getProof(block_id, contract_address.clone(), vec![key.clone()])
            .await?;
        tracing::info!(?proof, "getStorageAt");
        proof.verify(l1_root, contract_address, key, result.clone())?;
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
    use std::sync::Arc;

    use beerus_core::client::NodeData;
    use tokio::sync::RwLock;

    use crate::rpc::{BlockHash, BlockId, BlockNumber, BlockTag, Felt};

    use super::{client::Client, Context};

    #[tokio::test]
    async fn returns_latest_block_number() {
        let context = Context {
            client: Arc::new(Client::new("")),
            url: "".to_string(),
            node: Arc::new(RwLock::new(NodeData {
                l1_block_number: 3,
                ..Default::default()
            })),
        };
        assert_eq!(
            context
                .get_proper_block_id(BlockId::BlockNumber {
                    block_number: BlockNumber::try_new(27).unwrap()
                })
                .await
                .unwrap(),
            BlockId::BlockNumber {
                block_number: BlockNumber::try_new(3).unwrap()
            }
        );
    }

    #[tokio::test]
    async fn returns_historical_block_number() {
        let context = Context {
            client: Arc::new(Client::new("")),
            url: "".to_string(),
            node: Arc::new(RwLock::new(NodeData {
                l1_block_number: 42,
                ..Default::default()
            })),
        };
        assert_eq!(
            context
                .get_proper_block_id(BlockId::BlockNumber {
                    block_number: BlockNumber::try_new(27).unwrap()
                })
                .await
                .unwrap(),
            BlockId::BlockNumber {
                block_number: BlockNumber::try_new(27).unwrap()
            }
        );
    }

    #[tokio::test]
    async fn on_latest_and_pending_returns_latest_block_number() {
        let context = Context {
            client: Arc::new(Client::new("")),
            url: "".to_string(),
            node: Arc::new(RwLock::new(NodeData {
                l1_block_number: 42,
                ..Default::default()
            })),
        };
        assert_eq!(
            context
                .get_proper_block_id(BlockId::BlockTag(BlockTag::Latest))
                .await
                .unwrap(),
            BlockId::BlockNumber {
                block_number: BlockNumber::try_new(42).unwrap()
            }
        );
        assert_eq!(
            context
                .get_proper_block_id(BlockId::BlockTag(BlockTag::Pending))
                .await
                .unwrap(),
            BlockId::BlockNumber {
                block_number: BlockNumber::try_new(42).unwrap()
            }
        );
    }

    #[tokio::test]
    async fn on_hash_block_returns_hash_block() {
        let context = Context {
            client: Arc::new(Client::new("")),
            url: "".to_string(),
            node: Arc::new(RwLock::new(NodeData {
                l1_block_number: 42,
                ..Default::default()
            })),
        };
        assert_eq!(
            context
                .get_proper_block_id(BlockId::BlockHash {
                    block_hash: BlockHash(Felt::try_new("0xabc").unwrap())
                })
                .await
                .unwrap(),
            BlockId::BlockHash {
                block_hash: BlockHash(Felt::try_new("0xabc").unwrap())
            }
        );
    }
}
