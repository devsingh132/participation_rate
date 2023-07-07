use std::collections::{HashMap};
use mysql::*;
use mysql::prelude::*;

use crate::client::BlockDataModel::BlockDataModel;
use crate::model::CommitteeResponseData::{CommitteeResponse};
use crate::model::CommitteeSet::CommitteeSet;
use crate::model::ValidatorResponse::ValidatorResponse;
use crate::model::ValidatorData::ValidatorData;
use reqwest::{ Client, Response,};

const EPOCH_SLOTS : u64 = 1;
pub struct APIClient {
    base_url: String,
    client: Client,
    connection : PooledConn
}

impl APIClient {
    pub fn new(base_url: &str) -> Result<Self> {
        let client = Client::new();
        let url = std::env::var("MYSQL_URL").unwrap();
        let mut connection = Pool::new(url.as_str())?.get_conn()?;
        connection.query_drop(r"DROP TABLE IF EXISTS attestations")?;
        connection.query_drop(r"CREATE TABLE attestations (
            slot BIGINT,
            epoch BIGINT,
            pubkey varchar(256),
            attested bool
        )")?;
        Ok(APIClient {
            base_url: base_url.to_string(),
            client,
            connection
        })
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

    pub async fn send_get_request(&mut self, endpoint:& String) -> Option<Response> {
        let mut url = self.base_url.clone();
        url.push_str(endpoint.as_str());
        // println!("In get request");
        // let response_result = self.client.get(url).send().await?;
        // let response_result = reqwest::blocking::get(url).send();
        // Ok(response_result)
        let response_result = self.client.get(url).send().await;
        // println!("Got request {}", endpoint);
        match response_result {
            Ok(result) => {
                Some(result)
            },
            Err(_) => None
        }
        // return Ok(response_result);
    }

    pub async fn get_block(&mut self, block_id : & String) -> Option<BlockDataModel> {
        let mut url = format!("/eth/v2/beacon/blocks/{}",block_id);
        let response = self.send_get_request(&url).await?;
        // return Ok(response.json::<BlockDataModel>().await?);
        if response.status() == reqwest::StatusCode::OK {
            // println!("{}", &response.text().await.unwrap());
            match response.json::<BlockDataModel>().await {
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
            println!("Unable to fetch data with error {}", response.text().await.unwrap());
            return None;
        }
        /*
        // match response {
        //     Some(r) => {
        //         if r.status() == reqwest::StatusCode::OK {
        //             // println!("{}", &r.text().await.unwrap());
        //             match r.json::<BlockDataModel>().await {
        //                 Ok(block) => {
        //                     // println!("{}", text.finalized);
        //                     return Some(block);
        //                     // let block_data : BlockDataModel = serde_json::from_str(&text).unwrap();
        //                     // println!("{}",block_data.finalized);
        //                 },
        //                 Err(e) => {
        //                     println!("Unbale to parse {}",e.to_string());
        //                     return None;
        //                 }
        //             }
                    
        //         } else {
        //             println!("Unable to fetch data with error {}", r.text().await.unwrap());
        //             return None;
        //         }
        //     },
        //     None => {
        //         println!("Unable to get block {} error", block_id);
        //         return None;
        //     }
        // };*/
    }
    pub async fn get_slot_committees(&mut self, block_id : & String) -> Option<CommitteeSet> {

        let mut url = format!("/eth/v1/beacon/states/{}/committees?slot={}",block_id,block_id);
        let response = self.send_get_request(&url).await?;

        if response.status() == reqwest::StatusCode::OK {
            match response.json::<CommitteeResponse>().await {
                Ok(validator_response) => {
                    let validator_set = CommitteeSet::new(validator_response);
                    return Some(validator_set);
                },
                Err(e) => {
                    println!("Unbale to parse committees: {}",e.to_string());
                    return None;
                }
            }
        } else {
            println!("Unable to fetch data with error {}", response.text().await.unwrap());
            return None;
        }
        /*
        match response {
            Some(r) => {
            },
            None => {
                println!("Unable to get block {} error", block_id);
                return None;
            }
        };
        */
    }


    /**
     * Returns a Map of validator index and validator pubkey
     */
    pub async fn get_validators_pubkey(&mut self, block_id : & String) -> Option<HashMap<String, String>> {

        let url = format!("/eth/v1/beacon/states/{}/validators",block_id);
        let response = self.send_get_request(&url).await?;
        if response.status() == reqwest::StatusCode::OK {
            match response.json::<ValidatorResponse>().await {
                Ok(mut validator_response) => {
                    return  Some(validator_response.validator_key_index_map());
                },
                Err(e) => {
                    println!("Unbale to parse {}",e.to_string());
                    return None;
                }
            }
        } else {
            println!("Unable to fetch data with error {}", response.text().await.unwrap());
            return None;
        }
    }

    /**
     * Returns a Map of valiadator index and boolean denoting whether it attested or not
     */
    pub async fn get_participated_validators(&mut self, block_id : & String) -> Option<HashMap<String, bool>> {

        let block_data = self.get_block(block_id).await?;
        if block_data.finalized {

            let committees = self.get_slot_committees(block_data.get_slot()).await?;
            //Map storing whether the validator index and whether it attested or not.
            let mut validator_check:HashMap<String, bool> = HashMap::new();
            let mut count = 0;
            println!("sep");
            for attestations in block_data.get_attestations() {
                let aggregation_bits_cpy = String::from(&attestations.aggregation_bits[2..]);
                //Convert the hexstring to array of vector of u8, u8 is of little Edian.
                let bytes = hex::decode(aggregation_bits_cpy.as_str()).unwrap();
                // println!("{:?}", bytes);
                let committee_index: &String = &attestations.data.index;
                println!("{}", committee_index);
                let commitee_members: &Vec<String> = committees.validators.get(committee_index)?;
                count = count + commitee_members.len();
                for i in 0..commitee_members.len() {
                    let byte_index = i/8;
                    let bit = (7 - (i as u64)%8)%8; //Little Edian hence convert to big edian
                    let validator_index = commitee_members.get(i)?.clone();
                    if ((bytes.get(byte_index)?) & (1<<bit)) != 0 {
                        validator_check.insert(validator_index, true);

                    } else {
                        // println!("Missed {}",i);
                        if !(validator_check.contains_key(&validator_index)) {
                            validator_check.insert(validator_index, false);
                        }
                    }
                }
            }
            println!("count is {} {} {}",block_id, count, validator_check.len());
            // if count != validator_check.len() {
            // }
            return Some(validator_check);
        } else {
            println!("Block isn't finalized yet {}", block_id);
            return None;
        }
        /*
            match self.get_slot_committees(block_data.get_slot()).await {
                Some(committees) => {
                    // for validator in validator_check {
                    //     println!("{} {}", validator.0, validator.1);
                    // }
                },
                None => {
                    println!("Unable to find data for {}", block_id);
                    return None;
                }
            }

        match self.get_block(block_id).await {
            Some(block) => {
            },
            None => {
                println!("Unable to find data for {}", block_id);
                return None;
            }
        };
        */
    }

    pub async fn process_current_epoch(&mut self, curr_epoch :u64) -> Option<()> {
        let first_block = curr_epoch*32;
        let last_epoch_block = first_block + EPOCH_SLOTS;
        let current_block_str = format!("{}",first_block);
        
        // validator_attested_arr.push()
        for slot_index in first_block..last_epoch_block {
            
            let mut validator_attested_arr : Vec<ValidatorData> = Vec::new();
            let slot_index_str = format!("{}",slot_index);
            let mut validator_attestation_map: HashMap<String, bool> = Default::default();
            match self.get_participated_validators(&slot_index_str).await {
                Some(result) => {
                    validator_attestation_map = result;
                },
                None => {
                    continue;
                }
            };
            let mut validators_key_map: HashMap<String, String> = Default::default();
            match self.get_validators_pubkey(&slot_index_str).await {
                Some(result) => {
                    validators_key_map = result;
                },
                None => {
                    continue;
                }
            };
            for (validator_index, is_attested) in validator_attestation_map.iter()  {

                if validators_key_map.contains_key(validator_index) {
                    let validator_pubkey = validators_key_map.get(validator_index).unwrap();
                    validator_attested_arr.push(ValidatorData::new(validator_pubkey.clone(), *is_attested));
                    // if *is_attested {
                    //     // println!("Attested data for index {} {}",
                    //     //         validator_index, validator_pubkey);
                    // } else {
                    //     println!("Unattested data for index {} {}",
                    //             validator_index, validator_pubkey);
                    // }
                } else {
                    println!("Validator data for index {}, epoch {}not found",
                                validator_index, curr_epoch);
                }
            }
            self.insert_slot_validators_db(curr_epoch, slot_index, validator_attested_arr);
        }
        Some(())
        /*
        match self.get_participated_validators(&slot_index_str).await {
            Some(validator_attestation_map) => {

            },
            None => println!("Unable to retrieve map attested data with validator index")
        };
        match self.get_validators_pubkey(&current_block_str).await {
            Some(validators_key_map) => {
            },
            None => println!("Unable to get the validators from the API")
        }
         */
    }
    pub async fn get_recent_5_epochs(&mut self) -> Option<()> {
        let latest_block = format!("finalized");
        // executor::block_on(f);
        let block_data = self.get_block(&latest_block).await?;
        // println!("Got bloc kdata");
        let slot_index : u64 = block_data.data.message.slot.parse().unwrap();
        let current_epoch = slot_index/32 + 1;
        let prev_5_epoch = current_epoch-1;
        // println!("{} {}", slot_index, current_epoch);
        for epoch in (prev_5_epoch)..(current_epoch) {
            self.process_current_epoch(epoch).await?;
        }
        Some(())
        /*
        match self.get_block(&latest_block).await {
            Some(block) => {
            },
            None => println!("Unable to get latest finalized block")
        }*/
    }

    pub fn insert_slot_validators_db(&mut self, epoch : u64, slot : u64, validators : Vec<ValidatorData> ) -> Result<()>{
        println!("{}", validators.len());
        self.connection.exec_batch(r"INSERT INTO attestations (epoch, slot, pubkey, attested)
        VALUES (:epoch, :slot, :pubkey, :attested)", 
        validators.iter().map(|p| params! {
            "epoch" => epoch,
            "slot" => slot,
            "pubkey" => p.pubkey.clone(),
            "attested" => p.is_attested
        }))?;
        Ok(())
    }
}
//*/