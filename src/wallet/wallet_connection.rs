use super::ConnectedWalletAccount;

pub struct WalletConnection {
    wallet_base_url: String,
    auth_data_key: String,
    key_store: Box<dyn KeyStore>,
    auth_data: serde_json::Value,
    network_id: String,
    near: Client,
    connected_account: ConnectedWalletAccount,
}

impl WalletConnection {
    pub fn new(
        wallet_base_url: String,
        auth_data_key: String,
        key_store: Box<dyn KeyStore>,
        auth_data: serde_json::Value,
        network_id: String,
        near: Client,
        connected_account: ConnectedWalletAccount,
    ) -> Self {
        Self {
            wallet_base_url,
            auth_data_key,
            key_store,
            auth_data,
            network_id,
            near,
            connected_account,
        }
    }
}
