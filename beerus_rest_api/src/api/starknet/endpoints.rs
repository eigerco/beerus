use std::str::FromStr;

use crate::api::starknet::resp::QueryStateRootResponse;

use super::resp::QueryContractViewResponse;
use crate::api::ApiResponse;
use beerus_core::lightclient::beerus::BeerusLightClient;
use eyre::Result;
use log::debug;
use rocket::{get, State};
use starknet::core::types::FieldElement;

#[get("/starknet/state/root")]
pub async fn query_starknet_state_root(
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryStateRootResponse> {
    ApiResponse::from_result(query_starknet_state_root_inner(beerus).await)
}

/// Query the state root of StarkNet.
pub async fn query_starknet_state_root_inner(
    beerus: &State<BeerusLightClient>,
) -> Result<QueryStateRootResponse> {
    debug!("Querying StarkNet state root");
    // Call the StarkNet contract to get the state root.
    let state_root = beerus.starknet_state_root().await?;
    Ok(QueryStateRootResponse {
        state_root: state_root.to_string(),
    })
}

#[get("/starknet/<contract>/<selector>?<calldata>")]
pub async fn query_starknet_contract_view(
    beerus: &State<BeerusLightClient>,
    contract: String,
    selector: String,
    calldata: Option<String>,
) -> ApiResponse<QueryContractViewResponse> {
    ApiResponse::from_result(
        query_starknet_contract_view_inner(beerus, contract, selector, calldata).await,
    )
}

/// Query a contract view.
pub async fn query_starknet_contract_view_inner(
    beerus: &State<BeerusLightClient>,
    contract_address: String,
    selector: String,
    calldata: Option<String>,
) -> Result<QueryContractViewResponse> {
    debug!("Querying StarkNet contract view");
    let contract_address = FieldElement::from_str(&contract_address)?;
    let selector = FieldElement::from_str(&selector)?;
    let mut felt_calldata = vec![];
    match calldata {
        Some(calldata) => {
            let calldata: Vec<&str> = calldata.split(',').into_iter().collect();
            for i in 0..calldata.len() {
                felt_calldata.push(FieldElement::from_str(calldata[i])?);
            }
        }
        None => {}
    }

    // Call the StarkNet contract to get the state root.
    let view = beerus
        .starknet_call_contract(contract_address, selector, felt_calldata)
        .await?;

    Ok(QueryContractViewResponse {
        result: view.into_iter().map(|x| x.to_string()).collect(),
    })
}
