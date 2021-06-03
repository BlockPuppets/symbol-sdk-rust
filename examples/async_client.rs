use symbol_sdk::{Client, Retry};

#[tokio::main]
async fn main() {
    let client = Client::from_url(
        "http://ngl-dual-101.testnet.symboldev.network:3000",
        Retry::default(),
    )
    .await
    .unwrap();

    println!("Network_type: {}", client.network_type);
    println!("Generation_hash: {:X}", client.generation_hash);

    match client.block_routes().get_block_by_height(2).await {
        Ok(block_info) => {
            println!("{}", block_info)
        }
        Err(err) => {
            println!("{}", err)
        }
    };
}
