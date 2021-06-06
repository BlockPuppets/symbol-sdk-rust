use symbol_sdk::mosaic::{Mosaic, MosaicId};

fn main() {
    let id = MosaicId::from_hex("85BBEA6CC462B244").unwrap();

    let mosaic_absolute = Mosaic::create(id, 10_000);
    match mosaic_absolute {
        Ok(mosaic) => {
            println!("{:?}", mosaic)
        }
        Err(err) => {
            panic!("{:?}", err)
        }
    }

    let mosaic_relative = Mosaic::create_relative(id, 10_000, 6);
    match mosaic_relative {
        Ok(mosaic) => {
            println!("{:?}", mosaic)
        }
        Err(err) => {
            panic!("{:?}", err)
        }
    }
}
