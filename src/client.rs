pub mod APIClient;
pub mod BlockDataModel;
/*use reqwest::{Error, Client,};
use reqwest::StatusCode;

pub struct APIClient {
    base_url: String,
    client: Client,
}

impl APIClient {
    pub fn new(base_url: &str) -> Self {
        let client = Client::new();
        APIClient {
            base_url: base_url.to_string(),
            client,
        }
    }

    pub async fn send_get_request(&self, endpoint: &str) {
        // self.base_url = "".to_string();
        // endpoint = "eth/v2/beacon/blocks/finalized";
        let url = "https://smart-dark-patron.ethereum-sepolia.discover.quiknode.pro/faddff8c83bf08ff052b5272475c273afada1a79/eth/v2/beacon/blocks/finalized".to_string();
        let response1 = self.client.get(url).send().await.unwrap().text().await;
        println!("{:?}", response1);
        // if response1.status() == reqwest::StatusCode::OK {
        //     println!("{:?}", response1.text().await);
        // }
    }
}
 */