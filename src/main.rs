extern crate dotenv;
use dotenv::dotenv;
use crate::client::{apiclient, apiserver};
pub mod client;
mod model;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();
    let base = std::env::var("ETH_BASE_URL").unwrap();
    let mut client = apiclient::APIClient::new(base.as_str()).unwrap();

    let thread1 = std::thread::spawn(move || {
        client.index_new_slots();
    });
    
    let thread2 = std::thread::spawn(move || {
        let api_server = apiserver::APIServer::new().unwrap();
        api_server.start_server().unwrap();
    });
    
    let base = std::env::var("ETH_BASE_URL").unwrap();
    let mut client = apiclient::APIClient::new(base.as_str()).unwrap();
    match client.get_recent_5_epochs().await {
        Some(_) => {
            println!("Sucessfully indexed last 5 epoch");
        },
        None => println!("Unable to index last 5 epochs")
    }
    thread1.join().unwrap();
    thread2.join().unwrap();
    Ok(())

}
