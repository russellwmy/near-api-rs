use super::KeyStore;
use crate::key_pair::{KeyPair, KeyPairED25519};
use async_trait::async_trait;
use core::fmt;
use core::str::FromStr;
use hashbrown::HashMap;

pub struct InMemoryKeyStore {
    keys: HashMap<String, String>,
}

impl InMemoryKeyStore {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }
}

#[async_trait]
impl KeyStore for InMemoryKeyStore {
    async fn set_key<T: KeyPair + fmt::Display + Send>(
        &mut self,
        network_id: &str,
        account_id: &str,
        key_pair: T,
    ) {
        let key = format!("{}:{}", account_id, network_id);
        let value = key_pair.to_string();
        self.keys.insert(key, value);
    }

    async fn get_key(&self, network_id: &str, account_id: &str) -> Box<dyn KeyPair> {
        let key = format!("{}:{}", account_id, network_id);
        let value = self.keys.get(&key).expect("Invalid key");
        let key_pair = KeyPairED25519::from_str(value.as_str()).expect("Invalid key data");

        Box::new(key_pair)
    }
    async fn remove_key(&mut self, network_id: &str, account_id: &str) {
        let key = format!("{}:{}", account_id, network_id);

        self.keys.remove(&key);
    }
    async fn clear(&mut self) {
        self.keys.clear();
    }
    async fn get_networks(&self) -> Vec<String> {
        self.keys
            .keys()
            .map(|s| s.split(':').collect::<Vec<&str>>()[1].to_string())
            .collect()
    }
    async fn get_accounts(&self, network_id: &str) -> Vec<String> {
        self.keys
            .keys()
            .filter(|s| s.split(':').collect::<Vec<&str>>()[0] == network_id)
            .map(|s| s.split(':').collect::<Vec<&str>>()[1].to_string())
            .collect()
    }
}
