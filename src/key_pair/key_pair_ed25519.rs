use super::{KeyPair, KeyType, PublicKey, Signature};
use borsh::{BorshDeserialize, BorshSerialize};
use core::{fmt, str};
use ed25519_dalek as ed25519;
use ed25519_dalek::{Signer, Verifier};

pub struct KeyPairED25519 {
    public_key: PublicKey,
    secret_key_data: Vec<u8>,
    key_pair: ed25519::Keypair,
}

impl KeyPair for KeyPairED25519 {
    fn sign(self, message: Vec<u8>) -> Signature {
        let signature = self.key_pair.sign(&message);

        Signature {
            signature: signature.to_bytes().to_vec(),
            public_key: self.public_key.clone(),
        }
    }

    fn verify(self, message: Vec<u8>, signature: Vec<u8>) -> bool {
        let public_key = ed25519::PublicKey::from_bytes(&self.public_key.key_data)
            .expect("Invalid ED25519 Public Key");
        let signature = ed25519::Signature::from_bytes(&signature).expect("Invalid signature");

        public_key.verify(&message, &signature).is_ok()
    }

    fn get_public_key(self) -> PublicKey {
        self.public_key
    }
}

impl str::FromStr for KeyPairED25519 {
    type Err = fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let secret_key_data = s.try_to_vec().expect("Invalid key data");
        let secret_key =
            ed25519::SecretKey::from_bytes(&secret_key_data).expect("Invalid key data");
        let public_key = ed25519::PublicKey::from(&secret_key);
        let key_pair_bytes = [secret_key.to_bytes(), public_key.to_bytes()].concat();
        let key_pair = ed25519::Keypair::from_bytes(&key_pair_bytes)
            .expect("Fail to create key piar from secret");

        let key_pair = Self {
            secret_key_data,
            public_key: PublicKey {
                key_type: KeyType::ED25519,
                key_data: public_key.to_bytes().to_vec(),
            },
            key_pair,
        };

        Ok(key_pair)
    }
}

impl fmt::Display for KeyPairED25519 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let secret_key_str = <String>::try_from_slice(&self.secret_key_data)
            .expect("Fail to covert bytes to string");
        write!(f, "ed25519:{}", secret_key_str)
    }
}
