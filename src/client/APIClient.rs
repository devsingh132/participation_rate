use std::collections::{HashSet, HashMap};


use crate::client::BlockDataModel::BlockDataModel;
use crate::model::CommitteeDataResponse::{CommitteeResponse};
use crate::model::ValidatorSet::ValidatorSet;
use crate::model::ValidatorResponse::ValidatorResponse;
use hex::*;
use reqwest::{Error, Client, Response,};
use reqwest::StatusCode;


pub struct APIClient {
    base_url: String,
    client: Client,
    START_EPOCH:u64
}

impl APIClient {
    pub async fn new(base_url: &str) -> Self {
        let client = Client::new();
        APIClient {
            base_url: base_url.to_string(),
            client,
            START_EPOCH :0
        }
    }
    // pub async fn get_genesis(&mut self) {
    //     let genesis_url = format!("/eth/v1/beacon/genesis");
    //     self.START_EPOCH = 0;
    //     match self.send_get_request(&genesis_url).await {
    //         Some(response) => {
    //             if response.status() == StatusCode::OK {
    //                 match response.json::<GenesisResponse>().await {
    //                     Ok(r) => {
    //                         // println!("{}", text.finalized);
    //                         self.START_EPOCH =  r.data.genesis_time.parse().expect(format!("Unable to format data {}", r.data.genesis_time).as_str());
    //                         println!("{}", self.START_EPOCH);
    //                         // let block_data : BlockDataModel = serde_json::from_str(&text).unwrap();
    //                         // println!("{}",block_data.finalized);
    //                     },
    //                     Err(e) => {
    //                         println!("Unbale to parse {}",e.to_string());
    //                     }
    //                 }
    //             } else {
    //                 println!("Unable to fetch data with error {}", response.text().await.unwrap());

    //             }
    //         },
    //         None => println!("Unable to get the genesis url")
    //     }
    // }

    pub async fn send_get_request(&self, endpoint:& String) -> Option<Response> {
        let mut url = self.base_url.clone();
        url.push_str(endpoint.as_str());
        // println!("In get request");
        let response_result = self.client.get(url).send().await;
        // let response_result = reqwest::blocking::get(url)?;
        match response_result {
            Ok(result) => {
                Some(result)
            },
            Err(_) => None
        }    
    }

    pub async fn get_block(&self, block_id : & String) -> Option<BlockDataModel> {
        let mut url = format!("/eth/v2/beacon/blocks/{}",block_id);
        println!("{}",&url);
        let response = self.send_get_request(&url).await;
        match response {
            Some(r) => {
                if r.status() == reqwest::StatusCode::OK {
                    // println!("{}", &r.text().await.unwrap());
                    match r.json::<BlockDataModel>().await {
                        Ok(block) => {
                            // println!("{}", text.finalized);
                            return Some(block);
                            // let block_data : BlockDataModel = serde_json::from_str(&text).unwrap();
                            // println!("{}",block_data.finalized);
                        },
                        Err(e) => {
                            println!("Unbale to parse {}",e.to_string());
                            return None;
                        }
                    }
                    
                } else {
                    println!("Unable to fetch data with error {}", r.text().await.unwrap());
                    return None;
                }
            },
            None => {
                println!("Unable to get block {} error", block_id);
                return None;
            }
        };
    }
    pub async fn get_slot_validators(&self, block_id : & String) -> Option<ValidatorSet> {
        let mut url = format!("/eth/v1/beacon/states/{}/committees?slot={}",block_id,block_id);
        let response = self.send_get_request(&url).await;
        match response {
            Some(r) => {
                if r.status() == reqwest::StatusCode::OK {
                    match r.json::<CommitteeResponse>().await {
                        Ok(validator_response) => {
                            let validator_set = ValidatorSet::new(validator_response);
                            return Some(validator_set);
                        },
                        Err(e) => {
                            println!("Unbale to parse {}",e.to_string());
                            return None;
                        }
                    }
                    
                } else {
                    println!("Unable to fetch data with error {}", r.text().await.unwrap());
                    return None;
                }
            },
            None => {
                println!("Unable to get block {} error", block_id);
                return None;
            }
        };
    }
    pub async fn get_participated_validators(&self, block_id : & String) -> Option<HashMap<String, bool>> {
        match self.get_block(block_id).await {
            Some(block) => {
                if block.finalized {
                    match self.get_slot_validators(block.get_slot()).await {
                        Some(validators) => {
                            let mut validator_check:HashMap<String, bool> = HashMap::new();
                            for attestations in block.get_attestations() {
                                let aggregation_bits_cpy = String::from(&attestations.aggregation_bits[2..]);
    
                                // println!("{}", &aggregation_bits_cpy);
                                // let mut decoded = [0;256];
                                let bytes: Vec<u8> = hex::decode(aggregation_bits_cpy.as_str()).expect("Failed to decode hexadecimal string");
                                let committee_index: &String = &attestations.data.index;
                                let commitee_members: &Vec<String> = validators.validators.get(committee_index).expect(format!("No commitee for the index {}", committee_index).as_str());
                                // println!("Bytes: {:?}", &bytes);
                                // println!("validator size {}", commitee_members.len());
                                for i in 0..commitee_members.len() {
                                    // println!("{}",i);
                                    let mut index : usize = (i)/8;
                                    let bit = (i as u64)%8;
                                    // println!("{} {} {}",format!("{}",bytes.get(index).expect(format!("Index out of bounds {}", index).as_str())),
                                    //         format!("{}",1<<&bit), format!("{}", ((bytes.get(index).expect(format!("Index out of bounds {}", index).as_str()) & (1<<bit)))));
                                    let validator_index = commitee_members.get(i).expect("always in range").clone();
                                    if ((bytes.get(index).expect(format!("Index out of bounds {}", index).as_str()) & (1<<bit)) != 0) {
                                        // println!("given member has attested {}", commitee_members[i]);
                                        validator_check.insert(validator_index, true);
                                    } else {
                                        if !(validator_check.contains_key(&validator_index)) {
                                            validator_check.insert(validator_index, false);
                                        }
                                        // println!("given member hasn't attested {}", i);
                                    }
                                }
                            }
                            return Some(validator_check);
                            // for validator in validator_check {
                            //     println!("{} {}", validator.0, validator.1);
                            // }
                        },
                        None => {
                            println!("Unable to find data for {}", block_id);
                            return None;
                        }
                    }
                } else {
                    println!("Block isn't finalized yet {}", block_id);
                    return None;
                }
            },
            None => {
                println!("Unable to find data for {}", block_id);
                return None;
            }
        };
    }

    pub async fn get_validators(&self, block_id : & String) -> Option<HashMap<String, String>> {
        let url = format!("/eth/v1/beacon/states/{}/validators",block_id);
        let response = self.send_get_request(&url).await;
        match response {
            Some(r) => {
                if r.status() == reqwest::StatusCode::OK {
                    match r.json::<ValidatorResponse>().await {
                        Ok(mut validator_response) => {
                            // let validator_set = ValidatorSet::new(validator_response);
                            return  Some(validator_response.validator_key_index_map());
                        },
                        Err(e) => {
                            println!("Unbale to parse {}",e.to_string());
                            return None;
                        }
                    }
                    
                } else {
                    println!("Unable to fetch data with error {}", r.text().await.unwrap());
                    return None;
                }
            },
            None => {
                println!("Unable to get block {} error", block_id);
                return None;
            }
        };
    }

    pub async fn process_current_epoch(&self, curr_epoch :u64) {
        let latest_block = curr_epoch*32;
        let current_block_str = format!("{}",latest_block);
        match self.get_validators(&current_block_str).await {
            Some(validators_key_map) => {
                for slot_index in latest_block..(latest_block+1) {

                    let slot_index_str = format!("{}",slot_index);
    
                    match self.get_participated_validators(&slot_index_str).await {
                        Some(validator_map) => {

                            for validator in validator_map  {
                                let validator_index = validator.0;
                                if validators_key_map.contains_key(&validator_index) {
                                    let validator_pubkey = validators_key_map.get(&validator_index).unwrap();
                                    if validator.1 {
                                        println!("Attested data for index {} {}",
                                                validator_index, validator_pubkey);
                                    } else {
                                        println!("Unattested data for index {} {}",
                                                validator_index, validator_pubkey);
                                    }
                                } else {
                                    println!("Validator data for index {}, epoch {}not found",
                                                validator_index, curr_epoch);
                                }
                            }
                        },
                        None => println!("Unable to retrieve map attested data with validator index")
                    };
                }
            },
            None => println!("Unable to get the validators from the API")
        }
    }
    pub async fn get_recent_5_epochs(&self) {
        let latest_block = format!("finalized");
        match self.get_block(&latest_block).await {
            Some(block) => {
                let slot_index : u64 = block.data.message.slot.parse().
                                            expect(format!("Cannot parse to int {}", block.data.message.slot).as_str());
                let curren_epoch = slot_index/32;
                println!("{} {}", slot_index, curren_epoch);
                for epoch in (curren_epoch)..(curren_epoch+1) {
                    self.process_current_epoch(epoch).await;
                }
            },
            None => println!("Unable to get latest finalized block")
        }
    }
}
//*/