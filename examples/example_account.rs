use symbol_sdk::account::Account;
use symbol_sdk::network::NetworkType;

fn main() {
    let account_one = Account::random(NetworkType::TestNet);
    println!("network_type:\t{}", account_one);

    let private_key: &str = "75027D85CE92E2C469297F4C91E4E88AE03868A91B23C835AEF7C5EFDAD0DBDB";
    let account_two = Account::from_hex_private_key(private_key, NetworkType::TestNet).unwrap();
    println!("account_two: {}", account_two);

    let data = "Symbol is wonderful";
    let signature = account_two.sign_data(data).unwrap();
    println!("signature: {:X}", signature);

    let verify = account_two.verify_signature(data.as_ref(), signature);
    println!("Verify: {}\n", verify.is_ok());

    let (account_three, mnemonic) =
        Account::create_with_mnemonic("any_password", NetworkType::TestNet).unwrap();
    println!("Account Three: {}", account_three);
    println!("Mnemonic Three: {}\n", mnemonic);

    let account_five =
        Account::from_mnemonic(&mnemonic, "any_password", NetworkType::TestNet).unwrap();
    println!("Account Four: {}", account_five);
}
