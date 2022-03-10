mod account;
mod account_creator;

use account::*;
use account_creator::*;
use near_account_id::AccountId;
use near_crypto::PublicKey;
use near_primitives::{types::Balance, views::ServerError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalance {
    pub total: Balance,
    pub state_staked: Balance,
    pub staked: Balance,
    pub available: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountAuthorizedApp {
    pub contract_id: AccountId,
    pub amount: Balance,
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignAndSendTransactionOptions {
    pub receiver_id: AccountId,
    pub wallet_meta: Option<String>,
    pub wallet_callback_url: Option<String>,
    pub return_error: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionCallOptions {
    pub contract_id: AccountId,
    pub method_name: String,
    pub args: serde_json::Value,
    pub gas: Option<Balance>,
    pub attached_deposit: Option<Balance>,
    pub wallet_meta: Option<String>,
    pub wallet_callback_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiptLogWithFailure {
    pub receipt_ids: Vec<AccountId>,
    pub logs: Vec<String>,
    pub failure: Option<ServerError>,
}
