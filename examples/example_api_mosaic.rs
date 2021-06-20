use symbol_sdk::account::Address;
use symbol_sdk::mosaic::MosaicId;
use symbol_sdk::{Client, MosaicSearchCriteria, Retry};

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

    let mosaic_id = MosaicId::from_hex("01F3E8CED4AD45A3").unwrap();
    match client.mosaic_routes().get_mosaic(mosaic_id).await {
        Ok(mosaic_info) => {
            println!("{}\n", mosaic_info)
        }
        Err(err) => {
            println!("{}", err)
        }
    };

    let mosaic_ids = vec![
        MosaicId::from_hex("01F3E8CED4AD45A3").unwrap(),
        MosaicId::from_hex("7D4C02F219E72CE9").unwrap(),
    ];

    match client.mosaic_routes().get_mosaics(mosaic_ids).await {
        Ok(mosaics) => mosaics
            .into_iter()
            .for_each(|mosaic_info| println!("{}\n", mosaic_info)),
        Err(err) => {
            println!("{}", err)
        }
    };

    match client.mosaic_routes().get_mosaic_merkle(mosaic_id).await {
        Ok(mosaic_info) => {
            println!("{}\n", mosaic_info)
        }
        Err(err) => {
            println!("{}", err)
        }
    };

    let criteria = MosaicSearchCriteria {
        owner_address: Some(Address::from_raw("TCMIVBYRZH7KWYNHGZFX2O2SG5XNLDVMZZ45INQ").unwrap()),
        param: None,
    };

    match client.mosaic_routes().search_mosaics(Some(criteria)).await {
        Ok(mosaics) => mosaics
            .into_iter()
            .for_each(|mosaic_info| println!("{}\n", mosaic_info)),
        Err(err) => {
            println!("{}", err)
        }
    };
}
