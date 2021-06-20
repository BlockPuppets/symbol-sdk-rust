use symbol_sdk::{Client, Retry};

#[tokio::main]
async fn main() {
    let client = Client::from_url("http://ngl-dual-101.testnet.symboldev.network:3000", Retry::default())
        .await
        .unwrap();

    println!("Network_type: {}", client.network_type);
    println!("Generation_hash: {:X}", client.generation_hash);

    match client.node_routes().get_node_health().await {
        Ok(chain_info) => {
            println!("{}", chain_info)
        }
        Err(err) => {
            println!("{}", err)
        }
    };

    match client.node_routes().get_node_info().await {
        Ok(node_info) => {
            println!("{}", node_info)
        }
        Err(err) => {
            println!("{}", err)
        }
    };

    match client.node_routes().get_node_peers().await {
        Ok(nodes) => nodes
            .into_iter()
            .for_each(|node_info| println!("{}\n", node_info)),
        Err(err) => {
            println!("{}", err)
        }
    };

    match client.node_routes().get_storage_info().await {
        Ok(storage_info) => {
            println!("{}", storage_info)
        }
        Err(err) => {
            println!("{}", err)
        }
    };

    match client.node_routes().get_node_time().await {
        Ok(storage_info) => {
            println!("{:?}", storage_info)
        }
        Err(err) => {
            println!("{}", err)
        }
    };

    match client.node_routes().get_server_info().await {
        Ok(server_info) => {
            println!("{:?}", server_info)
        }
        Err(err) => {
            println!("{}", err)
        }
    };

    match client.node_routes().get_unlocked_accounts().await {
        Ok(accounts) => accounts
            .into_iter()
            .for_each(|account| println!("{}\n", account)),
        Err(err) => {
            println!("{}", err)
        }
    };
}
