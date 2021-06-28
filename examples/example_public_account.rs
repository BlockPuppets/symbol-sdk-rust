use std::str::FromStr;

use symbol_crypto_core::prelude::Signature;

use symbol_sdk::account::PublicAccount;
use symbol_sdk::network::NetworkType;

fn main() {
    let data = "Symbol is wonderful";
    let public_key: &str = "758E6754F6B9C611F1DC78A2AAAAB756F500CD77232DD187E5EA3DA10E382A5B";

    let public_account = PublicAccount::from_public_key(public_key, NetworkType::TestNet)
        .unwrap();
    println!("{}", public_account);

    let signature = Signature::from_str
        ("350974B30761F27AE755FCF86D7A79212419EF86671A23A4330E324DA6CE78946151400D0FDD4EFB3214DE450BA7A4839EB8F1F43C568237E3FCBBF49D3F6D02").unwrap();

    println!(
        "{}\n",
        public_account.verify_signature(data, signature).is_ok()
    );
}
