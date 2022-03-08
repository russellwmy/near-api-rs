use core::fmt;
use hashbrown::HashMap;
use near_account_id::AccountId;
use near_crypto::{PublicKey, Signer};
use std::borrow::Borrow;

use crate::key_stores::KeyStore;

pub struct ClientConfig {
    pub key_store: Option<Box<dyn KeyStore>>,
    pub signer: Option<Box<dyn Signer>>,
    pub helper_url: Option<String>,
    pub initial_balance: Option<String>,
    pub master_account: Option<AccountId>,
    pub network_id: String,
    pub node_url: String,
    pub wallet_url: Option<String>,
    pub headers: HashMap<String, String>,
    pub key_path: Option<String>,
}

impl ClientConfig {
    pub fn new(network_id: &str, node_url: &str) -> Self {
        Self {
            network_id: String::from(network_id),
            node_url: String::from(node_url),
            key_store: None,
            signer: None,
            helper_url: None,
            initial_balance: None,
            master_account: None,
            wallet_url: None,
            headers: HashMap::new(),
            key_path: None,
        }
    }
}

impl fmt::Display for ClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let signer = match self.signer.as_ref() {
            Some(signer) => signer.public_key().to_string(),
            _ => String::from("No signer"),
        };

        write!(
            f,
            "network_id: {}, node_url: {}, key: {:?}",
            self.network_id, self.node_url, signer
        )
    }
}
