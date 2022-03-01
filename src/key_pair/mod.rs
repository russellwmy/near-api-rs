mod key_pair_ed25519;
mod key_type;
mod public_key;

use core::str;
pub use key_pair_ed25519::*;
use key_type::*;
use public_key::PublicKey;

pub struct Signature {
    pub signature: Vec<u8>,
    pub public_key: PublicKey,
}

pub trait KeyPair {
    fn sign(self, message: Vec<u8>) -> Signature;
    fn verify(self, message: Vec<u8>, signature: Vec<u8>) -> bool;
    fn get_public_key(self) -> PublicKey;
}
