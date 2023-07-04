use serde::{Serialize, Deserialize};

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
pub struct Validators {
    pub index : String,
    pub slot : String,
    pub validator_data : ValidatorData
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CommitteeResponse {
    pub finalized : bool,
    pub data : Vec<SlotValidators>
}