use rand::RngCore;

pub fn random_bytes<const COUNT: usize>() -> [u8; COUNT] {
    let mut rng = rand::thread_rng();
    let mut buf = [0u8; COUNT];
    rng.try_fill_bytes(&mut buf).unwrap();
    buf
}