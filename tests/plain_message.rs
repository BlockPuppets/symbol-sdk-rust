use symbol_sdk::message::{Message, MessageType, PlainMessage};

#[test]
fn test_default_plain_message() {
    let plain_message = PlainMessage::default();

    assert_eq!(plain_message.payload, "");
    assert_eq!(plain_message.message_type(), MessageType::PlainMessageType);
}

#[test]
fn test_create_plain_message() {
    let payload = "test-message";

    let plain_message = PlainMessage::create(payload);
    assert_eq!(plain_message.payload(), payload);
    assert_eq!(plain_message.to_dto(), "00746573742D6D657373616765");
}

#[test]
fn test_create_plain_message_with_static_method() {
    let payload = "746573742D6D657373616765";

    let plain_message = PlainMessage::from_payload(payload).unwrap();

    assert_eq!(plain_message.payload, "test-message");
    assert_eq!(plain_message.to_dto(), "00746573742D6D657373616765");
}