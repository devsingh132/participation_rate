use std::collections::{HashMap};
use mysql::*;
use mysql::prelude::*;

use crate::model::block_data_model::BlockDataModel;
use crate::model::committee_response_data::{CommitteeResponse};
use crate::model::committee_set::CommitteeSet;
use crate::model::validator_response::ValidatorResponse;
use crate::model::validator_data::ValidatorData;
use reqwest::{ Client, Response,};

const EPOCH_SLOTS : u64 = 32;
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
            attested bool,
            primary key (slot, epoch, pubkey)
        )")?;
        Ok(APIClient {
            base_url: base_url.to_string(),
            client,
            connection
        })
    }

    /**
     * Sends a get request to Beacon Node API.
     */
    async fn send_get_request(&mut self, endpoint:& String) -> Option<Response> {
        let mut url = self.base_url.clone();
        url.push_str(endpoint.as_str());
        let mut retry = 5;
        while retry > 0 {
            let response_result = self.client.get(url.clone()).send().await;
            retry-=1;
            match response_result {
                Ok(result) => {
                    if result.status() == reqwest::StatusCode::OK {
                        return Some(result);
                    } else if result.status() == reqwest::StatusCode::NOT_FOUND {
                        println!("Unable to find resource {}", result.text().await.unwrap());
                        return None;
                    }
                },
                Err(err) => {
                    println!("Error while sending get request {} error: {}", url, err.to_string());
                }
            }
            println!("Retrying the request {}", endpoint);
        }
        None
    }

    /**
     * Fetch the given block details.
     */
    async fn get_block(&mut self, block_id : & String) -> Option<BlockDataModel> {

        let url = format!("/eth/v2/beacon/blocks/{}",block_id);
        let response = self.send_get_request(&url).await?;
        match response.json::<BlockDataModel>().await {
            Ok(block) => {
                return Some(block);
            },
            Err(e) => {
                println!("Unbale to parse {}",e.to_string());
                return None;
            }
        }
    }

    async fn get_latest_finalized_block(&mut self) -> Option<BlockDataModel>{
        let block_id = format!("finalized");
        self.get_block(&block_id).await
    }

    /**
     * Get the committee for a particluar slot
     */
    async fn get_slot_committees(&mut self, block_id : & String) -> Option<CommitteeSet> {

        let url = format!("/eth/v1/beacon/states/{}/committees?slot={}",block_id,block_id);
        let response = self.send_get_request(&url).await?;

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
    }


    /**
     * Returns a Map of validator index and validator pubkey
     */
    async fn get_validators_pubkey(&mut self, block_id : & String) -> Option<HashMap<String, String>> {

        let url = format!("/eth/v1/beacon/states/{}/validators",block_id);
        let response = self.send_get_request(&url).await?;
        match response.json::<ValidatorResponse>().await {
            Ok(validator_response) => {
                return  Some(validator_response.validator_key_index_map());
            },
            Err(e) => {
                println!("Unbale to parse {}",e.to_string());
                return None;
            }
        }
    }

    /**
     * Determines which indexes attested the block.
     * Returns a Map of valiadator index and boolean denoting whether it attested or not
     */
    async fn get_participated_validators(&mut self, block_id : & String) -> Option<HashMap<String, bool>> {

        let block_data = self.get_block(block_id).await?;
        if block_data.finalized {

            let committees = self.get_slot_committees(block_data.get_slot()).await?;
            //Map storing whether the validator index and whether it attested or not.
            let mut validator_check:HashMap<String, bool> = HashMap::new();

            for attestations in block_data.get_attestations() {
                let aggregation_bits_cpy = String::from(&attestations.aggregation_bits[2..]);
                //Convert the hexstring to array of vector of u8, u8 is converted in little Edian we want bigedian format.
                let bytes = hex::decode(aggregation_bits_cpy.as_str()).unwrap();
                let committee_index: &String = &attestations.data.index;
                let commitee_members: &Vec<String> = committees.validators.get(committee_index)?;

                for i in 0..commitee_members.len() {
                    let byte_index = i/8;
                    let bit = (7 - (i as u64)%8)%8; //Little Edian hence convert to big edian
                    let validator_index = commitee_members.get(i)?.clone();
                    if ((bytes.get(byte_index)?) & (1<<bit)) != 0 {
                        validator_check.insert(validator_index, true);

                    } else {
                        if !(validator_check.contains_key(&validator_index)) {
                            validator_check.insert(validator_index, false);
                        }
                    }
                }
            }
            return Some(validator_check);
        } else {
            println!("Block isn't finalized yet {}", block_id);
            return None;
        }
    }

    /**
     * Determines the pubkey of the validators who attested the block and sends them to DB.
     */
    async fn process_current_block(&mut self, slot_index : u64, curr_epoch :u64) -> Option<()>{

        let mut validator_attested_arr : Vec<ValidatorData> = Vec::new();
            let slot_index_str = format!("{}",slot_index);
            let validator_attestation_map: HashMap<String, bool>;
            match self.get_participated_validators(&slot_index_str).await {
                Some(result) => {
                    validator_attestation_map = result;
                },
                None =>{ 
                    return None;
                }
            };
            let validators_key_map: HashMap<String, String>;
            match self.get_validators_pubkey(&slot_index_str).await {
                Some(result) => {
                    validators_key_map = result;
                },
                None =>{ 
                    return None;
                }
            };
            for (validator_index, is_attested) in validator_attestation_map.iter()  {

                if validators_key_map.contains_key(validator_index) {
                    let validator_pubkey = validators_key_map.get(validator_index).unwrap();
                    validator_attested_arr.push(ValidatorData::new(validator_pubkey.clone(), *is_attested));
                } else {
                    println!("Validator data for index {}, slot index {} not found",
                                validator_index, slot_index);
                }
            }
            self.insert_slot_validators_db(curr_epoch, slot_index, validator_attested_arr);

            return Some(());
    }

    /**
     * Process all the blocks in the current epoch.
     */
    async fn process_current_epoch(&mut self, curr_epoch :u64) -> Option<()> {
        println!("processing epoch {}", curr_epoch);
        let first_block = curr_epoch*32;
        let last_epoch_block = first_block + EPOCH_SLOTS;

        for slot_index in first_block..last_epoch_block {

            match self.process_current_block(slot_index, curr_epoch).await {
                Some(_) | None => continue
            }
        }
        Some(())
    }

    /**
     * Process the recent 5 indexes.
     */
    pub async fn get_recent_5_epochs(&mut self) -> Option<()> {
        println!("Started indexing 5 recent blocks");
        let block_data = self.get_latest_finalized_block().await?;
        // println!("Got bloc kdata");
        let slot_index : u64 = block_data.data.message.slot.parse().unwrap();
        let current_epoch = slot_index/32 ;
        let prev_5_epoch = current_epoch-5;
        // println!("{} {}", slot_index, current_epoch);
        for epoch in (prev_5_epoch)..(current_epoch) {
            self.process_current_epoch(epoch).await?;
        }
        Some(())
    }

    /**
     * Populates the DB with the indexed data.
     * Db is populated as epoch, slot, pubkey, attested/not
     */
    fn insert_slot_validators_db(&mut self, epoch : u64, slot : u64, validators : Vec<ValidatorData> ) -> Result<()>{
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

    /**
     * Start indexing new slots.
     */
    #[tokio::main]
    pub async fn index_new_slots(&mut self) {
        println!("Started indexing new slots");
        let mut last_finalized_epoch :u64 = 0;
        loop {
            let latest_block = format!("finalized");
            let block_data : BlockDataModel;
            match self.get_block(&latest_block).await {
                Some(data) => {
                    block_data = data;
                },
                None => continue
            }
            let slot_index : u64 = block_data.data.message.slot.parse().unwrap();
            let curr_epoch = slot_index/32 - 1;     //Last finalized epoch.
            if curr_epoch > last_finalized_epoch {
                match self.process_current_epoch(curr_epoch).await {
                    Some(_) => {
                        last_finalized_epoch = curr_epoch
                    },
                    None =>{
                        println!("Unable to process current epoch: {}", curr_epoch);
                    }
                }
            } else {
                println!("prev epoch {} <= current epoch {}", last_finalized_epoch, curr_epoch);
            }
            // Sleep for 1/2 epoch
            std::thread::sleep(std::time::Duration::from_secs(192));    //Timeout of 3.2 mins as processing the current block will also take time.
        }
    }
}
//*/