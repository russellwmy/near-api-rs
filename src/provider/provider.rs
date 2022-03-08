use super::{
    types::{
        AccessKeyWithPublicKey, BlockChangeResult, BlockId, BlockReference, BlockResult,
        ChangeResult, ChunkId, ChunkResult, CryptoHash, EpochValidatorInfo, FinalExecutionOutcome,
        GasPrice, LightClientProof, LightClientProofRequest, NearProtocolConfig, NodeStatusResult,
        QueryResponseKind, RpcQueryRequest, SignedTransaction,
    },
    ConnectionInfo,
};
use crate::{client::ClientConfig, fetch_json};
use borsh::BorshSerialize;
use near_account_id::AccountId;
use near_jsonrpc_primitives::{
    errors::RpcError,
    message::{Message, Response as JsonRpcResponse},
};
use near_primitives::{serialize::to_base64, types::StoreKey};
use serde::Deserialize;
use serde_json::{json, Value};

pub type Provider = JsonRpcProvider;

#[derive(Clone)]
pub struct JsonRpcProvider {
    connection_info: ConnectionInfo,
}

impl JsonRpcProvider {
    pub fn new(connection_info: ConnectionInfo) -> Self {
        Self { connection_info }
    }
}

impl JsonRpcProvider {
    async fn send_jsonrpc(
        &self,
        method: String,
        params: Option<Value>,
    ) -> Result<JsonRpcResponse, RpcError> {
        let url = &self.connection_info.url;
        let message = Message::request(method, params);
        let data: String = message.into();
        let response = fetch_json(url, &data).await;

        match response {
            Ok(response) => {
                let response: JsonRpcResponse = response.into_serde().unwrap();

                Ok(response)
            }
            _ => Err(RpcError::parse_error(format!(
                "Failed to parse JSON RPC response"
            ))),
        }
    }

    pub async fn status(&self) -> Result<NodeStatusResult, RpcError> {
        let response = self.send_jsonrpc("status".to_owned(), None).await;
        match response {
            Ok(response) => Ok(NodeStatusResult::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!("Failed to send status"))),
        }
    }

    pub async fn send_transaction(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<FinalExecutionOutcome, RpcError> {
        let bytes = signed_transaction.try_to_vec().unwrap();
        let response = self
            .send_jsonrpc(
                "broadcast_tx_commit".to_owned(),
                Some(to_base64(&bytes).into()),
            )
            .await;
        match response {
            Ok(response) => {
                Ok(FinalExecutionOutcome::deserialize(response.result.unwrap()).unwrap())
            }
            Err(_) => Err(RpcError::parse_error(format!("Failed to send transaction"))),
        }
    }

    pub async fn send_transaction_async(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<FinalExecutionOutcome, RpcError> {
        let bytes = signed_transaction.try_to_vec().unwrap();
        let response = self
            .send_jsonrpc(
                "broadcast_tx_async".to_owned(),
                Some(to_base64(&bytes).into()),
            )
            .await;
        match response {
            Ok(response) => {
                Ok(FinalExecutionOutcome::deserialize(response.result.unwrap()).unwrap())
            }
            Err(_) => Err(RpcError::parse_error(format!("Failed to send transaction"))),
        }
    }
    pub async fn tx_status(
        &self,
        tx_hash: CryptoHash,
        account_id: AccountId,
    ) -> Result<FinalExecutionOutcome, RpcError> {
        let tx_hash = format!("{}", tx_hash);
        let params = serde_json::to_value([tx_hash.as_str(), account_id.as_str()]).unwrap();
        let response = self.send_jsonrpc("tx".to_owned(), Some(params)).await;
        match response {
            Ok(response) => {
                Ok(FinalExecutionOutcome::deserialize(response.result.unwrap()).unwrap())
            }
            Err(_) => Err(RpcError::parse_error(format!("Failed to get tx status"))),
        }
    }

    pub async fn tx_status_receipts(
        &self,
        tx_hash: CryptoHash,
        account_id: AccountId,
    ) -> Result<FinalExecutionOutcome, RpcError> {
        let tx_hash = format!("{}", tx_hash);
        let params = serde_json::to_value([tx_hash.as_str(), account_id.as_str()]).unwrap();
        let response = self
            .send_jsonrpc("EXPERIMENTAL_tx_status".to_owned(), Some(params))
            .await;
        match response {
            Ok(response) => {
                Ok(FinalExecutionOutcome::deserialize(response.result.unwrap()).unwrap())
            }
            Err(_) => Err(RpcError::parse_error(format!("Failed to get tx status"))),
        }
    }

    pub async fn query(&self, params: RpcQueryRequest) -> Result<QueryResponseKind, RpcError> {
        let params = serde_json::to_value(params).unwrap();
        let response = self.send_jsonrpc("query".to_owned(), Some(params)).await;
        match response {
            Ok(response) => Ok(QueryResponseKind::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!("Failed to get tx status"))),
        }
    }
    // async fn query<T: QueryResponseKind>(self, path: String, data: String) -> T{todo!();}

    // // TODO: BlockQuery type?
    pub async fn block(&self, block_query: BlockId) -> Result<BlockResult, RpcError> {
        let byes = match block_query {
            BlockId::Height(v) => v.to_string(),
            BlockId::Hash(v) => v.to_string(),
        };

        let params = serde_json::to_value([&byes]).unwrap();
        let response = self.send_jsonrpc("block".to_owned(), Some(params)).await;
        match response {
            Ok(response) => Ok(BlockResult::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!("Failed to get block info"))),
        }
    }
    pub async fn block_changes(
        &self,
        block_reference: BlockReference,
    ) -> Result<BlockChangeResult, RpcError> {
        let params = json!(block_reference);
        let response = self
            .send_jsonrpc("EXPERIMENTAL_changes_in_block".to_owned(), Some(params))
            .await;
        match response {
            Ok(response) => Ok(BlockChangeResult::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!(
                "Failed to get block changes"
            ))),
        }
    }
    pub async fn chunk(&self, chunk_id: ChunkId) -> Result<ChunkResult, RpcError> {
        let params = json!(chunk_id);
        let response = self.send_jsonrpc("chunk".to_owned(), Some(params)).await;
        match response {
            Ok(response) => Ok(ChunkResult::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!(
                "Failed to get block changes"
            ))),
        }
    }
    // // TODO: Use BlockQuery?
    pub async fn validators(&self, block_id: BlockId) -> Result<EpochValidatorInfo, RpcError> {
        let params = json!([block_id]);
        let response = self
            .send_jsonrpc("validators".to_owned(), Some(params))
            .await;
        match response {
            Ok(response) => Ok(EpochValidatorInfo::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!(
                "Failed to get block changes"
            ))),
        }
    }
    pub async fn experimental_protocol_config(
        &self,
        block_reference: BlockReference,
    ) -> Result<NearProtocolConfig, RpcError> {
        let params = json!([block_reference]);
        let response = self
            .send_jsonrpc("EXPERIMENTAL_protocol_config".to_owned(), Some(params))
            .await;
        match response {
            Ok(response) => Ok(NearProtocolConfig::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!(
                "Failed to get block changes"
            ))),
        }
    }
    pub async fn light_client_proof(
        &self,
        request: LightClientProofRequest,
    ) -> Result<LightClientProof, RpcError> {
        let params = json!(request);
        let response = self
            .send_jsonrpc("EXPERIMENTAL_protocol_config".to_owned(), Some(params))
            .await;
        match response {
            Ok(response) => Ok(LightClientProof::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!(
                "Failed to get block changes"
            ))),
        }
    }

    pub async fn gas_price(&self, block_id: BlockId) -> Result<GasPrice, RpcError> {
        let byes = match block_id {
            BlockId::Height(v) => v.to_string(),
            BlockId::Hash(v) => v.to_string(),
        };

        let params = serde_json::to_value([&byes]).unwrap();
        let response = self
            .send_jsonrpc("gas_price".to_owned(), Some(params))
            .await;
        match response {
            Ok(response) => Ok(GasPrice::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!("Failed to get gas price"))),
        }
    }

    pub async fn access_key_changes(
        &self,
        account_id_array: Vec<AccountId>,
        block_query: BlockReference,
    ) -> Result<ChangeResult, RpcError> {
        let block_id = match block_query.clone() {
            BlockReference::BlockId(block_id) => match block_id {
                BlockId::Hash(block_id) => block_id.to_string(),
                _ => "Missing block id".to_string(),
            },
            _ => "Missing block id".to_owned(),
        };
        let finality = match block_query.clone() {
            BlockReference::Finality(finality) => format!("{:?}", finality),
            _ => "Missing finality".to_string(),
        };

        let params = json!({
            "changes_type": "all_access_key_changes",
            "account_ids": account_id_array,
            "block_id":block_id,
            "finality":finality,
        });
        let response = self
            .send_jsonrpc("EXPERIMENTAL_changes".to_owned(), Some(params))
            .await;
        match response {
            Ok(response) => Ok(ChangeResult::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!(
                "Failed to get block changes"
            ))),
        }
    }

    pub async fn single_access_key_changes(
        &self,
        access_key_array: Vec<AccessKeyWithPublicKey>,
        block_query: BlockReference,
    ) -> Result<ChangeResult, RpcError> {
        let block_id = match block_query.clone() {
            BlockReference::BlockId(block_id) => match block_id {
                BlockId::Hash(block_id) => block_id.to_string(),
                _ => "Missing block id".to_string(),
            },
            _ => "Missing block id".to_owned(),
        };
        let finality = match block_query.clone() {
            BlockReference::Finality(finality) => format!("{:?}", finality),
            _ => "Missing finality".to_string(),
        };
        let params = json!({
            "changes_type": "single_access_key_changes",
            "keys": access_key_array,
            "block_id":block_id,
            "finality":finality,
        });
        let response = self
            .send_jsonrpc("EXPERIMENTAL_changes".to_owned(), Some(params))
            .await;
        match response {
            Ok(response) => Ok(ChangeResult::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!(
                "Failed to get block changes"
            ))),
        }
    }

    pub async fn account_changes(
        &self,
        account_id_array: Vec<AccountId>,
        block_query: BlockReference,
    ) -> Result<ChangeResult, RpcError> {
        let block_id = match block_query.clone() {
            BlockReference::BlockId(block_id) => match block_id {
                BlockId::Hash(block_id) => block_id.to_string(),
                _ => "Missing block id".to_string(),
            },
            _ => "Missing block id".to_owned(),
        };
        let finality = match block_query.clone() {
            BlockReference::Finality(finality) => format!("{:?}", finality),
            _ => "Missing finality".to_string(),
        };

        let params = json!({
            "changes_type": "account_changes",
            "account_ids": account_id_array,
            "block_id":block_id,
            "finality":finality,
        });
        let response = self
            .send_jsonrpc("EXPERIMENTAL_changes".to_owned(), Some(params))
            .await;
        match response {
            Ok(response) => Ok(ChangeResult::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!(
                "Failed to get block changes"
            ))),
        }
    }

    pub async fn contract_state_changes(
        &self,
        account_id_array: Vec<String>,
        block_query: BlockReference,
        key_prefix: StoreKey,
    ) -> Result<ChangeResult, RpcError> {
        let block_id = match block_query.clone() {
            BlockReference::BlockId(block_id) => match block_id {
                BlockId::Hash(block_id) => block_id.to_string(),
                _ => "Missing block id".to_string(),
            },
            _ => "Missing block id".to_owned(),
        };
        let finality = match block_query.clone() {
            BlockReference::Finality(finality) => format!("{:?}", finality),
            _ => "Missing finality".to_string(),
        };

        let params = json!({
            "changes_type": "data_changes",
            "account_ids": account_id_array,
            "key_prefix_base64": base64::encode(key_prefix),
            "block_id":block_id,
            "finality":finality,
        });

        let response = self
            .send_jsonrpc("EXPERIMENTAL_changes".to_owned(), Some(params))
            .await;
        match response {
            Ok(response) => Ok(ChangeResult::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!(
                "Failed to get block changes"
            ))),
        }
    }

    pub async fn contract_code_changes(
        &self,
        account_id_array: Vec<AccountId>,
        block_query: BlockReference,
    ) -> Result<ChangeResult, RpcError> {
        let block_id = match block_query.clone() {
            BlockReference::BlockId(block_id) => match block_id {
                BlockId::Hash(block_id) => block_id.to_string(),
                _ => "Missing block id".to_string(),
            },
            _ => "Missing block id".to_owned(),
        };
        let finality = match block_query.clone() {
            BlockReference::Finality(finality) => format!("{:?}", finality),
            _ => "Missing finality".to_string(),
        };

        let params = json!({
            "changes_type": "contract_code_changes",
            "account_ids": account_id_array,
            "block_id":block_id,
            "finality":finality,
        });

        let response = self
            .send_jsonrpc("EXPERIMENTAL_changes".to_owned(), Some(params))
            .await;
        match response {
            Ok(response) => Ok(ChangeResult::deserialize(response.result.unwrap()).unwrap()),
            Err(_) => Err(RpcError::parse_error(format!(
                "Failed to get block changes"
            ))),
        }
    }
}

impl From<ClientConfig> for JsonRpcProvider {
    fn from(config: ClientConfig) -> JsonRpcProvider {
        let connection_info = ConnectionInfo::new(config.node_url);

        JsonRpcProvider::new(connection_info)
    }
}
