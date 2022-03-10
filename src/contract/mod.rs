mod contract;

pub struct ChangeMethodOptions {
    pub args: Value,
    pub method_name: String,
    pub gas: Option<Balance>,
    pub amount: Option<Balance>,
    pub meta: String,
    pub callback_url: Option<String>,
}

pub struct ContractMethods {
    pub change_methods: Vec<String>,
    pub view_methods: Vec<String>,
}
