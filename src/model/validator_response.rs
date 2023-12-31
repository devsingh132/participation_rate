use serde::{Serialize, Deserialize};
use std::collections::{HashMap};

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
/**
 * Struct to map the response from the get validators api.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorResponse {
    pub finalized : bool,
    pub data : Vec<Validator>
}

impl ValidatorResponse {
    /**
     * Return a Map which maps the validator index to publickey
     */
    pub fn validator_key_index_map(& self) -> HashMap<String, String> {
        let mut map : HashMap<String, String> = HashMap::new();
        let validators = &self.data;
        for validator in validators {
            map.insert(validator.index.clone(), validator.validator.pubkey.clone());
        }
        map
    }
}