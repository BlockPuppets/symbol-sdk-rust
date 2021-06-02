use symbol_sdk::{Client, Retry, H192};
use symbol_sdk::account::{PublicAccount, Address};

#[tokio::main]
async fn main() {
    let client = Client::from_url("http://61.27.29.85:3000", Retry::default())
        .await
        .unwrap();

    println!("Network_type: {}", client.network_type);
    println!("Generation_hash: {:X}", client.generation_hash);

    match client.block_routes().get_block_by_height(1000).await {
        Ok(block_info) => {
            println!("{}", block_info)
        }
        Err(err) => {
            println!("{}", err)
        }
    };

    let beneficiary_address = Address::<H192>::from_public_key("0a8a89c6e3602a1d858b11e0942918c5a19cac305f7adfd3768ad499e58cc37e", client.network_type).unwrap();

    match client.block_routes().search_blocks(None, Some(beneficiary_address), None, None, None, None, None).await {
        Ok(block_info) => {
            println!("{:?}", block_info)
        }
        Err(err) => {
            println!("{}", err)
        }
    };
}
