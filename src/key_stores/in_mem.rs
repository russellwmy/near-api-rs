use super::{KeyPair, KeyStore};
use core::fmt;
use core::str::FromStr;
use hashbrown::HashMap;
use near_account_id::AccountId;

#[derive(Clone)]
pub struct InMemKeyStore {
    storage: HashMap<String, String>,
}

impl InMemKeyStore {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    fn _get_storage_key(account_id: &str, network_id: &str) -> String {
        format!("{}:{}", account_id, network_id)
    }

    fn _extract_storage_key(storage_key: &str) -> Vec<String> {
        storage_key
            .split(':')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }
}

impl fmt::Display for InMemKeyStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InMemKeyStore")
    }
}

impl KeyStore for InMemKeyStore {
    fn set_key(&mut self, account_id: AccountId, network_id: &str, key_pair: KeyPair) {
        let key = Self::_get_storage_key(account_id.as_str(), network_id);
        let value = key_pair.to_string();
        self.storage.insert(key, value);
    }

    fn get_key(&self, account_id: AccountId, network_id: &str) -> KeyPair {
        let key = Self::_get_storage_key(account_id.as_str(), network_id);
        let value = self.storage.get(&key).expect("Invalid key");
        let key_pair = KeyPair::from_str(value.as_str()).expect("Invalid key data");

        key_pair
    }
    fn remove_key(&mut self, account_id: AccountId, network_id: &str) {
        let key = Self::_get_storage_key(account_id.as_str(), network_id);

        self.storage.remove(&key);
    }
    fn clear(&mut self) {
        self.storage.clear();
    }
    fn get_networks(&self) -> Vec<String> {
        self.storage
            .keys()
            .map(|s| Self::_extract_storage_key(&s)[1].clone())
            .collect()
    }
    fn get_accounts(&self, network_id: &str) -> Vec<String> {
        self.storage
            .keys()
            .filter(|s| Self::_extract_storage_key(s)[0] == network_id)
            .map(|s| Self::_extract_storage_key(s)[0].clone())
            .collect()
    }
}
