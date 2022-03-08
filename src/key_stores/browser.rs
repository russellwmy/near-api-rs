use near_account_id::AccountId;

use super::{KeyPair, KeyStore};
use core::fmt;
use core::str::FromStr;

const LOCAL_STORAGE_KEY_PREFIX: &str = "near-api-rs_keystore";

#[derive(Clone)]
pub struct BrowserKeyStore {
    prefix: String,
    storage: web_sys::Storage,
}

impl BrowserKeyStore {
    pub fn new() -> Self {
        Self::new_with_prefix(LOCAL_STORAGE_KEY_PREFIX)
    }

    pub fn new_with_prefix(prefix: &str) -> Self {
        let window = web_sys::window().expect("Fail to get window object");
        let storage = window
            .local_storage()
            .unwrap()
            .expect("Fail to get local storage");
        Self {
            prefix: String::from(prefix),
            storage,
        }
    }

    fn _get_storage_key(prefix: &str, account_id: &str, network_id: &str) -> String {
        format!("{}:{}:{}", prefix, account_id, network_id)
    }

    fn _extract_storage_key(storage_key: &str) -> Vec<String> {
        storage_key
            .split(':')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }
}

impl fmt::Display for BrowserKeyStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BrowserKeyStore")
    }
}

impl KeyStore for BrowserKeyStore {
    fn set_key(&mut self, account_id: AccountId, network_id: &str, key_pair: KeyPair) {
        let key = Self::_get_storage_key(&self.prefix, account_id.as_str(), network_id);
        let value = key_pair.to_string();

        self.storage
            .set_item(&key, &value)
            .expect("Fail to delete key from storage");
    }

    fn get_key(&self, account_id: AccountId, network_id: &str) -> KeyPair {
        let key = Self::_get_storage_key(&self.prefix, account_id.as_str(), network_id);
        let value = self.storage.get_item(&key).expect("Invalid key").unwrap();
        let key_pair = KeyPair::from_str(value.as_str()).expect("Invalid key data");

        key_pair
    }
    fn remove_key(&mut self, account_id: AccountId, network_id: &str) {
        let key = Self::_get_storage_key(&self.prefix, account_id.as_str(), network_id);

        self.storage
            .delete(&key)
            .expect("Fail to delete key from storage");
    }
    fn clear(&mut self) {
        self.storage.clear().expect("Fail to clear storage");
    }
    fn get_networks(&self) -> Vec<String> {
        let l = self.storage.length().unwrap();
        let mut keys = vec![];
        for i in 0..l {
            let key = self.storage.key(i);
            if key.is_ok() {
                let key_str = Self::_extract_storage_key(&key.unwrap().unwrap());
                if key_str.len() == 3 {
                    keys.push(key_str[2].clone());
                }
            }
        }
        keys
    }

    fn get_accounts(&self, network_id: &str) -> Vec<String> {
        let l = self.storage.length().unwrap();
        let mut keys = vec![];
        for i in 0..l {
            let key = self.storage.key(i);
            if key.is_ok() {
                let key_str = Self::_extract_storage_key(&key.unwrap().unwrap());
                if key_str.len() == 3 && key_str[2] == network_id {
                    keys.push(key_str[1].clone());
                }
            }
        }
        keys
    }
}
