use symbol_sdk::{Client, Retry};

#[tokio::main]
async fn main() {
    let client = Client::from_url("http://ngl-dual-101.testnet.symboldev.network:3000", Retry::default())
        .await
        .unwrap();

    println!("Network_type: {}", client.network_type);
    println!("Generation_hash: {:X}", client.generation_hash);

    match client.network_routes().get_network_name().await {
        Ok(network_name) => {
            println!("{}", network_name)
        }
        Err(err) => {
            println!("{}", err)
        }
    };

    match client.network_routes().get_network_properties().await {
        Ok(properties) => {
            println!("{}", properties)
        }
        Err(err) => {
            println!("{}", err)
        }
    };
}
