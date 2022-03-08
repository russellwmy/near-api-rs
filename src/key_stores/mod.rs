mod browser;
mod in_mem;

pub use browser::*;
pub use in_mem::*;
use near_account_id::AccountId;
use near_crypto::SecretKey;

pub type KeyPair = SecretKey;

pub trait KeyStore: CloneKeyStore {
    fn set_key(&mut self, account_id: AccountId, network_id: &str, key_pair: KeyPair);
    fn get_key(&self, account_id: AccountId, network_id: &str) -> KeyPair;
    fn remove_key(&mut self, account_id: AccountId, network_id: &str);
    fn clear(&mut self);
    fn get_networks(&self) -> Vec<String>;
    fn get_accounts(&self, network_id: &str) -> Vec<String>;
}

pub trait CloneKeyStore {
    fn clone_key_store<'a>(&self) -> Box<dyn KeyStore>;
}

impl<T> CloneKeyStore for T
where
    T: KeyStore + Clone + 'static,
{
    fn clone_key_store(&self) -> Box<dyn KeyStore> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn KeyStore> {
    fn clone(&self) -> Self {
        self.clone_key_store()
    }
}
