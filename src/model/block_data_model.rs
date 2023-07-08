
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AggregationData {
    pub slot : String,
    pub index : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attestations {
    pub aggregation_bits : String,
    pub data : AggregationData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub attestations : Vec<Attestations>
} 
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub slot : String,
    pub body : Body
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockData {
    pub signature : String,
    pub message : Message
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockDataModel {
    pub finalized : bool,
    pub data : BlockData
}

impl BlockDataModel {
    pub fn get_attestations(&self) -> &Vec<Attestations> {
        &self.data.message.body.attestations
    }

    pub fn get_slot(&self) -> &String {
        &self.data.message.slot
    }
}