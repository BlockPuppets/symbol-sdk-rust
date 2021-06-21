use symbol_sdk::account::Account;
use symbol_sdk::message::EncryptedMessage;
use symbol_sdk::network::NetworkType;

fn main() {
    let network_type = NetworkType::TEST_NET;
    let sender = Account::from_hex_private_key(
        "2602F4236B199B3DF762B2AAB46FC3B77D8DDB214F0B62538D3827576C46C108",
        network_type,
    )
        .unwrap();

    let recipient = Account::from_hex_private_key(
        "B72F2950498111BADF276D6D9D5E345F04E0D5C9B8342DA983C3395B4CF18F08",
        network_type,
    )
        .unwrap();

    let encrypted_message = EncryptedMessage::create(
        b"746573742D6D657373616765",
        &sender.private_key_to_hex(),
        &recipient.public_key_to_hex(),
    )
        .unwrap();
    println!("{:?}", encrypted_message);

    let decrypt_message = encrypted_message
        .decrypt(&recipient.private_key_to_hex(), &sender.public_key_to_hex())
        .unwrap();
    println!("{:?}", decrypt_message);

    // from Account objets

    let encrypted_message = sender.encrypt_message("test-message", recipient.public_account);
    println!("{:?}", encrypted_message);

    let decrypt_message =
        recipient.decrypt_message(&encrypted_message.unwrap(), sender.public_account);
    println!("{:?}", decrypt_message.unwrap());
}
