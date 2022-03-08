// Map the views and types

use serde::{Deserialize, Serialize};

pub type BlockChangeResult =
    near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockByTypeResponse;
pub type ChangeResult = near_primitives::views::StateChangesView;
pub type BlockResult = near_primitives::views::BlockView;
pub type ChunkId = near_primitives::hash::CryptoHash;
pub type CryptoHash = near_primitives::hash::CryptoHash;
pub type NodeStatusResult = near_primitives::views::StatusResponse;
pub type FinalExecutionOutcome = near_primitives::views::FinalExecutionOutcomeView;
pub type ChunkResult = near_primitives::views::ChunkView;
pub type SignedTransaction = near_primitives::transaction::SignedTransaction;
pub type BlockId = near_primitives::types::BlockId;
pub type EpochValidatorInfo = near_primitives::views::EpochValidatorInfo;
pub type BlockReference = near_primitives::types::BlockReference;
pub type QueryResponseKind = near_jsonrpc_primitives::types::query::QueryResponseKind;
pub type RpcQueryRequest = near_jsonrpc_primitives::types::query::RpcQueryRequest;
pub type GasPrice = near_primitives::views::GasPriceView;
pub type LightClientProof =
    near_jsonrpc_primitives::types::light_client::RpcLightClientExecutionProofResponse;
pub type LightClientProofRequest =
    near_jsonrpc_primitives::types::light_client::RpcLightClientExecutionProofRequest;

#[derive(Serialize, Deserialize)]
pub struct NearProtocolConfig {
    pub runtime_config: near_primitives::runtime::config::RuntimeConfig,
}

#[derive(Serialize, Deserialize)]
pub struct AccessKeyWithPublicKey {
    pub account_id: near_account_id::AccountId,
    pub public_key: near_crypto::PublicKey,
}
