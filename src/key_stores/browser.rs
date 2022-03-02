use super::KeyStore;
use crate::key_pair::{KeyPair, KeyPairED25519};
use core::fmt;
use core::str::FromStr;

const LOCAL_STORAGE_KEY_PREFIX: &str = "near-api-rs:keystore";

#[derive(Debug)]
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

impl KeyStore for BrowserKeyStore {
    fn set_key<T: KeyPair + fmt::Display + 'static>(
        &mut self,
        network_id: &str,
        account_id: &str,
        key_pair: T,
    ) {
        let key = Self::_get_storage_key(&self.prefix, account_id, network_id);
        let value = key_pair.to_string();

        self.storage
            .set_item(&key, &value)
            .expect("Fail to delete key from storage");
    }

    fn get_key(&self, network_id: &str, account_id: &str) -> Box<dyn KeyPair> {
        let key = Self::_get_storage_key(&self.prefix, account_id, network_id);
        let value = self.storage.get_item(&key).expect("Invalid key").unwrap();
        let key_pair = KeyPairED25519::from_str(value.as_str()).expect("Invalid key data");

        Box::new(key_pair)
    }
    fn remove_key(&mut self, network_id: &str, account_id: &str) {
        let key = Self::_get_storage_key(&self.prefix, account_id, network_id);

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
