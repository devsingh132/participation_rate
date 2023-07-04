use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitteeData {
    pub index : String,
    pub slot : String,
    pub validators : Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitteeResponse {
    pub finalized : bool,
    pub data : Vec<CommitteeData>
}