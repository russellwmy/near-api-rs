use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcProtocolConfigRequest {
    #[serde(flatten)]
    pub block_reference: near_primitives::types::BlockReference,
}

impl RpcProtocolConfigRequest {
    pub fn parse(
        value: Option<Value>,
    ) -> Result<RpcProtocolConfigRequest, crate::errors::RpcParseError> {
        crate::utils::parse_params::<near_primitives::types::BlockReference>(value)
            .map(|block_reference| RpcProtocolConfigRequest { block_reference })
    }
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[serde(tag = "name", content = "info", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RpcProtocolConfigError {
    #[error("Block has never been observed: {error_message}")]
    UnknownBlock {
        #[serde(skip_serializing)]
        error_message: String,
    },
    #[error("The node reached its limits. Try again later. More details: {error_message}")]
    InternalError { error_message: String },
}
