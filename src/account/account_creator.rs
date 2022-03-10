use near_crypto::PublicKey;
use near_primitives::types::Balance;

use crate::{fetch_json, Connection};

use super::account::Account;

pub struct LocalAccountCreator {
    master_account: Account,
    intial_balance: Balance,
}
impl LocalAccountCreator {
    pub fn new(master_account: Account, intial_balance: Balance) -> Self {
        Self {
            master_account,
            intial_balance,
        }
    }
    async fn create_account(self, new_account_id: &str, public_key: PublicKey) {
        self.master_account
            .create_account(new_account_id, public_key, self.intial_balance)
            .await;
    }
}

pub struct UrlAccountCreator {
    connection: Connection,
    helper_url: String,
}
impl UrlAccountCreator {
    pub fn new(connection: Connection, helper_url: String) -> Self {
        Self {
            connection,
            helper_url,
        }
    }
    async fn create_account(self, new_account_id: &str, public_key: PublicKey) {
        let url = format!("{}/account", self.helper_url);
        let data = serde_json::json!({
            "newAccountId": new_account_id,
            "newAccountPublicKey":  public_key.to_string(),
        });
        let response = fetch_json(&url, &data.to_string()).await;

        match response {
            Ok(_) => log::info!("Success to create account: {}", new_account_id),
            Err(_) => log::warn!("Fail to create account: {}", new_account_id),
        }
    }
}
