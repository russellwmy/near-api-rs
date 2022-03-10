use core::panic;
use std::io::Error;

use hashbrown::HashMap;
use near_account_id::AccountId;
use near_crypto::PublicKey;
use near_jsonrpc_primitives::{errors::RpcError, message::Response};
use near_primitives::{
    transaction::Action,
    types::{Balance, BlockReference, Finality},
    views::{AccessKeyInfoView, AccessKeyView, QueryResponse},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    provider::types::{FinalExecutionOutcome, QueryResponseKind, RpcQueryRequest},
    Connection,
};

use super::{AccountAuthorizedApp, AccountBalance, FunctionCallOptions, ReceiptLogWithFailure};

pub struct Account {
    connection: Connection,
    account_id: AccountId,
    access_key_cache: HashMap<PublicKey, AccessKeyView>,
}

impl Account {
    pub fn new(connection: Connection, account_id: AccountId) -> Self {
        Self {
            connection,
            account_id,
            access_key_cache: HashMap::new(),
        }
    }

    pub async fn state(self) -> Result<QueryResponseKind, RpcError> {
        let value = json!({
            "request_type": "view_account",
            "account_id": self.account_id,
            "finality": "optimistic",
        });

        let params = match RpcQueryRequest::parse(Some(value)) {
            Ok(reqest) => reqest,
            Err(_) => panic!("Invalid value"),
        };

        self.connection.provider.query(params).await
    }

    fn _print_logs_and_failures(contract_id: &str, results: Vec<ReceiptLogWithFailure>) {
        for result in results {
            let receipt_log = match result.receipt_ids.len() {
                1 => format!("Recept: {}", result.receipt_ids[0]),
                _ => format!("Recepts: {}", result.receipt_ids.join(", ")),
            };
            log::info!("{}", receipt_log);
            Self::_print_logs(contract_id, result.logs, None);
            if result.failure.is_some() {
                log::warn!("\tFailure [{}]: {:?}", contract_id, result.failure.unwrap());
            }
        }
    }

    fn _print_logs(contract_id: &str, logs: Vec<String>, prefix: Option<String>) {
        let prefoix = prefix.unwrap_or("".to_string());

        for log_entry in logs {
            log::info!("{}Log [{}]: {}", prefoix, contract_id, log_entry);
        }
    }

    pub async fn sign_transaction(&mut self, receiver_id: AccountId, actions: Vec<Action>) {
        let access_key_info = self.find_access_key(receiver_id, actions).await;
        let access_key_info = match access_key_info {
            Ok(access_key_info) => access_key_info,
            Err(_) => panic!(
                "Can not sign transactions for account {} on network {}, no matching key pair found in {}.",
                 self.account_id,
                 self.connection.network_id,
                 self.connection.signer.public_key().to_string()
            )
        };

        let access_key = access_key_info.access_key;
        let block_query = BlockReference::Finality(Finality::Final);
        let block = self.connection.provider.block(block_query).await;
        let block_hash = match block {
            Ok(block) => block.header.hash,
            _ => panic!("Fail to get block info"),
        };

        let nouce = 1 + access_key.nonce;

        // TODO:
        // Transaction::sign_transaction(receiver_id, actions).await
    }

    pub async fn sign_and_send_transaction() {
        todo!();
    }

    pub async fn find_access_key(
        &mut self,
        receiver_id: AccountId,
        actions: Vec<Action>,
    ) -> Result<AccessKeyInfoView, Error> {
        let public_key = self.connection.signer.public_key();

        match self.access_key_cache.get(&public_key) {
            Some(access_key_view) => Ok(AccessKeyInfoView {
                public_key,
                access_key: access_key_view.clone(),
            }),
            None => {
                let value = json!({
                    "request_type": "view_access_key",
                    "account_id": self.account_id,
                    "public_key": public_key,
                    "finality": "optimistic",
                });

                let params = match RpcQueryRequest::parse(Some(value)) {
                    Ok(reqest) => reqest,
                    Err(_) => panic!("Invalid value"),
                };

                let access_key_query_result = self.connection.provider.query(params).await;
                let response_kind = match access_key_query_result {
                    Ok(response) => response,
                    Err(_) => panic!("Fail to the get access key"),
                };
                let access_key = match response_kind {
                    QueryResponseKind::AccessKey(access_key_view) => access_key_view,
                    _ => panic!("Fail to the get access key"),
                };

                self.access_key_cache
                    .insert(public_key.clone(), access_key.clone());

                Ok(AccessKeyInfoView {
                    access_key,
                    public_key,
                })
            }
        }
    }

    pub async fn create_and_deploy_contract(
        contract_id: AccountId,
        public_key: PublicKey,
        data: Vec<u8>,
        amount: Balance,
    ) -> Account {
        todo!();
    }

    pub async fn send_money(receiver_id: AccountId, amount: Balance) -> Account {
        todo!();
    }

    pub async fn create_account(
        self,
        new_account_id: &str,
        public_key: PublicKey,
        amount: Balance,
    ) -> FinalExecutionOutcome {
        todo!();
    }

    pub async fn delete_account(beneficiary_id: AccountId) -> FinalExecutionOutcome {
        todo!();
    }

    pub async fn deploy_contract(data: Vec<u8>) -> FinalExecutionOutcome {
        todo!();
    }

    pub async fn function_call(props: FunctionCallOptions) -> FinalExecutionOutcome {
        todo!();
    }

    pub async fn add_key(
        public_key: PublicKey,
        contract_id: Option<AccountId>,
        method_names: Vec<String>,
        amount: Balance,
    ) -> FinalExecutionOutcome {
        todo!();
    }

    pub async fn delete_key(public_key: PublicKey) -> FinalExecutionOutcome {
        todo!();
    }

    pub async fn stake(public_key: PublicKey, amount: Balance) -> FinalExecutionOutcome {
        todo!();
    }

    pub async fn view_function(
        contract_id: AccountId,
        method_name: String,
        args: Value,
    ) -> FinalExecutionOutcome {
        todo!();
    }

    pub async fn view_state(prefix: &str, block_query: BlockReference, args: Value) {
        todo!();
    }

    pub async fn get_access_keys() -> Vec<AccessKeyInfoView> {
        todo!();
    }

    pub async fn get_account_details() -> Vec<AccountAuthorizedApp> {
        todo!();
    }

    pub async fn get_account_balance() -> AccountBalance {
        todo!();
    }
}
