use std::iter::Map;
use crate::model::CommitteeResponseData::CommitteeResponse;
use std::collections::hash_map::HashMap;

#[derive(Default)]
pub struct CommitteeSet {
    pub validators : HashMap<String, Vec<String>> //map of committee index and vector of validator indexes
}

impl CommitteeSet {
    pub fn new(validator_res : CommitteeResponse) -> Self {
        let mut temp :HashMap<String, Vec<String>> = HashMap::new();
        for validator in validator_res.data {
            temp.insert(validator.index, validator.validators);
        }
        Self { validators : temp }
    }
}