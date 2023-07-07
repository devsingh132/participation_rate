extern crate dotenv;
use dotenv::dotenv;
use crate::client::{APIClient, APIServer};
pub mod client;
mod model;
use mysql::*;
use mysql::prelude::*;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();
    let mut base = std::env::var("ETH_BASE_URL")?;

    let mut client = APIClient::APIClient::new(base.as_str())?;
    match client.get_recent_5_epochs().await {
        Some(_) => println!("passed"),
        None => println!("stoopped")
    }
    let api_server = APIServer::APIServer::new().unwrap();
    api_server.start_server()?;
    Ok(())

}
