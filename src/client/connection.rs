use near_crypto::Signer;

use crate::provider::{ConnectionInfo, JsonRpcProvider, Provider};

use super::ClientConfig;

pub struct Connection {
    pub network_id: String,
    pub provider: Provider,
    pub signer: Box<dyn Signer>,
}

impl Connection {
    pub fn new(network_id: &str, signer: Box<dyn Signer>, provider: Provider) -> Self {
        Self {
            network_id: network_id.to_string(),
            signer,
            provider,
        }
    }
}

impl From<ClientConfig> for Connection {
    fn from(config: ClientConfig) -> Self {
        let signer = config.signer.expect("Missing signer");
        let provider = JsonRpcProvider::new(ConnectionInfo::new(config.node_url));

        Self {
            signer,
            provider,
            network_id: config.network_id,
        }
    }
}
