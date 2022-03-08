use serde::{Deserialize, Serialize};
use serde_json::Value;

use near_primitives::types::AccountId;

#[derive(Debug, Clone)]
pub struct RpcBroadcastTransactionRequest {
    pub signed_transaction: near_primitives::transaction::SignedTransaction,
}

#[derive(Debug)]
pub struct RpcTransactionStatusCommonRequest {
    pub transaction_info: TransactionInfo,
}

#[derive(Clone, Debug)]
pub enum TransactionInfo {
    Transaction(near_primitives::transaction::SignedTransaction),
    TransactionId {
        hash: near_primitives::hash::CryptoHash,
        account_id: near_primitives::types::AccountId,
    },
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[serde(tag = "name", content = "info", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RpcTransactionError {
    #[error("An error happened during transaction execution: {context:?}")]
    InvalidTransaction {
        #[serde(skip_serializing)]
        context: near_primitives::errors::InvalidTxError,
    },
    #[error("Node doesn't track this shard. Cannot determine whether the transaction is valid")]
    DoesNotTrackShard,
    #[error("Transaction with hash {transaction_hash} was routed")]
    RequestRouted {
        transaction_hash: near_primitives::hash::CryptoHash,
    },
    #[error("Transaction {requested_transaction_hash} doesn't exist")]
    UnknownTransaction {
        requested_transaction_hash: near_primitives::hash::CryptoHash,
    },
    #[error("The node reached its limits. Try again later. More details: {debug_info}")]
    InternalError { debug_info: String },
    #[error("Timeout")]
    TimeoutError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcTransactionResponse {
    #[serde(flatten)]
    pub final_execution_outcome: near_primitives::views::FinalExecutionOutcomeViewEnum,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcBroadcastTxSyncResponse {
    pub transaction_hash: near_primitives::hash::CryptoHash,
}

impl RpcBroadcastTransactionRequest {
    pub fn parse(value: Option<Value>) -> Result<Self, crate::errors::RpcParseError> {
        let signed_transaction = crate::utils::parse_signed_transaction(value)?;
        Ok(Self { signed_transaction })
    }
}

impl RpcTransactionStatusCommonRequest {
    pub fn parse(value: Option<Value>) -> Result<Self, crate::errors::RpcParseError> {
        if let Ok((hash, account_id)) = crate::utils::parse_params::<(
            near_primitives::hash::CryptoHash,
            AccountId,
        )>(value.clone())
        {
            let transaction_info = TransactionInfo::TransactionId { hash, account_id };
            Ok(Self { transaction_info })
        } else {
            let signed_transaction = crate::utils::parse_signed_transaction(value)?;
            let transaction_info = TransactionInfo::Transaction(signed_transaction);
            Ok(Self { transaction_info })
        }
    }
}

impl From<near_primitives::views::FinalExecutionOutcomeViewEnum> for RpcTransactionResponse {
    fn from(
        final_execution_outcome: near_primitives::views::FinalExecutionOutcomeViewEnum,
    ) -> Self {
        Self {
            final_execution_outcome,
        }
    }
}
