use core::{fmt, str};

#[derive(Clone)]
pub enum KeyType {
    ED25519,
}

impl str::FromStr for KeyType {
    type Err = fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ed25519" => Ok(KeyType::ED25519),
            _ => panic!("Unknown key type"),
        }
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyType::ED25519 => write!(f, "ed25519"),
            _ => panic!("Unknown key type"),
        }
    }
}
