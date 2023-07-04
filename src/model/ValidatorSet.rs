use std::iter::Map;
use crate::model::CommitteeDataResponse::CommitteeResponse;
use std::collections::hash_map::HashMap;

#[derive(Default)]
pub struct ValidatorSet {
    pub validators : HashMap<String, Vec<String>>
}

impl ValidatorSet {
    pub fn new(validator_res : CommitteeResponse) -> Self {
        let mut temp :HashMap<String, Vec<String>> = HashMap::new();
        for validator in validator_res.data {
            temp.insert(validator.index, validator.validators);
        }
        Self { validators : temp }
    }
}