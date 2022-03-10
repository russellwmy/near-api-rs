use super::WalletConnection;
use crate::account::Account;

pub struct ConnectedWalletAccount {
    account: Account,
    wallet_connection: WalletConnection,
}

impl ConnectedWalletAccount {
    pub fn new(
        wallet_connection: WalletConnection,
        connection: Connection,
        account_id: String,
    ) -> Self {
        let account = Account::new(connection, account_id);

        Self {
            wallet_connection,
            account,
        }
    }

    pub fn sign_and_send_transaction(self) -> FinalExecutionOutcome {
        todo!();
    }

    pub fn access_key_matches_transaction(self) -> FinalExecutionOutcome {
        todo!();
    }

    pub fn access_key_for_transaction(self) -> FinalExecutionOutcome {
        todo!();
    }
}
