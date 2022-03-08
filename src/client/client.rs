use core::fmt;
use std::str::FromStr;

use near_account_id::AccountId;
use near_crypto::{InMemorySigner, KeyType, SecretKey};

use crate::{
    key_stores::{BrowserKeyStore, KeyPair, KeyStore},
    provider::{ConnectionInfo, JsonRpcProvider},
};

use super::{ClientConfig, Connection};

pub struct Client {
    pub config: ClientConfig,
    pub connection: Connection,
}

impl Client {
    pub fn new(config: ClientConfig) -> Self {
        let account_id: AccountId = AccountId::from_str("test.near").unwrap();
        let secret_key = SecretKey::from_random(KeyType::ED25519);
        let signer = Box::new(InMemorySigner::from_secret_key(
            account_id.clone(),
            secret_key.clone(),
        ));
        let provider = JsonRpcProvider::new(ConnectionInfo::new(config.node_url.clone()));
        let mut config = config;
        let mut key_store = BrowserKeyStore::new();
        let key_pair: KeyPair = secret_key.clone();

        key_store.set_key(account_id, config.network_id.as_str(), key_pair);

        config.signer = Some(signer.clone());
        config.key_store = Some(Box::new(key_store));

        Self {
            connection: Connection::new(config.network_id.as_str(), signer, provider),
            config,
        }
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.config.to_string())
    }
}
