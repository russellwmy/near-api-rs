mod builder;

use near_primitives::{
    account::{AccessKey, AccessKeyPermission, FunctionCallPermission},
    types::Balance,
};

pub fn full_access_key() -> AccessKey {
    AccessKey {
        nonce: 0,
        permission: AccessKeyPermission::FullAccess,
    }
}

pub fn function_call_access_key(
    receiver_id: String,
    method_names: Vec<String>,
    allowance: Option<Balance>,
) -> AccessKey {
    AccessKey {
        nonce: 0,
        permission: AccessKeyPermission::FunctionCall(FunctionCallPermission {
            receiver_id,
            method_names,
            allowance,
        }),
    }
}
