mod connected_wallet_account;
mod wallet_connection;

pub use connected_wallet_account::*;
pub use wallet_connection::*;

use near_primitives::transaction::Transaction;
use serde::{Deserialize, Serialize};

use crate::{key_stores::KeyStore, Client};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInOptions {
    pub contract_id: Option<String>,
    pub method_names: Option<Vec<String>>,
    pub success_url: Option<String>,
    pub failure_url: Option<String>,
}

#[derive(Debug)]
pub struct RequestSignTransactionsOptions {
    // list of transactions to sign
    pub transactions: Vec<Transaction>,
    //  url NEAR Wallet will redirect to after transaction signing is complete
    pub callback_url: Option<String>,
    // meta information NEAR Wallet will send back to the application. `meta` will be attached to the `callbackUrl` as a url search param
    pub meta: Option<String>,
}
