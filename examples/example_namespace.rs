use symbol_sdk::namespace::NamespaceId;

fn main() {
    let namespace_id = NamespaceId::create_from_name("nem.subnem");

    match namespace_id {
        Ok(id) => {
            println!("{}", id);
        }
        Err(err) => {
            panic!("{:?}", err)
        }
    }
}