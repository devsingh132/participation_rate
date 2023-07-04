use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GenesisData {
    pub genesis_time : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenesisResponse {
    pub data : GenesisData
}