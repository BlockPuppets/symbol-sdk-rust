use symbol_sdk::{H192, H200, KpNis1, KpSym};
use symbol_sdk::account::Account;
use symbol_sdk::network::NetworkType;

fn main() {
    let account_one = Account::<KpNis1, H200>::random(NetworkType::TEST_NET);
    println!("network_type:\t{}", account_one);

    let private_key: &str = "75027D85CE92E2C469297F4C91E4E88AE03868A91B23C835AEF7C5EFDAD0DBDB";
    let account_two =
        Account::<KpSym, H192>::from_hex_private_key(private_key, NetworkType::TEST_NET).unwrap();
    println!("account_two: {}", account_two);

    let private_key: &str = "75027D85CE92E2C469297F4C91E4E88AE03868A91B23C835AEF7C5EFDAD0DBDB";
    let account_two =
        Account::<KpNis1, H200>::from_hex_private_key(private_key, NetworkType::TEST_NET).unwrap();
    println!("account_two: {}", account_two);

    let data = "Symbol is wonderful";
    let signature = account_two.sign_data(data).unwrap();
    println!("signature: {:X}", signature);

    let verify = account_two.verify_signature(data.as_ref(), signature);
    println!("Verify: {}\n", verify.is_ok());

    let (account_three, mnemonic) =
        Account::<KpSym, H192>::create_with_mnemonic("any_password", NetworkType::TEST_NET).unwrap();
    println!("Account Three: {}", account_three);
    println!("Mnemonic Three: {}\n", mnemonic);

    let account_four =
        Account::<KpSym, H192>::from_mnemonic(&mnemonic, "any_password", NetworkType::TEST_NET).unwrap();
    println!("Account Four: {}", account_four);
}
