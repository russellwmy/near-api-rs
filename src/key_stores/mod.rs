mod in_mem_key_store;

use crate::key_pair::KeyPair;
use async_trait::async_trait;
use core::fmt;
pub use in_mem_key_store::InMemoryKeyStore;

#[async_trait]
pub trait KeyStore {
    async fn set_key<T: KeyPair + fmt::Display + Send>(
        &mut self,
        network_id: &str,
        account_id: &str,
        key_pair: T,
    );
    async fn get_key(&self, network_id: &str, account_id: &str) -> Box<dyn KeyPair>;
    async fn remove_key(&mut self, network_id: &str, account_id: &str);
    async fn clear(&mut self);
    async fn get_networks(&self) -> Vec<String>;
    async fn get_accounts(&self, network_id: &str) -> Vec<String>;
}
