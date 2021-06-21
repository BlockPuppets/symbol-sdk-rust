use symbol_sdk::account::Address;
use symbol_sdk::network::NetworkType;

fn main() {
    let public_key = "C72DBC5B76E18E291A07CBD20E1A112F05AEEB5625196F42B3DC9978671BCD12";

    let address_sym = Address::from_raw("TBJSRVRYE2EPT33F5PHDRBLZCDMKMQBRW6CLYFY").unwrap();
    println!("Sym from_raw: {}\n", address_sym.prettify());

    let address_sym = Address::from_public_key(public_key, NetworkType::TEST_NET).unwrap();
    println!("Sym from_public_key: {}\n", address_sym);

    let address_sym =
        Address::from_encoded("985328D6382688F9EF65EBCE38857910D8A64031B784BC17").unwrap();
    println!("Sym from_encoded: {}", address_sym);
}
