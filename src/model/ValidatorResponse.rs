use serde::{Serialize, Deserialize};
use std::collections::{HashSet, HashMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct SlotValidators {
    pub index : String,
    pub slot : String,
    pub validators : Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorData  {
    pub pubkey : String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Validator {
    pub index : String,
    pub validator : ValidatorData
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorResponse {
    pub finalized : bool,
    pub data : Vec<Validator>
}

impl ValidatorResponse {
    pub fn validator_key_index_map(&mut self) -> HashMap<String, String> {
        let mut map : HashMap<String, String> = HashMap::new();
        let validators = &self.data;
        for validator in validators {
            map.insert(validator.index.clone(), validator.validator.pubkey.clone());
        }
        map
    }
}