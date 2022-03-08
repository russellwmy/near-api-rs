use near_primitives::network::PeerId;
use near_primitives::types::AccountId;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcPeerInfo {
    pub id: PeerId,
    pub addr: Option<SocketAddr>,
    pub account_id: Option<AccountId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcKnownProducer {
    pub account_id: AccountId,
    pub addr: Option<SocketAddr>,
    pub peer_id: PeerId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcNetworkInfoResponse {
    pub active_peers: Vec<RpcPeerInfo>,
    pub num_active_peers: usize,
    pub peer_max_count: u32,
    pub sent_bytes_per_sec: u64,
    pub received_bytes_per_sec: u64,
    /// Accounts of known block and chunk producers from routing table.
    pub known_producers: Vec<RpcKnownProducer>,
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[serde(tag = "name", content = "info", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RpcNetworkInfoError {
    #[error("Internal error: {error_message}")]
    InternalError { error_message: String },
}

impl From<String> for RpcNetworkInfoError {
    fn from(error_message: String) -> Self {
        Self::InternalError { error_message }
    }
}

impl From<RpcNetworkInfoError> for crate::errors::RpcError {
    fn from(error: RpcNetworkInfoError) -> Self {
        let error_data = match serde_json::to_value(error) {
            Ok(value) => value,
            Err(err) => {
                return Self::new_internal_error(
                    None,
                    format!("Failed to serialize RpcNetworkInfoError: {:?}", err),
                )
            }
        };
        Self::new_internal_or_handler_error(Some(error_data.clone()), error_data)
    }
}