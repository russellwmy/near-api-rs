pub struct Contract {
    account: Account,
    contract_id: AccountId,
}

impl Contract {
    pub fn new(account: Account, contract_id: AccountId, options: ContractMethods) -> Self {
        Self {
            account,
            contract_id,
        }
    }
}
