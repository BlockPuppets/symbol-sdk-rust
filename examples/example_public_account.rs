use std::str::FromStr;

use symbol_crypto_core::prelude::Signature;

use symbol_sdk::{H192, H200};
use symbol_sdk::account::PublicAccount;
use symbol_sdk::network::NetworkType;

fn main() {
    let data = "Symbol is wonderful";
    let public_key: &str = "758E6754F6B9C611F1DC78A2AAAAB756F500CD77232DD187E5EA3DA10E382A5B";

    let public_account = PublicAccount::<H192>::from_public_key(public_key, NetworkType::TEST_NET)
        .unwrap();
    println!("{}", public_account);

    let signature = Signature::from_str
        ("350974B30761F27AE755FCF86D7A79212419EF86671A23A4330E324DA6CE78946151400D0FDD4EFB3214DE450BA7A4839EB8F1F43C568237E3FCBBF49D3F6D02").unwrap();

    println!(
        "{}\n",
        public_account.verify_signature(data, signature).is_ok()
    );

    let public_key: &str = "4EFBA35DC3C6FB13ED323BA50611F44DE5CD1A34E514AF6B48341086B2B7680D";

    let public_account = PublicAccount::<H200>::from_public_key(public_key, NetworkType::TEST_NET)
        .unwrap();
    println!("{}", public_account);

    let signature = Signature::from_str
        ("C364CAC8CDA90C63A3669ABAF7A0D96A527B93BC23CBBF5E7DF9F86056D8902739841FD47FFDC1F226AFC4FB9FAB429E96D491969DDCFE018679F90B66CD7A09").unwrap();

    println!(
        "{}",
        public_account.verify_signature(data, signature).is_ok()
    );
}
