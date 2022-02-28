use super::key_type::KeyType;
use borsh::{BorshDeserialize, BorshSerialize};
use core::{fmt, str};
use ed25519_dalek as ed25519;
use ed25519_dalek::Verifier;

#[derive(Clone)]
pub struct PublicKey {
    pub key_type: KeyType,
    pub key_data: Vec<u8>,
}

impl str::FromStr for PublicKey {
    type Err = fmt::Error;
    fn from_str(encoded_key: &str) -> Result<Self, Self::Err> {
        let parts = encoded_key.split(':').collect::<Vec<&str>>();

        if parts.len() == 0 || parts.len() > 2 {
            panic!("Invalid encoded key format, must be <curve>:<encoded key>")
        }

        let key_type = match parts.len() {
            1 => KeyType::ED25519,
            2 => KeyType::from_str(parts[0]).expect("Invalid key type"),
            _ => panic!("Invalid key type"),
        };

        let key_data = match parts.len() {
            1 => parts[0].try_to_vec().expect("Invalid key data"),
            2 => parts[2].try_to_vec().expect("Invalid key data"),
            _ => panic!("Invalid key data"),
        };

        Ok(Self { key_type, key_data })
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data_str =
            <String>::try_from_slice(&self.key_data).expect("failed to deserialize a string");
        let key_type_str = self.key_type.to_string();
        write!(f, "{}:{}", key_type_str, data_str)
    }
}

impl PublicKey {
    pub fn verify(self, message: Vec<u8>, signature: Vec<u8>) -> bool {
        match self.key_type {
            KeyType::ED25519 => {
                let public_key = ed25519::PublicKey::from_bytes(&self.key_data)
                    .expect("Invalid ED25519 Public Key");
                let signature =
                    ed25519::Signature::from_bytes(&signature).expect("Invalid signature");

                public_key.verify(&message, &signature).is_ok()
            }
            _ => false,
        }
    }
}
