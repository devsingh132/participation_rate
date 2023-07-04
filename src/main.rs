use crate::client::APIClient;
pub mod client;
mod model;
use mysql::*;
use mysql::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>>{
    // let apiclient = APIClient::new();    
    let mut client = APIClient::APIClient::new("https://smart-dark-patron.ethereum-sepolia.discover.quiknode.pro/faddff8c83bf08ff052b5272475c273afada1a79").await;
    // let url = "https://smart-dark-patron.ethereum-sepolia.discover.quiknode.pro/faddff8c83bf08ff052b5272475c273afada1a79/eth/v1/beacon/blocks/2714880/attestations".to_string();
        // let response1 = client.get(&url).send().await.unwrap().text().await;
        // println!("{:?}", response1);
    let block_id = String::from("finalized");
    // let block =
    // client.get_participated_validators(&block_id).await;
    // client.get_genesis().await;
    // client.get_recent_5_epochs().await;
    // match block {
    //     Some(p) => {
    //         // println!("{}", p.get_slot());
    //         client.get_slot_validators(p.get_slot()).await;
    //     },
    //     None => {
    //         println!("No data");
    //     }
    // }
    // let hex = "0xfffffffeffffff7f";
    // let hex_string = "fffffff7f"; // Example hexadecimal string

    // Convert the hexadecimal string to a Vec<u8>
    // let bytes = hex::decode(hex_string).expect("Failed to decode hexadecimal string");

    // Print the resulting Vec<u8>
    // println!("Bytes: {:?}", bytes);
    let url = "mysql://root:password@localhost:3307/pp_rate";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    Ok(())

}
