use symbol_sdk::message::PlainMessage;

fn main() {
    let plain_message = PlainMessage::default();
    println!("{:?}", plain_message);

    let raw_message = "test message from rust sdk";
    let hex_message = "74657374206D6573736167652066726F6D20727573742073646B";

    let plain_message = PlainMessage::create(raw_message);
    println!("{:?}", plain_message);

    let plain_message = PlainMessage::from_payload(hex_message).unwrap();
    println!("{:?}", plain_message);
}
