use symbol_sdk::{Client, Retry,H192, H256};
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

    match client.block_routes().get_block_by_height(1).await {
        Ok(block_info) => {
            println!("{}", block_info)
        }
        Err(err) => {
            println!("{}", err)
        }
    };

    // if feature = "nis1" hash type must be specified.
    // H192 = symbol.
    // H200 = nis1.
    let beneficiary_address = Address::<H192>::from_raw("TBGMAET6V4Q6CKO5R44C25UUPCAUEXES4QVSKXY").unwrap();

    let criteria = BlockSearchCriteria{
        signer_public_key: None,
        beneficiary_address: Some(beneficiary_address),
        order_by: None
    };

    match client.block_routes().search_blocks(Some(criteria), None, None, None, None).await {
        Ok(block_info) => {
           block_info.iter().for_each(|info|  println!("{}", info))
        }
        Err(err) => {
            println!("{}", err)
        }
    };


    let hash = H256::from_str("6FADDE5DBE9B77DFF674439D5C2F341D2E26F12F4FE5E47470100CD2A2DF6563").unwrap();

    match client.block_routes().get_merkle_receipts(126, hash).await {
        Ok(block_info) => {
            println!("{}", block_info)
        }
        Err(err) => {
            println!("{}", err)
        }
    };
}
