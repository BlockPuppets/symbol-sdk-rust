use symbol_sdk::{Client, Retry, H256};
use symbol_sdk::account::{PublicAccount, Address};
use symbol_sdk::block_search_criteria::BlockSearchCriteria;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let client = Client::from_url("http://ngl-dual-101.testnet.symboldev.network:3000", Retry::default())
        .await
        .unwrap();

    println!("Network_type: {}", client.network_type);
    println!("Generation_hash: {:X}", client.generation_hash);

    match client.chain_routes().get_chain_info().await {
        Ok(chain_info) => {
            println!("{}", chain_info)
        }
        Err(err) => {
            println!("{}", err)
        }
    };
}
