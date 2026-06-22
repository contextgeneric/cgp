use crate::types::delegate_and_check_components::{CheckParamsAttribute, DelegateAndCheckKey};

pub struct DelegateAndCheckEntry {
    pub check_params: CheckParamsAttribute,
    pub key: DelegateAndCheckKey,
}
