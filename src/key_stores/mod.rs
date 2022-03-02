mod browser;
mod in_mem;

use crate::key_pair::KeyPair;
use core::fmt::Display;
pub use in_mem::*;

pub trait KeyStore {
    fn set_key<T: KeyPair + Display + 'static>(
        &mut self,
        network_id: &str,
        account_id: &str,
        key_pair: T,
    );
    fn get_key(&self, network_id: &str, account_id: &str) -> Box<dyn KeyPair>;
    fn remove_key(&mut self, network_id: &str, account_id: &str);
    fn clear(&mut self);
    fn get_networks(&self) -> Vec<String>;
    fn get_accounts(&self, network_id: &str) -> Vec<String>;
}
